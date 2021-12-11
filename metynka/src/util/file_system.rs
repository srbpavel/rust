use std::path::Path;
use std::fs::{self, OpenOptions, File};
use std::io;

use crate::util::template_formater::tuple_formater;

use crate::TomlConfig;


#[allow(dead_code)]
pub fn create_new_dir(config: &TomlConfig,
                      full_path: &Path,
                      measurement: &String) {

    fs::create_dir_all(&full_path).unwrap_or_else(|err| {
        let print_formated = tuple_formater(&"\nERROR >> METRIC <{m}> failed to create BACKUP DIR: {d}\nREASON: >>> {e}".to_string(),
                                            &vec![
                                                ("m", &measurement),
                                                ("d", &config.work_dir),
                                                ("e", &err.to_string())
                                            ],
                                            config.flag.debug_template_formater
        );
        
        println!("{}", print_formated);
        eprintln!("{}", print_formated);
        
    });
}


#[allow(dead_code)]
pub fn create_new_file(today_file_name: &Path) -> Result<File, io::Error> {
    match File::create(&today_file_name) { // LEARN TO write TEST for this
        Err(why) => {
            eprintln!("\nEXIT: COULD NOT CREATE {}\nREASON: >>> {}",
                      &today_file_name.display(),
                      why);

            Err(why)

        },
        
        Ok(file) => Ok(file),
    }
}


// FN used so why warning never used ? 
#[allow(dead_code)]
pub fn open_file_to_append(today_file_name: &Path) -> Result<File, io::Error> {
    //println!("open_file..append..data");
    match OpenOptions::new()
        .write(true)
        .append(true)
        .open(&today_file_name)
    {
        Ok(file) => Ok(file),

        // FAIL TO OPEN FOR WRITE
        Err(why) => {
            eprintln!("\nERROR >> FILE WRITE permission: {}\nREASON: >>> {}",
                      &today_file_name.display(),
                      why);

            Err(why)
        },
    }
}
