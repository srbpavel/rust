use std::process::Command;
use std::{thread, time};
use std::collections::HashMap;

use crate::settings::{TomlConfig, Influx, TemplateSensors};

use crate::measurement::{Record};

use crate::util::template_formater::tuple_formater;


//#[derive(Debug, Clone)]
#[derive(Debug)]
pub struct InfluxCall {
    pub uri_write: String,
    pub uri_query: String,
    pub auth: String,
    pub accept: String,
    pub content: String,
}

//impl Copy for InfluxCall { }

/*
impl Clone for InfluxCall {
    fn clone(&self) -> InfluxCall {
        *self
    }
}
*/

/*
#[derive(Debug)]
pub struct InfluxData<'a> {
    pub properties: &'a InfluxCall,
    pub lp: String,
}
*/
#[derive(Debug)]
pub struct InfluxData {
    //pub settings: Influx,
    pub config: Influx,
    pub properties: InfluxCall,
    pub lp: String,
}

/*
impl InfluxData<'_> {
    pub fn new(properties: &InfluxCall,
*/
impl InfluxData {
    pub fn _new(//settings: Influx,
                config: Influx,
                properties: InfluxCall,
                lp: String) -> InfluxData {
        
        InfluxData {
            //settings,
            config,
            properties,
            lp,
        }
    }
    
    pub fn default() -> InfluxData {
        InfluxData {
            config: Influx { ..Influx::default()

            },

            properties: InfluxCall {
                uri_write: "".to_string(),
                uri_query: "".to_string(),
                auth: "".to_string(),
                accept: "".to_string(),
                content: "".to_string(),
            },
            lp: "".to_string(),
        }
    }

    pub fn import_lp<'a>(&self,
                         config: &TomlConfig) {
        
        import_lp_via_curl(config,
                           &self) // &
    }

}


pub fn prepare_csv_header_format(config: &TomlConfig,
                                 metric: &TemplateSensors) -> String {

    tuple_formater(&metric.annotated_header, 
                   &vec![
                       ("tag_machine", &metric.tag_machine),
                       ("tag_carrier", &metric.tag_carrier),
                       ("tag_valid", &metric.tag_valid),
                       ("tag_id", &metric.tag_id),
                       ("field", &metric.field),
                   ],
                   config.flag.debug_template_formater
    )
}


pub fn csv_display_header(datatype: &String,
                          tags_and_fields: &String) {

    println!("{}\n{}",
             &datatype,
             tags_and_fields,
    );
}


pub fn prepare_csv_record_format(config: &TomlConfig,
                                 record: &Record,
                                 metric: &TemplateSensors) -> String {

    tuple_formater(&metric.csv_annotated, 
                   &vec![
                       ("measurement", &record.measurement),
                       ("host", &record.host),
                       ("machine", &record.machine),
                       ("carrier", &record.carrier),
                       ("valid", &record.valid.to_string()),
                       ("ts", &record.ts.to_string()),
                       ("id", &record.id.to_string()),
                       ("value", &record.value.to_string()),
                   ],
                   config.flag.debug_template_formater
    )
}


// JUST FOR TEST -> future use
fn display_yield_data(records: Vec<HashMap<String, String>>,
                      config_metric: &TemplateSensors) {

    let tag = &config_metric.tag_id;

    for r in records.into_iter() {
        // DEBUG record hash_map
        //println!("\nr: {:#?}", r);

        // DEBUG key,value pair
        /*
        for key in r.keys() {
            println!("{k}: {v}",
                     k=key,
                     v=r.get(key).unwrap(),
            );
        }
        */

        let tag_value = match r.get(&tag.to_string()) {
            Some(value) => value.to_string(),
            None => format!("ERROR >>> no KEY: {:#?}", &tag),
        };

        let field = match r.get("_field"){
            Some(value) => value.to_string(),
            None => format!("ERROR >>> no KEY: {:#?}", "_field"),
        };

        let value = match r.get("_value"){
            Some(value) => value.to_string(),
            None => format!("ERROR >>> no KEY: {:#?}", "_value"),
        };
        
        println!("{tag}: {tag_value} / {f}: {v}",
                 tag=tag,
                 tag_value=tag_value,
                 f=field,
                 v=value,
        );
    }
}


