use std::fs;
//use std::error::Error;
use toml;
use std::process;

//mod config_struct;
//use config_struct::{TomlConfig};


/// parse toml config file as cmd program argument
///
//pub fn parse_toml_config(filename: String) -> Result<TomlConfig, Box<dyn Error>> {
//pub fn parse_toml_config(filename: String) -> Result<toml::Value, Box<dyn Error>> {

pub fn parse_toml_config(filename: &String) -> Result<toml::Value, toml::de::Error> {
    println!("\n #EASY_CONFIG -> TOML:\n {:}\n", filename);

    let toml_file = fs::read_to_string(&filename).unwrap_or_else(|err| {
        eprintln!("\nEXIT: error reading config file: {}\nREASON >>> {e}",
                  c=&filename,
                  e=err);
        
        process::exit(1);
    });


    let toml_config: Result<toml::Value, toml::de::Error> = toml::from_str(&toml_file);



    toml_config
    

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
