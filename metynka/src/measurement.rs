use std::process::{Command, Stdio};

use std::path::Path;

use std::fs::{self, OpenOptions, File};

use std::io::{self, Write};

use std::any::{Any};

use std::fmt::Debug;

use strfmt::strfmt;

use std::collections::HashMap;

use std::{thread, time};

use crate::util::ts::{Dt};
use crate::util::template_formater::tuple_formater;
use metynka::{TomlConfig, Influx, TemplateSensors, Sensor};

use crate::various;


#[derive(Debug)]
struct InfluxCall {
    uri_write: String,
    uri_query: String,
    auth: String,
    accept: String,
    content: String,
}


#[derive(Debug)]
struct PreRecord {
    key: String,
    measurement: String,

    value: String,
    id: String,

    machine: String,
    carrier: String,
    valid: String,
    
    host: String,
    ts: u64,
}


impl PreRecord {
    fn new(config: &TomlConfig,
           key: &str,
           ts: u64,
           value: f64,
           id: &str) -> PreRecord {
        
        // HERE I CAN put some TEST's to handle ERROR / raise WARNING
        PreRecord {
            key: key.to_string(),
            ts: ts,
            value: value.to_string(),
            id: id.to_string(),
            measurement: config.metrics[key].measurement.to_string(),
            host: config.host.to_string(),
            ..PreRecord::default()
        }
    }


    // maybe set default empty "" for generic verify ?
    fn default() -> PreRecord {
        PreRecord {
            key: "KEY".to_string(),
            ts: 0,
            value: "0".to_string(),
            
            id: "ID".to_string(),
            measurement: "MEASUREMENT".to_string(),
            host: "HOST".to_string(),
            
            machine: "MACHINE".to_string(),
            carrier: "CARRIER".to_string(),
            valid: "false".to_string(),
        }
    }
}


// compare STRUCT's via contains
impl PartialEq for PreRecord
{
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}


/*
debug_vector(result_list,
             "\n#RECORDS: not SAVED into BACKUP !!! \n{r}",
             "r",
);
*/
fn _debug_vector<T: Any + Debug>(vec: &Vec<T>,
                                 template: &str,
                                 key: &str) {
    
    let template = String::from(template);

    let hash_map = HashMap::from([
        (key.to_string(),
         format!("{:#?}", vec),
        )
    ]);

    println!("{}", strfmt(&template, &hash_map).unwrap());
}


fn trim_quotes(string: &str) -> Option<f64> {
    /*
    match str::replace(string, "\"", "") // "string"
        .trim()
        .parse::<f64>() {
    */

    let trim_chars = ['"', '\''];
    
    match string
        //.trim_matches(|p| p == '"' || p == '\'' )
        .trim_matches(|p| p == trim_chars[0] || p == trim_chars[1] ) // 'char'
        .parse::<f64>() {
            Ok(number) => Some(number),

            Err(why) => {
                eprintln!("\nPOINTER_value_TRIM: <{:?}> not succeded\nTRIM_CHARS: {:?}\nREASON>>> {:?}",
                          string,
                          trim_chars,
                          why,
                );
                
                None
            }
        } 
}


// https://doc.rust-lang.org/std/any/index.html
// https://doc.rust-lang.org/std/any/trait.Any.html
// DOUCIT + POCHOPIT zpusob volani
fn verify_pointer_type<T: Any + Debug>(value: &T) -> Option<f64> {
    let value_any = value as &dyn Any;

    let value = match value_any.downcast_ref::<String>() {
        Some(as_string) => {             

            // JSON VALUE
            match as_string.parse::<f64>() {
                // not f64 -> TRIM " 
                Err(_why) => {
                    trim_quotes(as_string)
                },
                // is f64 
                Ok(number) => {
                    Some(number)
                },
            }
        }
        
        None => {
            println!("POINTER_TYPE_ANY: {:?}", value);

            None
        }
    };

    value
}


fn open_file_to_append(today_file_name: &Path) -> Result<File, io::Error> {
    match OpenOptions::new()
        .write(true)
        .append(true)
        .open(&today_file_name)
    {
        Ok(file) => Ok(file),

        // FAIL TO OPEN FOR WRITE
        Err(why) => {
            eprintln!("\nERROR >> FILE WRITE permission: {}\nREASON: >>> {}",
                      &today_file_name.display(),
                      why);

            Err(why)
        },
    }
}


