use std::process;
use std::path::Path;

use easy_config;

mod config_struct;
use config_struct::{TomlConfig};


fn main() {
    // FILE_NAME
    let config_filename = String::from("/home/conan/soft/rust/easy_config/sample_config.toml");
    println!("#FILE_NAME: {}",
             config_filename,
    );

    // PATH
    let path = Path::new(&config_filename);
    let path_status = path.exists();
    
    if path_status {
        println!("#PATH [{}]: {:#?}",
                 path_status,
                 path,
        );
    } else {
        println!("#PATH [{}]: ERROR\nREASON >>> Path.metadata() -> {:#?}",
                 path_status,
                 path.metadata(),
        );
        
        process::exit(1);
    }
    
    // TOML_VALUE
    let toml_value = easy_config::parse_toml_config(&config_filename).unwrap_or_else(|err| {
        eprintln!("\nEXIT: error parsing TOML config file: {c}\nREASON >>> {e}",
                  c=config_filename,
                  e=err);
        
        process::exit(1);
    });

    // TOML CONFIG Struct
    let config: TomlConfig = match toml_value.try_into() {
        Ok(config) => config,

        Err(why) => {
            eprintln!("\nERROR: parsing config\nREASON >>> {}", why);
            
            process::exit(1);
        }
    };

    // DEBUG Struct
    if config.flag.debug_config {
        println!("\n#CONFIG:\n{:?}",
                 config,
        );
    } else {
        println!("\n#CONFIG:\n{:?}",
                 config.name,
        );
    }
}
