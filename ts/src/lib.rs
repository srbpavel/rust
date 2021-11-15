use std::fs;
use std::error::Error;
use std::env;
// use std::io::Read;

// TOML CONFIG
//use std::fs;
use toml;
use serde::{Serialize, Deserialize};


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

    // VEC
    pub all_influx: AllInflux,
    pub all_sensors: AllSensors,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Flag {
    pub debug_config_data: bool,
    pub debug_new_config: bool,
    
    pub debug_ts: bool,
    pub debug_ts_to_dt: bool,

    pub debug_sensor_output: bool,
    pub debug_sensor_instances: bool,

    pub debug_pointer_output: bool,

    pub debug_influx_uri: bool,
    pub debug_influx_lp: bool,
    pub debug_influx_output: bool,
    pub debug_influx_instances: bool,

}


#[derive(Serialize, Deserialize, Debug)]
pub struct Delay {
    pub second: u8,
    pub minute: u8,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AllInflux {
    /*
    default: Influx,
    backup: Influx,
     */
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

    pub measurement: String,
    pub machine_id: String,
    pub carrier: String,
    pub flag_valid_default: bool,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub curl: TemplateCurl,
    pub sensors: TemplateSensors,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateCurl {
    pub program: String,
    pub param_1: String,
    pub param_2: String,
    pub param_3: String,
    pub param_4: String,
    pub param_5: String,
    pub influx_uri: String,
    pub influx_auth: String,
    pub influx_lp: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateSensors {
    pub program: String,
    pub param_1: String,
}


/*
#[derive(Serialize, Deserialize, Debug)]
struct AllSensors {
    one: Sensor,
    two: Sensor,
    three: Sensor,
    four: Sensor,
}
*/


#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    pub status: bool,
    pub name: String, // mozna u8
    pub pointer: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AllSensors {
    //values: [i32; 3], // three int's
    //values: Vec<i32>, // unlimited vector
    pub values: Vec<Sensor>,
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
    data
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()

    /*
    // changes String -> &str slice
    let query = query.to_lowercase(); //oproti me pouzivaji & az pri volani .contains(&query)

    let mut results = Vec::new();

    for line in data.lines() {
        if line.to_lowercase().contains(&query) { // changes String -> &str slice
            results.push(line);
        }
    }

    results
    */
}


// <'a> lifetime
pub fn search<'a>(query: &str, data: &'a str) -> Vec<&'a str> {
    data
        .lines()
        .filter(|line| line.contains(query))
        .collect()
    
    /*
    let mut results = Vec::new();

    // MY_SHADOW // let query = &query.to_lowercase(); // QUERY TO lower_case
    for line in data.lines() {
        // MY LOWER_CASE // if line.to_lowercase().contains(query) { // DATA LINE TO lower_case
        if line.contains(query) { // DATA LINE TO lower_case
            results.push(line);
        }
        
    }

    // vec![] // TEST FAILED as we return empty vector
    results // TEST OK
    */
}


pub fn read_config(config: Config) -> Result<(), Box<dyn Error>> {
    // sice namem PANIC! ale nevypisuju filename parametr jako pro .unwrap_or_else(|err|    
    let data = fs::read_to_string(&config.filename)?;

    /*
    //let toml_config: TomlConfig = toml::from_str(&data.unwrap()).unwrap();
    let toml_config: TomlConfig = toml::from_str(&data).unwrap();
    println!("\nTOML_CONFIG::\nINFLUX\n{i:#?}\nSENSOR\n{s:?}",
             s =toml_config.all_sensors,
             i = toml_config.all_influx,
    );
    */
    
    /* TROSKU JINE VOLANI
    let mut data = String::new();
    fs::File::open(&config.filename)?.read_to_string(&mut data)?;
    */
    
    /*
    let data = fs::read_to_string(&config.filename)
        .expect(&format!("ERROR reading file: {}", &config.filename)); // zatim nevim proc &format!
    */

    //
    /* DEBUG
    println!("\n###DATA_START [{f}]:\n{d}\n###DATA_END\n",
             d=data,
             f=config.filename);
    //
    */

    /*
    let mut count: u8 = 0;
    for line in search(&config.query, &data) {
        count += 1;
        println!("[{i}] result_line: {l}",
                 l=line,
                 i=count);
     */

    let results = if config.case_sensitive {
        search(&config.query, &data)
    } else {
        search_case_insensitive(&config.query, &data)
    };

    let mut count: u8 = 0;
    //let count_closure = |x| x + 1;
    // let count_closure = |x: u8| -> u8 { x + 1 };

    for line in results {
        count += 1;
        //count = count_closure(count);
        println!("[{i:?}] result_line: {l}",
                 l=line,
                 i=count);
    }
    
    Ok(())
}


pub struct Config {
    // do not forget to chance ARG_COUNT verification 
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}


impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        println!("#COMMAND: {:#?}",
                 args);

        const ARG_COUNT: usize = 4; // struct Config members + 1 as also PROGRAM
        
        if args.len() < ARG_COUNT {
            return Err("not enough arguments")
        } else if args.len() > ARG_COUNT {
            return Err("too many arguments")
        }

        let _program = match args.next() { // FUTURE_USE
            Some(arg) => arg,
            None => return Err("Should never fail"),
        };

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a QUERY string [keyword]"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a FILE NAME [path]"),
        };

        /* ENV
        // terminal: $ export CASE_INSENSITIVE=1
        // terminla: $ unset CASE_INSENSITIVE
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
         */
        let case_sensitive = match args.next() {
            Some(arg) => if String::from(&arg) == "true" || String::from(&arg) == "false" {
                arg.parse::<bool>().unwrap()
            } else {
                return Err("CASE SENSITIVE not true/false BOOL")
            },
            None => return Err("Didn't get a CASE SENSITIVE"), // probably will never happen ?
        };
        
        return Ok(Config {query, filename, case_sensitive});
    }
}


pub fn parse_toml_config(config: Config) -> Result<TomlConfig, Box<dyn Error>> {
    println!("\n#PARSE:\n{:}", config.filename);

    let toml_file = fs::read_to_string(config.filename);
    let toml_config: TomlConfig = toml::from_str(&toml_file.unwrap()).unwrap();

    Ok(toml_config)
}