fn yield_flux_result_records(records: Vec<HashMap<String, String>>,
                             config_metric: &TemplateSensors) {

    // flux result -> filter via measurement -> for _value change type from int to float
    let new_records = records
        .into_iter() // VEC
        .map(|r| {
            match r.get("_measurement").unwrap() == "temperature" { 
                true => 
                    r
                    .into_iter() // HASH_MAP
                    .map(|(key,value)| if key.trim() == "_value" {
                        (key,
                         match value.parse::<f64>() {
                             Ok(v) => format!("{:.3}", v),

                             Err(why) => {
                                 eprintln!("ERROR: \"_value\" cannot be converted to float: <{}>\nREASON >>> {}",
                                           value,
                                           why,
                                 );

                                 value
                             },
                         }
                        )
                    } else {
                        (key, value)
                    }
                    )
                    .collect::<HashMap<String, String>>(),

                false => r
            }
        }
        )
        .collect::<Vec<_>>();
    
    // DEBUG record hash_map
    //println!("\nnew_records: {:?}", new_records);

    // -> display
    display_yield_data(new_records,
                       config_metric);
}


fn flux_responde_data_line_to_vec(lines: Option<String>) -> Vec<String> {
    let data = lines
        .unwrap_or_else(|| {
            eprintln!("\nERROR: in FLUX RESPONDE DATA line");
            "error".to_string()
        })
        .split(',')
        .map(|k| k.to_string())
        .collect::<Vec<String>>();

    // DEBUG 
    //println!("DATA: {:?}", data);

    data
}


fn pair_to_hash_map(config: &TomlConfig,
                    keys: Vec<String>,
                    values: Vec<Vec<String>>) -> Vec<HashMap<String, String>> {

    let mut records: Vec<HashMap<String, String>> = Vec::new();

    for v in values.iter() {
        let mut record: HashMap<String, String> = HashMap::new();
        
        for (k,v) in keys.iter().zip(v.into_iter()) {
            if k != "" {
                record.insert(k.to_string(),
                              v.to_string(),
                );
            }
        }
        records.push(record)
    }
    
    if config.flag.debug_flux_records {
        println!("\nrecords: {:#?}", records);
    }

    records 
}


fn flux_csv_to_hash(config: &TomlConfig,
                    data: String) -> Vec<HashMap<String, String>> {

    let mut keys: Vec<String> = Vec::new();
    let mut keys_len: usize = 0;
    let mut values: Vec<Vec<String>> = Vec::new();

    let mut lines = data
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .into_iter();

    for i in 1..=lines.len() {
        match i {
            // FIRST line to Vec<keys>
            1 => {
                keys = flux_responde_data_line_to_vec(lines.next());

                keys_len = keys.len();
            },
            // OTHER lines APPEND to Vec<values> if valid
            _ => {
                let items = flux_responde_data_line_to_vec(lines.next());

                if items.len() == keys_len {
                    values.push(items)
                }
                else { 
                    if config.flag.debug_flux_result_invalid_line {
                        eprintln!("\nWARNING: in FLUX RESPONDE DATA line >>> values does not fit keys: values_len: {vl} / keys_len: {kl}\nVALUES: {:?}",
                                  i=items,
                                  vl=items.len(),
                                  kl=keys_len,
                        );
                    }
                }
            }
        }
    }

    if config.flag.debug_flux_pairs {
        println!("keys: {:?}", keys);
        println!("values: {:?}", values);
    }

    // HASH_MAP pair: key + value
    pair_to_hash_map(config,
                     keys,
                     values)
}


fn parse_flux_result(config: &TomlConfig,
                     data: String,
                     config_metric: &TemplateSensors) {

    if &data == "\r\n" {
        eprintln!("\nWARNING: not valid flux result\ndata_len: {}\ndata: {:#?}",
                  data.len(),
                  data,
        );
    }

    let records = flux_csv_to_hash(&config,
                                   data);

    if config.flag.yield_flux_records {
        yield_flux_result_records(records,
                                  &config_metric);
    }
}


fn verify_flux_result(_config: &TomlConfig,
                      data: &String) -> bool {

    //match data.trim() == "\r\n" {
    match ["\r\n", ""].contains(&data.trim()) {
        true => {
            eprintln!("\nWARNING: flux result\ndata: {:#?}",
                      data,
            );

            true
        },

        false => false
            
    }
}


