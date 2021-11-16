use chrono::{DateTime, Utc, Local, NaiveDateTime, Datelike, Timelike};

#[derive(Debug)]
pub struct Dt {
    pub ts: i64,
    //pub local: DateTime,
    pub local_influx_format: String,
}


//pub fn ts_now(debug: bool) -> i64 {
pub fn ts_now(debug: bool) -> Dt {
    // MOZNA VRACET Struct { datetime, ts }

    let local = Local::now();
    let ts: i64 = local.timestamp_millis();

    let local_influx_format = format!("{:04}-{:02.}-{:02.}T{:02}:{:02.}:{:02.}.{}Z",
                                      local.year(),
                                      local.month(),
                                      local.day(),
                                      local.hour() -1 ,
                                      local.minute(),
                                      local.second(),
                                      &format!("{:09}", local.nanosecond())[0..3],
    );

    println!("\n#DATE_TIME_INFLUX_FORMAT: {}", local_influx_format);
    
    if debug {
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

        println!("
#DATE_TIME:
local:    {l}
formated: {l_formated}
sec:    {l_ts_sec}
ms:     {l_ts_ms}",
    		 l=local,
		 l_ts_sec=local.timestamp(),
		 l_ts_ms=ts,
                 l_formated=local_formated
        );
    }
    //return ts
    //return Dt {ts, local, local_influx_format};
    return Dt {ts, local_influx_format};
}


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
