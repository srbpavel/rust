use std::process::{Command, Stdio};

use std::path::Path;

use std::fs::{self, OpenOptions, File};

use std::io::{self, Write};

use std::any::{Any};

use std::fmt::Debug;

use strfmt::strfmt;

use std::collections::HashMap;

use crate::util::ts::{Dt};
use crate::util::template_formater::tuple_formater;
use metynka::{TomlConfig, Influx, TemplateSensors, Sensor};


#[derive(Debug)]
struct InfluxCall {
    uri_write: String,
    uri_query: String,
    auth: String,
    accept: String,
    content: String,
}


// BACKUP CVS 
#[derive(Debug)]
struct Record {
    measurement: String,

    value: String,
    id: String,

    machine: String,
    carrier: String,
    valid: String,

    host: String,
    ts: u64,
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
impl PartialEq for Record
{
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}


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


fn csv_display_header(datatype: &String,
                      tags_and_fields: &String) {
    println!("{}\n{}",
             &datatype,
             tags_and_fields,
    );
}


fn backup_data(config: &TomlConfig,
               result_list: &Vec<Record>,
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
            csv_display_header(&metric.annotated_datatype,
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
                // RESULT_LIST
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
            },

            _ => {
                println!("\n#RECORDS: not SAVED into BACKUP !!! \n{:#?}", result_list);
            }
        }
    } /* if full_path.exists */
    else {
        println!("\n#RECORDS: not SAVED into BACKUP !!! \n{:#?}", result_list);
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
                             record: &Record,
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
                                     generic_record: &Record,
                                     metric: &TemplateSensors,
                                     utc_influx_format: &String) -> String {

    let flux_template = match config.flag.add_flux_query_verify_record_suffix {
        true => format!("{}{}",
                metric.generic_query_verify_record.to_string(),
                config.template.flux.query_verify_record_suffix.to_string(),
        ),
        false => metric.generic_query_verify_record.to_string()
    };    

    tuple_formater(&flux_template,
                   &vec![
                       ("tag_carrier", &metric.tag_carrier),
                       ("tag_valid", &metric.tag_valid),
                       ("tag_id", &metric.tag_id),
                       
                       ("bucket", &single_influx.bucket),
                       ("start", &config.template.flux.query_verify_record_range_start),
                       ("measurement", &metric.measurement),
                       
                       // COMPARE only id + time // if needed can add _VALUE
                       ("id", &generic_record.id.to_string()),
                       ("dtif", utc_influx_format), // rfc3339 Date_Time Influx Format -> 2021-11-16T13:20:10.233Z
                   ],
                   config.flag.debug_template_formater
    )
}


fn parse_flux_result(stdout: Vec<u8>,
                     _stderr: Vec<u8>) {

    /* FUTURE USE 
    let error_data = String::from_utf8(stderr).expect("Found invalid UTF-8");
    eprintln!("STDERR: {:#?}", error_data);
    */

    let data = String::from_utf8(stdout).expect("Found invalid UTF-8");

    let data_len = data.len();
    
    if data_len < 1 {
        eprintln!("WARNING: flux result len: {}", data_len);
    }
    
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
                        println!("WARNING: flux result: not 1\nDATA >>> {}\n",
                                 data,
                        );
                    },
                },
                _ => {
                    println!("flux RESULT: EMPTY\nDATA >>> {}\n",
                             data,
                    );
                }
            }
        }
    }
}


