use std::process;
use std::process::{Command, Stdio};

use std::path::Path;

use std::fs;
use std::fs::{OpenOptions, File};

use std::io::Write;

use std::any::{Any};

use std::fmt::Debug;

pub use crate::util::ts::{Dt};
pub use crate::util::template_formater::tuple_formater;
use metynka::{TomlConfig, Influx, TemplateSensors};


// BACKUP CVS 
#[derive(Debug)]
pub struct Record {
    pub measurement: String,

    pub value: String,
    pub id: String,

    pub machine: String,
    pub carrier: String,
    pub valid: String,

    pub host: String,

    pub ts: i64,
}


#[derive(Debug)]
pub struct PreRecord {
    pub key: String,

    pub measurement: String,

    pub value: String,
    pub id: String,

    /*
    pub machine: String,
    pub carrier: String,
    pub valid: String,
    */
    
    pub host: String,

    pub ts: i64,    
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


// https://doc.rust-lang.org/std/any/index.html
// https://doc.rust-lang.org/std/any/trait.Any.html
// DOUCIT + POCHOPIT zpusob volani
fn verify_pointer_type<T: Any + Debug>(value: &T) -> (f64, bool) {
    let value_any = value as &dyn Any;

    let (value, status): (f64, bool) = match value_any.downcast_ref::<String>() {
        Some(as_string) => {             
            /*
            println!("STRING len: {len} / str: <{str}> / debug: {str:?}",
                     len=as_string.len(),
                     str=as_string,
            );
            */

            match as_string.parse::<f64>() {
                Err(_why) => {
                    // JSON VALUE wrapped with "
                    // TEST THIS insted replace -> https://doc.rust-lang.org/std/str/trait.FromStr.html
                    let float_via_replace = str::replace(as_string, "\"", "").parse::<f64>().unwrap();

                    /*
                    println!("   ERROR_to_f64: {w} REPLACE: <{r}> / debug: {r:?}",
                             w = why,
                             r = float_via_replace,
                    );
                    */

                    (float_via_replace, true)
                },
                Ok(number) => {
                    /*
                    println!("   FLOAT: {f} / debug: {f:?}", f=number);
                     */
                    
                    (number, true)
                },
            }
        }

        None => {
            println!("ANY: {:?}", value);

            (0.0, false)
        }
    };

    (value, status)
}


pub fn backup_data(config: &TomlConfig,
                   result_list: &Vec<Record>,
                   today_file_name: &String,
                   metric: &TemplateSensors) {

    let full_path = Path::new(&config.work_dir).join(&config.backup.dir);
    /* FOR TEST error handling */
    //let full_path = Path::new("/root/").join(&config.backup.dir); // ROOT owner
    //let full_path = Path::new("/home/conan/").join(&config.backup.dir); // PERMISSION write ROOT
    //println!(" @@@ DEBUG @@@ FULL_PATH: {:#?}", full_path);
    //_
    
    // DIR CREATE if not EXISTS
    if !full_path.exists() {
        fs::create_dir_all(&full_path).unwrap_or_else(|err| {
            let print_formated = tuple_formater(&"\nERROR >> METRIC <{m}> failed to create BACKUP DIR: {d}\nREASON: >>> {e}".to_string(),
                                                   &vec![
                                                       ("m", &metric.measurement),
                                                       ("d", &config.work_dir),
                                                       ("e", &err.to_string())
                                                   ],
                                                config.flag.debug_template_formater
            );
            
            println!("{}", print_formated);
            eprintln!("{}", print_formated);
            
        });
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
    
        if !today_file_name.exists() {
            let mut file = match File::create(&today_file_name) { // LEARN TO write TEST for this
                Err(why) => {
                    eprintln!("\nEXIT: COULD NOT CREATE {}\nREASON: >>> {}",
                              &today_file_name.display(),
                              why);

                    // uz nevim proc to tu mam? PODLE ME KDYZ ZAKLADAM NOVEJ SOUBOR
                    OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(&today_file_name)
                        .expect("FAILED TO OPEN FILE")
                        //.unwrap() // ALSO NEED TEST
                },
                
                Ok(file) => file,
            };

            // TEST if DIR+FILE OK but FULL_DISC
            // CSV_DATATYPE
            writeln!(file, "{}", &metric.annotated_datatype).unwrap_or_else(|err| {  // TAG_ID
                eprintln!("\nERROR EXIT: APPEND DATA to file failed\nREASON: >>> {}", err);
                //process::exit(1);  // this will not exit but raise ALARM
            });

            // CSV_HEADER
            writeln!(file, "{}", csv_header).unwrap_or_else(|err| {
                eprintln!("\nERROR EXIT: APPEND DATA to file failed\nREASON: >>> {}", err);
                //process::exit(1);  // this will not exit but raise ALARM
            });

            // you have it twice, fix it to have it only once
            if config.flag.debug_backup {
                println!("{}\n{}",
                         &metric.annotated_datatype,
                         csv_header,
                );
            }
        }
        else {
            if config.flag.debug_backup {
                println!("{}\n{}",
                         &metric.annotated_datatype,
                         csv_header,
                );
            }
        }

        let (mut file, file_status) = match OpenOptions::new()
            .write(true)
            .append(true)
            .open(&today_file_name)
        {
            Ok(file) => (file, true),

            Err(why) => {
                eprintln!("\nERROR >> FILE WRITE permission: {}\nREASON: >>> {}",
                          &today_file_name.display(),
                          why);

                (OpenOptions::new()
                 .read(true)
                 .open(&today_file_name)
                 .expect("FAILED TO OPEN FILE just for READ"),
                 false)
            },
        };

        /* TRY TO FIND write true/false, but not in metadata ? 
        println!("\n@@@FILE:\nAAA: {:#?}\nBBB: {:#?}]\nCCC: {:#?}",
                 file.metadata(),
                 /*
                 match file.metadata() {
                     Ok(data) => data,
                     Err(_) => process::exit(1)
                 },
                 */
                 file.metadata().unwrap().permissions(),
                 file); // cannot acces File field ? as this is method
        */
       
        if !file_status {
            println!("\n@@@ WE CANNOT WRITE as READ_ONLY"); // DEL when done
            println!("\n#RECORDS: not SAVED into BACKUP !!! \n{:?}", result_list);
        }
        else {
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
                        eprintln!("\nERROR EXIT: APPEND DATA to file failed\nREASON: >>> {}", err);
                        //process::exit(1);  // this will not exit but raise ALARM
                    });
                }
            }
        }
    } /* if full_path_create_status */
    else {
        println!("\n#RECORDS: not SAVED into BACKUP !!! \n{:#?}", result_list);
    }
}


