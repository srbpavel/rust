use chrono::{Utc, Local, Datelike, Timelike};


#[derive(Debug)]
pub struct Dt {
    pub ts: u64,
    pub local_influx_format: String,
    pub utc_influx_format: String,
    pub today_file_name: String,
    pub local_formated: String,
}


#[allow(dead_code)]
pub fn ts_now(debug: bool) -> Dt {
    let local = Local::now();
    let utc = Utc::now();
    let ts: u64 = local.timestamp_millis() as u64;

    let local_influx_format = format!("{:04}-{:02.}-{:02.}T{:02}:{:02.}:{:02.}.{}Z",
                                      local.year(),
                                      local.month(),
                                      local.day(),
                                      local.hour(),
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