fn create_new_dir(config: &TomlConfig,
                  full_path: &Path,
                  measurement: &String) {

    fs::create_dir_all(&full_path).unwrap_or_else(|err| {
        let print_formated = tuple_formater(&"\nERROR >> METRIC <{m}> failed to create BACKUP DIR: {d}\nREASON: >>> {e}".to_string(),
                                            &vec![
                                                ("m", &measurement),
                                                ("d", &config.work_dir),
                                                ("e", &err.to_string())
                                            ],
                                            config.flag.debug_template_formater
        );
        
        println!("{}", print_formated);
        eprintln!("{}", print_formated);
        
    });
}


fn create_new_file(today_file_name: &Path) -> Result<File, io::Error> {
    match File::create(&today_file_name) { // LEARN TO write TEST for this
        Err(why) => {
            eprintln!("\nEXIT: COULD NOT CREATE {}\nREASON: >>> {}",
                      &today_file_name.display(),
                      why);

            Err(why)

        },
        
        Ok(file) => Ok(file),
    }
}


fn _csv_display_header(datatype: &String,
                      tags_and_fields: &String) {
    println!("{}\n{}",
             &datatype,
             tags_and_fields,
    );
}


fn backup_data(config: &TomlConfig,
               result_list: &Vec<PreRecord>, //backup 
               today_file_name: &String,
               metric: &TemplateSensors) {
    
    let full_path = Path::new(&config.work_dir).join(&config.backup.dir);
    /* FOR ERROR HANDLING TEST */
    //let full_path = Path::new("/root/").join(&config.backup.dir); // ROOT owner
    //let full_path = Path::new("/home/conan/").join(&config.backup.dir); // PERMISSION read/write USER or GROUP
    
    // DIR CREATE if not EXISTS
    if !full_path.exists() {
        create_new_dir(&config,
                       &full_path,
                       &metric.measurement);
    }

    // WE HAVE DIR and VERIFIED but still PERMISION's CAN CHANGE for FILE to WRITE
    if full_path.exists() {
        let today_file_name = full_path.join(format!("{t}_{n}_{m}.{e}",
                                                     t=&today_file_name,
                                                     n=&config.name,
                                                     m=&metric.measurement,
                                                     e=&config.backup.file_extension,
        ));

        // FILE CREATE or APPEND
        if config.flag.debug_backup {
            println!("\n#CSV_ANNOTATED: {}\n#", &today_file_name.display());
        }

        // format CSV HEADER
        let csv_header = prepare_csv_header_format(&config,
                                                   &metric);

        if config.flag.debug_backup {
            _csv_display_header(&metric.annotated_datatype,
                                &csv_header);
        }
        
        if !today_file_name.exists() {
            let file = create_new_file(&today_file_name); //mut

            match file {
                // TEST if DIR+FILE OK but what about FULL_DISC ?
                Ok(mut file) => writeln!(file, "{}\n{}",
                                         &metric.annotated_datatype,
                                         csv_header,
                ).unwrap_or_else(|err| {
                    eprintln!("\nERROR: APPEND DATA to file failed\nREASON: >>> {}", err)
                }),
                _ => ()
            }
        }

        // GET OLD FILE for append OR NEW 
        let file = open_file_to_append(&today_file_name); // mut

        match file {
            Ok(mut file) => {
                // SINGLE_RESULT
                /*
                if &metric.measurement == &single_record.measurement {
                    let csv_record = prepare_csv_record_format(&config,
                                                               // &single_record, //pr
                                                               &single_record,
                                                               &metric,
                    );
                    
                    if config.flag.debug_backup {
                        println!("{}", &csv_record);
                    }
                    
                    // APPEND SINGLE RECORD to backup_file
                    writeln!(file, "{}", &csv_record).unwrap_or_else(|err| {
                        eprintln!("\nERROR: APPEND DATA to file failed\nREASON: >>> {}", err);
                    });
                }
                */

                // RESULT_LIST
                // /*
                for single_record in result_list {
                    if &metric.measurement == &single_record.measurement {
                        let csv_record = prepare_csv_record_format(&config,
                                                                   &single_record,
                                                                   &metric,
                        );
                        
                        if config.flag.debug_backup {
                            println!("{}", &csv_record);
                        }
                        
                        // APPEND SINGLE RECORD to backup_file
                        writeln!(file, "{}", &csv_record).unwrap_or_else(|err| {
                            eprintln!("\nERROR: APPEND DATA to file failed\nREASON: >>> {}", err);
                        });
                    }
                }
                // */

                // WARNING via EMAIL
                if config.email.status {
                    various::easy_email(&config,
                                        // SUBJECT
                                        format!("{h} -> {r} {m}",
                                                h=config.email.sender_machine,
                                                m=metric.measurement,
                                                r="metynka::",
                                        ).as_str(),

                                        // BODY
                                        format!("{:#?}",
                                                result_list
                                                .into_iter()
                                                .filter(|r| r.measurement==metric.measurement)
                                                .collect::<Vec<&PreRecord>>()
                                        ).as_str(),

                                        // SMS
                                        false,
                                        //true,
                    );
                }
                
            },

            _ => {
                println!("\n#RECORDS: not SAVED into BACKUP !!! \n{:#?}", result_list);
                //println!("\n#RECORDS: not SAVED into BACKUP !!! \n{:#?}", single_record);
            }
        }
    } /* if full_path.exists */
    else {
        println!("\n#RECORDS: not SAVED into BACKUP !!! \n{:#?}", result_list);
        //println!("\n#RECORDS: not SAVED into BACKUP !!! \n{:#?}", single_record);
    }
}


