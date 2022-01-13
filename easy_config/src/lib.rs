use std::fs;
use std::path::Path;
use std::process;
use toml;


/// read data from file path into string
fn open_config_file(file_path: &Path) -> String {
    let toml_data = fs::read_to_string(&file_path).unwrap_or_else(|err| {
        eprintln!("\nEXIT: error reading config file: {}\nREASON >>> {e}",
                  c=file_path.display(),
                  e=err);
        
        process::exit(1);
    });

    toml_data
}


/// path if filename as string is valid
fn string_to_path(filename: &String) -> &Path {
    let path = Path::new(filename);
    let path_status = path.exists();
    
    if path_status {
        println!("#PATH [{}]: {:#?}",
                 path_status,
                 path.display(),
        );
        
        path
            
    } else {
        println!("#PATH [{}]: ERROR\nREASON >>> Path.metadata() -> {:#?}",
                 path_status,
                 path.metadata(),
        );
        
        process::exit(1);
    }
}


/// receive filename and return toml::Value
pub fn read_toml_config(filename: &String) -> Result<toml::Value, toml::de::Error>
/*
pub fn parse_toml_config<T>(filename: T) -> Result<toml::Value, toml::de::Error>
where
    std::string::String: From<T>
*/
{
    /*
    // T String or Path
    let toml_file = open_config_file(&String::from(filename));
    */
    
    let toml_file_path = string_to_path(&filename);
    
    let toml_data = open_config_file(toml_file_path);

    // Value
    toml::from_str(&toml_data)
}


/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
*/
