use std::fs;
use std::error::Error;
use std::env;
use toml;
use serde::{Serialize, Deserialize};
use std::process;
use std::collections::HashMap;

use crate::util::template_formater::tuple_formater;


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
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Flag {
    pub debug_ts: bool,

    pub debug_email: bool,
    pub debug_email_body: bool,
    
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
    pub debug_flux_query: bool,
    pub debug_flux_result: bool,
    pub parse_flux_result: bool,
    
    pub debug_flux_records: bool,
    pub debug_flux_result_invalid_line: bool,
    pub debug_flux_pairs: bool,
    pub yield_flux_records: bool,
    
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

    pub flux_query_sleep_duration_ms: u64,
    pub flux_repeat_query_count: u64,
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
                                       false, // HARDCODED -> no need to debug
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
                                       false, // HARDCODED -> no need to debug
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


/// parse toml config file as cmd program argument
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
