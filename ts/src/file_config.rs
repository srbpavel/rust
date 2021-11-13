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

    pub measurement: String,
    pub machine_id: String, // here same as host, normaly different: T4/esp32/..
    pub carrier: String,
    pub flag_valid_default: bool,
}


#[derive(Debug)]
pub struct Template {
    pub cmd_program: String, // /usr/bin/curl
    pub cmd_param_1: String, // -k
    pub cmd_param_2: String, // --request
    pub cmd_param_3: String, // POST
    pub cmd_param_4: String, // --header
    pub cmd_param_5: String, // --data-raw
    pub influx_uri: String, // https://ruth:8086
    pub influx_token: String, // Authorization: Token {token}
    pub influx_lp: String, // temperature,host=
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
        let debug_config_data = false;
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
        let influx_default = Influx {
            status: true,
            secure: String::from("https"),
            server: String::from("ruth"),
            port: 8086,
            
            bucket: String::from("test_rust"),
            token: String::from("riMIsymqgtxF6vGnTfhpSCWPcijRRQ2ekwbS5H8BkPXHr_HtCNUqKLwOnyHpMjQB-L6ZscVFo8PsGbGgoxEFLw=="),
            org: String::from("foookin_paavel"),
            precision: String::from("ms"),
            
            measurement: String::from("temperature"),
            machine_id: String::from("spongebob"),
            carrier: String::from("cargo"),
            flag_valid_default: true,
        };

        // [influx_backup]
        let influx_backup = Influx {
            status: false,
            secure: String::from("http"),
            server: String::from("jozefina"),
            port: 8086,

            bucket: String::from("backup_test_rust"),
            token: String::from(""),
            org: String::from("foookin_paavel"),
            precision: String::from("ms"),

            measurement: String::from("temperature"),
            machine_id: String::from("spongebob"),
            carrier: String::from("cargo"),
            flag_valid_default: true,
        };
        
        // [template]
        let template = Template {

            cmd_program: String::from("/usr/bin/curl"),

            cmd_param_1: String::from("-k"),
            cmd_param_2: String::from("--request"),
            cmd_param_3: String::from("POST"),

            influx_uri: String::from("{secure}://{server}:{port}/api/v2/write?org={org}&bucket={bucket}&precision={precision}"),

            cmd_param_4: String::from("--header"),

            influx_token: String::from("Authorization: Token {token}"),
            
            cmd_param_5: String::from("--data-raw"),

            influx_lp: String::from("{measurement},host={host},Machine={machine_id},SensorId={sensor_id},SensorCarrier={sensor_carrier},SensorValid={sensor_valid} TemperatureDecimal={temperature_decimal} {ts}"),
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
            all_sensors,
        }
    }
}
