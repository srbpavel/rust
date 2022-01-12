use std::process;
use std::path::Path;

use easy_config;

mod config_struct;
use config_struct::{TomlConfig};


fn main() {
    println!("#MAIN:");

    let config_filename = String::from("/home/conan/soft/rust/easy_config/sample_config.toml");

    let path = Path::new(&config_filename);

    if path.exists() {
        println!("#PATH: {:#?}",
                 path,
        );
    } else {
        println!("\nERROR: file not found\nREASON >>> Path.metadata() -> {:#?}",
                 path.metadata(),
        );
        
        process::exit(1);
    }
    
    // TOML_CONFIG
    let toml_value = easy_config::parse_toml_config(&config_filename).unwrap_or_else(|err| {
        eprintln!("\nEXIT: error parsing TOML config file: {c}\nREASON >>> {e}",
                  c=config_filename,
                  e=err);
        
        process::exit(1);
    });
    
    let config: TomlConfig = match toml_value.try_into() {
        Ok(config) => config,

        Err(why) => {
            eprintln!("\nERROR: parsing config\nREASON >>> {}", why);
            
            process::exit(1);
        }
    };

    println!("\n#TOML_VALUE:\n{:#?}",
             config,
    );

    // DEBUG DateTime Struct
    /*
    println!("\n#CONFIG:\n{:#?}",
             config.name,
    );
    */
}