fn prepare_csv_header_format(config: &TomlConfig,
                             metric: &TemplateSensors) -> String {

    tuple_formater(&metric.annotated_header, 
                   &vec![
                       ("tag_machine", &metric.tag_machine),
                       ("tag_carrier", &metric.tag_carrier),
                       ("tag_valid", &metric.tag_valid),
                       ("tag_id", &metric.tag_id),
                       ("field", &metric.field),
                   ],
                   config.flag.debug_template_formater
    )
}


fn prepare_csv_record_format(config: &TomlConfig,
                             record: &PreRecord,
                             metric: &TemplateSensors) -> String {

    tuple_formater(&metric.csv_annotated, 
                   &vec![
                       ("measurement", &record.measurement),
                       ("host", &record.host),
                       ("machine", &record.machine),
                       ("carrier", &record.carrier),
                       ("valid", &record.valid.to_string()),
                       ("ts", &record.ts.to_string()),
                       ("id", &record.id.to_string()),
                       ("value", &record.value.to_string()),
                   ],
                   config.flag.debug_template_formater
    )
}


fn prepare_generic_flux_query_format(config: &TomlConfig,
                                     single_influx: &Influx,
                                     generic_pre_record: &PreRecord,
                                     metric: &TemplateSensors,
                                     utc_influx_format: &String) -> String {

    let flux_template = metric.generic_query_verify_record.to_string();
    
    /* OBSOLETE -> TO_DEL

    let flux_template = match config.flag.add_flux_query_verify_record_suffix {
        true => format!("{}{}",
                metric.generic_query_verify_record.to_string(),
                config.template.flux.query_verify_record_suffix.to_string(),
        ),
        false => metric.generic_query_verify_record.to_string()
    };
    */

    tuple_formater(&flux_template,
                   &vec![
                       ("tag_carrier", &metric.tag_carrier),
                       ("tag_valid", &metric.tag_valid),
                       ("tag_id", &metric.tag_id),
                       
                       ("bucket", &single_influx.bucket),
                       ("start", &config.template.flux.query_verify_record_range_start),
                       ("measurement", &metric.measurement),
                       
                       // COMPARE only id + time // if needed can add _VALUE or implement INCREMENT_id
                       ("id", &generic_pre_record.id.to_string()),
                       ("dtif", utc_influx_format), // rfc3339 Date_Time Influx Format -> 2021-11-16T13:20:10.233Z
                   ],
                   config.flag.debug_template_formater
    )
}


fn verify_flux_result(_config: &TomlConfig,
                      data: &String) -> bool {

    //match data.trim() == "\r\n" {
    match ["\r\n", ""].contains(&data.trim()) {
        true => {
            eprintln!("\nWARNING: flux result\ndata: {:#?}",
                      data,
            );

            true
        },

        false => false
            
    }
}


