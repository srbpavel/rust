use std::process;

// Struct for TOML config
use crate::mqtt_toml_config_struct::{TomlConfig};

// local crate
use easy_config::{read_toml_config};


pub fn sample() -> TomlConfig{
    println!("#SAMPLE <EASY_CONFIG>\n");

    // FILE_NAME -> here from code / later via CmdArg or ...
    let config_filename = "/home/conan/soft/rust/lib_test/src/mqtt_config.toml";
    
    /* DEBUG
    println!("#FILE_NAME: {}",
             config_filename,
    );
    */

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
        println!("\n#CONFIG:\n{:#?}",
                 config,
        );
    };

    config
}
