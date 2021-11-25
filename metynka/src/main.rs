// CMD_ARGUMENTS
use std::env;
use std::process;

// /LIB.RS
/* use metynka as bronco; */

// /UTIL/TS
mod util;
use util::ts as timestamp;

// /SENSORS
mod measurement;
//use measurement::parse_sensors_data;

// DEBUG -> measurement
//use metynka::{TomlConfig, Influx};
//use crate::metynka::TemplateSensors;


// /VARIOUS
mod various;


#[allow(unused)] // for quick sample test's at bottom
fn main() {
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
    //parse_sensors_data(&config,
    measurement::parse_sensors_data(&config,
                                    &dt,
    );

    // QUICK SAMPLE CODE TEST



    /*
    let some_number = Some(false);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("\n<{:?}>\n<{:?}>\n<{:?}>",
             some_number,
             some_string,
             absent_number,
             );
    */
    
    /*
    let _s_string = String::from("foookin paavel");
    //let some_words = various::parse_sentence(&_s_string); // String
    
    let _s = "foookin paavel";
    let some_words = various::parse_sentence(_s); // str

    println!("\n#SOME_WORDS: {}", some_words);
    */

    /*
    let line = "1\n2\n3\n4\nBUCKET";
    
    for num in line.lines() {
        match num.parse::<u32>().map_or(666, |i| i * 2) {
            n => println!("num: {} / <n: {:#?}>", num, n),
        }
        /*
        match num.parse::<u8>().map(|i| i * 2) {
            Ok(n) => println!("num: {} / <n: {:#?}>", num, n),
            Err(why) => eprintln!("\n#ERROR: num: <{}> \n>>> REASON: {}", num, why),
            //Err(..) => {},
        }
        */
    }
    */

    
    /*
    enum Example {
        Data(i32),
    }
    
    let x = Example::Data(123); // wrap the data
    let Example::Data(y) = x;   // unwrap the data via a pattern
    
    dbg!(y); // prints 'y = 123'
    */
}
