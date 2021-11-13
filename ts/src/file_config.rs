#[derive(Debug)]
pub struct Data {
    pub work_dir: String,
    pub name: String,
    pub host: String,

    pub flag: Flag,

    pub delay: Delay,
    
    pub influx_default: Influx,
    pub influx_backup: Influx,

    pub all_sensors: AllSensors,
    //pub sensor: Sensor,

    pub template: Template,
}


#[derive(Debug)]
pub struct Flag {
    // ALL ARE BOOL, try to define only ONCE !!!
    pub debug_config_data: bool,
    pub debug_ts: bool,
    pub debug_ts_to_dt: bool,
    pub debug_sensor_output: bool,
}


#[derive(Debug)]
pub struct Delay {
    pub second: u8,
    pub minute: u32,
}


#[derive(Debug)]
pub struct Influx {
    pub status: bool,
    pub secure: String,
    pub server: String,
    pub port: u16,
    pub bucket: String,
    pub token: String,
    pub org: String,
    pub precision: String,
    pub default_carrier: String,
    pub default_valid_status: String,
}


#[derive(Debug)]
pub struct Template {
    pub curl_influx: String,
}

#[derive(Debug)]
pub struct AllSensors {
    pub values: Vec<Sensor>,
}

#[derive(Debug)]
pub struct Sensor {
    pub status: bool,
    pub name: String, // mozna u8
    pub pointer: String,
}


impl Data {
    pub fn start() -> Data {
        let work_dir = String::from("/home/conan/soft/rust/ts");
        let name = String::from("config_sensor_001");
        let host = String::from("spongebob");

        // [flag]
        let debug_config_data = true; //false;
        let debug_ts = false;
        let debug_ts_to_dt = false;
        let debug_sensor_output = false;
        
        let flag = Flag {
            debug_config_data,
            debug_ts,
            debug_ts_to_dt,
            debug_sensor_output,
        };

        
        // [delay]
        let second = 60;
        let minute = 1;
        
        let delay = Delay {
            second,
            minute,
        };
        
        // [influx_default]
        let status = true;
        let secure = String::from("https");
        let server = String::from("ruth");
        let port = 8086;
        let bucket = String::from("test_rust");
        let token = String::from("riMIsymqgtxF6vGnTfhpSCWPcijRRQ2ekwbS5H8BkPXHr_HtCNUqKLwOnyHpMjQB-L6ZscVFo8PsGbGgoxEFLw==");
        let org = String::from("foookin_paavel");
        let precision = String::from("ms");
        let default_carrier = String::from("cargo");
        let default_valid_status = String::from("true");

        
        let influx_default = Influx {
            status,
            secure,
            server,
            port,
            bucket,
            token,
            org,
            precision,
            default_carrier,
            default_valid_status,
        };

        // [influx_backup]
        let status = false;
        let secure = String::from("http");
        let server = String::from("jozefina");
        let port = 8086;
        let bucket = String::from("backup_test_rust");
        let token = String::from("");
        let org = String::from("foookin_paavel");
        let precision = String::from("ms");
        let default_carrier = String::from("cargo");
        let default_valid_status = String::from("true");

        
        let influx_backup = Influx {
            status,
            secure,
            server,
            port,
            bucket,
            token,
            org,
            precision,
            default_carrier,
            default_valid_status,
        };
        

        // [template]
        let curl_influx = String::from("curl -k --request POST \"{secure}://{server}:{port}/api/v2/write?org={org}&bucket={bucket}&precision={precision}\" --header \"Authorization: Token {token}\" --data-raw \"{measurement},host={host},Machine={machine_id},DsId={ds_id},DsCarrier={ds_carrier},DsValid={ds_valid},DsPin={ds_pin} DsDecimal={ds_decimal} {ts}\"");
        
        let template = Template {
            curl_influx,
        };

        // [sensor]
        let sensor_one = Sensor {
            status: true,
            name: String::from("0"),
            pointer: String::from("/coretemp-isa-0000/Core 0/temp2_input"),
        };

        let sensor_two = Sensor {
            status: true,
            name: String::from("1"),
            pointer: String::from("/coretemp-isa-0000/Core 1/temp3_input"),
        };

        /*
        let mut vs = Vec::new();
        vs.push(sensor_one);
        vs.push(sensor_two);
        */
        let vs = vec![sensor_one, sensor_two];
        let all_sensors  = AllSensors {values: vs};
        
        // RETURN
        Data {
            work_dir,
            name,
            host,
            flag,
            delay,
            influx_default,
            influx_backup,
            template,
            //sensor,
            all_sensors,
        }
    }
}
