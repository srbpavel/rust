use serde::{Serialize, Deserialize};
use std::collections::HashMap;

type MyStr = Box<str>; 


#[derive(Serialize, Deserialize, Debug)]
pub struct TomlConfig {
    // ROOT
    //pub user: MyStr,

    //pub work_dir: MyStr,
    //pub name: MyStr,
    //pub host: MyStr,

    //iter via HASH key
    pub metrics: HashMap<MyStr, TemplateSensors>,
    
    // STRUCT
    pub flag: Flag,
    //pub backup: Backup,
    pub template: Template,
    pub delay: Delay,
    //pub email: Email,
    
    // VEC
    pub all_influx: AllInflux,
}


/*
#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
    pub status: bool,
    pub smtp_server: MyStr,
    pub port: u16,

    pub source_email: MyStr,
    pub v_pass: MyStr,

    pub target_email: MyStr,
    pub sms_email: MyStr,
}
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct Flag {
    pub debug_config: bool,
    
    //pub debug_ts: bool,

    pub debug_reqwest: bool,
    
    //pub debug_email: bool,
    //pub debug_email_body: bool,
    
    pub debug_template_formater: bool,
    
    //pub debug_sensor_output: bool,
    //pub debug_metric_instances: bool,
    //pub debug_pointer_output: bool,
    //pub debug_metric_record: bool,

    pub debug_influx_instances: bool,
    //pub debug_influx_lp: bool,
    //pub debug_influx_uri: bool,
    //pub debug_influx_auth: bool,
    //pub debug_influx_output: bool,

    //pub run_flux_verify_record: bool,
    pub debug_flux_query: bool,
    //pub debug_flux_result: bool,
    //pub parse_flux_result: bool,
    
    //pub debug_flux_records: bool,
    //pub debug_flux_result_invalid_line: bool,
    //pub debug_flux_pairs: bool,
    //pub yield_flux_records: bool,
    
    //pub run_egrep: bool,
    //pub debug_egrep: bool,

    //pub debug_backup: bool,

    //pub influx_skip_import: bool,
}

/*
#[derive(Serialize, Deserialize, Debug)]
pub struct Backup {
    pub dir: MyStr,
    pub file_extension: MyStr,
}
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct Delay {
    /*
    pub second: u8,
    pub minute: u8,
    */

    pub flux_query_sleep_duration_ms: u64,
    pub flux_repeat_query_count: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AllInflux {
    pub values: Vec<Influx>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Influx {
    pub name: MyStr,
    pub status: bool,

    pub secure: MyStr,

    pub server: MyStr,
    pub port: u16,

    pub bucket: MyStr,
    pub token: MyStr,
    pub org: MyStr,
    pub precision: MyStr,

    pub machine_id: MyStr,
    pub carrier: MyStr,
    pub flag_valid_default: bool,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateSensors {
    pub flag_status: bool,
    pub measurement: MyStr,

    //pub program: MyStr,
    //pub args: Vec<MyStr>,

    //pub flag_pipe: bool,
    //pub pipe_program: MyStr,
    //pub pipe_args: Vec<MyStr>,

    //pub values: Vec<Sensor>,
    
    pub tag_machine: MyStr,
    pub tag_id: MyStr,
    pub tag_carrier: MyStr,
    pub tag_valid: MyStr,

    pub field: MyStr,

    //pub annotated_datatype: MyStr,
    //pub annotated_header: MyStr,
    //pub csv_annotated: MyStr,

    //pub generic_lp: MyStr,
    //pub generic_query_verify_record: MyStr,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub curl: TemplateCurl,
    pub flux: TemplateFlux,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateFlux {
    pub query_verify_record_range_start: MyStr,
    pub query_verify_record_suffix: MyStr,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateCurl {
    pub program: MyStr,

    pub param_insecure: MyStr,
    pub param_request: MyStr,
    pub param_post: MyStr,
    pub param_header: MyStr,
    pub param_data: MyStr,
    
    pub influx_uri_api: MyStr,
    pub influx_uri_write: MyStr,
    pub influx_uri_query: MyStr,

    //pub influx_auth: MyStr,
    pub influx_auth: Vec<MyStr>,
    pub influx_accept: Vec<MyStr>,
    pub influx_content: Vec<MyStr>,
}

/*
#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    pub status: bool,
    pub name: MyStr,
    pub pointer: MyStr,
}
*/
