use std::process;

#[allow(unused_imports)]
use std::path::Path;

use easy_config;

mod toml_config_struct;
use toml_config_struct::{TomlConfig};


fn main() {
    // FILE_NAME -> here from code / later via CmdArg or ...
    let config_filename = String::from("/home/conan/soft/rust/easy_config/sample_config.toml");
    println!("#FILE_NAME: {}",
             config_filename,
    );

    // /*
    // PATH
    let path = Path::new(&config_filename);
    let path_status = path.exists();

    if path_status {
        println!("#PATH [{}]: {:#?}",
                 path_status,
                 path, //path.display(),
        );
    } else {
        println!("#PATH [{}]: ERROR\nREASON >>> Path.metadata() -> {:#?}",
                 path_status,
                 path.metadata(),
        );
        
        process::exit(1);
    }
    // */
    
    // TOML_VALUE

    // String
    //let toml_value = easy_config::parse_toml_config(&config_filename).unwrap_or_else(|err| {

    // Path to str -> Some(str)
    // T 
    //let toml_value = easy_config::parse_toml_config(&path.to_str().unwrap().to_string()).unwrap_or_else(|err| {
    /*
    let path_string = String::from(path.to_str().unwrap()); // not safe

    let toml_value = easy_config::parse_toml_config(&path_string).unwrap_or_else(|err| {
    */
    let toml_value = easy_config::parse_toml_config(&path).unwrap_or_else(|err| {
    //let toml_value = easy_config::parse_toml_config(&path).unwrap_or_else(|err| {
        eprintln!("\nEXIT: error parsing TOML config file: {c}\nREASON >>> {e}",
                  c=config_filename,
                  e=err);
        
        process::exit(1);
    });

    // TOML CONFIG Struct
    //let config: TomlConfig = match toml_value.try_into() {
    let config: TomlConfig = match toml_value.try_into() {
        Ok(config) => config,

        Err(why) => {
            eprintln!("\nEXIT: ERROR parsing config\nREASON >>> {}", why);
            
            process::exit(1);
        }
    };

    // DEBUG Struct
    if config.flag.debug_config {
        println!("\n#CONFIG:\n{:#?}",
                 config,
        );
    };
    
    /*
    // by +  
    println!("\n#CONFIG:\n name + host: {:#?}\n user + workdir: {:#?}",
             // /* str */ config.name.to_string() + " at " + &config.host.to_string(),
             /* str */ config.name.to_string() + " at " + &config.host.to_string(),
    
    /* String */ config.user
    + &String::from("@:")
    + &config.work_dir
    + &String::from("$"),
    );
    */

    // /*
    // by format!
    println!("\n#CONFIG:\n name + host: {} at {}\n user + workdir: {}{}{}{}",
             // str
             config.name,
             config.host,
             
             // String
             config.user,
             "@:",
             config.work_dir,
                 "$",
    );
    // */
}
