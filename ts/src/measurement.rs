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

use ts::TomlConfig;
use ts::Influx;
use ts::Sensor;


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

        writeln!(file, "{}", &config.template.csv.annotated_datatype).unwrap_or_else(|err| {  // TAG_ID
            eprintln!("\nEXIT: APPEND DATA to file failed\nREASON: >>> {}", err);
            process::exit(1);
        });

        writeln!(file, "{}", csv_header).unwrap_or_else(|err| {
            eprintln!("\nEXIT: APPEND DATA to file failed\nREASON: >>> {}", err);
            process::exit(1);
        });

        if config.flag.debug_backup {
            println!("{}\n{}",
                     &config.template.csv.annotated_datatype,
                     csv_header,
            );
        }
    }
    else {
        if config.flag.debug_backup {
            println!("{}\n{}",
                     &config.template.csv.annotated_datatype,
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

    let csv_header_template = String::from(&config.template.csv.annotated_header);
    let mut csv_header = HashMap::new();
    csv_header.insert("tag_machine".to_string(), &config.template.csv.tag_machine);
    csv_header.insert("tag_carrier".to_string(), &config.template.csv.tag_carrier);
    csv_header.insert("tag_valid".to_string(), &config.template.csv.tag_valid);
    csv_header.insert("tag_id".to_string(), &config.template.csv.tag_id);
    csv_header.insert("field".to_string(), &config.template.csv.field);

    return strfmt(&csv_header_template, &csv_header).unwrap()
}


pub fn prepare_csv_record_format(config: &TomlConfig,
                                 record: &Record) -> String {

    let csv_record_template = String::from(&config.template.csv.csv_annotated);
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


pub fn prepare_flux_query_format(config: &TomlConfig,
                                 single_influx: &Influx,
                                 single_sensor: &Sensor,
                                 temperature_decimal: String,
                                 utc_influx_format: &String) -> String {

    let flux_template = match config.flag.add_flux_query_verify_record_suffix {
        true => format!("{}{}",
                String::from(&config.template.flux.query_verify_record),
                String::from(&config.template.flux.query_verify_record_suffix),
        ),
        false => String::from(&config.template.flux.query_verify_record)
    };    
    
    let mut flux = HashMap::new();
    flux.insert("bucket".to_string(), String::from(&single_influx.bucket));
    flux.insert("start".to_string(), String::from(&config.template.flux.query_verify_record_range_start));
    flux.insert("measurement".to_string(), String::from(&config.all_sensors.measurement));

    // GENERIC to CHANGE
    flux.insert("sensor_id".to_string(), String::from(&single_sensor.name.to_string()));
    flux.insert("temperature_decimal".to_string(), String::from(&temperature_decimal.to_string())); // NO NEED TO FILTER _field as we have only one for now

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
    let sensor_output = Command::new(&config.template.sensors.program)
        .arg(&config.template.sensors.param_1)
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


pub fn prepare_sensor_format(config: &TomlConfig,
                             influx_inst: &Influx,
                             sensor_inst: &Sensor,
                             temperature_decimal: String,
                             ts: i64)  -> String {

    let lp_template = String::from(&config.template.curl.influx_lp);
    let mut lp = HashMap::new();
    lp.insert("measurement".to_string(), String::from(&config.all_sensors.measurement));
    lp.insert("host".to_string(), String::from(&config.host));
    lp.insert("machine_id".to_string(), String::from(&influx_inst.machine_id));
    lp.insert("sensor_carrier".to_string(), String::from(&influx_inst.carrier));
    lp.insert("sensor_valid".to_string(), String::from(&influx_inst.flag_valid_default.to_string()));
    lp.insert("ts".to_string(), String::from(ts.to_string()));
    lp.insert("sensor_id".to_string(), sensor_inst.name.to_string());
    lp.insert("temperature_decimal".to_string(), String::from(&temperature_decimal.to_string()));

    return strfmt(&lp_template, &lp).unwrap()
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
    let mut result_list:Vec<Record> = Vec::new();
    
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
            let generic_record = cmd_generic(&config,
                                             &single_influx,
                                             dt);
            println!("\n#GENERIC_RECORD:\n{:#?}", generic_record);
            // CMD_GENERIC -> END
            
            // SENSOR INSTANCES
            for single_sensor in &config.all_sensors.values {

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
                    let single_sensor_lp = prepare_sensor_format(&config,
                                                                 &single_influx,
                                                                 &single_sensor,
                                                                 json_pointer_value.to_string(),
                                                                 dt.ts);

                    if config.flag.debug_influx_lp {
                        println!("\n#LINE_PROTOCOL:\n{}", single_sensor_lp);
                    }

                    // RECORD // pouzit record v LP
                    let single_record = Record {
                        ts: dt.ts,
                        value: json_pointer_value.to_string(),
                        carrier: single_influx.carrier.to_string(),
                        id: single_sensor.name.to_string(),
                        valid: single_influx.flag_valid_default.to_string(),
                        machine: single_influx.machine_id.to_string(),
                        measurement: config.all_sensors.measurement.to_string(),
                        host: config.host.to_string(),
                    };

                    // RECORD -> Vec<Record>
                    if !result_list.contains(&single_record) { 
                        result_list.push(single_record)
                    }
                
                    // OS_CMD <- CURL
                    os_call_curl(&config,
                                 &influx_uri_write,
                                 &influx_auth,
                                 &single_sensor_lp);


                    // OS_CMD <- FLUX_QUERY
                    let influx_query = prepare_flux_query_format(
                        &config,
                        &single_influx,
                        &single_sensor,
                        json_pointer_value.to_string(),
                        &dt.utc_influx_format);

                    if config.flag.debug_flux_query {
                        println!("\n#QUERY:\n{}",
                                 influx_query,
                        );
                    }
                    
                    if config.flag.run_flux_verify_record {
                        os_call_curl_flux(&config,
                                          &influx_uri_query,
                                          &influx_auth,
                                          &influx_accept,
                                          &influx_content,
                                          &influx_query);
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
