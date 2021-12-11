use std::process::{Command};
use std::{thread, time};
use std::collections::HashMap;

use metynka::{TomlConfig, Influx, TemplateSensors};

use crate::measurement::{PreRecord};

use crate::util::template_formater::tuple_formater;


#[derive(Debug)]
pub struct InfluxCall {
    pub uri_write: String,
    pub uri_query: String,
    pub auth: String,
    pub accept: String,
    pub content: String,
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


pub fn prepare_csv_record_format(config: &TomlConfig,
                                 record: &PreRecord,
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


fn yield_flux_result_records(records: Vec<HashMap<String, String>>,
                             config_metric: &TemplateSensors) {

    // sample -> tag_id
    let tag = &config_metric.tag_id;
    
    for r in records.into_iter() {
        /* 
        // DEBUG record hash_map
        println!("\nr: {:#?}", r);

        // DEBUG key,value pair
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
        
        println!("\n{tag}: {tag_value} / {f}: {v}",
                 tag=tag,
                 tag_value=tag_value,
                 f=field,
                 v=value,
        );
    }
}


fn flux_csv_to_hash(config: &TomlConfig,
                    data: String) -> Vec<HashMap<String, String>> {

    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<Vec<String>> = Vec::new();
    let mut records: Vec<HashMap<String, String>> = Vec::new();
    
    let lines_count = data.lines().count();
    let mut lines = data.lines();

    /* // CONFIG
    println!("\nresponde lines_count: {:#?}\n{:?}",
             lines_count,
             &data,
    );
    */

    for i in 1..lines_count {
        match i {
            // FIRST line to Vec<keys>
            1 => {
                keys = lines
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|k| k.to_string())
                    .collect::<Vec<String>>();
            },
            // OTHER lines APPEND to Vec<values> 
            _ => {
                values.push(lines
                            .next()
                            .unwrap()
                            .split(',')
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                );
            }
        }
    }

    if config.flag.debug_flux_pairs {
        println!("keys: {:?}", keys);
        println!("values: {:?}", values);
    }

    // HASH_MAP pair: key + value
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
    
    /* flux query .. |> count / OBSOLETE -> TO_DEL 
    match config.flag.add_flux_query_verify_record_suffix {
        true => {
            for line in data.lines() {
                if !line.contains("value") && line.trim().len() != 0 {
                    match line.split(",").last() {
                        Some(value) => match value.parse::<u64>() {
                            // FUTURE USE
                            Ok(1) => {
                                println!("flux result count: {}", //\n{:#?}",
                                         1,
                                );
                            },
                            _ => {
                                println!("WARNING: flux result count: not 1\nDATA >>> {}\n",
                                         data,
                                );
                            },
                        },
                        _ => {
                            println!("flux RESULT count: EMPTY\nDATA >>> {}\n",
                                     data,
                            );
                        }
                    }
                }
            }
        },
        false => {
            println!("flux RESULT: {}",
                     data,
                     );
        }
    }
    */
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
    
    let out_data = match String::from_utf8(curl_output.stdout.to_vec()) {
        Ok(data) => data,
        Err(why) => {
            eprintln!("\nERROR: flux result read data problem\nREASON >>> {}", why);
            
            "".to_string()
        }
    };

    if config.flag.debug_flux_result {
        let data_len = out_data.len();

        println!("\ndata_len: {:#?}\ndata: {:#?}",
                 &data_len,
                 out_data,
        );
    }

    if config.flag.parse_flux_result {
        // VERIFY
        let flux_result_status = verify_flux_result(&config,
                                                    &out_data);
        
        if !flux_result_status {
            parse_flux_result(&config,
                              out_data,
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
                                     generic_pre_record: &PreRecord,
                                     metric: &TemplateSensors,
                                     utc_influx_format: &String) -> String {

    let flux_template = metric.generic_query_verify_record.to_string();
    
    /* OBSOLETE -> TO_DEL

    let flux_template = match config.flag.add_flux_query_verify_record_suffix {
        true => format!("{}{}",
                metric.generic_query_verify_record.to_string(),
                config.template.flux.query_verify_record_suffix.to_string(),
        ),
        false => metric.generic_query_verify_record.to_string()
    };
    */

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
                      single_influx: &Influx,
                      metric_pre_result: &PreRecord,
                      utc_influx_format: &String,
                      influx: &InfluxCall) {

    let generic_influx_query = prepare_generic_flux_query_format(
        &config,
        &single_influx,
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
                                                     &influx,
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


pub fn prepare_generic_lp_format(config: &TomlConfig,
                                 generic_pre_record: &PreRecord,
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
