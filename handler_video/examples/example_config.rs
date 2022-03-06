use std::process;
use crate::example_fill_toml_config_struct::{TomlConfig};
use easy_config::{read_toml_config};


pub fn sample_config(config_filename: &str) -> TomlConfig {
    // TOML_VALUE
    let toml_value = read_toml_config(&String::from(config_filename)).unwrap_or_else(|err| {
        eprintln!("\nEXIT: error parsing TOML config file: {c}\nREASON >>> {e}",
                  c=config_filename,
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
        log::debug!("\n#CONFIG:\n{:#?}",
                    config,
        );
    };

    config
}