fn flux_csv_to_hash(config: &TomlConfig,
                    data: String) -> Vec<HashMap<String, String>> {

    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<Vec<String>> = Vec::new();
    let mut records: Vec<HashMap<String, String>> = Vec::new();
    
    let lines_count = data.lines().count();
    let mut lines = data.lines();

    /* // CONFIG
    println!("\nresponde lines_count: {:#?}\n{:?}",
             lines_count,
             &data,
    );
    */
    
    for i in 1..lines_count {
        match i {
            // NEW vec<keys>
            1 => {
                keys = lines
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|k| k.to_string())
                    .collect::<Vec<String>>();
            },
            // APPEND single record vec<values> 
            _ => {
                values.push(lines
                            .next()
                            .unwrap()
                            .split(',')
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                );
            }
        }
    }

    if config.flag.debug_flux_pairs {
        println!("keys: {:?}", keys);
        println!("values: {:?}", values);
    }
    
    for v in values.iter() {
        let mut record: HashMap<String, String> = HashMap::new();
        
        for (k,v) in keys.iter().zip(v.into_iter()) {
            if k != "" {
                record.insert(k.to_string(),
                              v.to_string(),
                );
            }
        }
        records.push(record)
    }

    if config.flag.debug_flux_records {
        println!("\nrecords: {:#?}", records);
    }
    
    records
}


fn yield_flux_result_records(records: Vec<HashMap<String, String>>,
                             config_metric: &TemplateSensors) {

    // sample -> tag_id
    let tag = &config_metric.tag_id;
    
    for r in records.into_iter() {
        /*
        println!("\nr: {:#?}", r);

        for key in r.keys() {
            println!("{k}: {v}",
                     k=key,
                     v=r.get(key).unwrap(),
            );
        }
        */
        
        let tag_value = match r.get(&tag.to_string()) {
            Some(value) => value.to_string(),
            None => format!("ERROR >>> no KEY: {:#?}", &tag),
        };

        let field = match r.get("_field"){
            Some(value) => value.to_string(),
            None => format!("ERROR >>> no KEY: {:#?}", "_field"),
        };

        let value = match r.get("_value"){
            Some(value) => value.to_string(),
            None => format!("ERROR >>> no KEY: {:#?}", "_value"),
        };
        
        println!("\n{tag}: {tag_value} / {f}: {v}",
                 tag=tag,
                 tag_value=tag_value,
                 f=field,
                 v=value,
        );
    }
}


fn parse_flux_result(config: &TomlConfig,
                     data: String,
                     config_metric: &TemplateSensors) {

    if &data == "\r\n" {
        eprintln!("\nWARNING: not valid flux result\ndata_len: {}\ndata: {:#?}",
                  data.len(),
                  data,
        );
    }

    let records = flux_csv_to_hash(&config,
                                   data);

    if config.flag.yield_flux_records {
        yield_flux_result_records(records,
                                  &config_metric);
    }
    
    /* |> count / OBSOLETE -> TO_DEL 
    match config.flag.add_flux_query_verify_record_suffix {
        true => {
            for line in data.lines() {
                if !line.contains("value") && line.trim().len() != 0 {
                    match line.split(",").last() {
                        Some(value) => match value.parse::<u64>() {
                            // FUTURE USE
                            Ok(1) => {
                                println!("flux result count: {}", //\n{:#?}",
                                         1,
                                );
                            },
                            _ => {
                                println!("WARNING: flux result count: not 1\nDATA >>> {}\n",
                                         data,
                                );
                            },
                        },
                        _ => {
                            println!("flux RESULT count: EMPTY\nDATA >>> {}\n",
                                     data,
                            );
                        }
                    }
                }
            }
        },
        false => {
            println!("flux RESULT: {}",
                     data,
                     );
        }
    }
    */
}


