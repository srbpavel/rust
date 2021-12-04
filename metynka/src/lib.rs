use std::fs;
use std::error::Error;
use std::env;

use toml;
use serde::{Serialize, Deserialize};

use std::process;

use std::collections::HashMap;

mod util;
pub use util::template_formater::tuple_formater;


pub struct CmdArgs {
    // when modified DO NOT forget to change ARG_COUNT verification -> learn to count struct descendants / not via hash_map
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}


impl CmdArgs {
    pub fn new(mut args: env::Args) -> Result<CmdArgs, &'static str> {
        // HARDCODED as config with debug_flag's not parsed yet
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
                arg.to_lowercase().parse::<bool>().unwrap()
            } else {
                return Err("CASE SENSITIVE not true/false BOOL")
            },
            None => return Err("no CASE SENSITIVE cmd_argument"), // probably will never happen ?
            */

            Some(arg) => arg
                .to_lowercase()
                .parse::<bool>()
                .unwrap_or_else(|err| {
                    eprintln!("\nEXIT: CASE SENSITIVE argument not true/false\nREASON: >>> {}", err);
                    process::exit(1);
                }),
            None => {
                eprintln!("no CASE SENSITIVE cmd_argument"); // probably will never happen but we need to cover ?
                process::exit(1);
            }
        };

        return Ok(CmdArgs {query, filename, case_sensitive});
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TomlConfig {
    // ROOT
    pub work_dir: String,
    pub name: String,
    pub host: String,

    //iter via HASH key
    pub metrics: HashMap<String, TemplateSensors>,
    
    // STRUCT
    pub flag: Flag,
    pub backup: Backup,
    pub template: Template,
    pub delay: Delay,
    pub email: Email,
    
    // VEC
    pub all_influx: AllInflux,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
    pub status: bool,
    pub smtp_server: String,
    pub port: u16,

    pub source_email: String,
    pub v_pass: String,

    pub target_email: String,
    pub sms_email: String,

    pub sender_machine: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Flag {
    pub debug_ts: bool,

    pub debug_template_formater: bool,
    
    pub debug_sensor_output: bool,
    pub debug_metric_instances: bool,
    pub debug_pointer_output: bool,
    pub debug_metric_record: bool,

    pub debug_influx_instances: bool,
    pub debug_influx_lp: bool,
    pub debug_influx_uri: bool,
    pub debug_influx_auth: bool,
    pub debug_influx_output: bool,

    pub run_flux_verify_record: bool,
    pub add_flux_query_verify_record_suffix: bool,   
    pub debug_flux_query: bool,
    pub debug_flux_result: bool, 
        
    pub run_egrep: bool,
    pub debug_egrep: bool,

    pub debug_backup: bool,

    pub influx_skip_import: bool,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Backup {
    pub dir: String,
    pub file_extension: String,
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

    pub machine_id: String,
    pub carrier: String,
    pub flag_valid_default: bool,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateSensors {
    pub flag_status: bool,
    pub measurement: String,

    pub program: String,
    pub args: Vec<String>,

    pub flag_pipe: bool,
    pub pipe_program: String,
    pub pipe_args: Vec<String>,

    pub values: Vec<Sensor>,
    
    pub tag_machine: String,
    pub tag_id: String,
    pub tag_carrier: String,
    pub tag_valid: String,

    pub field: String,

    pub annotated_datatype: String,
    pub annotated_header: String,
    pub csv_annotated: String,

    pub generic_lp: String,
    pub generic_query_verify_record: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub curl: TemplateCurl,
    pub flux: TemplateFlux,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateFlux {
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
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    pub status: bool,
    pub name: String,
    pub pointer: String,
}


fn verify_influx_contains_field_loop(filename: &String,
                                     influx: &Influx,
                                     mut bool_list: Vec<bool>) -> Vec<bool>{

    let fields_to_verify_contains = vec![
        // VARIABLE_NAME / VALUE / only allowed VALUES
        ("secure", &influx.secure[..], vec!["http", "https"]),
        ("precision", &influx.precision[..], vec!["s", "ms", "ns"]), // future use -> for now only MS format
    ];
    
    for fc in &fields_to_verify_contains {
        let (field_name,
             field_value,
             field_members) = (&fc.0,
                               &fc.1,
                               &fc.2,
        );
        
        bool_list.push(verify_influx_contains_field(&field_members,
                                                    &field_value,
                                                    &vec![
                                                        ("influx_instance", &influx.name), 
                                                        ("file", &filename),
                                                        ("variable",
                                                         &format!("{}", field_name)
                                                        ),
                                                        ("value", &field_value),
                                                        ("members",
                                                         &format!("{:?}", field_members)
                                                        ),
                                                    ],
        ))
    }

    bool_list
}


fn verify_influx_contains_field(allowed_values: &Vec<&str>,
                                value: &str,
                                msg_tuple_list: &Vec<(&str, &str)>) -> bool {
    
    let err_msg = "\n#ERROR: config file: {file} influx <{influx_instance}> settings \"{variable}={value}\" value  not in {members}\n\n>>> EXIT";

    if !allowed_values.contains(&&value.to_lowercase()[..]) { // ONLY ALLOWED VALUES
        eprintln!("{}", tuple_formater(&err_msg.to_string(),
                                       &msg_tuple_list,
                                       false // HARDCODED -> no need to debug
        ));
        
        false

    } else {

        true
    }
}


fn verify_influx_empty_field_loop(filename: &String,
                                  influx: &Influx) -> Vec<bool>{

    let mut bool_list = Vec::new();
    
    let fields_to_verify_non_empty = vec![
        // VARIABLE_NAME / VALUE
        ("name", &influx.name),
        ("server", &influx.server),
        ("bucket", &influx.bucket),
        ("token", &influx.token),
        ("org", &influx.org),
        ("machine_id", &influx.machine_id),
        ("carrier", &influx.carrier),
    ];
    
    for f in &fields_to_verify_non_empty {
        let (field_name, field_str) = (&f.0, &f.1[..]);
        
        bool_list.push(verify_influx_empty_field(field_str,
                                                 &vec![
                                                     ("influx_instance", &influx.name), 
                                                     ("file", &filename),
                                                     ("variable",
                                                      &format!("{}", field_name)
                                                     ),
                                                     ("value", field_str),
                                                     ("msg", "is EMPTY"),
                                                 ])
        )
    }

    bool_list
}


fn verify_influx_empty_field(value: &str,
                             msg_tuple_list: &Vec<(&str, &str)>) -> bool {
    
    let err_msg = "\n#ERROR: config file: {file} influx <{influx_instance}> settings \"{variable}={value}\" {msg}\n\n>>> EXIT";
    
    if matches!(value, "") { // search for EMPTY field values
        eprintln!("{}", tuple_formater(&err_msg.to_string(),
                                       &msg_tuple_list,
                                       false // HARDCODED -> no need to debug
        ));
        
        false
    }
    else {

        true
    }
}
            

fn verify_influx_config(filename: &String,
                       influx: &Influx) {

    // EMPTY FIELDS
    let bool_list = verify_influx_empty_field_loop(filename,
                                                   influx,
    );

    // ONLY ALLOWED VALUES in FIELDS
    let bool_list = verify_influx_contains_field_loop(filename,
                                                      influx,
                                                      bool_list,
    );

    // EXIT if any FALSE
    if bool_list.contains(&false) {
        eprintln!("{:#?}\n>>> EXIT",
                  &influx
        );
        process::exit(1);
    }
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


pub fn search_case_sensitive<'a>(query: &str, data: &'a str) -> Vec<&'a str> {
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


// EGREP tutorial
pub fn read_config(args: CmdArgs) -> Result<(), Box<dyn Error>> {
    /*
    let mut data = String::new();
    fs::File::open(&args.filename)?.read_to_string(&mut data)?;
     */
    
    let data = fs::read_to_string(&args.filename)?;
    
    let results = match args.case_sensitive {
        true => search_case_sensitive(&args.query, &data),
        false => search_case_insensitive(&args.query, &data)
    };

    let mut count: u8 = 0;
    let count_closure = |x: u8| -> u8 { x + 1 };

    println!("\n#EGREP:\nfile: {f}\nquery: \"{q}\"\ncase_sensitive: {cs}\n\nRESULTS:",
             f=&args.filename,
             q=&args.query,
             cs=args.case_sensitive);
    
    for line in results {
        count = count_closure(count); // INSTEAD count += 1;
        println!("[{i:?}]: {l}",
                 l=line.trim(),
                 i=count);
    }
    
    Ok(())
}

/// parse toml config file as program argument
///
pub fn parse_toml_config(cmd_args: &CmdArgs) -> Result<TomlConfig, Box<dyn Error>> {
    println!("\n#PARSE file_config -> TOML:\n{:}\n", &cmd_args.filename);

    let toml_file = fs::read_to_string(&cmd_args.filename).unwrap_or_else(|err| {
        eprintln!("\nEXIT: error reading config file: {}\nREASON >>> {e}",
                  c=&cmd_args.filename,
                  e=err);

        process::exit(1);
    });

    let toml_config: TomlConfig = toml::from_str(&toml_file).unwrap_or_else(|err| {
        eprintln!("\nEXIT: error parsing TOML config file: {c}\nREASON >>> {e}",
                  c=&cmd_args.filename,
                  e=err);

        process::exit(1);
    });

    // CONFIG FIELD's validation -> learn better way, let's say via Struct default ?
    for single_influx in &toml_config.all_influx.values {
        verify_influx_config(&cmd_args.filename,
                             &single_influx);
    }

    Ok(toml_config)
}
