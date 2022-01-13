use std::fs;
use toml;
use std::process;
use std::path::Path;


/// read file as path into string
//fn open_config_file(filename: &String) -> String {
//    let toml_data = fs::read_to_string(&filename).unwrap_or_else(|err| {
// /*
fn open_config_file(filename: &Path) -> String {
    let toml_data = fs::read_to_string(&filename).unwrap_or_else(|err| {
        eprintln!("\nEXIT: error reading config file: {}\nREASON >>> {e}",
                  //c=&filename,
                  c=filename.display(),
                  e=err);
        
        // EXIT as config file is essential
        process::exit(1);
    });
// */
    /*
    println!("FILE: {:?}",
             toml_data,
    );
    */

    toml_data
}


/// path if filename string is valid
//fn string_to_path<'p>(filename: &'p String) -> Option<&'p str> {
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
pub fn parse_toml_config(filename: &String) -> Result<toml::Value, toml::de::Error>
//pub fn parse_toml_config(filename: &Path) -> Result<toml::Value, toml::de::Error>

/*
pub fn parse_toml_config<T>(filename: T) -> Result<toml::Value, toml::de::Error>
where
    std::string::String: From<T>
*/
{
    // String
    //let toml_file = open_config_file(&filename);

    let toml_path = string_to_path(&filename);
    
    let toml_data = open_config_file(toml_path);
    
    /*
    // Path
    let path = Path::new(&filename);
    let path_status = path.exists();

    let toml_file = if path_status {
        println!("#PATH [{}]: {:#?}",
                 path_status,
                 path.display(),
        );
        
        //open_config_file(&String::from(filename.to_str().unwrap()))
        //open_config_file(&filename)
        open_config_file(&path)
            
    } else {
        println!("#PATH [{}]: ERROR\nREASON >>> Path.metadata() -> {:#?}",
                 path_status,
                 path.metadata(),
        );
        
        process::exit(1);
    };
    */

    /*
    let toml_data = match filename.to_str() {
        Some(f) => open_config_file(&f.to_string()),
        None => {
            println!("\nEXIT: XXX {c:#?}",
                  c=filename);
        
            process::exit(1);
        }
    };
    */

    /*
    // T String or Path
    let toml_file = open_config_file(&String::from(filename));
    */

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