fn os_call_curl_flux(config: &TomlConfig,
                     influx: &InfluxCall,
                     influx_query: &String,
                     config_metric: &TemplateSensors) -> bool {

    let curl_output = Command::new(&config.template.curl.program)
        .args([
            &config.template.curl.param_insecure,
            &config.template.curl.param_request,
            &config.template.curl.param_post,
            &influx.uri_query, // #URI
            &config.template.curl.param_header,
            &influx.auth, // #AUTH
            &config.template.curl.param_header,
            &influx.accept,
            &config.template.curl.param_header,
            &influx.content,
            &config.template.curl.param_data,
            influx_query, // #QUERY
        ])
        .output().expect("failed to execute command");

    /* FUTURE USE 
    let error_data = String::from_utf8(stderr).expect("Found invalid UTF-8");
    eprintln!("STDERR: {:#?}", error_data);
    */
    
    let out_data = match String::from_utf8(curl_output.stdout.to_vec()) {
        Ok(data) => data,
        Err(why) => {
            eprintln!("\nERROR: flux result read data problem\nREASON >>> {}", why);
            
            "".to_string()
        }
    };

    if config.flag.debug_flux_result {
        let data_len = out_data.len();

        println!("\ndata_len: {:#?}\ndata: {:#?}",
                 &data_len,
                 out_data,
        );
    }

    if config.flag.parse_flux_result {
        // VERIFY
        let flux_result_status = verify_flux_result(&config,
                                                    &out_data);
        
        if !flux_result_status {
            parse_flux_result(&config,
                              out_data,
                              config_metric);
            //}
            
            false
                
        } else {
            println!("\n#: FLUX responde -> niet goed");
            true
        }
    } else {

        false
    }
}


fn os_call_metric(config: &TomlConfig,
                  metric: &TemplateSensors) -> Option<String> {

    // os call program
    match Command::new(&metric.program)
        .args(&metric.args)
        .output() {

            // program ok
            Ok(data) => {
                let sensor_stdout_string = String::from_utf8_lossy(&data.stdout);
                let sensor_stderr_string = String::from_utf8_lossy(&data.stderr);
                
                if config.flag.debug_sensor_output {
                    println!("\n#METRIC SENSOR:
stdout: {}
stderr: {}",
                             sensor_stdout_string,
                             sensor_stderr_string,
                    );
                }
                
                Some(sensor_stdout_string.to_string())

            },

            // program error
            Err(why) => {
                eprintln!("\nWARNING: Problem parsing METRIC <{}> OS CALL: program\nREASON >>> {}",
                          metric.measurement,
                          why);
                
                None
            }
        }
}


// too long and too deep
fn os_call_metric_pipe(config: &TomlConfig,
                       metric: &TemplateSensors) -> Option<String> {

    // os call program
    match Command::new(&metric.program)
        .args(&metric.args)
        .stdout(Stdio::piped())
        .spawn() {

            // program ok
            Ok(data) => {
                
                // program stdout
                match data.stdout {
                    
                    // stdout ok 
                    Some(stdout) => {
                        
                        // | call pipe
                        match Command::new(&metric.pipe_program)
                            .args(&metric.pipe_args)
                            .stdin(stdout) // data from os program call output
                            .output() {
                            
                                // pipe ok 
                                Ok(piped_data) => {
                                    let cmd_pipe_output_stdout = String::from_utf8_lossy(&piped_data.stdout);
                                    let cmd_pipe_output_stderr = String::from_utf8_lossy(&piped_data.stderr);
                                    
                                    if config.flag.debug_sensor_output {
                                        println!("\n#METRIC_SENSOR:
stdout: {}
stderr: {}",
                                                 cmd_pipe_output_stdout,
                                                 cmd_pipe_output_stderr,
                                        );
                                    }
                                    
                                    Some(cmd_pipe_output_stdout.to_string())
                                },
                            
                                // pipe error
                                Err(why) => { eprintln!("\nWARNING: Problem parsing METRIC <{}> OS CALL PIPE: program_pipe\nREASON >>> {}",
                                                        metric.measurement,
                                                        why);
                                              None
                                }
                            }
                        
                    },
                    
                    // program stdout error
                    None => { eprintln!("\nWARNING: Problem parsing METRIC <{}> OS CALL PIPE: stdout",
                                        metric.measurement);
                              None
                    } 
                }
            },
            
            // program error
            Err(why) => {
                eprintln!("\nWARNING: Problem parsing METRIC <{}> OS CALL PIPE: program\nREASON >>> {}",
                          metric.measurement,
                          why);
                
                None
            }
        }
}


