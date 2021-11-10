use chrono::{DateTime, Utc, Local, NaiveDateTime};


pub fn ts_now() -> i64 {
    // /* DEBUG
    let local = Local::now();
    let ts: i64 = local.timestamp_millis();

    println!("local: {}\nsec:    {}\nms:     {}",
    		     local,
		     local.timestamp(),
		     ts);

    return ts
    // */
    
    //Local::now().timestamp_millis()
}


pub fn ts_now_utc() -> i64 {
    /*
    let utc = Utc::now();
    let ts: i64 = utc.timestamp_millis();

    println!("utc: {}\nsec:    {}\nms:     {}",
    		     utc,
		     utc.timestamp(),
		     ts);

    return ts
    */
    Utc::now().timestamp_millis()
}

pub fn ts_to_datetime(ts_ms: i64) -> () {
    let ts_sec: f64 = ts_ms as f64 / 1000.0;

    // INT
    let sec: i64 = ts_sec.trunc() as i64;
    // FRACT
    let millis: u32 = (ts_sec.fract() * 1_000_000_000.0) as u32;

    println!("ts_sec: {} sec: {} millis: {}", ts_sec, sec, millis);

    let nvdt = NaiveDateTime::from_timestamp(sec, millis); 
    let dt_utc: DateTime<Utc> = DateTime::from_utc(nvdt, Utc);
    let dt_local: DateTime<Local> = dt_utc.with_timezone(&Local);

    println!("\nnaive: {}\nutc:   {}\nlocal: {}", nvdt, dt_utc, dt_local);
}