fn os_call_curl_flux(config: &TomlConfig,
                     influx: &InfluxCall,
                     influx_query: &String) {

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

    // parse FLUX stdout responde
    if config.flag.debug_flux_result {
        parse_flux_result(curl_output.stdout,
                          curl_output.stderr)
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

// too long and too deep ?
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
                             generic_record: &Record,
                             metric: &TemplateSensors)  -> String {

    tuple_formater(&metric.generic_lp,
                   &vec![
                       ("tag_machine", &metric.tag_machine),
                       ("tag_carrier", &metric.tag_carrier),
                       ("tag_valid", &metric.tag_valid),
                       ("tag_id", &metric.tag_id),
                       ("field", &metric.field),
                       ("measurement", &generic_record.measurement),
                       ("host", &generic_record.host),
                       ("machine_id", &generic_record.machine),
                       
                       ("carrier", &generic_record.carrier),
                       ("valid", &generic_record.valid),
                       
                       ("id", &generic_record.id),
                       ("value", &generic_record.value.to_string()),
                       
                       ("ts", &generic_record.ts.to_string()),
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
                  metric_result: &Record,
                  utc_influx_format: &String,
                  influx: &InfluxCall) {

    let generic_influx_query = prepare_generic_flux_query_format(
        &config,
        &single_influx,
        &metric_result,
        &config_metric,
        &utc_influx_format);
    
    if config.flag.debug_flux_query {
        println!("\n#QUERY:\n{}",
                 generic_influx_query,
        );
    }

    os_call_curl_flux(&config,
                      &influx,
                      &generic_influx_query);
}


fn run_all_influx_instances(config: &TomlConfig,
                            result_list: & mut Vec<Record>,
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
            
            //METRIC_RESULT_LIST <- measured sensors values data
            //for single_metric_result in metric_result_list.into_iter() {
            for single_metric_result in metric_result_list.iter_mut() { // https://doc.rust-lang.org/book/ch13-02-iterators.html
                let new_single_metric_result = Record {
                    measurement: single_metric_result.measurement.to_string(),

                    id: single_metric_result.id.to_string(),
                    value: single_metric_result.value.to_string(),

                    carrier: single_influx.carrier.to_string(),
                    valid: single_influx.flag_valid_default.to_string(),
                    machine: single_influx.machine_id.to_string(),

                    host: single_metric_result.host.to_string(),
                    ts: single_metric_result.ts,
                };
                
                // *& / UPDATE Struct, but still cannot use PreRecord in Vec !!!
                single_metric_result.machine=single_influx.machine_id.to_string();
                single_metric_result.carrier=single_influx.carrier.to_string();
                single_metric_result.valid=single_influx.flag_valid_default.to_string();
                // DEBUG println!("<UPDATED> {:?}", single_metric_result);

                // display PreRecord <- Record populated with Influx properties
                if config.flag.debug_metric_record {
                    println!("\n{:?}", new_single_metric_result);
                }
                
                // LP via Record
                let generic_lp = prepare_generic_lp_format(&config,
                                                           &new_single_metric_result,
                                                           &config.metrics[&single_metric_result.key.to_string()],
                );
                
                if config.flag.debug_influx_lp {
                    println!("\n#LP:\n{}", generic_lp);
                }
                
                // OS_CMD <- CURL
                os_call_curl(&config,
                             &influx_properties,
                             &generic_lp);
                
                // OS_CMD <- GENERIC FLUX_QUERY
                if config.flag.run_flux_verify_record {
                    run_flux_query(
                        &config,
                        &config.metrics[&single_metric_result.key.to_string()],
                        &single_influx,
                        &new_single_metric_result,
                        &dt.utc_influx_format,
                        &influx_properties,
                    );
                }

                if !result_list.contains(&new_single_metric_result) { 
                    result_list.push(new_single_metric_result)
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
                    
                    // METRIC RECORD_LIST -> Vec<Record> / trait PartialEq
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

    let mut result_list: Vec<Record> = Vec::new();
    let mut metric_result_list: Vec<PreRecord> = Vec::new();

    // loop via METRICS
    println!("\n#METRIC:");
    
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
                                                   &dt);
                            
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

    if config.flag.debug_metric_record {
        for single_metric_result in &metric_result_list {
            println!("\n {:?}", single_metric_result);
        }
    }

    // INFLUX INSTANCES
    run_all_influx_instances(&config,
                             & mut result_list,
                             & mut metric_result_list,
                             &dt);

    // BACKUP
    for key in config.metrics.keys() {
        if config.metrics[key].flag_status && result_list.len() != 0 {
            backup_data(&config,
                        &result_list,
                        &dt.today_file_name,
                        &config.metrics[key]);
        }
    }
}