pub fn prepare_csv_header_format(config: &TomlConfig,
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


pub fn prepare_csv_record_format(config: &TomlConfig,
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


pub fn prepare_generic_flux_query_format(config: &TomlConfig,
                                         single_influx: &Influx,
                                         generic_record: &Record,
                                         metric: &TemplateSensors,
                                         utc_influx_format: &String) -> String {

    let flux_template = match config.flag.add_flux_query_verify_record_suffix {
        true => format!("{}{}",
                //String::from(&metric.generic_query_verify_record),
                //String::from(&config.template.flux.query_verify_record_suffix),
                metric.generic_query_verify_record.to_string(),
                config.template.flux.query_verify_record_suffix.to_string(),
        ),
        //false => String::from(&metric.generic_query_verify_record)
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


pub fn os_call_curl_flux(config: &TomlConfig,
                         influx_uri: &String,
                         influx_auth: &String,
                         influx_header_accept: &String,
                         influx_header_content: &String,
                         influx_query: &String) {

    let curl_output = Command::new(&config.template.curl.program)
        .args([
            &config.template.curl.param_insecure,
            &config.template.curl.param_request,
            &config.template.curl.param_post,
            influx_uri, // #URI
            &config.template.curl.param_header,
            influx_auth, // #AUTH
            &config.template.curl.param_header,
            influx_header_accept,
            &config.template.curl.param_header,
            influx_header_content,
            &config.template.curl.param_data,
            influx_query, // #QUERY
        ])
        .output().expect("failed to execute command");

    if config.flag.debug_flux_result {
        // NO stderr for now
        println!("\n#QUERY_RESULT:stdout: {}", String::from_utf8_lossy(&curl_output.stdout));
    }
}


pub fn os_call_metric(config: &TomlConfig,
                      metric: &TemplateSensors) -> String {
    
    let sensor_output = Command::new(&metric.program)
        .args(&metric.args)
        .output().expect("failed to execute command");
    
    let sensor_stdout_string = String::from_utf8_lossy(&sensor_output.stdout);
    let sensor_stderr_string = String::from_utf8_lossy(&sensor_output.stderr);

    if config.flag.debug_sensor_output {
        println!("\n#METRIC SENSOR:
stdout: {}
stderr: {}",
                 sensor_stdout_string,
                 sensor_stderr_string,
        );
    }

    return sensor_stdout_string.to_string()
}


pub fn os_call_metric_pipe(config: &TomlConfig,
                           memory: &TemplateSensors) -> String {

    let cmd_output = Command::new(&memory.program)
        .args(&memory.args)
        .stdout(Stdio::piped()).spawn().unwrap();

    let cmd_pipe_output = Command::new(&memory.pipe_program)
        .args(&memory.pipe_args)
        .stdin(cmd_output.stdout.unwrap())
        .output().expect("failed to execute command");

    let cmd_pipe_output_stdout = String::from_utf8_lossy(&cmd_pipe_output.stdout);
    let cmd_pipe_output_stderr = String::from_utf8_lossy(&cmd_pipe_output.stderr);

    if config.flag.debug_sensor_output {
        println!("\n#MEMORY_SENSOR:
stdout: {}
stderr: {}",
                 cmd_pipe_output_stdout,
                 cmd_pipe_output_stderr,
        );
    }

    return cmd_pipe_output_stdout.to_string()
}


pub fn os_call_curl(config: &TomlConfig,
                   influx_uri: &String,
                   influx_auth: &String,
                   single_sensor_lp: &String) {

    let curl_output = Command::new(&config.template.curl.program)
        .args([
            &config.template.curl.param_insecure,
            &config.template.curl.param_request,
            &config.template.curl.param_post,
            influx_uri, // #URI
            &config.template.curl.param_header,
            influx_auth, // #AUTH
            &config.template.curl.param_data,
            single_sensor_lp, // #LINE_PROTOCOL
        ])
        .output().expect("failed to execute command");

    if config.flag.debug_influx_output {
        println!("\nstdout: {}", String::from_utf8_lossy(&curl_output.stdout));
        println!("\nstderr: {}", String::from_utf8_lossy(&curl_output.stderr));
    
    }
}


pub fn prepare_generic_lp_format(config: &TomlConfig,
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


pub fn prepare_influx_format(config: &TomlConfig,
                             influx_inst: &Influx) -> (String, String, String, String, String) {

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

    (uri_write,
     uri_query,
     auth,
     accept_template,
     content_template,
    )
}


pub fn parse_sensors_data(config: &TomlConfig,
                          dt: &Dt) {

    // RESULT_LIST
    let mut result_list: Vec<Record> = Vec::new();

    // METRIC_RESULT_LIST
    let mut metric_result_list: Vec<PreRecord> = Vec::new();

    // METRICS
    //println!("\n#METRICS:\n{:?}", &config.metrics.keys());

    for key in config.metrics.keys() {
        if config.metrics[key].flag_status {
            println!("#METRIC: {metric} -> MEASUREMENT: {measurement}",
                     measurement=config.metrics[key].measurement,
                     metric=config.metrics[key].field,
            );


            // OS CALL program or pipe_program
            let metric_json: serde_json::Value = match config.metrics[key].flag_pipe {
                // no PIPE
                false => {
                    let metric_stdout = os_call_metric(&config,
                                                       &config.metrics[key]);

                    serde_json::from_str(&metric_stdout).unwrap_or_else(|err| {
                        eprintln!("\nEXIT: Problem parsing METRIC JSON from OS CALL program\nREASON >>> {}", err);
                        process::exit(1);
                    })
                },

                // PIPE is here
                true => {
                    let metric_stdout = os_call_metric_pipe(&config,
                                                       &config.metrics[key]);

                    serde_json::from_str(&metric_stdout).unwrap_or_else(|err| {
                        eprintln!("\nEXIT: Problem parsing METRIC JSON from OS CALL pipe_program\nREASON >>> {}", err);
                        process::exit(1);
                    })
                }
            };
            
            for single_sensor in &config.metrics[key].values {
                if single_sensor.status {
                    // JSON single POINTER
                    let (single_sensor_pointer_value, pointer_path_status) = match metric_json.pointer(&single_sensor.pointer) {
                        Some(value) => {
                            if config.flag.debug_pointer_output {
                                println!("\n#POINTER_PATH: SOME: <{v}> / {v:?}",
                                         v=value);
                            }
                            
                            (Some(value), true)
                        },

                        None => {
                            println!("\n#POINTER_PATH: !!! NONE !!!");
                            eprintln!("\nWARNING: Problem parsing METRIC: {m} -> JSON PATH: {jp}",
                                      jp=&single_sensor.pointer,
                                      m=config.metrics[key].measurement,
                            );
                            (None, false)
                        }
                    };

                // TO KEEP IN MIND
                
                /* !panic if WRONG POINTER path
                let single_sensor_pointer_value = metric_json.pointer(&single_sensor.pointer).unwrap(); //&
                */
                
                /* EXIT if WRONG POINTER path but we want to get rid of all exit
                let single_sensor_pointer_value = metric_json.pointer(&single_sensor.pointer).unwrap_or_else(||{
                    eprintln!("\nEXIT: Problem parsing POINTER from METRIC JSON\nREASON >>> maybe not valid JSON path: <{}>",
                              single_sensor.pointer);
                    process::exit(1);
                });
                */

                    // DEBUG true/false SENSORS
                    if config.flag.debug_pointer_output {
                        println!("path_status: {ps}\nstatus: {s}\nname: {n}\npointer: {p}\nvalue: {v:?}",
                                 s=single_sensor.status,
                                 n=single_sensor.name,
                                 p=single_sensor.pointer,
                                 v=&single_sensor_pointer_value,
                                 ps=&pointer_path_status,
                        );
                    }

                    // JSON PATH failed
                    if pointer_path_status {
                        let (pointer_parsed_float, pointer_type_status): (f64, bool) = match single_sensor_pointer_value {
                            Some(value) => {
                                if config.flag.debug_pointer_output {
                                    println!("unwrap_some: {s} / {s:?}",
                                             s=value);
                                }
                                verify_pointer_type(&value.to_string())
                            },
                            None => {
                                eprintln!("unwrap: !!! none !!!");
                                process::exit(1); // tohle neumim nahradit, ale nemelo by nastat ?
                            }
                        };
                    
                        if pointer_type_status {
                            let single_record = PreRecord {
                                key:key.to_string(),
                                ts: dt.ts,
                                value: pointer_parsed_float.to_string(),
                                id: single_sensor.name.to_string(),
                                measurement: config.metrics[key].measurement.to_string(),
                                host: config.host.to_string(),

                                /*
                                machine: String::from(""),
                                carrier: String::from(""),
                                valid: String::from(""),
                                */
                            };
                    
                            // METRIC RECORD_LIST -> Vec<Record>
                            if !metric_result_list.contains(&single_record) { 
                                metric_result_list.push(single_record)
                            }
                        }
                    } /* path status */
                } /* if single_sensorstatus */
            } /* for single_sensor in each metric*/
        }
    } /* for key in metrics */

    if config.flag.debug_metric_record {
        for single_metric_result in &metric_result_list {
            println!("\n {:?}", single_metric_result);
        }
    }


    // INFLUX INSTANCES
    for single_influx in &config.all_influx.values {
        // match http/https -> future use
        match &single_influx.secure[..] {
            "http" => {
                eprintln!("\nhttp://{}:{}", single_influx.server, single_influx.port);
            },
            "https" => {
                eprintln!("\nhttps://{}:{}", single_influx.server, single_influx.port);
            },
            other => {
                eprintln!("\n#WARNING:\ninvalid influx secure -> <{}>", other); // this should never happen as config init verification
            },
        }


        if single_influx.status {

            // ARGS for CURL
            let (influx_uri_write,
                 influx_uri_query,
                 influx_auth,
                 influx_accept,
                 influx_content ) = prepare_influx_format(&config, &single_influx); // TUPLE OF 5
            
            if config.flag.debug_influx_uri {
                println!("\n#URI<{n}>:\n{w}\n{q}",
                         n=single_influx.name,
                         w=&influx_uri_write,
                         q=&influx_uri_query
                );
            }
            
            if config.flag.debug_influx_auth {
                println!("\n#AUTH:\n{}", &influx_auth);
            }


            //METRIC_RESULT_LIST
            for single_metric_result in &metric_result_list { // neumim updatovat Struct in Vec !!! borrow ERR
                let new_single_metric_result = Record {
                    ts: single_metric_result.ts,
                    value: single_metric_result.value.to_string(),
                    carrier: single_influx.carrier.to_string(),
                    id: single_metric_result.id.to_string(),
                    valid: single_influx.flag_valid_default.to_string(),
                    machine: single_influx.machine_id.to_string(),
                    measurement: single_metric_result.measurement.to_string(),
                    host: single_metric_result.host.to_string(),
                    //..single_metric_result // Record -> Record
                };

                //POKUS
                /*
                let update_single_record = PreRecord {
                    machine: single_influx.machine_id.to_string(),
                    carrier: single_influx.carrier.to_string(),
                    valid: single_influx.flag_valid_default.to_string(),
                    ..single_metric_result // * &
                    
                };
                println!("\n#PRE_RECORD <updated>:{:?}", update_single_record);
                */

                // PreRecord <- Record populated with Influx properties
                if config.flag.debug_metric_record {
                    println!("\n{:?}", new_single_metric_result);
                }
                
                // LP via Record
                let generic_lp = prepare_generic_lp_format(&config,
                                                           &new_single_metric_result,
                                                           &config.metrics[&single_metric_result.key.to_string()]);

                if config.flag.debug_influx_lp {
                    println!("\n#LP:\n{}", generic_lp);
                }

                // OS_CMD <- CURL
                os_call_curl(&config,
                             &influx_uri_write,
                             &influx_auth,
                             &generic_lp);

                // OS_CMD <- GENERIC FLUX_QUERY
                if config.flag.run_flux_verify_record {
                    let generic_influx_query = prepare_generic_flux_query_format(
                        &config,
                        &single_influx,
                        &new_single_metric_result,
                        &config.metrics[&single_metric_result.key.to_string()],
                    
                        &dt.utc_influx_format);
                        
                    if config.flag.debug_flux_query {
                        println!("\n#QUERY:\n{}",
                                 generic_influx_query,
                        );
                    }

                    os_call_curl_flux(&config,
                                      &influx_uri_query,
                                      &influx_auth,
                                      &influx_accept,
                                      &influx_content,
                                      &generic_influx_query);
                }
                
                // RECORD_LIST -> Vec<Record>
                if !result_list.contains(&new_single_metric_result) { 
                    result_list.push(new_single_metric_result)
                }
            } /* for single_metric */
        } /* single_influx.status*/
    } /* all_influx.values */
    
    // BACKUP
    for key in config.metrics.keys() {
        if config.metrics[key].flag_status {
            backup_data(&config,
                        &result_list,
                        &dt.today_file_name,
                        &config.metrics[key]);
        }
    }
}