fn os_call_curl(config: &TomlConfig,
                influx: &InfluxCall,
                single_sensor_lp: &String) {

    let curl_output = Command::new(&config.template.curl.program)
        .args([
            &config.template.curl.param_insecure,
            &config.template.curl.param_request,
            &config.template.curl.param_post,
            &influx.uri_write, // #URI
            &config.template.curl.param_header,
            &influx.auth, // #AUTH
            &config.template.curl.param_data,
            single_sensor_lp, // #LINE_PROTOCOL
        ])
        .output().expect("failed to execute command");

    if config.flag.debug_influx_output {
        println!("\nstdout: {}", String::from_utf8_lossy(&curl_output.stdout));
        println!("\nstderr: {}", String::from_utf8_lossy(&curl_output.stderr));
    
    }
}


fn prepare_generic_lp_format(config: &TomlConfig,
                             generic_pre_record: &PreRecord,
                             metric: &TemplateSensors)  -> String {

    tuple_formater(&metric.generic_lp,
                   &vec![
                       ("tag_machine", &metric.tag_machine),
                       ("tag_carrier", &metric.tag_carrier),
                       ("tag_valid", &metric.tag_valid),
                       ("tag_id", &metric.tag_id),
                       ("field", &metric.field),

                       ("measurement", &generic_pre_record.measurement),
                       ("host", &generic_pre_record.host),
                       ("machine_id", &generic_pre_record.machine),
                       
                       ("carrier", &generic_pre_record.carrier),
                       ("valid", &generic_pre_record.valid),
                       
                       ("id", &generic_pre_record.id),
                       ("value", &generic_pre_record.value.to_string()),
                       
                       ("ts", &generic_pre_record.ts.to_string()),
                   ],
                   config.flag.debug_template_formater
    )
}


fn prepare_influx_format(config: &TomlConfig,
                         influx_inst: &Influx) -> InfluxCall {
    
    // URI_WRITE 
    let uri_write = tuple_formater(&format!("{}{}",
                                            &config.template.curl.influx_uri_api,
                                            &config.template.curl.influx_uri_write),
                                   &vec![
                                       ("secure", &influx_inst.secure),
                                       ("server", &influx_inst.server),
                                       ("port", &influx_inst.port.to_string()),
                                       ("org", &influx_inst.org),
                                       ("bucket", &influx_inst.bucket),
                                       ("precision", &influx_inst.precision),
                                   ],
                                   config.flag.debug_template_formater
    );

    // URI_QUERY
    let uri_query = tuple_formater(&format!("{}{}",
                                            &config.template.curl.influx_uri_api,
                                            &config.template.curl.influx_uri_query),
                                   &vec![
                                       ("secure", &influx_inst.secure),
                                       ("server", &influx_inst.server),
                                       ("port", &influx_inst.port.to_string()),
                                       ("org", &influx_inst.org),
                                   ],
                                   config.flag.debug_template_formater
    );
    
    // AUTH
    let auth = tuple_formater(&config.template.curl.influx_auth,
                              &vec![
                                  ("token", &influx_inst.token),
                              ],
                              config.flag.debug_template_formater
    );
    
    // ACCEPT
    let accept_template = String::from(&config.template.curl.influx_accept);

    // CONTENT
    let content_template = String::from(&config.template.curl.influx_content);

    InfluxCall {uri_write: uri_write,
                uri_query: uri_query,
                auth: auth,
                accept: accept_template,
                content: content_template,
    }
}


fn run_flux_query(config: &TomlConfig,
                  config_metric: &TemplateSensors,
                  single_influx: &Influx,
                  metric_pre_result: &PreRecord,
                  utc_influx_format: &String,
                  influx: &InfluxCall) {

    let generic_influx_query = prepare_generic_flux_query_format(
        &config,
        &single_influx,
        &metric_pre_result,
        &config_metric,
        &utc_influx_format);

    if config.flag.debug_flux_query {
        println!("\n#QUERY:\n{}",
                 generic_influx_query,
        );
    }

    let delay = time::Duration::from_millis(config.delay.flux_query_sleep_duration_ms);

    for i in 1..config.delay.flux_repeat_query_count + 1 {
        if i != 1 {
            println!("\n#[{}]: sleeping before next try", i);

            thread::sleep(delay);
        }

        let flux_result_status = os_call_curl_flux(&config,
                                                   &influx,
                                                   &generic_influx_query,
                                                   &config_metric);

        if flux_result_status {
            println!("\n#QUERY:\n{}", generic_influx_query);

            println!("\n#[{}]: FLUX_RESULT_STATUS: {} >>> REPEAT",
                     i,
                     flux_result_status,
            );
        } else {
            break;
        }
    }
}


