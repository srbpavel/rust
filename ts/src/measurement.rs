use std::process::{Command};

extern crate strfmt;
use strfmt::strfmt;
use std::collections::HashMap;

pub use crate::file_config::Data;


pub fn get_sensors_data(config: &Data, ts_ms: i64) {
    // SENSOR
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
    
    let value: serde_json::Value = serde_json::from_str(&sensor_stdout_string).unwrap();

    // INFLUX
    for single_influx in &config.all_influx.values {
        println!("\n#INFLUX:
NAME: {}
STATUS: {}
SERVER: {}",
                 &single_influx.name,
                 &single_influx.status,
                 &single_influx.server,
        );  

        if single_influx.status {
        
            // URI
            let uri_template = String::from(&config.template.curl.influx_uri);
            let mut uri = HashMap::new();

            uri.insert("secure".to_string(), String::from(&single_influx.secure));
            uri.insert("server".to_string(), String::from(&single_influx.server));
            uri.insert("port".to_string(), String::from(&single_influx.port.to_string()));
            uri.insert("org".to_string(), String::from(&single_influx.org));
            uri.insert("bucket".to_string(), String::from(&single_influx.bucket));
            uri.insert("precision".to_string(), String::from(&single_influx.precision));

            if config.flag.debug_influx_uri {
                println!("\nURI: {}", strfmt(&uri_template, &uri).unwrap());
            }

            // TOKEN
            let auth_template = String::from(&config.template.curl.influx_auth);
            let mut auth = HashMap::new();
            auth.insert("token".to_string(), String::from(&single_influx.token));
    
            // LP
            let lp_template = String::from(&config.template.curl.influx_lp);

            let mut lp = HashMap::new();
            lp.insert("measurement".to_string(), String::from(&single_influx.measurement));
            lp.insert("host".to_string(), String::from(&config.host));
            lp.insert("machine_id".to_string(), String::from(&single_influx.machine_id));
            lp.insert("sensor_carrier".to_string(), String::from(&single_influx.carrier));
            lp.insert("sensor_valid".to_string(), String::from(&single_influx.flag_valid_default.to_string()));
            lp.insert("ts".to_string(), String::from(ts_ms.to_string()));
    
            // JSON POINTER
            for single_sensor in &config.all_sensors.values {
                let pointer_value = &value.pointer(&single_sensor.pointer).unwrap();
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
            
                lp.insert("sensor_id".to_string(), single_sensor.name.to_string()); // SENSOR_ID
                lp.insert("temperature_decimal".to_string(), pointer_value.to_string()); // TEMPERATURE_DECIMAL
        
                // INFLUX default + backup + ...
                if single_sensor.status {
                    if config.flag.debug_influx_lp {
                        println!("\nLINE_PROTOCOL: {}", strfmt(&lp_template, &lp).unwrap());
                    }
                    // CURL
                    let curl_output = Command::new(&config.template.curl.program)
                        .arg(&config.template.curl.param_1)
                        .arg(&config.template.curl.param_2)
                        .arg(&config.template.curl.param_3)
                        .arg(strfmt(&uri_template, &uri).unwrap()) // URI
                        .arg(&config.template.curl.param_4)
                        .arg(strfmt(&auth_template, &auth).unwrap()) // AUTH
                        .arg(&config.template.curl.param_5)
                        .arg(strfmt(&lp_template, &lp).unwrap()) // LINE_PROTOCOL
                        .output().expect("failed to execute command");

                    if config.flag.debug_influx_output {
                        
                        println!("\nstdout: {}", String::from_utf8_lossy(&curl_output.stdout));
                        println!("\nstderr: {}", String::from_utf8_lossy(&curl_output.stderr));
                    }
                }
            }
        }
        
    }
}
