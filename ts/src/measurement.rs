use std::process::{Command};

extern crate strfmt;
use strfmt::strfmt;
use std::collections::HashMap;

//mod file_config;
pub use crate::file_config::Data;


pub fn mmm() -> i64 {
    return 666
}


//pub fn get_sensors_data(config: &Data) -> &String{
pub fn get_sensors_data(config: &Data, ts_ms: i64) {

    let sensor_output = Command::new(&config.template.sensors.program)
        .arg(&config.template.sensors.param_1)
        .output().expect("failed to execute command");

    let sensor_stdout_string = String::from_utf8_lossy(&sensor_output.stdout);
    let sensor_stderr_string = String::from_utf8_lossy(&sensor_output.stderr);

    println!("\n#MMM:
stdout: {}
stderr: {:?}",
             sensor_stdout_string,
             sensor_stderr_string,
    );
    
    let value: serde_json::Value = serde_json::from_str(&sensor_stdout_string).unwrap();

    // URI
    let uri_template = String::from(&config.template.curl.influx_uri);
    let mut uri = HashMap::new();

    uri.insert("secure".to_string(), String::from(&config.influx_default.secure));
    uri.insert("server".to_string(), String::from(&config.influx_default.server));
    uri.insert("port".to_string(), String::from(&config.influx_default.port.to_string()));
    uri.insert("org".to_string(), String::from(&config.influx_default.org));
    uri.insert("bucket".to_string(), String::from(&config.influx_default.bucket));
    uri.insert("precision".to_string(), String::from(&config.influx_default.precision));
    //println!("URI: {}", strfmt(&uri_template, &uri).unwrap());

    // TOKEN
    let auth_template = String::from(&config.template.curl.influx_auth);
    let mut auth = HashMap::new();
    auth.insert("token".to_string(), String::from(&config.influx_default.token));
    
    // LP
    let lp_template = String::from(&config.template.curl.influx_lp);

    let mut lp = HashMap::new();
    lp.insert("measurement".to_string(), String::from(&config.influx_default.measurement));
    lp.insert("host".to_string(), String::from(&config.host));
    lp.insert("machine_id".to_string(), String::from(&config.influx_default.machine_id));
    lp.insert("sensor_carrier".to_string(), String::from(&config.influx_default.carrier));
    lp.insert("sensor_valid".to_string(), String::from(&config.influx_default.flag_valid_default.to_string()));
    lp.insert("ts".to_string(), String::from(ts_ms.to_string()));
    
    // POINTER
    for single_sensor in &config.all_sensors.values {
        let pointer_value = &value.pointer(&single_sensor.pointer).unwrap();
        
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

        lp.insert("sensor_id".to_string(), single_sensor.name.to_string()); // SENSOR_ID
        lp.insert("temperature_decimal".to_string(), pointer_value.to_string()); // TEMPERATURE_DECIMAL
        

        // INFLUX IMPORT
        if single_sensor.status {
            println!("\nLINE_PROTOCOL: {}", strfmt(&lp_template, &lp).unwrap());
            
            /* CURL */
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

            /*
            let curl_output = Command::new("/usr/bin/curl")
            .arg("-k")
            .arg("--request")
            .arg("POST")
            .arg(strfmt(&uri_template, &uri).unwrap()) // URI
            .arg("--header")
            .arg(strfmt(&token_template, &token).unwrap()) // TOKEN
            .arg("--data-raw")
            .arg(strfmt(&lp_template, &lp).unwrap())// LINE_PROTOCOL
            .output().expect("failed to execute command");
             */
        
            //println!("\nstdout: {:?}", &output);
            println!("\nstdout: {}", String::from_utf8_lossy(&curl_output.stdout));
            println!("\nstderr: {}", String::from_utf8_lossy(&curl_output.stderr));
        }
    }

}


/*
pub fn ts_now(debug: bool) -> i64 {
    //MOZNA VRACET Struct { datetime, ts }

    let local = Local::now();
    let ts: i64 = local.timestamp_millis();

    if debug {

        let local_formated = format!("{}_{:02.}_{:02.} {:02}:{:02.}:{:02.}.{:09} {} {}",
                                     local.year(),
                                     local.month(),
                                     local.day(),
                                     
                                     local.hour(),
                                     local.minute(),
                                     local.second(),
                                     local.nanosecond(),
                                     
                                     local.weekday(),
                                     local.offset(),
        );

        println!("
#DATE_TIME:
local:    {l}
formated: {l_formated}
sec:    {l_ts_sec}
ms:     {l_ts_ms}",
    		 l=local,
		 l_ts_sec=local.timestamp(),
		 l_ts_ms=ts,
                 l_formated=local_formated
        );
    }
    return ts
}
 */
