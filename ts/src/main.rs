// CMD_ARGUMENTS
use std::env;
use std::process;

// LIB.RS
use ts as metynka;

// /UTIL/TS
mod util;
pub use util::ts as timestamp;

// SENSORS
mod measurement;

// CMD
use std::process::{Command};
use std::process::*; 


fn main() {
    // -> START --> CMD ARGS 
    let cmd_cat = Command::new("/bin/cat")
        .arg("/proc/meminfo")
        .stdout(Stdio::piped()).spawn().unwrap();
        
    let cmd_jq = Command::new("jq")
        .args(["--slurp",
               "--raw-input",
               "split(\"\n\") | map(select(. != \"\") | split(\":\") | {\"key\": .[0], \"value\": (.[1:]| map_values(.[0:-3]) | join(\"\") | split(\" \") | .[1:] | join(\"\"))}) | from_entries"
        ])
        .stdin(cmd_cat.stdout.unwrap())
        .output().expect("failed to execute command");

    //println!("\n#CMD_Jq:stdout: {:#?}", String::from_utf8_lossy(&cmd_jq.stdout));
    //println!("\n#CMD_Jq:stdERR: {:#?}", String::from_utf8_lossy(&cmd_jq.stderr));

    let mem_info_json: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&cmd_jq.stdout)).unwrap();
    //println!("\n#JSON:\n{:?}", mem_info_json);

    let json_pointer_value: i64 = mem_info_json.pointer("/MemTotal").unwrap().as_str().unwrap().parse().unwrap();
    println!("\n#POINTER:\nMemInfo[i64]: {} kB",
             json_pointer_value);
    // <- END

    
    // COMMAND ARGS
    let cmd_args = metynka::CmdArgs::new(env::args()).unwrap_or_else(|err| {
         eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err);
        process::exit(1);
    });

    // TOML_CONFIG
    let new_config = metynka::parse_toml_config(&cmd_args).unwrap_or_else(|err| {
        eprintln!("\nEXIT: reading file\nREASON >>> {}", err);
        process::exit(1);
    });

    // EGREP
    if new_config.flag.run_egrep && new_config.flag.debug_egrep {
        if let Err(e) = metynka::read_config(cmd_args) {
            eprintln!("\nEXIT: reading file\nREASON >>> {}", e);
            process::exit(1);
        }
    }
    
    // DT Struct
    let dt = timestamp::ts_now(new_config.flag.debug_ts);

    // DEBUG: ALL_INFLUX
    if new_config.flag.debug_influx_instances {
        for single_influx in &new_config.all_influx.values {
            let status = match single_influx.status {
                true => "true",
                false => "false",
            };

            println!("INFLUX [{}]: {}",
                     status,
                     single_influx.name);
        }
    }

    // DEBUG: ALL_SENSOR
    if new_config.flag.debug_sensor_instances {
        for single_sensor in &new_config.all_sensors.values {
            let status = match single_sensor.status {
                true => "true",
                false => "false",
            };

            println!("SENSOR [{}]: {}",
                     status,
                     single_sensor.name);
        }
    }
    
    // SENSOR
    measurement::parse_sensors_data(&new_config,
                                    &dt,
    );
}
