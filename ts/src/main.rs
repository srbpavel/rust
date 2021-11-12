// TIMESTAMP [MS] FOR INFLUXDB
//
//#[derive(Debug)]
//
mod util;
pub use util::ts as timestamp;

mod file_config;
pub use file_config::Data;

//use std::env;
//use std::process;
//use ts::Config;

use chrono::{Datelike, Timelike};

use std::process::{Command};

//use reqwest::{Client};

//use serde_json::json;
//use serde_json::Value;

extern crate strfmt;
use strfmt::strfmt;
use std::collections::HashMap;

/*
use toml::Value;

struct FileConfig {
    work_dir: String,
    influx: Influx,
}

struct Influx{
    host: String,
    port: Option<u16>
}
*/


fn main() {
    // /*
    /* TIMESTAMP */
    // GET LOCAL TIMESTAMP
    let debug_ts: bool = false;
    let ts_ms: i64 = timestamp::ts_now(debug_ts);
    println!("\nTS: {}", ts_ms);

    // TS to DATETIME for better visual reading
    // https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html
    let debug_ts_to_dt: bool = false;
    let ts_dt = timestamp::ts_to_datetime(ts_ms, debug_ts_to_dt);
    println!("\nDT:       {:?}\nformated: {}",
             ts_dt,
             format!("{}_{:02.}_{:02.} {:02}:{:02.}:{:02.}.{:09} {} {}",
                     ts_dt.year(),
                     ts_dt.month(),
                     ts_dt.day(),
                     
                     ts_dt.hour(),
                     ts_dt.minute(),
                     ts_dt.second(),
                     ts_dt.nanosecond(),
                     
                     ts_dt.weekday(),
                     ts_dt.offset(),
             )
    );
    // */

    let config_data = Data::start();
    /*
    println!("\nCONFIG_DATA:\n{:?}\n{}\n{}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
             config_data.work_dir,
             config_data.name,
             config_data.host,
             config_data.flag,
             config_data.delay,
             config_data.influx_default,
             config_data.influx_backup,
             config_data.template,
    );
    */

    /*
    let curl_cmd = format!("curl -k --request POST \"{secure}://{server}:{port}/api/v2/write?org={org}&bucket={bucket}&precision={precision}\" --header \"Authorization: Token {token}\" --data-raw \"{measurement},host={host},Machine={machine_id},SensorId={sensor_id},SensorCarrier={sensor_carrier},SensorValid={sensor_valid} SensorDecimal={sensor_decimal} {ts}\"",
                           secure=config_data.influx_default.secure,
                           server=config_data.influx_default.server,
                           port=config_data.influx_default.port,
                           org=config_data.influx_default.org,
                           bucket=config_data.influx_default.bucket,
                           precision=config_data.influx_default.precision,

                           token=config_data.influx_default.token,
                     
                           measurement="laptop",
                           host=config_data.host,
                           
                           machine_id="spongebob",
                           sensor_id=1,
                           sensor_carrier="cargo",
                           sensor_valid="true",
                           sensor_decimal=123.456,
                           
                           ts=ts_ms,
    );
    
    
    println!("\nTEMPLATE:\n{}",
             curl_cmd
    );
    */

    /*
    let df_cmd = format!("touch x.x 1>1_df.log 2>2_df.log");

    println!("\ndf_cmd:\n{}",
             df_cmd
    );
    */


    /* SENSOR */
    let sensor_output = Command::new("/usr/bin/sensors")
        .arg("-j")
        //.arg("|")
        //.arg("egrep")
        //.arg("\"Core (0|1)\"")
        .output().expect("failed to execute command");

    let sensor_string = String::from_utf8_lossy(&sensor_output.stdout);
    // println!("sensor_string: {}", sensor_string);

    let sensor_value:serde_json::Value = serde_json::from_str(&sensor_string).unwrap();
    // println!("\nsensor_value: {}", sensor_value);

    let name_core_0 = 0;
    let name_core_1 = 1;
    let temperature_core_0 = &sensor_value["coretemp-isa-0000"]["Core 0"]["temp2_input"].to_string();
    let temperature_core_1 = &sensor_value["coretemp-isa-0000"]["Core 1"]["temp3_input"].to_string();

    println!("\nCore 0: {}", temperature_core_0);
    println!("\nCore 1: {}", temperature_core_1);
    
    //let sensor_value: Value = Value::Object(serde_json::from_str(&sensor_string));
    
    //let sensor_data  = serde_json::from_str(&sensor_string);


    /* CURL */
    let uri_template = String::from("{secure}://{server}:{port}/api/v2/write?org={org}&bucket={bucket}&precision={precision}");
    let mut uri = HashMap::new();
    uri.insert("secure".to_string(), config_data.influx_default.secure);
    uri.insert("server".to_string(), config_data.influx_default.server);
    uri.insert("port".to_string(), config_data.influx_default.port.to_string());
    uri.insert("org".to_string(), config_data.influx_default.org);
    uri.insert("bucket".to_string(), config_data.influx_default.bucket);
    uri.insert("precision".to_string(), config_data.influx_default.precision);
    
    println!("URI: {}", strfmt(&uri_template, &uri).unwrap());


    let lp_template = String::from("{measurement},host={host},Machine={machine_id},SensorId={sensor_id},SensorCarrier={sensor_carrier},SensorValid={sensor_valid} TemperatureDecimal={temperature_decimal} {ts}");

    let mut lp = HashMap::new();
    lp.insert("measurement".to_string(), String::from("temperature"));
    lp.insert("host".to_string(), config_data.host);
    lp.insert("machine_id".to_string(), String::from("spongebob"));

    lp.insert("sensor_carrier".to_string(), String::from("cargo"));
    lp.insert("sensor_valid".to_string(), String::from("true"));

    lp.insert("ts".to_string(), ts_ms.to_string());


    for single_temp in &[(name_core_0, temperature_core_0), (name_core_1, temperature_core_1)] {
        let sensor_id = single_temp.0.to_string(); // SENSOR_ID
        lp.insert("sensor_id".to_string(), sensor_id.to_string());    

        let temperature_decimal = single_temp.1;//&temperature_core_0; // TEMPERATURE_DECIMAL
        lp.insert("temperature_decimal".to_string(), temperature_decimal.to_string());
        
        println!("LINE_PROTOCOL: {}", strfmt(&lp_template, &lp).unwrap());

        let curl_output = Command::new("/usr/bin/curl")
            .arg("-k")
            .arg("--request")
            .arg("POST")
            .arg(strfmt(&uri_template, &uri).unwrap()) // URI
            .arg("--header")
            .arg(format!("Authorization: Token {token}",
                         token=config_data.influx_default.token))
            .arg("--data-raw")
            .arg(strfmt(&lp_template, &lp).unwrap())// LINE_PROTOCOL
            .output().expect("failed to execute command");
        
        //println!("\nstdout: {:?}", &output);
        println!("\nstdout: {}", String::from_utf8_lossy(&curl_output.stdout));
        println!("\nstderr: {}", String::from_utf8_lossy(&curl_output.stderr));
        
    }
    
    
    

    



    
    /*
    let output = Command::new("/usr/bin/curl")
        .arg("-k")
        .arg("--request")
        .arg("POST")
        .arg(format!("{secure}://{server}:{port}/api/v2/write?org={org}&bucket={bucket}&precision={precision}",
                     secure=config_data.influx_default.secure,
                     server=config_data.influx_default.server,
                     port=config_data.influx_default.port,
                     org=config_data.influx_default.org,
                     bucket=config_data.influx_default.bucket,
                     precision=config_data.influx_default.precision))
        .arg("--header")
        .arg(format!("Authorization: Token {token}",
                     token=config_data.influx_default.token))
        .arg("--data-raw")
        .arg(format!("{measurement},host={host},Machine={machine_id},SensorId={sensor_id},SensorCarrier={sensor_carrier},SensorValid={sensor_valid} SensorDecimal={sensor_decimal} {ts}", 
                     measurement="laptop",
                     host=config_data.host,
                     machine_id="spongebob",
                     sensor_id=1,
                     sensor_carrier="cargo",
                     sensor_valid="true",
                     sensor_decimal=123.4567,
                     ts=ts_ms))
        .output().expect("failed to execute command");

    //println!("\nstdout: {:?}", &output);
    println!("\nstdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("\nstderr: {}", String::from_utf8_lossy(&output.stderr));
    */

    /*
    // https://www.reddit.com/r/rust/comments/ndrd0b/how_to_translate_this_curl_request_into_rusts/
    let client = Client::new();
    let res = client
        .post(format!("{secure}://{server}:{port}/api/v2/write?org={org}&bucket={bucket}&precision={precision}",
                      secure=config_data.influx_default.secure,
                      server=config_data.influx_default.server,
                      port=config_data.influx_default.port,
                      org=config_data.influx_default.org,
                      bucket=config_data.influx_default.bucket,
                      precision=config_data.influx_default.precision))
        .header("Authorization", format!("Token {token}", token=config_data.influx_default.token))
        .body(format!("{measurement},host={host},Machine={machine_id},SensorId={sensor_id},SensorCarrier={sensor_carrier},SensorValid={sensor_valid} SensorDecimal={sensor_decimal} {ts}", 
                      measurement="laptop",
                      host=config_data.host,
                      machine_id="spongebob",
                      sensor_id=1,
                      sensor_carrier="cargo",
                      sensor_valid="true",
                      sensor_decimal=123.4567,
                      ts=ts_ms))
        .send();
    */

    // */

    /* TOML CONFIG
    let toml_conf_port = "INFLUX_PORT = 8086".parse::<Value>().unwrap();
    let toml_conf_host = "INFLUX_HOST = 'ruth'".parse::<Value>().unwrap();

    println!("\nTOML_CONF:\n{}{}",
             toml_conf_port,
             toml_conf_host);


    let file_config = FileConfig {
        work_dir: "/home/conan/soft/rust/ts".to_string(),
        influx: Influx {
            host: "ruth".to_string(),
            port: Some(8086),
        },
    };

    println!("\nFILE_CONFIG:\n{}\n{}:{}",
             &file_config.work_dir,
             &file_config.influx.host,
             &file_config.influx.port.unwrap());
    */

 
 
    /* CONFIG */
    /*
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err); //err RETURN_MSG from Config::new
        process::exit(1);
    });
     */

    /*
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        //eprintln!("Problem parsing arguments: {}", err);
        eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err); //err RETURN_MSG from Config::new
        process::exit(1);
    });
    

    if let Err(e) = ts::read_config(config) { //ts. je muj METYNKA
        eprintln!("\nEXIT: reading file\nREASON >>> {}", e);

        process::exit(1);
    }
    */
}
