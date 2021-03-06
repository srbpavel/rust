use std::process;

use crate::influxdb_toml_config_struct::{TomlConfig};

use easy_config::{read_toml_config};


const CONFIG_FILENAME: &str = "/home/conan/soft/rust/influxdb_client/examples/influxdb_config.toml";


pub fn sample_config() -> TomlConfig {
    // TOML_VALUE
    let toml_value = read_toml_config(&String::from(CONFIG_FILENAME)).unwrap_or_else(|err| {
        eprintln!("\nEXIT: error parsing TOML config file: {c}\nREASON >>> {e}",
                  c=CONFIG_FILENAME,
                  e=err);
        
        process::exit(1);
    });

    // CONFIG
    let config: TomlConfig = match toml_value.try_into() {
        Ok(config) => config,

        Err(why) => {
            eprintln!("\nEXIT: ERROR parsing config\nREASON >>> {}", why);
            
            process::exit(1);
        }
    };

    // DEBUG CONFIG
    if config.flag.debug_config {
        println!("\n#CONFIG:\n{:#?}",
                 config,
        );
    };

    config
}
