use std::process::{Command};

extern crate strfmt;
use strfmt::strfmt;
use std::collections::HashMap;

use ts::TomlConfig;
use ts::Influx;
use ts::Sensor;


pub fn os_call_sensors(config: &TomlConfig) -> serde_json::Value {
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

    // JSON 
    let value: serde_json::Value = serde_json::from_str(&sensor_stdout_string).unwrap();

    return value
}


pub fn os_call_curl(config: &TomlConfig,
                   influx_uri: &String,
                   influx_auth: &String,
                   single_sensor_lp: &String) {

    let curl_output = Command::new(&config.template.curl.program)
        .arg(&config.template.curl.param_1)
        .arg(&config.template.curl.param_2)
        .arg(&config.template.curl.param_3)
        .arg(influx_uri) // #URI
        .arg(&config.template.curl.param_4)
        .arg(influx_auth) // #AUTH
        .arg(&config.template.curl.param_5)
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
                             sensor_value: &serde_json::Value,
                             ts_ms: &i64) -> String {

    let lp_template = String::from(&config.template.curl.influx_lp);

    let mut lp = HashMap::new();
    lp.insert("measurement".to_string(), String::from(&influx_inst.measurement));
    lp.insert("host".to_string(), String::from(&config.host));
    lp.insert("machine_id".to_string(), String::from(&influx_inst.machine_id));
    lp.insert("sensor_carrier".to_string(), String::from(&influx_inst.carrier));
    lp.insert("sensor_valid".to_string(), String::from(&influx_inst.flag_valid_default.to_string()));
    lp.insert("ts".to_string(), String::from(ts_ms.to_string()));
    
    let pointer_value = &sensor_value.pointer(&sensor_inst.pointer).unwrap();
    lp.insert("sensor_id".to_string(), sensor_inst.name.to_string()); // SENSOR_ID
    lp.insert("temperature_decimal".to_string(), pointer_value.to_string()); // TEMPERATURE_DECIMAL

    return strfmt(&lp_template, &lp).unwrap()
}


pub fn prepare_influx_format(config: &TomlConfig,
                             influx_inst: &Influx) -> (String, String) {

    // URI
    let uri_template = String::from(&config.template.curl.influx_uri);
    let mut uri_data = HashMap::new();
    
    uri_data.insert("secure".to_string(), String::from(&influx_inst.secure));
    uri_data.insert("server".to_string(), String::from(&influx_inst.server));
    uri_data.insert("port".to_string(), String::from(&influx_inst.port.to_string()));
    uri_data.insert("org".to_string(), String::from(&influx_inst.org));
    uri_data.insert("bucket".to_string(), String::from(&influx_inst.bucket));
    uri_data.insert("precision".to_string(), String::from(&influx_inst.precision));

    // AUTH
    let auth_template = String::from(&config.template.curl.influx_auth);
    let mut auth_data = HashMap::new();
    auth_data.insert("token".to_string(), String::from(&influx_inst.token));

    (strfmt(&uri_template, &uri_data).unwrap(),
     strfmt(&auth_template, &auth_data).unwrap())
}


pub fn parse_sensors_data(config: &TomlConfig, ts_ms: i64) {
    // OS_CMD <- LM-SENSORS
    let sensors_json = os_call_sensors(&config);

    // INFLUX INSTANCES
    for single_influx in &config.all_influx.values {
        if single_influx.status {
            let (influx_uri, influx_auth) = prepare_influx_format(&config, &single_influx);
            
            if config.flag.debug_influx_uri {
                println!("\n#URI:\n{}", &influx_uri);
            }
            
            if config.flag.debug_influx_auth {
                println!("\n#AUTH:\n{}", &influx_auth);
            }

            if config.flag.debug_influx_lp {
                println!("\n#LINE_PROTOCOL:");
            }

            // SENSOR INSTANCES
            for single_sensor in &config.all_sensors.values {
                // JSON POINTER
                let pointer_value = &sensors_json.pointer(&single_sensor.pointer).unwrap();
                
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
                             v=pointer_value,
                    );
                }

                if single_sensor.status {
                    let single_sensor_lp = prepare_sensor_format(&config,
                                                                 &single_influx,
                                                                 &single_sensor,
                                                                 &sensors_json,
                                                                 &ts_ms);

                    if config.flag.debug_influx_lp {
                        println!("{}", single_sensor_lp);
                    }


                    // OS_CMD <- CURL
                    os_call_curl(&config,
                                 &influx_uri,
                                 &influx_auth,
                                 &single_sensor_lp);
                }
            }
        }
    }
}