fn run_all_influx_instances(config: &TomlConfig,
                            metric_result_list: & mut Vec<PreRecord>,
                            dt: &Dt) {

    for single_influx in &config.all_influx.values {
        // MATCH HTTP/HTTPS -> future use
        /* arm_secure(&single_influx); */
        
        if single_influx.status {
            // ARGS for CURL
            let influx_properties = prepare_influx_format(&config, &single_influx);
            
            if config.flag.debug_influx_uri {
                println!("\n#URI<{n}>:\n{w}\n{q}",
                         n=single_influx.name,
                         w=influx_properties.uri_write,
                         q=influx_properties.uri_query,
                );
            }
            
            if config.flag.debug_influx_auth {
                println!("\n#AUTH:\n{}", influx_properties.auth);
            }
            
            let _metric_result_list_len = metric_result_list.len();

            //METRIC_RESULT_LIST <- measured sensors values data
            // https://doc.rust-lang.org/book/ch13-02-iterators.html

            //for single_metric_result in metric_result_list.into_iter() { // owned values
            //for single_metric_result in metric_result_list.iter() { // imutable references
            for single_metric_result in metric_result_list.iter_mut() { // mutable references
                single_metric_result.machine=single_influx.machine_id.to_string();
                single_metric_result.carrier=single_influx.carrier.to_string();
                single_metric_result.valid=single_influx.flag_valid_default.to_string();

                // DISPLAY PreRecord populated with Influx properties
                if config.flag.debug_metric_record {
                    println!("\n{:?}", single_metric_result);
                }
                
                // LP via Record
                let generic_lp = prepare_generic_lp_format(&config,
                                                           &single_metric_result,
                                                           &config.metrics[&single_metric_result.key.to_string()],
                );

                if config.flag.debug_influx_lp {
                    println!("{}", generic_lp);
                }

                //BACKUP before IMPORT
                /*
                if _metric_result_list_len != 0 {
                    backup_data(&config,
                                &single_metric_result,
                                &dt.today_file_name,
                                &config.metrics[&single_metric_result.key]);
                }
                */
                
                // OS_CMD <- CURL
                if !config.flag.influx_skip_import {
                    os_call_curl(&config,
                                 &influx_properties,
                                 &generic_lp);
                }
                
                // OS_CMD <- GENERIC FLUX_QUERY
                if config.flag.run_flux_verify_record {
                    run_flux_query(
                        &config,
                        &config.metrics[&single_metric_result.key.to_string()],
                        &single_influx,
                        &single_metric_result,
                        &dt.utc_influx_format,
                        &influx_properties,
                    );
                }
            } /* for single_metric */
        } /* single_influx.status*/
    } /* all_influx.values */
}


fn os_call_program(config: &TomlConfig,
                   key: &String) -> Option<serde_json::Value> {
    
    match config.metrics[key].flag_pipe {
        // no PIPE in OS_CALL
        false => {
            match os_call_metric(&config,
                                 &config.metrics[key]) {
                
                Some(data) => verify_metric_output(&data),

                None => {
                    None
                }
            }
        },
        
        // PIPE in OS_CALL
        true => {
            match os_call_metric_pipe(&config,
                                      &config.metrics[key]) {
                
                Some(data) => verify_metric_output(&data),
                
                None => {
                    None
                }
            }
        }
    }
}


//FUTURE USE
#[allow(dead_code)]
fn arm_secure(single_influx: &Influx) {
    match &single_influx.secure.to_lowercase()[..] {
        "http" => {
            eprintln!("\n{} / http://{}:{}",
                      single_influx.status,
                      single_influx.server,
                      single_influx.port);
        },
        "https" => {
            eprintln!("\n{} / https://{}:{}",
                      single_influx.status,
                      single_influx.server,
                      single_influx.port);
        },
        other => {
            eprintln!("\n#WARNING:\ninvalid influx <{}> \"secure={}\"",
                      single_influx.server,
                      other); // this should never happen as config init verification
        },
    }
}


