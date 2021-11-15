// MY FIRST LESSON

// TS
mod util;
pub use util::ts as timestamp;

// SENSORS
mod measurement;

// CONFIG_ARG
use std::env;
use std::process;
//use ts::Config; // METYNKA Config Struct + Impl <- ARGUMENTS
use ts as metynka; // METYNKA Config Struct + Impl <- ARGUMENTS


fn main() {
    /* CONFIG ARG */
    let config = metynka::Config::new(env::args()).unwrap_or_else(|err| {
         eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err);
        process::exit(1);
    });

    /* TOML CONFIG */
    /*
    if let Err(e) = ts::parse_toml_config(config) { //ts. je muj METYNKA
        eprintln!("\nEXIT: reading file\nREASON >>> {}", e);

        process::exit(1);
    }
    */

    let new_config = ts::parse_toml_config(config).unwrap_or_else(|err| { // LIB.RS
        eprintln!("\nEXIT: reading file\nREASON >>> {}", err);
        process::exit(1);
    });

    /*
    let new_config = ts::parse_toml_config(config).unwrap(); // pac REF

    if new_config.flag.debug_new_config {
        println!("\nNEW_CONFIG: {:#?}",
                 new_config,
        );
    }
    */

    /*
    println!("\nTOML_CONFIG::\nINFLUX\n{i:#?}\nSENSOR\n{s:?}",
             s = new_config.all_sensors,
             i = new_config.all_influx,
    );
    */

    // ALL_INFLUX
    if new_config.flag.debug_influx_instances {
        for single_influx in &new_config.all_influx.values {
            if single_influx.status {
                println!("INFLUX [true]: {}",
                         single_influx.name);
            }
            else {
                println!("INFLUX [false]: {}",
                         single_influx.name);
            }
        }
    }

    // ALL_SENSOR
    if new_config.flag.debug_sensor_instances {
        for single_sensor in &new_config.all_sensors.values {
            if single_sensor.status {
                println!("SENSOR [true]: {}",
                         single_sensor.name);
            }
            else {
                println!("SENSOR [false]: {}",
                         single_sensor.name);
            }
        }
    }
    // */
    
    
    /* CONFIR.RS */
    //let toml_config = config::parse_toml_config(&config);
    //config::parse_toml_config(&config);

    //XXX parse_toml_config(&config.filename);
    
    
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
    
    // #RS let toml_file = fs::read_to_string(config.filename);
    
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

    /* #RS TOML START
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
    #RS TOML END */
    
        
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
    let ts_ms: i64 = timestamp::ts_now(new_config.flag.debug_ts);
    println!("\n#TS:\n{}", ts_ms);

    
    /* SENSOR */
    //measurement::get_sensors_data(&config_data,
    measurement::get_sensors_data(&new_config,
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
