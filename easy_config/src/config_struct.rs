use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlConfig {
    // ROOT
    pub work_dir: String,
    pub name: String,

    pub host: String,
    //pub host: &'static str,

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


#[derive(Serialize, Deserialize, Debug, Clone)]
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

/*
impl Influx {
    pub fn default() -> Influx {
        Influx {
            name: "DEFAULT".to_string(),
            status: false,

            secure: "https".to_string(),
            
            server: "".to_string(),
            port: 8086,
            
            bucket: "BUCKET".to_string(),
            token: "TOKEN".to_string(),
            org: "ORG".to_string(),
            precision: "ms".to_string(),
            
            machine_id: "MACHINE_ID".to_string(),
            carrier: "CARRIER".to_string(),
            flag_valid_default: false,
        }
    }
}
*/


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



