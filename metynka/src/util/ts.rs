use chrono::{Utc, Local, Datelike, Timelike};


#[derive(Debug)]
pub struct Dt {
    pub ts: u64,
    pub utc_influx_format: String,
    pub today_file_name: String,
    pub local_formated: String,
}


#[allow(dead_code)]
pub fn ts_now() -> Dt {
    let utc = Utc::now();
    let local = utc.with_timezone(&Local);
    let ts: u64 = utc.timestamp_millis() as u64;

    // INFLUX query date_time UTC -> 2021-12-09T12:23:48.839Z / MS format
    let utc_influx_format = format!("{:04}-{:02.}-{:02.}T{:02}:{:02.}:{:02.}.{}Z",
                                    utc.year(),
                                    utc.month(),
                                    utc.day(),
                                    utc.hour(),
                                    utc.minute(),
                                    utc.second(),
                                    &format!("{:09}", utc.nanosecond())[0..3],
    );

    // PART OF BACKUP FILE
    let today_file_name = format!("{:04}_{:02.}_{:02.}",
                                  utc.year(),
                                  utc.month(),
                                  utc.day(),
    );

    // FOR EYES only
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

    Dt {ts: ts,
        utc_influx_format: utc_influx_format,
        today_file_name: today_file_name,
        local_formated: local_formated,
    }
}
