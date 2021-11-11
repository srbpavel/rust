// TIMESTAMP [MS] FOR INFLUXDB
//
//#[derive(Debug)]
//
mod util;
pub use util::ts as timestamp;

use std::env;
use std::process;
use ts::Config;

use chrono::{Datelike, Timelike};


fn main() {
    // /*
    /* TIMESTAMP */
    // GET LOCAL TIMESTAMP
    let debug_ts: bool = false;
    let ts_ms: i64 = timestamp::ts_now(debug_ts);
    println!("\nTS: {}", ts_ms);

    // TS to DATETIME for better visual reading
    // https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html
    let debug_ts_to_dt: bool = false;
    let ts_dt = timestamp::ts_to_datetime(ts_ms, debug_ts_to_dt);
    println!("\nDT:       {:?}\nformated: {}",
             ts_dt,
             format!("{}_{:02.}_{:02.} {:02}:{:02.}:{:02.}.{:09} {} {}",
                     ts_dt.year(),
                     ts_dt.month(),
                     ts_dt.day(),
                     
                     ts_dt.hour(),
                     ts_dt.minute(),
                     ts_dt.second(),
                     ts_dt.nanosecond(),
                     
                     ts_dt.weekday(),
                     ts_dt.offset(),
             )
    );
    // */

    
    /* CONFIG */
    /*
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err); //err RETURN_MSG from Config::new
        process::exit(1);
    });
     */

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        //eprintln!("Problem parsing arguments: {}", err);
        eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err); //err RETURN_MSG from Config::new
        process::exit(1);
    });
    

    if let Err(e) = ts::read_config(config) {
        eprintln!("\nEXIT: reading file\nREASON >>> {}", e);

        process::exit(1);
    }
}