fn flux_query_via_curl(config: &TomlConfig,
                       influx: &InfluxCall,
                       influx_query: &String,
                       config_metric: &TemplateSensors) -> bool {

    let curl_output = Command::new(&config.template.curl.program)
        .args([
            &config.template.curl.param_insecure,
            &config.template.curl.param_request,
            &config.template.curl.param_post,
            &influx.uri_query, // #URI
            &config.template.curl.param_header,
            &influx.auth, // #AUTH
            &config.template.curl.param_header,
            &influx.accept,
            &config.template.curl.param_header,
            &influx.content,
            &config.template.curl.param_data,
            influx_query, // #QUERY
        ])
        .output().expect("failed to execute command");

    /* FUTURE USE 
    let error_data = String::from_utf8(stderr).expect("Found invalid UTF-8");
    eprintln!("STDERR: {:#?}", error_data);
    */
    
    let stdout_data = match String::from_utf8(curl_output.stdout.to_vec()) {
        Ok(data) => data,
        Err(why) => {
            eprintln!("\nERROR: flux result read data problem\nREASON >>> {}", why);
            
            "".to_string()
        }
    };

    if config.flag.debug_flux_result {
        println!("\ndata_len: {:#?} / lines_count: {:#?}\ndata: {:#?}",
                 stdout_data.len(),
                 stdout_data.lines().count(),
                 stdout_data,
        );
    }

    if config.flag.parse_flux_result {
        // VERIFY
        let flux_result_status = verify_flux_result(&config,
                                                    &stdout_data);
        
        if !flux_result_status {
            parse_flux_result(&config,
                              stdout_data,
                              config_metric);

            false
                
        } else {
            println!("\n#: FLUX responde -> niet goed");
            true
        }
    } else {

        false
    }
}


fn prepare_generic_flux_query_format(config: &TomlConfig,
                                     single_influx: &Influx,
                                     generic_pre_record: &Record,
                                     metric: &TemplateSensors,
                                     utc_influx_format: &String) -> String {

    let flux_template = metric.generic_query_verify_record.to_string();
    
    tuple_formater(&flux_template,
                   &vec![
                       ("tag_carrier", &metric.tag_carrier),
                       ("tag_valid", &metric.tag_valid),
                       ("tag_id", &metric.tag_id),
                       
                       ("bucket", &single_influx.bucket),
                       ("start", &config.template.flux.query_verify_record_range_start),
                       ("measurement", &metric.measurement),
                       
                       // COMPARE only id + time // if needed can add _VALUE or implement INCREMENT_id
                       ("id", &generic_pre_record.id.to_string()),
                       ("dtif", utc_influx_format), // rfc3339 Date_Time Influx Format -> 2021-11-16T13:20:10.233Z
                   ],
                   config.flag.debug_template_formater
    )
}


pub fn run_flux_query(config: &TomlConfig,
                      config_metric: &TemplateSensors,

                      //single_influx: &Influx,
                      influx_data: &InfluxData,
                      
                      metric_pre_result: &Record,
                      utc_influx_format: &String,
                      //influx: &InfluxCall)
) {

    let generic_influx_query = prepare_generic_flux_query_format(
        &config,
        //&single_influx,
        &influx_data.config, //settings,

        &metric_pre_result,
        &config_metric,
        &utc_influx_format);

    if config.flag.debug_flux_query {
        println!("\n#QUERY:\n{}",
                 generic_influx_query,
        );
    }

    let delay = time::Duration::from_millis(config.delay.flux_query_sleep_duration_ms);

    for i in 1..config.delay.flux_repeat_query_count + 1 {
        if i != 1 {
            println!("\n#[{}]: sleeping before next try", i);

            thread::sleep(delay);
        }

        let flux_result_status = flux_query_via_curl(&config, //os_call_curl_flux
                                                     //&influx,
                                                     &influx_data.properties,
                                                     
                                                     &generic_influx_query,
                                                     &config_metric);

        if flux_result_status {
            println!("\n#QUERY:\n{}", generic_influx_query);

            println!("\n#[{}]: FLUX_RESULT_STATUS: {} >>> REPEAT",
                     i,
                     flux_result_status,
            );
        } else {
            break;
        }
    }
}


