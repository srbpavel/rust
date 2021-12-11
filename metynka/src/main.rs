// CMD_ARGUMENTS
use std::env;
use std::process;

// /UTIL/TS
mod util;
use util::ts as timestamp;

// /INFLUX_DB
mod influxdb;

// SENSORS
mod measurement;

// /VARIOUS
mod various;

// /UTIL/FILE_SYSTEM
use metynka::{TomlConfig};


#[allow(unused)]
fn main() {
    // DateTime Struct
    let dt = timestamp::ts_now();
    println!("{}", dt.local_formated);

    // QUICK SAMPLE TEST

    /*
    various::bin_shift(1024,
                       //"left",
                       "right",
                       7,
                       true, //false
    );

    various::bin_shift(8,
                       "left",
                       //"right",
                       8,
                       true, //false,
    );
    */

    // /*
    //various::update_vector();
    // */

    // COMMAND ARGS
    let cmd_args = metynka::CmdArgs::new(env::args()).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err);
        
        process::exit(1);
    });

    // TOML_CONFIG
    let config = match metynka::parse_toml_config(&cmd_args) {
        Ok(config) => config,

        Err(why) => {
            eprintln!("\nERROR: parsing config\nREASON >>> {}", why);

            process::exit(1);
        }
    };

    // DEBUG DateTime Struct
    if config.flag.debug_ts {
        println!("\n#DATE_TIME:\n{:#?}", dt);
    }
    
    // EGREP
    if config.flag.run_egrep && config.flag.debug_egrep {
        if let Err(e) = metynka::read_config(cmd_args) {
            eprintln!("\nEXIT: reading file\nREASON >>> {}", e);

            process::exit(1);
        }
    }

    // DEBUG: ALL_INFLUX
    if config.flag.debug_influx_instances {
        for single_influx in &config.all_influx.values {
            println!("INFLUX [{:13}]: {}",
                     format!("status: {}", single_influx.status), // just playin: instead concat strings
                     single_influx.name);
        }
    }
    
    // DEBUG: ALL_METRICS config
    if config.flag.debug_metric_instances {
        for key in config.metrics.keys() {
            println!("\n#METRIC:\n<{n}> / {s}\n\n{m:#?}",
                     n=&config.metrics[key].measurement,
                     s=&config.metrics[key].flag_status,
                     m=&config.metrics[key],
            );
        }
    }
    
    // SENSORS
    measurement::parse_sensors_data(&config,
                                    &dt,
    );

    // QUICK SAMPLE TEST
    /*
    various::easy_email(&config,
                        "rust::metik",
                        "wonka",
                        false, //true,
    );
    */
}
