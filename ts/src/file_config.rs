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
    pub machine_id: String,
    pub carrier: String,
    pub flag_valid_default: bool,
}


#[derive(Debug)]
pub struct Template {
    pub sensors: TemplateSensors,
    pub curl: TemplateCurl,
}

#[derive(Debug)]
pub struct TemplateCurl {
    pub program: String,
    pub param_1: String,
    pub param_2: String,
    pub param_3: String,
    pub param_4: String,
    pub param_5: String,
    pub influx_uri: String,
    pub influx_auth: String,
    pub influx_lp: String,
}


#[derive(Debug)]
pub struct TemplateSensors {
    pub program: String,
    pub param_1: String,
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
        let flag = Flag {
            debug_config_data: true, //false,
            debug_ts: false,
            debug_ts_to_dt: false,
            debug_sensor_output: false,
        };

        // [delay]
        let delay = Delay {
            second: 60,
            minute: 1,
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
            // FOR SENSORS
            sensors: TemplateSensors {
                program: String::from("/usr/bin/sensors"),
                param_1: String::from("-j"),
            },

            // FOR CURL
            curl: TemplateCurl {
                program: String::from("/usr/bin/curl"),
                param_1: String::from("-k"),
                param_2: String::from("--request"),
                param_3: String::from("POST"),
                
                influx_uri: String::from("{secure}://{server}:{port}/api/v2/write?org={org}&bucket={bucket}&precision={precision}"),

                param_4: String::from("--header"),

                influx_auth: String::from("Authorization: Token {token}"),
            
                param_5: String::from("--data-raw"),

                influx_lp: String::from("{measurement},host={host},Machine={machine_id},SensorId={sensor_id},SensorCarrier={sensor_carrier},SensorValid={sensor_valid} TemperatureDecimal={temperature_decimal} {ts}"),
            }
        };

        // [all_sensors]
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

        let sensor_three = Sensor {
            status: true, // false,
            name: String::from("2"),
            pointer: String::from("/acpitz-acpi-0/temp1/temp1_input"),
        };

        let sensor_four = Sensor {
            status: true, //false,
            name: String::from("3"),
            pointer: String::from("/acpitz-acpi-0/temp2/temp2_input"),
        };

        /*
        let mut vs = Vec::new();
        vs.push(sensor_one);
        vs.push(sensor_two);
         */

        /*
        let vs = vec![
            sensor_one,
            sensor_two,
            sensor_three,
            sensor_four,
        ];
        let all_sensors  = AllSensors {values: vs};
        */

        let all_sensors  = AllSensors {values:
                                       vec![
                                           sensor_one,
                                           sensor_two,
                                           sensor_three,
                                           sensor_four,
                                       ]
        };

        
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
