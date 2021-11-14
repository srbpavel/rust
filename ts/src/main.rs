// MY FIRST LESSON

// TS
mod util;
pub use util::ts as timestamp;

// SENSORS
mod measurement;

// CONFIG_HARD_CODED STRUCT
mod file_config;
pub use file_config::Data; // HARDCODED Struct

// CONFIG_ARG
use std::env;
use std::process;
use ts::Config; // METYNKA Config Struct + Impl <- ARGUMENTS

// CONFIG TOML 
use std::fs;
use toml;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct TomlConfig {
    // ROOT
    work_dir: String,
    name: String,
    host: String,

    // STRUCT
    flag: Flag,
    delay: Delay,
    template: Template,

    // VEC
    all_influx: AllInflux,
    all_sensors: AllSensors,

    //all_horses: AllHorses,
}


#[derive(Serialize, Deserialize, Debug)]
struct Flag {
    debug_config_data: bool,
    debug_ts: bool,
    debug_ts_to_dt: bool,
    debug_sensor_output: bool,
    debug_pointer_output: bool,
    debug_influx_uri: bool,
    debug_influx_lp: bool,
    debug_influx_output: bool,
}


#[derive(Serialize, Deserialize, Debug)]
struct Delay {
    second: u8,
    minute: u8,
}


#[derive(Serialize, Deserialize, Debug)]
struct AllInflux {
    /*
    default: Influx,
    backup: Influx,
     */
    values: Vec<Influx>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Influx {
    name: String,
    status: bool,
    secure: String,
    server: String,
    port: u16,

    bucket: String,
    token: String,
    org: String,
    precision: String,

    measurement: String,
    machine_id: String,
    carrier: String,
    flag_valid_default: bool,
}


#[derive(Serialize, Deserialize, Debug)]
struct Template {
    curl: TemplateCurl,
    sensors: TemplateSensors,
}


#[derive(Serialize, Deserialize, Debug)]
struct TemplateCurl {
    program: String,
    param_1: String,
    param_2: String,
    param_3: String,
    param_4: String,
    param_5: String,
    influx_uri: String,
    influx_auth: String,
    influx_lp: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct TemplateSensors {
    program: String,
    param_1: String,
}


/*
#[derive(Serialize, Deserialize, Debug)]
struct AllSensors {
    one: Sensor,
    two: Sensor,
    three: Sensor,
    four: Sensor,
}
*/


#[derive(Serialize, Deserialize, Debug)]
struct Sensor {
    status: bool,
    name: String, // mozna u8
    pointer: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct AllSensors {
    //values: [i32; 3],
    //values: Vec<i32>,
    values: Vec<Sensor>,
}


fn main() {
    let config_data = Data::start();

    if config_data.flag.debug_config_data {
        println!("\n#CONFIG_DATA:\n{:#?}",
                 config_data,
        );
    }

    /* CONFIG */
    //
    let config = Config::new(env::args()).unwrap_or_else(|err| { // LIB.RS
        //eprintln!("Problem parsing arguments: {}", err);
        eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err); //err RETURN_MSG from Config::new
        process::exit(1);
    });
    
    /* TOML */
    /* CONFIG Struct EXAMPLE
    //
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

    /*
    let fookume = "foookin = 'paavel'".parse::<Value>().unwrap();
    println!("\nTOML: {} <- {:?}",
             fookume["foookin"],
             fookume,
    );
    */
    
    let toml_file = fs::read_to_string(config.filename);
    /*
    println!("\nTOML_FILE: {:#?} <- {:?}",
             &toml_file,
             "",
    );
    */
    
    /*
    let toml_content = r#"
[flag]
debug_ts = "true"

[delay]
seconds = 60
minutes = 1
hours = 12
"#;

    let content: TomlConfig = toml::from_str(toml_content).unwrap();
    println!("\nTOML_CONTENT: {:?}\nTTT: {}",
             content,
             content.flag.debug_ts,
    );
    */

    let toml_config: TomlConfig = toml::from_str(&toml_file.unwrap()).unwrap();

    println!("\nTOML_CONFIG::\nINFLUX\n{i:#?}\nSENSOR\n{s:?}",
             s =toml_config.all_sensors,
             i = toml_config.all_influx,
    );

    // ALL_INFLUX
    for single_influx in toml_config.all_influx.values {
        if single_influx.status {
            println!("INFLUX [true]: {}",
                     single_influx.name);
        }
        else {
            println!("INFLUX [false]: {}",
                     single_influx.name);
        }
    }

    // ALL_SENSOR
    for single_sensor in toml_config.all_sensors.values {
        if single_sensor.status {
            println!("SENSOR [true]: {}",
                     single_sensor.name);
        }
        else {
            println!("SENSOR [false]: {}",
                     single_sensor.name);
        }
    }
        
    /*
    println!("\nTOML_CONFIG: {:#?}",
             toml_config,
    );
    */


    /* EGREP */
    /*
    if let Err(e) = ts::read_config(config) { //ts. je muj METYNKA
        eprintln!("\nEXIT: reading file\nREASON >>> {}", e);

        process::exit(1);
    }
    */
    
    
    /* TIMESTAMP */
    let ts_ms: i64 = timestamp::ts_now(config_data.flag.debug_ts);
    println!("\n#TS:\n{}", ts_ms);

    
    /* SENSOR */
    measurement::get_sensors_data(&config_data,
                                  ts_ms
    );
    
    /* START

    /* CALL DIRECTLY FROM rust as python.requests
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

}
