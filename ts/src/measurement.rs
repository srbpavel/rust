use std::process::{Command};

extern crate strfmt;
use strfmt::strfmt;
use std::collections::HashMap;

use ts::TomlConfig;
use ts::Influx;
use ts::Sensor;


// /*
#[derive(Debug)]
pub struct Record {
    pub ts: i64,
    pub temperature_decimal: String,
    pub carrier: String,
    pub id: String,
    pub valid: String,
    pub machine: String,
    pub measurement: String,
    pub host: String,
}
// */


pub fn prepare_csv_record_format(config: &TomlConfig,
                                 record: &Record) -> String {

    let csv_record_template = String::from(&config.template.csv.csv_annotated);

    let mut csv_record = HashMap::new();
    csv_record.insert("measurement".to_string(), String::from(&record.measurement));
    csv_record.insert("host".to_string(), String::from(&record.host));
    csv_record.insert("machine".to_string(), String::from(&record.machine));
    csv_record.insert("sensor_carrier".to_string(), String::from(&record.carrier));
    csv_record.insert("sensor_valid".to_string(), String::from(&record.valid.to_string()));
    csv_record.insert("ts".to_string(), String::from(&record.ts.to_string()));
    csv_record.insert("sensor_id".to_string(), record.id.to_string()); // SENSOR_ID
    csv_record.insert("temperature_decimal".to_string(), String::from(&record.temperature_decimal.to_string())); // TEMPERATURE_DECIMAL

    return strfmt(&csv_record_template, &csv_record).unwrap()
}


pub fn prepare_flux_query_format(config: &TomlConfig,
                                 single_influx: &Influx,
                                 single_sensor: &Sensor,
                                 temperature_decimal: String,
                                 dtif: &String) -> String {

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
    flux.insert("measurement".to_string(), String::from(&single_influx.measurement));
    flux.insert("sensor_id".to_string(), String::from(&single_sensor.name.to_string()));
    flux.insert("temperature_decimal".to_string(), String::from(&temperature_decimal.to_string())); // NO NEED TO FILTER _field as we have only one for now
    flux.insert("dtif".to_string(), String::from(dtif)); // RFC3339 date_time format -> 2021-11-16T13:20:10.233Z
    
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
                             ts_ms: &i64) -> String {

    let lp_template = String::from(&config.template.curl.influx_lp);

    let mut lp = HashMap::new();
    lp.insert("measurement".to_string(), String::from(&influx_inst.measurement));
    lp.insert("host".to_string(), String::from(&config.host));
    lp.insert("machine_id".to_string(), String::from(&influx_inst.machine_id));
    lp.insert("sensor_carrier".to_string(), String::from(&influx_inst.carrier));
    lp.insert("sensor_valid".to_string(), String::from(&influx_inst.flag_valid_default.to_string()));
    lp.insert("ts".to_string(), String::from(ts_ms.to_string()));
    lp.insert("sensor_id".to_string(), sensor_inst.name.to_string()); // SENSOR_ID
    lp.insert("temperature_decimal".to_string(), String::from(&temperature_decimal.to_string())); // TEMPERATURE_DECIMAL

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


pub fn parse_sensors_data(config: &TomlConfig, ts_ms: i64, dtif: String) {
    // OS_CMD <- LM-SENSORS
    let sensors_stdout = os_call_sensors(&config);

    // JSON 
    let sensors_json: serde_json::Value = serde_json::from_str(&sensors_stdout).unwrap();

    // INFLUX INSTANCES
    for single_influx in &config.all_influx.values {
        if single_influx.status {

            // RESULT_LIST
            let mut result_list = Vec::new();

            let (influx_uri_write,
                 influx_uri_query,
                 influx_auth,
                 influx_accept,
                 influx_content ) = prepare_influx_format(&config, &single_influx);
            
            if config.flag.debug_influx_uri {
                println!("\n#URI:\n{}\n{}", &influx_uri_write, &influx_uri_query);
            }
            
            if config.flag.debug_influx_auth {
                println!("\n#AUTH:\n{}", &influx_auth);
            }

            // SENSOR INSTANCES
            for single_sensor in &config.all_sensors.values {
                // JSON POINTER
                let json_pointer_value = &sensors_json.pointer(&single_sensor.pointer).unwrap();
                
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

                                                                 //&sensors_json,
                                                                 json_pointer_value.to_string(),

                                                                 //&dt.ts);
                                                                 &ts_ms);

                    if config.flag.debug_influx_lp {
                        println!("\n#LINE_PROTOCOL:\n{}", single_sensor_lp);
                    }

                    // RECORD
                    let single_record = Record {
                        ts: ts_ms,
                        temperature_decimal: json_pointer_value.to_string(),
                        carrier: single_influx.carrier.to_string(),
                        id: single_sensor.name.to_string(),
                        valid: single_influx.flag_valid_default.to_string(),
                        machine: single_influx.machine_id.to_string(),
                        measurement: single_influx.measurement.to_string(),
                        host: config.host.to_string(),
                    };

                    //println!("\n#RECORD:\n{:#?}", single_record);

                    result_list.push(single_record);
                
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
                        &dtif);

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
                }
            }

            println!("\n#CSV_ANNOTATED:\n{}\n{}",
                     &config.template.csv.annotated_datatype,
                     &config.template.csv.annotated_header,
            );

            /*
            println!("{}",
                     &config.template.csv.csv_annotated,
            );
            */
            
            //println!("\n#RESULT_LIST:");
            for v in result_list {
                let csv_record = prepare_csv_record_format(&config,
                                                           &v,
                );
                //println!("{:#?}", v);
                println!("{}", csv_record);
            }
        }
    }
}
