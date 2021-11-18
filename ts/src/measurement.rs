use std::process;
use std::process::{Command};
// CMD GENERIC
use std::process::*; 

extern crate strfmt;
use strfmt::strfmt;
use std::collections::HashMap;

use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Write;

pub use crate::util::ts::{Dt};

//use ts::TomlConfig;
//use ts::TemplateTemperature;
use ts::Influx;
use ts::*;


// BACKUP CVS 
#[derive(Debug)]
pub struct Record {
    pub ts: i64,
    pub value: String,
    pub carrier: String,
    pub id: String,
    pub valid: String,
    pub machine: String,
    pub measurement: String,
    pub host: String,
}


// STRUCT compare via contains
impl PartialEq for Record
{
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}


#[allow(dead_code)] // when FN commented
#[allow(unused_variables)]
pub fn cmd_generic(config: &TomlConfig,
                   single_influx: &Influx,
                   dt: &Dt) -> Record {

    let generic_measurement = "memory";
    let generic_id = "memory_free_0";
    let generic_pointer_path = "/MemFree";

    let cmd_program = "/bin/cat";
    let cmd_args = vec!["/proc/meminfo"];
    let cmd = Command::new(&cmd_program)
        .args(&cmd_args)
        .stdout(Stdio::piped()).spawn().unwrap();

    /*
    println!("\n#CMD_PROGRAM:\n{:#?}\n#CMD_ARGS:\n{:#?}",
             cmd_program,
             cmd_args,
    );
    */
    
    let cmd_pipe_program = "jq";
    let cmd_pipe_args = vec!["--slurp",
                             "--raw-input",
                             "split(\"\n\") | map(select(. != \"\") | split(\":\") | {\"key\": .[0], \"value\": (.[1:]| map_values(.[0:-3]) | join(\"\") | split(\" \") | .[1:] | join(\"\"))}) | from_entries"];

    /*
    println!("\n#CMD_pipe_PROGRAM:\n{:#?}\n#CMD_pipe_ARGS:\n{:#?}",
             cmd_pipe_program,
             cmd_pipe_args,
    );
    */

    let cmd_pipe = Command::new(cmd_pipe_program)
        .args(cmd_pipe_args)
        .stdin(cmd.stdout.unwrap())
        .output().expect("failed to execute command");
    
    //println!("\n#CMD_pipe:stdout: {:#?}", String::from_utf8_lossy(&cmd_pipe.stdout));
    //println!("\n#CMD_pipe:stdERR: {:#?}", String::from_utf8_lossy(&cmd_pipe.stderr));

    let mem_info_json: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&cmd_pipe.stdout)).unwrap();
    //println!("\n#JSON:\n{:?}", mem_info_json);

    let json_pointer_value: i64 = mem_info_json.pointer(generic_pointer_path).unwrap().as_str().unwrap().parse().unwrap();
    /*
    println!("\n#POINTER:\n{}[i64]: {} kB",
             generic_pointer_path,
             json_pointer_value);
    */

    let generic_record = Record {
        ts: dt.ts,
        value: json_pointer_value.to_string(), // _value
        carrier: single_influx.carrier.to_string(),
        id: generic_id.to_string(),
        valid: single_influx.flag_valid_default.to_string(),
        machine: single_influx.machine_id.to_string(),
        measurement: generic_measurement.to_string(),
        host: config.host.to_string(),
    };

    generic_record
}


pub fn backup_data(config: &TomlConfig,
                   result_list: Vec<Record>,
                   today_file_name: &String) {

    let full_path = Path::new(&config.work_dir).join(&config.backup.dir);

    // DIR CREATE if not EXISTS
    if !full_path.exists() {
        fs::create_dir_all(&full_path).unwrap_or_else(|err| {
            eprintln!("\nEXIT: CREATE DIR failed\nREASON: >>> {}", err);
            process::exit(1);
        });
    }

    let today_file_name = full_path.join(format!("{}_{}.csv",
                                                 &today_file_name,
                                                 &config.name,
    ));

    // FILE CREATE or APPEND
    if config.flag.debug_backup {
        println!("\n#CSV_ANNOTATED:");
    }

    // format CSV HEADER
    let csv_header = prepare_csv_header_format(&config);
    
    if !today_file_name.exists() {
        let mut file = match File::create(&today_file_name) { // LEARN TO write TEST for this
            Err(why) => {
                eprintln!("\nEXIT: COULD NOT CREATE {}\nREASON: >>> {}",
                         &today_file_name.display(),
                         why);
                
                fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&today_file_name)
                    .unwrap() // NO NEED TO TEST ? AS CREATED
            },
            Ok(file) => file,
        };

        writeln!(file, "{}", &config.template.temperature.annotated_datatype).unwrap_or_else(|err| {  // TAG_ID
            eprintln!("\nEXIT: APPEND DATA to file failed\nREASON: >>> {}", err);
            process::exit(1);
        });

        writeln!(file, "{}", csv_header).unwrap_or_else(|err| {
            eprintln!("\nEXIT: APPEND DATA to file failed\nREASON: >>> {}", err);
            process::exit(1);
        });

        if config.flag.debug_backup {
            println!("{}\n{}",
                     &config.template.temperature.annotated_datatype,
                     csv_header,
            );
        }
    }
    else {
        if config.flag.debug_backup {
            println!("{}\n{}",
                     &config.template.temperature.annotated_datatype,
                     csv_header,
            );
        }
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&today_file_name)
        .unwrap();
    
    // RESULT_LIST
    for single_record in result_list {
        let csv_record = prepare_csv_record_format(&config,
                                                   &single_record,
        );

        if config.flag.debug_backup {
            println!("{}", &csv_record);
        }
        
        writeln!(file, "{}", &csv_record).unwrap_or_else(|err| {
            eprintln!("\nEXIT: APPEND DATA to file failed\nREASON: >>> {}", err);
            process::exit(1);
        });
    }
}


