use std::fs;
use toml;
use std::process;


/// read file into string
fn open_config_file(filename: &String) -> String {
    let toml_file = fs::read_to_string(&filename).unwrap_or_else(|err| {
        eprintln!("\nEXIT: error reading config file: {}\nREASON >>> {e}",
                  c=&filename,
                  e=err);
        
        // EXIT as config is essential
        process::exit(1);
    });

    /*
    println!("FILE: {:?}",
             toml_file,
    );
    */

    toml_file
}

/// receive filename and return toml::Value
//pub fn parse_toml_config(filename: &String) -> Result<toml::Value, toml::de::Error> {
pub fn parse_toml_config<T>(filename: T) -> Result<toml::Value, toml::de::Error>
where
    std::string::String: From<T>
{
    //let toml_file = open_config_file(&filename);

    // T String or Path
    let toml_file = open_config_file(&String::from(filename));
    
    toml::from_str(&toml_file)
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
