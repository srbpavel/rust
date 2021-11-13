// MY FIRST LESSON

// TS
mod util;
pub use util::ts as timestamp;

// SENSORS
mod measurement;

// CONFIG_HARD_CODED STRUCT
mod file_config;
pub use file_config::Data;

// CONFIG_ARG
use std::env;
use std::process;
use ts::Config;

// CONFIG TOML 
use std::fs;
use toml;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Ccc {
    flag: Flag,
    delay: Delay,
}

#[derive(Serialize, Deserialize, Debug)]
struct Flag {
    debug_ts: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Delay {
    seconds: u32,
    minutes: u32,
    //hours: u32,
}


fn main() {
    let config_data = Data::start();

    if config_data.flag.debug_config_data {
        println!("\n#CONFIG_DATA:\n{:#?}\n >>> {:#?}",
                 config_data,
                 "" // config_data.all_sensors,
        );
    }

    
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

    /* TOML CONFIG
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
    // /*
    let config = Config::new(env::args()).unwrap_or_else(|err| { // LIB.RS
        //eprintln!("Problem parsing arguments: {}", err);
        eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err); //err RETURN_MSG from Config::new
        process::exit(1);
    });
    

    /*
    if let Err(e) = ts::read_config(config) { //ts. je muj METYNKA
        eprintln!("\nEXIT: reading file\nREASON >>> {}", e);

        process::exit(1);
    }
    */
    // */

    /* TOML */
    /*
    let fookume = "foookin = 'paavel'".parse::<Value>().unwrap();
    println!("\nTOML: {} <- {:?}",
             fookume["foookin"],
             fookume,
    );
    */

   
    let toml_file = fs::read_to_string(config.filename);
    println!("\nTOML_FILE: {:#?} <- {:?}",
             &toml_file,
             "",
    );

    let toml_content = r#"
[flag]
debug_ts = "true"

[delay]
seconds = 60
minutes = 1
hours = 12
"#;

    // /*
    let content: Ccc = toml::from_str(toml_content).unwrap();
    println!("\nTOML_CONTENT: {:?}\nTTT: {}",
             content,
             content.flag.debug_ts,
    );
    // */

    // /*
    //let toml_config: Ccc = toml::from_str(&toml_file);
    let toml_config: Ccc = toml::from_str(&toml_file.unwrap()).unwrap();
    //let toml_config: Result<Ccc, ::toml::de::Error> = toml::from_str(&toml_file);
    println!("\nTOML_CONFIG: {:?}",
             toml_config,
    );
    // */
}
