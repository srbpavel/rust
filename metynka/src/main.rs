// CMD_ARGUMENTS
use std::env;
use std::process;

// /UTIL/TS
mod util;
use util::ts as timestamp;

// SENSORS
mod measurement;

// /VARIOUS
mod various;


#[allow(unused)]
fn main() {
    // QUICK SAMPLE TEST
    // /*
    
    // */

    /*
    various::update_vector();
    */

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
            /*
            let status = match single_influx.status {
            true => "true",
            false => "false",
            };
             */
            
            println!("INFLUX [{:5?}]: {}",
                     //status,
                     single_influx.status,
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
    
    // METRICS -> SENSORS -> influx import + csv_backup
    measurement::parse_sensors_data(&config,
                                    &dt,
    );
}
