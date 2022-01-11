use std::env;
use std::process;

mod settings;
mod egrep;
mod influxdb;
mod measurement;
mod various;
mod util;
use util::ts as timestamp;


/*
fn mut_with_no_output(input: &u8, output: &mut u8) {
    if *input == 1 {
        *output = 10;
    }
    if *input != 1 {
        *output = 100;
    }
}
*/


//#[allow(unused)]
fn main() {
    // DateTime Struct
    let dt = timestamp::ts_now();
    println!("{}", dt.local_formated);

    // QUICK SAMPLE TEST

    /*
    for i in 0..=8 {
        various::test_parity(i);
    }
    */

    
    /* FN CHANGE MUTABLE WITHOUT RETURN
    let input = 2;
    let mut out: u8 = 1;
    println!("out_before: {}", &out);

    mut_with_no_output(&input,
                       &mut out,
    );
    
    println!("out_after: {}", out);
    */

    
    /*
    let x: usize;
    x = 6;
    */

    /* STACK || HEAP -> COPY / MOVE 
    let x1 = 42; // stack
    let y1 = Box::new(84); // heap

    {
        let z = (x1, // copied
                 y1, // moved
        );
    }

    let x2 = x1;
    // let y2 = y1; // err MOVED
    */


    
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

    // /* // HORSE_example
    //various::update_vector();
    // */

    // COMMAND ARGS
    let cmd_args = settings::CmdArgs::new(env::args()).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err);
        
        process::exit(1);
    });

    // TOML_CONFIG
    let config = match settings::parse_toml_config(&cmd_args) {
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
        if let Err(e) = egrep::read_config(cmd_args) {
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