pub fn prepare_influx_format(config: &TomlConfig,
                             influx_inst: &Influx) -> InfluxCall {
    
    // URI_WRITE 
    let uri_write = tuple_formater(&format!("{}{}",
                                            &config.template.curl.influx_uri_api,
                                            &config.template.curl.influx_uri_write),
                                   &vec![
                                       ("secure", &influx_inst.secure),
                                       ("server", &influx_inst.server),
                                       ("port", &influx_inst.port.to_string()),
                                       ("org", &influx_inst.org),
                                       ("bucket", &influx_inst.bucket),
                                       ("precision", &influx_inst.precision),
                                   ],
                                   config.flag.debug_template_formater
    );

    // URI_QUERY
    let uri_query = tuple_formater(&format!("{}{}",
                                            &config.template.curl.influx_uri_api,
                                            &config.template.curl.influx_uri_query),
                                   &vec![
                                       ("secure", &influx_inst.secure),
                                       ("server", &influx_inst.server),
                                       ("port", &influx_inst.port.to_string()),
                                       ("org", &influx_inst.org),
                                   ],
                                   config.flag.debug_template_formater
    );
    
    // AUTH
    let auth = tuple_formater(&config.template.curl.influx_auth,
                              &vec![
                                  ("token", &influx_inst.token),
                              ],
                              config.flag.debug_template_formater
    );
    
    // ACCEPT
    let accept_template = String::from(&config.template.curl.influx_accept);

    // CONTENT
    let content_template = String::from(&config.template.curl.influx_content);

    InfluxCall {uri_write: uri_write,
                uri_query: uri_query,
                auth: auth,
                accept: accept_template,
                content: content_template,
    }
}


/*
pub fn import_lp_via_curl(config: &TomlConfig,
                          influx: &InfluxCall,
                          single_sensor_lp: &String) {
    
    let curl_output = Command::new(&config.template.curl.program)
        .args([
            &config.template.curl.param_insecure,
            &config.template.curl.param_request,
            &config.template.curl.param_post,
            &influx.uri_write, // #URI
            &config.template.curl.param_header,
            &influx.auth, // #AUTH
            &config.template.curl.param_data,
            single_sensor_lp, // #LINE_PROTOCOL
        ])
        .output().expect("failed to execute command");

    if config.flag.debug_influx_output {
        println!("\nstdout: {}", String::from_utf8_lossy(&curl_output.stdout));
        println!("\nstderr: {}", String::from_utf8_lossy(&curl_output.stderr));
    
    }
}
*/
// /*
fn import_lp_via_curl<'a>(config: &TomlConfig,
                          data: &InfluxData) {
    
    let curl_output = Command::new(&config.template.curl.program)
        .args([
            &config.template.curl.param_insecure,
            &config.template.curl.param_request,
            &config.template.curl.param_post,
            &data.properties.uri_write, // #URI
            &config.template.curl.param_header,
            &data.properties.auth, // #AUTH
            &config.template.curl.param_data,
            &data.lp, // #LINE_PROTOCOL
        ])
        .output().expect("failed to execute command");

    if config.flag.debug_influx_output {
        println!("\nstdout: {}", String::from_utf8_lossy(&curl_output.stdout));
        println!("\nstderr: {}", String::from_utf8_lossy(&curl_output.stderr));
    
    }
}
// */


pub fn prepare_generic_lp_format(config: &TomlConfig,
                                 generic_pre_record: &Record,
                                 metric: &TemplateSensors)  -> String {

    tuple_formater(&metric.generic_lp,
                   &vec![
                       ("tag_machine", &metric.tag_machine),
                       ("tag_carrier", &metric.tag_carrier),
                       ("tag_valid", &metric.tag_valid),
                       ("tag_id", &metric.tag_id),
                       ("field", &metric.field),

                       ("measurement", &generic_pre_record.measurement),
                       ("host", &generic_pre_record.host),
                       ("machine_id", &generic_pre_record.machine),
                       
                       ("carrier", &generic_pre_record.carrier),
                       ("valid", &generic_pre_record.valid),
                       
                       ("id", &generic_pre_record.id),
                       ("value", &generic_pre_record.value.to_string()),
                       
                       ("ts", &generic_pre_record.ts.to_string()),
                   ],
                   config.flag.debug_template_formater
    )
}
