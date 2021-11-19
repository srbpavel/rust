// CMD_ARGUMENTS
use std::env;
use std::process;

// LIB.RS
use ts as metynka;

// /UTIL/TS
mod util;
pub use util::ts as timestamp;

// SENSORS
mod measurement;

// ENUM
//use ts::{TomlConfig, Influx, TemplateSensors};
//use crate::metynka::TemplateSensors;
//use ts::TemplateSensors;

//use ts::{TemplateSensors};


fn main() {
    // COMMAND ARGS
    let cmd_args = metynka::CmdArgs::new(env::args()).unwrap_or_else(|err| {
         eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err);
        process::exit(1);
    });

    // TOML_CONFIG
    let new_config = metynka::parse_toml_config(&cmd_args).unwrap_or_else(|err| {
        eprintln!("\nEXIT: reading file\nREASON >>> {}", err);
        process::exit(1);
    });

    // HASH
    /*
    println!("TOYS:\n{:#?}", &new_config.toys.keys());

    for key in new_config.toys.keys() {
        println!("KEY:{:#?}",
                 new_config.toys[key].measurement);
    }
    
    */
    //_

    
    // EGREP
    if new_config.flag.run_egrep && new_config.flag.debug_egrep {
        if let Err(e) = metynka::read_config(cmd_args) {
            eprintln!("\nEXIT: reading file\nREASON >>> {}", e);
            process::exit(1);
        }
    }
    
    // DT Struct
    let dt = timestamp::ts_now(new_config.flag.debug_ts);

    // DEBUG: ALL_INFLUX
    if new_config.flag.debug_influx_instances {
        for single_influx in &new_config.all_influx.values {
            let status = match single_influx.status {
                true => "true",
                false => "false",
            };

            println!("INFLUX [{}]: {}",
                     status,
                     single_influx.name);
        }
    }

    // DEBUG: ALL_SENSOR
    if new_config.flag.debug_sensor_instances {
        for single_sensor in &new_config.all_sensors.values {
            let status = match single_sensor.status {
                true => "true",
                false => "false",
            };

            println!("SENSOR [{}]: {}",
                     status,
                     single_sensor.name);
        }
    }
    
    // SENSOR
    measurement::parse_sensors_data(&new_config,
                                    &dt,
    );


    //ENUM
    /*
    #[derive(Debug)]
    enum DataType {
        Temperature(TemplateSensors),
        Memory(TemplateSensors),
    }
    //_

    //let data_type: DataType = DataType::TemplateSensors;

    let temperature: DataType = DataType::Temperature(new_config.template.temperature);
    let memory: DataType = DataType::Memory(new_config.template.memory);

    /*
    println!("ENUM:\n{:#?}\n{:#?}",
             temperature,
             memory,
    );
    */
    */

    /*
    for s in [&new_config.template.temperature, &new_config.template.memory] {
        println!("\nARRAY:\n{:#?}",
                 &s.measurement,
        );
    }
    */
    
}
