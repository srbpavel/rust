// TIMESTAMP [MS] FOR INFLUXDB
//
mod util;
pub use util::ts as timestamp;

use std::env;
use std::process;
use ts::Config;


fn main() {
    /* TIMESTAMP */

    /*
    // GET LOCAL TIMESTAMP
    let debug_ts: bool = false;
    let ts_ms: i64 = timestamp::ts_now(debug_ts);
    println!("\nTS: {}", ts_ms);

    // TS to DATETIME for better visual reading
    let debug_ts_to_dt: bool = false;
    let ts_dt = timestamp::ts_to_datetime(ts_ms, debug_ts_to_dt);
    println!("\nTS_DT: {:?}", ts_dt);
    */

    
    /* CONFIG */
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("EXIT: Problem parsing arguments\nREASON >>> {}", err); //err RETURN_MSG from Config::new
        process::exit(1);
    });

    
    if let Err(e) = ts::read_config(config) {
        println!("ERROR reading file: {}", e);

        process::exit(1);
    }
}
