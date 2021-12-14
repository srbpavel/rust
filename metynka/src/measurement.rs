use std::process::{Command, Stdio};
use std::path::Path;
use std::io::Write;
use std::any::Any;
use std::fmt::Debug;
use std::collections::HashMap;
use strfmt::strfmt;
use std::convert::TryFrom;

use crate::settings::{TomlConfig, Influx, TemplateSensors, Sensor};

use crate::util::{file_system,
                  ts::Dt,
                  {template_formater::tuple_formater}};

use crate::various;

use crate::influxdb::{self, InfluxData};


#[derive(Debug)]
pub struct Record {
    pub key: String,
    pub measurement: String,

    pub value: String,
    pub id: String,

    pub machine: String,
    pub carrier: String,
    pub valid: String,
    
    pub host: String,
    pub ts: u64,
}


impl Record {
    fn new(config: &TomlConfig,
           key: &str,
           ts: u64,
           value: f64,
           id: &str) -> Record {
        
        // HERE I CAN put some TEST's to handle ERROR / raise WARNING
        Record {
            key: key.to_string(),
            ts: ts,
            value: value.to_string(),
            id: id.to_string(),
            measurement: config.metrics[key].measurement.to_string(),
            host: config.host.to_string(),
            ..Record::default()
        }
    }


