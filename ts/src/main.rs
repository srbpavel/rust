// CMD_ARGUMENTS
use std::env;
use std::process;

// LIB.RS
use ts as metynka;

// UTIL/TS
mod util;
pub use util::ts as timestamp;

// SENSORS
mod measurement;

// DEBUG -> measurement
//use metynka::{TomlConfig, Influx};
//use crate::metynka::TemplateSensors;


fn main() {
    // FOR SAMPLE TEST
    /*
    enum Example {
        Data(i32),
    }
    
    let x = Example::Data(123); // wrap the data
    let Example::Data(y) = x;   // unwrap the data via a pattern
    
    dbg!(y); // prints 'y = 123'
    */
    //_

    // COMMAND ARGS
    let cmd_args = metynka::CmdArgs::new(env::args()).unwrap_or_else(|err| {
         eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err);
        process::exit(1);
    });

    // TOML_CONFIG
    let config = metynka::parse_toml_config(&cmd_args).unwrap();
    
    // EGREP
    if config.flag.run_egrep && config.flag.debug_egrep {
        if let Err(e) = metynka::read_config(cmd_args) {
            eprintln!("\nEXIT: reading file\nREASON >>> {}", e);
            process::exit(1);
        }
    }
    
    // DateTime Struct
    let dt = timestamp::ts_now(config.flag.debug_ts);

    // DEBUG: ALL_INFLUX
    if config.flag.debug_influx_instances {
        for single_influx in &config.all_influx.values {
            let status = match single_influx.status {
                true => "true",
                false => "false",
            };

            println!("INFLUX [{}]: {}",
                     status,
                     single_influx.name);
        }
    }

    // DEBUG: ALL_METRICS
    if config.flag.debug_metric_instances {
        for key in config.metrics.keys() {
            println!("\n#METRIC:\n<{n}> / {s}\n\n{m:#?}",
                     n=&config.metrics[key].measurement,
                     s=&config.metrics[key].flag_status,
                     m=&config.metrics[key],
            );
        }
    }

    // SENSOR
    measurement::parse_sensors_data(&config,
                                    &dt,
    );

}
