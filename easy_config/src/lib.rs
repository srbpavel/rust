use std::{fs,
          path::Path,
          process};

use toml;


/// read data from file path into string
fn open_config_file(file_path: &Path) -> Option<String> {
    let toml_data = match fs::read_to_string(&file_path) {
        Ok(data) => Some(data),
        Err(why) => {
            eprintln!("\nERROR: reading config file: {}\nREASON >>> {e}",
                      c=file_path.display(),
                      e=why);
            None
        },
    };

    toml_data
}


/// path if filename as string is valid
fn string_to_path(filename: &String) -> Option<&Path> {
    let path = Path::new(filename);
    let path_status = path.exists();
    
    if path_status {
        println!("#PATH [{}]: {:#?}",
                 path_status,
                 path.display(),
        );
        
        Some(path)
            
    } else {
        eprintln!("#PATH [{}]: ERROR in filename: {}\nREASON >>> Path.metadata() -> {:#?}",
                 path_status,
                 filename,
                 path.metadata(),
        );
        
        None
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

    let toml_file_path = match string_to_path(&filename) {
        Some(path) => path,
        None => {
            process::exit(1)
        },
    };
    
    let toml_data = match open_config_file(toml_file_path) {
        Some(data) => data,
        None => {
            process::exit(1)
        },
    };

    toml::from_str(&toml_data)
}


// /*
//#[cfg(test)]
//mod tests {
#[test]
fn path_valid() {
    let filename = String::from("/var");
    let path = string_to_path(&filename);
    
    assert_eq!(Path::new(&filename), path.unwrap());
}

#[test]
fn path_invalid() {
    let filename = String::from("/Wu_Tang_Clan");
    let path = string_to_path(&filename);

    assert_eq!(Option::None, path);
}

#[allow(unused_must_use)]
#[test]
fn read_config_valid() {
    use std::io::Write;
    use std::fs::{File, remove_file};
    
    let text = "TEXT TO BE READ FROM FILE WITH TEST\n";
    let filename = "test_file.toml";
    
    File::create(filename)
        .unwrap()
        .write_all(text
                   .as_bytes());

    let data = open_config_file(Path::new(filename));

    remove_file(filename);
    
    assert_eq!(text, data.unwrap());
}

//}
// */
