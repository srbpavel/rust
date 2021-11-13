// MY FIRST LESSON
mod util;
pub use util::ts as timestamp;

mod measurement;

mod file_config;
pub use file_config::Data;

//use std::env;
//use std::process;
//use ts::Config;


fn main() {
    let config_data = Data::start();

    if config_data.flag.debug_config_data {
        println!("\n#CONFIG_DATA:\n{:#?}\n >>> {:#?}",
                 config_data,
                 "" // config_data.all_sensors,
        );
    }

    
    /* TIMESTAMP */
    let ts_ms: i64 = timestamp::ts_now(config_data.flag.debug_ts);
    println!("\n#TS:\n{}", ts_ms);

    /* SENSOR */
    //let diablo = measurement::mmm();
    //println!("\n#DIABLO:\n{}", diablo);

    measurement::get_sensors_data(&config_data,
                                  ts_ms
    );
    
    /*
    let ccc = measurement::get_sensors_data(&config_data);
    println!("\n#CCC:\n{:?}", ccc);
    */

    /* START
    let sensor_output = Command::new(config_data.template.sensors.program)
        .arg(config_data.template.sensors.param_1)
        .output().expect("failed to execute command");

    let sensor_stdout_string = String::from_utf8_lossy(&sensor_output.stdout);
    let sensor_stderr_string = String::from_utf8_lossy(&sensor_output.stderr);
    
    if config_data.flag.debug_sensor_output {
        println!("\n#SENSOR_OUTPUT:
stdout: {}
stderr: {:?}",
                 sensor_stdout_string,
                 sensor_stderr_string,
        ); // xxx
    }

    // JSON
    let value: serde_json::Value = serde_json::from_str(&sensor_stdout_string).unwrap();
    //println!("\n#VALUE: {:?}", value);
    
    // DICT
    /*
    let dict = value.get("coretemp-isa-0000")
        .and_then(|v| v.get("Core 0"))
        .and_then(|v| v.get("temp2_input"))
        .unwrap();

    println!("\n#DICT: {}", dict);
    */
    
    // LIST
    /*
    let sensor_value: serde_json::Value = serde_json::from_str(&sensor_stdout_string).unwrap();
    println!("\n#SENSOR_VALUE: {}", sensor_value);

    let temperature_core_0 = &sensor_value["coretemp-isa-0000"]["Core 0"]["temp2_input"].to_string();
    let temperature_core_1 = &sensor_value["coretemp-isa-0000"]["Core 1"]["temp3_input"].to_string();
    
    println!("\nCore 0: {}", temperature_core_0);
    println!("\nCore 1: {}", temperature_core_1);
    */


    // URI
    let uri_template = String::from(config_data.template.curl.influx_uri);
    let mut uri = HashMap::new();
    uri.insert("secure".to_string(), config_data.influx_default.secure);
    uri.insert("server".to_string(), config_data.influx_default.server);
    uri.insert("port".to_string(), config_data.influx_default.port.to_string());
    uri.insert("org".to_string(), config_data.influx_default.org);
    uri.insert("bucket".to_string(), config_data.influx_default.bucket);
    uri.insert("precision".to_string(), config_data.influx_default.precision);
    //println!("URI: {}", strfmt(&uri_template, &uri).unwrap());

    // TOKEN
    let auth_template = String::from(config_data.template.curl.influx_auth);
    let mut auth = HashMap::new();
    auth.insert("token".to_string(), String::from(&config_data.influx_default.token));

    // LP
    let lp_template = String::from(config_data.template.curl.influx_lp);

    let mut lp = HashMap::new();
    lp.insert("measurement".to_string(), String::from("temperature"));
    lp.insert("host".to_string(), config_data.host);
    lp.insert("machine_id".to_string(), String::from("spongebob"));
    lp.insert("sensor_carrier".to_string(), String::from("cargo"));
    lp.insert("sensor_valid".to_string(), String::from("true"));
    lp.insert("ts".to_string(), ts_ms.to_string());

    // POINTER
    for single_sensor in &config_data.all_sensors.values {
        let pointer_value = &value.pointer(&single_sensor.pointer).unwrap();
        
        println!("
#POINTER_CFG:
status: {s}
name: {n}
pointer: {p}
value: {v}",
                 s=single_sensor.status,
                 n=single_sensor.name,
                 p=single_sensor.pointer,
                 v=pointer_value,
        );

        lp.insert("sensor_id".to_string(), single_sensor.name.to_string()); // SENSOR_ID
        lp.insert("temperature_decimal".to_string(), pointer_value.to_string()); // TEMPERATURE_DECIMAL
        

        // INFLUX IMPORT
        if single_sensor.status {
            println!("\nLINE_PROTOCOL: {}", strfmt(&lp_template, &lp).unwrap());
            
            /* CURL */
            let curl_output = Command::new(&config_data.template.curl.program)
                .arg(&config_data.template.curl.param_1)
                .arg(&config_data.template.curl.param_2)
                .arg(&config_data.template.curl.param_3)
                .arg(strfmt(&uri_template, &uri).unwrap()) // URI
                .arg(&config_data.template.curl.param_4)
                .arg(strfmt(&auth_template, &auth).unwrap()) // AUTH
                .arg(&config_data.template.curl.param_5)
                .arg(strfmt(&lp_template, &lp).unwrap()) // LINE_PROTOCOL
                .output().expect("failed to execute command");

            /*
            let curl_output = Command::new("/usr/bin/curl")
            .arg("-k")
            .arg("--request")
            .arg("POST")
            .arg(strfmt(&uri_template, &uri).unwrap()) // URI
            .arg("--header")
            .arg(strfmt(&token_template, &token).unwrap()) // TOKEN
            .arg("--data-raw")
            .arg(strfmt(&lp_template, &lp).unwrap())// LINE_PROTOCOL
            .output().expect("failed to execute command");
             */
        
            //println!("\nstdout: {:?}", &output);
            println!("\nstdout: {}", String::from_utf8_lossy(&curl_output.stdout));
            println!("\nstderr: {}", String::from_utf8_lossy(&curl_output.stderr));
        }
    }
    END */


    /* CALL DIRECTLY FROM rust as python.requests
    // https://www.reddit.com/r/rust/comments/ndrd0b/how_to_translate_this_curl_request_into_rusts/
    let client = Client::new();
    let res = client
        .post(format!("{secure}://{server}:{port}/api/v2/write?org={org}&bucket={bucket}&precision={precision}",
                      secure=config_data.influx_default.secure,
                      server=config_data.influx_default.server,
                      port=config_data.influx_default.port,
                      org=config_data.influx_default.org,
                      bucket=config_data.influx_default.bucket,
                      precision=config_data.influx_default.precision))
        .header("Authorization", format!("Token {token}", token=config_data.influx_default.token))
        .body(format!("{measurement},host={host},Machine={machine_id},SensorId={sensor_id},SensorCarrier={sensor_carrier},SensorValid={sensor_valid} SensorDecimal={sensor_decimal} {ts}", 
                      measurement="laptop",
                      host=config_data.host,
                      machine_id="spongebob",
                      sensor_id=1,
                      sensor_carrier="cargo",
                      sensor_valid="true",
                      sensor_decimal=123.4567,
                      ts=ts_ms))
        .send();
    */

    // */

    /* TOML CONFIG
    let file_config = FileConfig {
        work_dir: "/home/conan/soft/rust/ts".to_string(),
        influx: Influx {
            host: "ruth".to_string(),
            port: Some(8086),
        },
    };

    println!("\nFILE_CONFIG:\n{}\n{}:{}",
             &file_config.work_dir,
             &file_config.influx.host,
             &file_config.influx.port.unwrap());
    */

 
 
    /* CONFIG */
    /*
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err); //err RETURN_MSG from Config::new
        process::exit(1);
    });
     */

    /*
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        //eprintln!("Problem parsing arguments: {}", err);
        eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err); //err RETURN_MSG from Config::new
        process::exit(1);
    });
    

    if let Err(e) = ts::read_config(config) { //ts. je muj METYNKA
        eprintln!("\nEXIT: reading file\nREASON >>> {}", e);

        process::exit(1);
    }
     */
}