pub fn prepare_csv_header_format(config: &TomlConfig) -> String {

    let csv_header_template = String::from(&config.template.temperature.annotated_header);
    let mut csv_header = HashMap::new();
    csv_header.insert("tag_machine".to_string(), &config.template.temperature.tag_machine);
    csv_header.insert("tag_carrier".to_string(), &config.template.temperature.tag_carrier);
    csv_header.insert("tag_valid".to_string(), &config.template.temperature.tag_valid);
    csv_header.insert("tag_id".to_string(), &config.template.temperature.tag_id);
    csv_header.insert("field".to_string(), &config.template.temperature.field);

    return strfmt(&csv_header_template, &csv_header).unwrap()
}


pub fn prepare_csv_record_format(config: &TomlConfig,
                                 record: &Record) -> String {

    let csv_record_template = String::from(&config.template.temperature.csv_annotated);
    let mut csv_record = HashMap::new();
    csv_record.insert("measurement".to_string(), String::from(&record.measurement));
    csv_record.insert("host".to_string(), String::from(&record.host));
    csv_record.insert("machine".to_string(), String::from(&record.machine));
    csv_record.insert("carrier".to_string(), String::from(&record.carrier));
    csv_record.insert("valid".to_string(), String::from(&record.valid.to_string()));
    csv_record.insert("ts".to_string(), String::from(&record.ts.to_string()));
    csv_record.insert("id".to_string(), record.id.to_string());
    csv_record.insert("value".to_string(), String::from(&record.value.to_string()));

    return strfmt(&csv_record_template, &csv_record).unwrap()
}


pub fn prepare_generic_flux_query_format(config: &TomlConfig,
                                         single_influx: &Influx,
                                         generic_record: &Record,
                                         utc_influx_format: &String) -> String {

    let flux_template = match config.flag.add_flux_query_verify_record_suffix {
        true => format!("{}{}",
                String::from(&config.template.temperature.generic_query_verify_record),
                String::from(&config.template.flux.query_verify_record_suffix),
        ),
        false => String::from(&config.template.temperature.generic_query_verify_record)
    };    
    
    let mut flux = HashMap::new();
    flux.insert("tag_carrier".to_string(), String::from(&config.template.temperature.tag_carrier));
    flux.insert("tag_valid".to_string(), String::from(&config.template.temperature.tag_valid));
    flux.insert("tag_id".to_string(), String::from(&config.template.temperature.tag_id));
    
    flux.insert("bucket".to_string(), String::from(&single_influx.bucket));
    flux.insert("start".to_string(), String::from(&config.template.flux.query_verify_record_range_start));
    flux.insert("measurement".to_string(), String::from(&config.template.temperature.measurement));

    // COMPARE only id + time
    flux.insert("id".to_string(), String::from(&generic_record.id.to_string()));
    flux.insert("dtif".to_string(), String::from(utc_influx_format)); // rfc3339 Date_Time Influx Format -> 2021-11-16T13:20:10.233Z
  
    return strfmt(&flux_template, &flux).unwrap()
}


