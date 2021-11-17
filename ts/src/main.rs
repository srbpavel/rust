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

    // EGREP
    if new_config.flag.run_egrep {
        if new_config.flag.debug_egrep {
            if let Err(e) = metynka::read_config(cmd_args) {
                eprintln!("\nEXIT: reading file\nREASON >>> {}", e);
                process::exit(1);
            }
        }
    }
    
    // TIMESTAMP
    let dt = timestamp::ts_now(new_config.flag.debug_ts);
    println!("\n#TS:\n{:#?}\n", dt);

    // DEBUG: ALL_INFLUX
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

    // DEBUG: ALL_SENSOR
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
    
    // SENSOR
    measurement::parse_sensors_data(&new_config,
                                    &dt,
                                    //dt.ts,
                                    //dt.local_influx_format,
                                    //dt.today_file_name,
    );
}
