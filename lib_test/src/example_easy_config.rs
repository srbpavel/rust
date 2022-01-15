use std::process;

// Struct for TOML config
//mod crate::toml_config_struct;
use crate::toml_config_struct::{TomlConfig};

// local crate
use easy_config::{read_toml_config};


pub fn example() {
    // FILE_NAME -> here from code / later via CmdArg or ...
    let config_filename = "/home/conan/soft/rust/lib_test/src/config.toml";
    
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

    // just PLAYING with format! + String || str
    /*
    // string join by +  
    println!("\n#CONFIG:\n name + host: {:#?}\n user + workdir: {:#?}",
             /* str */ config.name.to_string() + " at " + &config.host.to_string(),
    
             /* String */ config.user // this should fail for -> typeMyStr = <Box>str
             + &String::from("@:")
             + &config.work_dir
             + &String::from("$"),
    );
    */
    
    // /*
    // string join by format!
    println!("\n#CONFIG:\n name + host: {} at {}\n user + workdir: {}{}{}{}",
             config.name,
             config.host,
             
             config.user,
             "@:",
             config.work_dir,
             "$",
    );
    // */
}