pub fn os_call_curl_flux(config: &TomlConfig,
                         influx_uri: &String,
                         influx_auth: &String,
                         influx_header_accept: &String,
                         influx_header_content: &String,
                         influx_query: &String) {

    let curl_output = Command::new(&config.template.curl.program)
        .arg(&config.template.curl.param_insecure)
        .arg(&config.template.curl.param_request)
        .arg(&config.template.curl.param_post)
        .arg(influx_uri) // #URI
        .arg(&config.template.curl.param_header)
        .arg(influx_auth) // #AUTH
        .arg(&config.template.curl.param_header)
        .arg(influx_header_accept)
        .arg(&config.template.curl.param_header)
        .arg(influx_header_content)
        .arg(&config.template.curl.param_data)
        .arg(influx_query) // #QUERY
        .output().expect("failed to execute command");

    if config.flag.debug_flux_result {
        // NO stderr for now
        println!("\n#QUERY_RESULT:stdout: {}", String::from_utf8_lossy(&curl_output.stdout));
    }
}


pub fn os_call_sensors(config: &TomlConfig) -> String {
    
    //let sensor_output = Command::new(&config.template.sensors.program)
    let sensor_output = Command::new(&config.template.temperature.program)
    //let sensor_output = Command::new(&program)

        //.arg(&config.template.sensors.param_1)
        //.args(&config.template.sensors.args)
        .args(&config.template.temperature.args)
        //.args(&args)


        .output().expect("failed to execute command");
    
    let sensor_stdout_string = String::from_utf8_lossy(&sensor_output.stdout);
    let sensor_stderr_string = String::from_utf8_lossy(&sensor_output.stderr);

    if config.flag.debug_sensor_output {
        println!("\n#SENSOR:
stdout: {}
stderr: {}",
                 sensor_stdout_string,
                 sensor_stderr_string,
        );
    }

    return sensor_stdout_string.to_string()
}


pub fn os_call_curl(config: &TomlConfig,
                   influx_uri: &String,
                   influx_auth: &String,
                   single_sensor_lp: &String) {

    let curl_output = Command::new(&config.template.curl.program)
        .arg(&config.template.curl.param_insecure)
        .arg(&config.template.curl.param_request)
        .arg(&config.template.curl.param_post)
        .arg(influx_uri) // #URI
        .arg(&config.template.curl.param_header)
        .arg(influx_auth) // #AUTH
        .arg(&config.template.curl.param_data)
        .arg(single_sensor_lp) // #LINE_PROTOCOL
        .output().expect("failed to execute command");

    if config.flag.debug_influx_output {
        println!("\nstdout: {}", String::from_utf8_lossy(&curl_output.stdout));
        println!("\nstderr: {}", String::from_utf8_lossy(&curl_output.stderr));
    
    }
}


pub fn prepare_generic_lp_format(config: &TomlConfig,
                                 //influx_inst: &Influx, // TO DEL
                                 generic_record: &Record)  -> String {

    let generic_lp_template = String::from(&config.template.temperature.generic_lp);
    let mut generic_lp = HashMap::new();
    generic_lp.insert("tag_machine".to_string(), String::from(&config.template.temperature.tag_machine));
    generic_lp.insert("tag_carrier".to_string(), String::from(&config.template.temperature.tag_carrier));
    generic_lp.insert("tag_valid".to_string(), String::from(&config.template.temperature.tag_valid));
    generic_lp.insert("tag_id".to_string(), String::from(&config.template.temperature.tag_id));
    generic_lp.insert("field".to_string(), String::from(&config.template.temperature.field));
    
    generic_lp.insert("measurement".to_string(), String::from(&generic_record.measurement));
    generic_lp.insert("host".to_string(), String::from(&generic_record.host));
    generic_lp.insert("machine_id".to_string(), String::from(&generic_record.machine));
    generic_lp.insert("sensor_carrier".to_string(), String::from(&generic_record.carrier));
    generic_lp.insert("sensor_valid".to_string(), String::from(&generic_record.valid));
    generic_lp.insert("ts".to_string(), String::from(&generic_record.ts.to_string()));
    generic_lp.insert("sensor_id".to_string(), String::from(&generic_record.id));
    generic_lp.insert("temperature_decimal".to_string(), String::from(&generic_record.value.to_string()));

    return strfmt(&generic_lp_template, &generic_lp).unwrap()
}