fn verify_metric_output(metric_stdout: &String) -> Option<serde_json::Value> {

    match serde_json::from_str(&metric_stdout) {
        Ok(value) => Some(value),
        Err(err) => {
            eprintln!("\nWARNING: Problem parsing METRIC JSON from OS CALL program pipe\nREASON >>> {}", err);
            
            None
        },
    }
}


fn parse_json_via_pointer(config: &TomlConfig,
                          metric_result_list: & mut Vec<PreRecord>,
                          single_sensor: &Sensor,
                          metric_json: &serde_json::Value,
                          key: &String,
                          dt: &Dt) {

    // DEBUG SENSOR values config
    if config.flag.debug_pointer_output {
        println!("\n#POINTER PATH:\nstatus: {s}\nname: {n}\npointer: {p}",
                 s=single_sensor.status,
                 n=single_sensor.name,
                 p=single_sensor.pointer,
        );
    }

    match metric_json.pointer(&single_sensor.pointer) {
        Some(value) => {
            if config.flag.debug_pointer_output {
                println!("json pointer value: {s} / {s:?}",
                         s=value);
            }
                    
            match verify_pointer_type(&value.to_string()) {
                Some(value) => {
                    let single_record = PreRecord::new(
                        &config,
                        key,
                        dt.ts,
                        value,
                        &single_sensor.name,
                    );
                    
                    // METRIC RECORD_LIST -> Vec<PreRecord> / trait PartialEq
                    if !metric_result_list.contains(&single_record) { 
                        metric_result_list.push(single_record)
                    }
                    
                },
                None => {
                    eprintln!("{}",
                              tuple_formater(&"JSON POINTER VALUE: metric: <{m}> Option: None !!!".to_string(),
                                             &vec![
                                                 ("m", key),
                                             ],
                                             false,
                              ),
                    );
                }
                
            }
        },

        None => {
            println!("\n#POINTER_PATH: !!! NONE !!!");
            eprintln!("\nWARNING: Problem parsing METRIC: {m} -> JSON PATH: {jp}",
                      jp=&single_sensor.pointer,
                      m=config.metrics[key].measurement,
            );
        }
    }
}


pub fn parse_sensors_data(config: &TomlConfig,
                          dt: &Dt) {

    let mut metric_result_list: Vec<PreRecord> = Vec::new();

    println!("\n#METRIC:");

    // loop METRICS
    for key in config.metrics.keys() {
        if config.metrics[key].flag_status {
            println!("{metric} -> MEASUREMENT: {measurement}",
                     measurement=config.metrics[key].measurement,
                     metric=config.metrics[key].field,
            );

            // METRIC data <- OS_CALL
            let metric_json = os_call_program(&config,
                                              key);

            match &metric_json {
                Some(json) => {
                    for single_sensor in &config.metrics[key].values {
                        if single_sensor.status {
                            // JSON single POINTER
                            parse_json_via_pointer(&config,
                                                   & mut metric_result_list,
                                                   &single_sensor,
                                                   &json,
                                                   &key,
                                                   &dt,
                            );
                            
                        }
                    }
                },
                None => {
                    eprintln!("ERROR: skip parsing all sensor values in metric: <{m}>",
                              m=&key);
                }
            }
        }
    }

    /*
    if config.flag.debug_metric_record {
        for single_metric_result in &metric_result_list {
            println!("\n {:?}", single_metric_result);
        }
    }
    */

    // INFLUX INSTANCES
    run_all_influx_instances(&config,
                             //& mut result_list,
                             & mut metric_result_list,
                             &dt,
    );

    // BACKUP: loop metrics again -> now with influx LP populated
    for key in config.metrics.keys() {
        if config.metrics[key].flag_status && metric_result_list.len() != 0 {
            backup_data(&config,
                        &metric_result_list,
                        &dt.today_file_name,
                        &config.metrics[key],
            );
        } else {
            println!("NO records to BACKUP");
        }
    }
}
