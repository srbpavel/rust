use chrono::{Utc, Local, Datelike, Timelike};


#[derive(Debug)]
pub struct Dt {
    pub ts: i64,
    pub local_influx_format: String,
    pub utc_influx_format: String,
    pub today_file_name: String,
    pub local_formated: String,
}


#[allow(dead_code)]
pub fn ts_now(debug: bool) -> Dt {
    let local = Local::now();
    let utc = Utc::now();
    let ts: i64 = local.timestamp_millis();

    // local - 1 HARDCODED -> niet goed jochie @= jaa matje
    let local_influx_format = format!("{:04}-{:02.}-{:02.}T{:02}:{:02.}:{:02.}.{}Z",
                                      local.year(),
                                      local.month(),
                                      local.day(),
                                      local.hour(), // -1
                                      local.minute(),
                                      local.second(),
                                      &format!("{:09}", local.nanosecond())[0..3],
    );

    // RUTH INFLUX query TIME utc 
    let utc_influx_format = format!("{:04}-{:02.}-{:02.}T{:02}:{:02.}:{:02.}.{}Z",
                                    utc.year(),
                                    utc.month(),
                                    utc.day(),
                                    utc.hour(),
                                    utc.minute(),
                                    utc.second(),
                                    &format!("{:09}", utc.nanosecond())[0..3],
    );

    let today_file_name = format!("{:04}_{:02.}_{:02.}",
                                  local.year(),
                                  local.month(),
                                  local.day(),
    );

    let local_formated = format!("{}_{:02.}_{:02.} {:02}:{:02.}:{:02.}.{:09} {} {}",
                                 local.year(),
                                 local.month(),
                                 local.day(),
                                 
                                 local.hour(),
                                 local.minute(),
                                 local.second(),
                                 local.nanosecond(),
                                 
                                 local.weekday(),
                                 local.offset(),
    );
    
    let dt = Dt {ts,
                 local_influx_format,
                 utc_influx_format,
                 today_file_name,
                 local_formated};

    if debug {
        println!("\n#DATE_TIME:\n{:#?}", dt);
    }
    
    return dt;
}

/*
// FUTURE USE
pub fn ts_to_datetime(timestamp: i64, debug: bool) -> DateTime<Local> {
    let ts_sec: f64 = timestamp as f64 / 1000.0;
    let sec: i64 = ts_sec.trunc() as i64;
    let millis: u32 = (ts_sec.fract() * 1_000_000_000.0) as u32;

    let nvdt = NaiveDateTime::from_timestamp(sec, millis); 

    let dt_utc: DateTime<Utc> = DateTime::from_utc(nvdt, Utc);
    let dt_local: DateTime<Local> = dt_utc.with_timezone(&Local);

    if debug {
        println!("ts_sec: {} sec: {} millis: {}", ts_sec, sec, millis);
    	println!("\nnaive: {}\nutc:   {}\nlocal: {}", nvdt, dt_utc, dt_local);
	}

    return dt_local
}
*/