pub fn prepare_influx_format(config: &TomlConfig,
                             influx_inst: &Influx) -> (String, String, String, String, String) {

    // URI_WRITE 
    let uri_write_template = String::from(format!("{}{}",
                                                  &config.template.curl.influx_uri_api,
                                                  &config.template.curl.influx_uri_write,
    ));
    
    let mut uri_data = HashMap::new();
    uri_data.insert("secure".to_string(), String::from(&influx_inst.secure));
    uri_data.insert("server".to_string(), String::from(&influx_inst.server));
    uri_data.insert("port".to_string(), String::from(&influx_inst.port.to_string()));
    uri_data.insert("org".to_string(), String::from(&influx_inst.org));
    uri_data.insert("bucket".to_string(), String::from(&influx_inst.bucket));
    uri_data.insert("precision".to_string(), String::from(&influx_inst.precision));

    // URI_QUERY
    let uri_query_template = String::from(format!("{}{}",
                                                  &config.template.curl.influx_uri_api,
                                                  &config.template.curl.influx_uri_query,
    ));
    
    let mut uri_query_data = HashMap::new();
    uri_query_data.insert("secure".to_string(), String::from(&influx_inst.secure));
    uri_query_data.insert("server".to_string(), String::from(&influx_inst.server));
    uri_query_data.insert("port".to_string(), String::from(&influx_inst.port.to_string()));
    uri_query_data.insert("org".to_string(), String::from(&influx_inst.org));
    
    // AUTH
    let auth_template = String::from(&config.template.curl.influx_auth);
    let mut auth_data = HashMap::new();
    auth_data.insert("token".to_string(), String::from(&influx_inst.token));

    // ACCEPT
    let accept_template = String::from(&config.template.curl.influx_accept);

    // CONTENT
    let content_template = String::from(&config.template.curl.influx_content);

    (strfmt(&uri_write_template, &uri_data).unwrap(),
     strfmt(&uri_query_template, &uri_query_data).unwrap(),
     strfmt(&auth_template, &auth_data).unwrap(),
     accept_template,
     content_template,
    )
}


//#[allow(unused_variables)]
pub fn parse_sensors_data(config: &TomlConfig,
                          dt: &Dt) {

    // OS_CMD <- LM-SENSORS
    let sensors_stdout = os_call_sensors(&config);

    // RESULT_LIST
    let mut result_list: Vec<Record> = Vec::new();
    
    // JSON 
    let sensors_json: serde_json::Value = serde_json::from_str(&sensors_stdout).unwrap();

    // INFLUX INSTANCES
    for single_influx in &config.all_influx.values {
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

            // CMD_GENERIC -> START
            /*
            let mem_generic_record = cmd_generic(&config,
                                                 &single_influx,
                                                 dt);

            println!("\n#MEM @ GENERIC_RECORD:\n{:#?}", mem_generic_record);

            let mem_generic_lp = prepare_generic_lp_format(&config,
                                                           &single_influx,
                                                           &mem_generic_record);

            println!("\n#MEM @ GENERIC_LP:\n{:#?}", mem_generic_lp);
            */
            // CMD_GENERIC -> END

            let mut group_temperature = Vec::new();
            let mut group_memory = Vec::new();

            for v in &config.all_sensors.values {
                if v.group == "temperature" && v.status {
                    //println!("TEMP: {}", v.group);
                    group_temperature.push(v);
                }
                else if v.group == "memory" && v.status {
                    //println!("MEM: {}", v.group);
                    group_memory.push(v);
                }
            }
            
            println!("\n#TEMP: {:?}", group_temperature);
            println!("\n#MEM: {:?}", group_memory);
            
            
            // SENSOR INSTANCES
            //for single_sensor in &config.all_sensors.values {
            for single_sensor in group_temperature {
                
                // JSON single POINTER
                let json_pointer_value = &sensors_json.pointer(&single_sensor.pointer).unwrap();

                // DEBUG true/false SENSORS
                if config.flag.debug_pointer_output {
                    println!("
#POINTER_CFG:
status: {s}
name: {n}
pointer: {p}
value: {v}",
                             s=single_sensor.status,
                             n=single_sensor.name,
                             p=single_sensor.pointer,
                             v=json_pointer_value,
                    );
                }

                if single_sensor.status {
                    // RECORD
                    let single_record = Record {
                        ts: dt.ts,
                        value: json_pointer_value.to_string(),
                        carrier: single_influx.carrier.to_string(),
                        id: single_sensor.name.to_string(),
                        valid: single_influx.flag_valid_default.to_string(),
                        machine: single_influx.machine_id.to_string(),
                        measurement: config.template.temperature.measurement.to_string(),
                        host: config.host.to_string(),
                    };

                    // LP via Record
                    let generic_lp = prepare_generic_lp_format(&config,
                                                               //&single_influx, TO DEL 
                                                               &single_record);


                    if config.flag.debug_influx_lp {
                        println!("\n#GENERIC_LP:\n{}", generic_lp);
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
                            &single_record,
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
                    if !result_list.contains(&single_record) { 
                        result_list.push(single_record)
                    }

                } /* single_sensor.status */
            } /* all_sensors.values */
        } /* single_influx.status*/
    } /* all_influx.values */

    // BACKUP
    backup_data(&config,
                result_list,
                &dt.today_file_name)

}