    // maybe set default empty "" for generic verify ?
    fn default() -> Record {
        Record {
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


#[derive(Debug, PartialEq)]
struct PointerFloat(f64);

impl TryFrom<String> for PointerFloat {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.parse::<f64>() {
            Ok(value) => Ok(PointerFloat(value)),
            Err(_) => Err(()),
        }
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


// https://doc.rust-lang.org/std/any/index.html
// https://doc.rust-lang.org/std/any/trait.Any.html
// DOUCIT + POCHOPIT zpusob volani
fn verify_pointer_type<T: Any + Debug>(value: &T) -> Option<f64> {
    let value_any = value as &dyn Any;

    let value = match value_any.downcast_ref::<String>() {
        Some(as_string) => {             

            // JSON VALUE
            match PointerFloat::try_from(as_string.to_string()) {
                // not f64 -> TRIM " 
                Err(_why) => {
                    pointer_trim_quotes(as_string.to_string())
                },
                // is f64 
                Ok(PointerFloat(number)) => {
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


fn pointer_trim_quotes(string: String) -> Option<f64> {
    /*
    match str::replace(string, "\"", "") // "string"
        .trim()
        .parse::<f64>() {
    */

    let trim_chars = ['"', '\''];

    match PointerFloat::try_from(string
                                 .trim_matches(|p| p == trim_chars[0] || p == trim_chars[1] ) // 'char'
                                 .to_string()) {

        Ok(PointerFloat(number)) => {
            Some(number)
        },
        
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


fn backup_data(config: &TomlConfig,
               result_list: &Vec<Record>, //backup 
               today_file_name: &String,
               metric: &TemplateSensors) {
    
    let full_path = Path::new(&config.work_dir).join(&config.backup.dir);
    /* FOR ERROR HANDLING TEST */
    //let full_path = Path::new("/root/").join(&config.backup.dir); // ROOT owner
    //let full_path = Path::new("/home/conan/").join(&config.backup.dir); // PERMISSION read/write USER or GROUP
    
    // DIR CREATE if not EXISTS
    if !full_path.exists() {
        file_system::create_new_dir(&config,
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
        let csv_header = influxdb::prepare_csv_header_format(&config,
                                                             &metric);

        if config.flag.debug_backup {
            influxdb::csv_display_header(&metric.annotated_datatype,
                                         &csv_header);
        }
        
        if !today_file_name.exists() {
            let file = file_system::create_new_file(&today_file_name);

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
        let file = file_system::open_file_to_append(&today_file_name);

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
                        let csv_record = influxdb::prepare_csv_record_format(
                            &config,
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
                                                h=config.host,
                                                m=metric.measurement,
                                                r="metynka::",
                                        ).as_str(),

                                        // BODY
                                        format!("{:#?}",
                                                result_list
                                                .into_iter()
                                                .filter(|r| r.measurement==metric.measurement)
                                                .collect::<Vec<&Record>>()
                                        ).as_str(),

                                        // SMS
                                        false, //true,
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

    
fn run_all_influx_instances(config: &TomlConfig,
                            metric_result_list: & mut Vec<Record>,
                            dt: &Dt) {

    for single_influx in &config.all_influx.values {
        // MATCH HTTP/HTTPS -> future use
        /* arm_secure(&single_influx); */
        
        if single_influx.status {
            // ARGS for CURL
            let influx_properties = influxdb::prepare_influx_format(&config,
                                                                    &single_influx,
            );
            
            if config.flag.debug_influx_uri {
                println!("\n#URI<{n}>:\n{w}\n{q}\n",
                         n=single_influx.name,
                         w=influx_properties.uri_write,
                         q=influx_properties.uri_query,
                );
            }
            
            if config.flag.debug_influx_auth {
                println!("\n#AUTH:\n{}", influx_properties.auth);
            }
            
            //let _metric_result_list_len = metric_result_list.len();

            //METRIC_RESULT_LIST <- measured sensors values data
            // https://doc.rust-lang.org/book/ch13-02-iterators.html
            // https://doc.rust-lang.org/rust-by-example/flow_control/for.html
            
            //for single_metric_result in metric_result_list.into_iter() { // owned values
            //for single_metric_result in metric_result_list.iter() { // imutable references
            for single_metric_result in metric_result_list.iter_mut() { // mutable references
                single_metric_result.machine=single_influx.machine_id.to_string();
                single_metric_result.carrier=single_influx.carrier.to_string();
                single_metric_result.valid=single_influx.flag_valid_default.to_string();

                // DISPLAY Record populated with Influx properties
                if config.flag.debug_metric_record {
                    println!("\n{:?}", single_metric_result);
                }
                
                // LP via Record
                let generic_lp = influxdb::prepare_generic_lp_format(&config,
                                                                     &single_metric_result,
                                                                     &config.metrics[&single_metric_result.key.to_string()],
                );

                if config.flag.debug_influx_lp {
                    println!("{}", generic_lp);
                }

                //BACKUP before IMPORT -> obsolete or future-use ?
                /*
                if _metric_result_list_len != 0 {
                    backup_data(&config,
                                &single_metric_result,
                                &dt.today_file_name,
                                &config.metrics[&single_metric_result.key]);
                }
                */

                /* STRUCT InfluxData
                let influx_data = InfluxData {
                    properties: &influx_properties,
                    lp: generic_lp,
                };
                */

                // IMPL::NEW InfluxData
                let influx_data = InfluxData::new(&influx_properties,
                                                  generic_lp,
                );
                
                // OS_CMD <- CURL
                if !config.flag.influx_skip_import {

                    /*
                    // ORIGINAL
                    influxdb::import_lp_via_curl(&config,
                                                 &influx_properties,
                                                 &generic_lp);
                    // VIA Struct
                    influxdb::import_lp_via_curl(&config,
                                                 &influx_data);
                    */

                    // IMPL import_lp
                    influx_data.import_lp(&config);
                    
                }
                
                // OS_CMD <- GENERIC FLUX_QUERY
                if config.flag.run_flux_verify_record {
                    influxdb::run_flux_query(
                        &config,
                        &config.metrics[&single_metric_result.key.to_string()],
                        &single_influx,
                        &single_metric_result,
                        &dt.utc_influx_format,
                        //&influx_properties,
                        &influx_data.properties,
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
                          metric_result_list: & mut Vec<Record>,
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
                    let single_record = Record::new(
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

    let mut metric_result_list: Vec<Record> = Vec::new();

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
