// TIMESTAMP [MS] FOR INFLUXDB

mod util;
pub use util::ts;


fn main() {
    //GET LOCAL TIMESTAMP
    let debug_ts: bool = false;
    let ts_ms: i64 = ts::ts_now(debug_ts);
    println!("\nTS: {}", ts_ms);

    // TS to DATETIME for better visual reading
    let debug_ts_to_dt: bool = false;
    let ts_dt = ts::ts_to_datetime(ts_ms, debug_ts_to_dt);
    println!("\nTS_DT: {:?}", ts_dt);
}
