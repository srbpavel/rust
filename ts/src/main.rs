// TIMESTAMP [MS] FOR INFLUXDB

mod util;
pub use util::ts;


fn main() {
    //GET LOCAL TIMESTAMP
    let ts_ms: i64 = ts::ts_now();
    println!("\nTS:     {}", ts_ms);

    // TS to DATETIME for better visual reading
    ts::ts_to_datetime(ts_ms);

}
