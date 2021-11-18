use std::fs;
use std::error::Error;
use std::env;

use toml;
use serde::{Serialize, Deserialize};

use std::process;


pub struct CmdArgs {
    // when modified DO NOT forget to change ARG_COUNT verification -> learn to count struct descendants
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TomlConfig {
    // ROOT
    pub work_dir: String,
    pub name: String,
    pub host: String,

    // STRUCT
    pub flag: Flag,
    pub delay: Delay,
    pub template: Template,
    pub backup: Backup,
    
    // VEC
    pub all_influx: AllInflux,
    pub all_sensors: AllSensors,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Flag {
    pub debug_new_config: bool,
    
    pub debug_ts: bool,
    pub debug_ts_to_dt: bool,

    pub debug_sensor_output: bool,
    pub debug_sensor_instances: bool,

    pub debug_pointer_output: bool,

    pub debug_influx_uri: bool,
    pub debug_influx_auth: bool,
    pub debug_influx_lp: bool,
    pub debug_influx_output: bool,
    pub debug_influx_instances: bool,

    pub run_flux_verify_record: bool,
    pub add_flux_query_verify_record_suffix: bool,   
    pub debug_flux_query: bool,
    pub debug_flux_result: bool, 
        
    pub run_egrep: bool,
    pub debug_egrep: bool,

    pub debug_backup: bool,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Backup {
    pub dir: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Delay {
    pub second: u8,
    pub minute: u8,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AllInflux {
    pub values: Vec<Influx>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Influx {
    pub name: String,
    pub status: bool,
    pub secure: String,
    pub server: String,
    pub port: u16,

    pub bucket: String,
    pub token: String,
    pub org: String,
    pub precision: String,

    //pub measurement: String, ### TO_DEL
    pub machine_id: String,
    pub carrier: String,
    pub flag_valid_default: bool,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub csv: TemplateCsv,
    pub curl: TemplateCurl,
    pub sensors: TemplateSensors,
    pub flux: TemplateFlux,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateCsv {

    pub tag_machine: String,
    pub tag_id: String,
    pub tag_carrier: String,
    pub tag_valid: String,
    pub field: String,

    pub annotated_datatype: String,
    pub annotated_header: String,
    pub csv_annotated: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateFlux {
    pub query_verify_record: String,
    pub query_verify_record_range_start: String,
    pub query_verify_record_suffix: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateCurl {
    pub program: String,

    pub param_insecure: String,
    pub param_request: String,
    pub param_post: String,
    pub param_header: String,
    pub param_data: String,
    
    pub influx_uri_api: String,
    pub influx_uri_write: String,
    pub influx_uri_query: String,

    pub influx_auth: String,
    pub influx_accept: String,
    pub influx_content: String,

    pub influx_lp: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateSensors {
    pub program: String,
    pub param_1: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    pub status: bool,
    pub name: String,
    pub pointer: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AllSensors {
    //values: [i32; 3], // fixed array with three int's
    //values: Vec<i32>, // unlimited size vector
    pub values: Vec<Sensor>,
    pub measurement: String,
}


#[cfg(test)]
mod tests {
    use super::*; // GLOB

    /*
    #[test]
    fn panic_test() {
        panic!("### MAKE THIS TEST FAIL");
    }
    */

    
    #[test]
    fn one_result() {
        let query = "duct"; //SEARCH STRING
        // start's with \ no new_line \n
        // tim padem se me vrati jen radek ktery obsahuje QUERY
        let contents = "\
Rust:
safe, fast, productive.
Pick three."; // DATA

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }


    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}


pub fn search_case_insensitive<'a>(query: &str, data: &'a str) -> Vec<&'a str> {
    // NOT CASE SENSITIVE
    
    data
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()

    /*
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in data.lines() {
        if line.to_lowercase().contains(&query) { // changes String -> &str slice
            results.push(line);
        }
    }

    results
    */
}


pub fn search<'a>(query: &str, data: &'a str) -> Vec<&'a str> {
    // CASE SENSITIVE

    data
        .lines()
        .filter(|line| line.contains(query))
        .collect()
    
    /*
    let mut results = Vec::new();

    for line in data.lines() {
        // MY LOWER_CASE 
        if line.contains(query) { // DATA LINE TO lower_case
            results.push(line);
        }
        
    }

    // vec![] // TEST FAILED as we return empty vector
    results // TEST OK
    */
}


pub fn read_config(config: CmdArgs) -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string(&config.filename)?;

    /*
    let mut data = String::new();
    fs::File::open(&config.filename)?.read_to_string(&mut data)?;
    */
    
    let results = match config.case_sensitive {
        true => search(&config.query, &data),
        false => search_case_insensitive(&config.query, &data)
    };

    let mut count: u8 = 0;
    let count_closure = |x: u8| -> u8 { x + 1 };

    println!("\n#EGREP:\nfile: {f}\nquery: {q}\ncase_sensitive: {cs}\n\nRESULTS:",
             f=&config.filename,
             q=&config.query,
             cs=config.case_sensitive);
    
    for line in results {
        count = count_closure(count); // INSTEAD count += 1;
        println!("[{i:?}]: {l}",
                 l=line.trim(),
                 i=count);
    }
    
    Ok(())
}


impl CmdArgs {
    pub fn new(mut args: env::Args) -> Result<CmdArgs, &'static str> {
        println!("\n#COMMAND: {:#?}",
                 args);

        const ARG_COUNT: usize = 4; // sum of struct CmdArgs members + 1 PROGRAM
        
        if args.len() < ARG_COUNT {
            return Err("not enough arguments")
        } else if args.len() > ARG_COUNT {
            return Err("too many arguments")
        }

        let _program = match args.next() { // FUTURE_USE
            Some(arg) => arg,
            None => return Err("should never fail"),
        };

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no QUERY string [keyword] cmd_argument"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("no FILE NAME [path] cmd_argument"),
        };

        /* ENV
        // terminal: $ export CASE_INSENSITIVE=1
        // terminla: $ unset CASE_INSENSITIVE
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
         */

        let case_sensitive = match args.next() {
            /* LONG WAY
            Some(arg) => if vec!["true", "false"].contains(&arg.to_lowercase().as_str()) {
                //arg.to_lowercase().parse::<bool>().unwrap()
                arg.to_lowercase().parse::<bool>().unwrap()
            } else {
                return Err("CASE SENSITIVE not true/false BOOL")
            },
            None => return Err("no CASE SENSITIVE cmd_argument"), // probably will never happen ?
            */

            Some(arg) => arg.to_lowercase().parse::<bool>().unwrap_or_else(|err| {
                eprintln!("\nEXIT: CASE SENSITIVE argument not true/false\nREASON: >>> {}", err);
                process::exit(1);
            }),
            None => return Err("no CASE SENSITIVE cmd_argument"), // probably will never happen ?
        };

        return Ok(CmdArgs {query, filename, case_sensitive});
    }
}


pub fn parse_toml_config(cmd_args: &CmdArgs) -> Result<TomlConfig, Box<dyn Error>> {
    println!("\n#PARSE file_config -> TOML:\n{:}", &cmd_args.filename);

    let toml_file = fs::read_to_string(&cmd_args.filename);
    let toml_config: TomlConfig = toml::from_str(&toml_file.unwrap()).unwrap();

    /*
    let fookume = "foookin = 'paavel'".parse::<Value>().unwrap();
    println!("\nTOML: {} <- {:?}",
             fookume["foookin"],
             fookume,
    );
     */

    Ok(toml_config)
}
