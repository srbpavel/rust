// dummy settings and data via config
use crate::influxdb_toml_config_struct::{TomlConfig};

use influxdb_client;

use reqwest::blocking::{RequestBuilder};

use csv::StringRecord;

use serde::Deserialize;

use std::collections::HashMap;

use chrono::{DateTime,
             Utc,
};


// TEMPLATE_FORMATER
use template_formater::tuple_formater;


const RECORD_STRUCT: bool = true; // fields has to be exact as flux_output
//const RECORD_STRUCT: bool = false; // here you do not need to know field


/// this depend on flux query result
/// we start with &str and parse bool/u64/datetime later
/// very nice tutorial rust csv explained https://blog.burntsushi.net/csv/
///
///",result,table,_start,_stop,_time,_value,DsCarrier,DsId,DsPin,DsValid,Machine,_field,_measurement,host\r\n,_result,0,2022-02-16T08:45:43.372462165Z,2022-02-16T20:45:43.372462165Z,2022-02-16T20:45:09.299Z,20.5625,labjack,1052176647976,14,true,mrazak,DsDecimal,dallas,ruth\r\n
///
///Record { annotation: "", result: "_result", table: 0, start: "2022-02-16T08:45:43.372462165Z", stop: "2022-02-16T20:45:43.372462165Z", time: "2022-02-16T20:45:09.299Z", value: "20.5625", machine: "mrazak", ds_carrier: "labjack", ds_id: "1052176647976", ds_pin: "14", ds_valid: true, field: "DsDecimal", measurement: "dallas", host: "ruth" }
///
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Record<'h> {

    // this field has no name as it is "" , so we need to rename
    // see influx annotation info
    // https://docs.influxdata.com/influxdb/v2.1/reference/syntax/annotated-csv/
    // this is needed only when .deserialize(Some(headers)
    #[serde(rename = "")]
    annotation: &'h str,

    result: &'h str,
    table: u64,

    // we do not let Serde to parse DateTime as we do ourself
    // Option<DateTime<Utc>>
    #[serde(rename = "_start")]
    start: &'h str,
    #[serde(rename = "_stop")]
    stop: &'h str,
    #[serde(rename = "_time")]
    time: &'h str,

    #[serde(rename = "_value")]
    value: &'h str,
    #[serde(rename = "Machine")]
    machine: &'h str,

    /* 
    #[serde(rename = "MemoryCarrier")]
    memory_carrier: &'h str,
    #[serde(rename = "MemoryId")]
    memory_id: &'h str,
    #[serde(rename = "MemoryValid")]
    memory_valid: bool,
    */

    // /*
    #[serde(rename = "DsCarrier")]
    ds_carrier: &'h str,
    #[serde(rename = "DsId")]
    ds_id: &'h str,
    #[serde(rename = "DsPin")]
    ds_pin: &'h str,
    #[serde(rename = "DsValid")]
    ds_valid: bool,
    // */

    #[serde(rename = "_field")]
    field: &'h str,
    #[serde(rename = "_measurement")]
    measurement: &'h str,
    host: &'h str,
}


///  
///",result,table,_start,_stop,_time,_value,DsCarrier,DsId,DsPin,DsValid,Machine,_field,_measurement,host\r\n,_result,0,2022-02-16T18:30:58.441692585Z,2022-02-17T06:30:58.441692585Z,2022-02-17T06:30:09.737Z,19.0625,labjack,1052176647976,14,true,mrazak,DsDecimal,dallas,ruth\r\n
///
///{"": "", "DsPin": "14", "_start": "2022-02-16T18:30:58.441692585Z", "_stop": "2022-02-17T06:30:58.441692585Z", "DsId": "1052176647976", "_time": "2022-02-17T06:30:09.737Z", "DsValid": "true", "Machine": "mrazak", "_field": "DsDecimal", "_value": "19.0625", "table": "0", "_measurement": "dallas", "result": "_result", "host": "ruth", "DsCarrier": "labjack"}
///
type HashRecord = HashMap<String, String>;


/// &str -> DateTime instead Serde parsing
pub fn parse_time(data: &str) -> Option<DateTime<Utc>> {
    
    match data.parse() {
        Ok(t) => Some(t),
        Err(why) => {
            eprintln!("ERROR: conversion &str -> DateTime \nREASON >>>{why}");

            None
        }
    }
}


/// Hash
pub fn record_via_hash<'r>(rec: &'r StringRecord,
                           headers: &StringRecord) -> Result<HashRecord, csv::Error> {

    rec.deserialize(Some(headers))
}


/// Struct
pub fn record_via_struct<'r>(rec: &'r StringRecord,
                             headers: &'r StringRecord) -> Result<Record<'r>, csv::Error> {

    let record: Record = rec.deserialize(Some(headers))?;

    Ok(record)
}


///CSV
/// https://docs.influxdata.com/influxdb/v2.1/reference/syntax/annotated-csv/#annotations
pub fn parse_csv(response: &str) -> Result<(), csv::Error> {

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',') // ; -> for Error simulation
        // FUTURE USE
        //.quote(b'"')
        //.double_quote(true)
        //.escape(Some(b'\\'))
        //.flexible(true)
        //.trim(csv::Trim::All)
        //.comment(Some(b'#'))
        .from_reader(response.as_bytes());

    // HEADER clone as needed later for single_record -> CSV StringRecord
    let headers = &reader.headers()?.clone();
    println!("\nHEADER: {:?}",
             headers,
    );

    let mut record_counter = 0;
    
    for single_record in reader.records() { // .records() -> iterator

        record_counter += 1;
        
        match single_record {
            
            Ok(rec) => { // StringRecord
                
                if RECORD_STRUCT { 
                    let s_record = record_via_struct(&rec,
                                                     &headers,
                    )?;

                    /* DEBUG 
                    println!("{:?}\n<{}>",
                             s_record,
                             s_record.memory_valid,
                    );
                    */

                    let time = parse_time(s_record.time); // .value -> Error 
                    let start = parse_time(s_record.start);
                    let stop = parse_time(s_record.stop);
                    
                    println!("\nRECORD[{record_counter}]: {:?}\ntime: {:?}\nts: {:?}\nstart: {:?}\nstop: {:?}",
                             s_record,
                             time,
                             
                             match time {
                                 Some(t) => t.timestamp_millis(),
                                 None => 0,
                             },
                             
                             start,
                             stop,
                    );
                    
                // FUTURE_USE
                } else {
                    let h_record = record_via_hash(&rec,
                                                   &headers,
                    )?;

                    println!("{:?}\nfield: <{}>",
                             h_record,
                             h_record["_field"],
                    );
                }

            },

            Err(why) => {
                eprintln!("ERROR: record\nREASON >>>: {why:?}");

            },
        };
    }

    Ok(())
}


// START
pub fn start(config: TomlConfig) -> Result<(), reqwest::Error> {
    // JOZEFINA -> single INFLUX settings
    //println!("\n@TOML Influx: {:#?}", &config.all_influx.values[1]);

    let active_config = &config.all_influx.values[1];

    // initial Config, here copied from TOML but can be from elsewhere
    let influx_config = influxdb_client::connect::InfluxConfig::new(
        &active_config.name,
        active_config.status,

        &active_config.secure,
        
        &active_config.server,
        active_config.port,
        
        &active_config.bucket,
        &active_config.token,
        &active_config.org,
        &active_config.precision,
        
        &active_config.machine_id,
        &active_config.carrier,

        active_config.flag_valid_default,
    );

    println!("\n@LIB InfluxConfig: {influx_config:#?}");
    
    const MEASUREMENT: &str = "dallas";

    let uri_write = influxdb_client::format::uri_write(
        &format!("{}{}",
                 &config.template.curl.influx_uri_api,
                 &config.template.curl.influx_uri_write),
        
        &influx_config,
        
        config.flag.debug_tuple_formater,
    );
    
    /*
    let uri_write = influxdb_client::format::uri_write(
        &format!("{}{}",
                 &config.template.curl.influx_uri_api,
                 &config.template.curl.influx_uri_write),
        vec![
            ("secure", &active_config.secure),
            ("server", &active_config.server),
            ("port", &active_config.port.to_string()),
            
            ("org", &active_config.org),
            ("bucket", &active_config.bucket),
            ("precision", &active_config.precision),
            
        ],
        false, // DEBUG flag via config
    );
    */

    /*
    let uri_write = tuple_formater(&format!("{}{}",
                                            &config.template.curl.influx_uri_api,
                                            &config.template.curl.influx_uri_write),
                                   &vec![
                                       ("secure", &active_config.secure),
                                       ("server", &active_config.server),
                                       ("port", &active_config.port.to_string()),

                                       ("org", &active_config.org),
                                       ("bucket", &active_config.bucket),
                                       ("precision", &active_config.precision),

                                   ],
                                   false, // DEBUG flag via config
    );
    */

    let uri_query = influxdb_client::format::uri_query(
        &format!("{}{}",
                 &config.template.curl.influx_uri_api,
                 &config.template.curl.influx_uri_query),
        
        &influx_config,
        
        config.flag.debug_tuple_formater,
    );
    
    /*
    let uri_query = tuple_formater(&format!("{}{}",
                                            &config.template.curl.influx_uri_api,
                                            &config.template.curl.influx_uri_query),
                                   &vec![
                                       ("secure", &active_config.secure),
                                       ("server", &active_config.server),
                                       ("port", &active_config.port.to_string()),

                                       ("org", &active_config.org),
                                   ],
                                   false, // DEBUG flag via config
    );
    */

    //println!("URI: {uri_query:?}");
    
    let flux_query = format!("from(bucket:\"{bucket}\") |> range(start:{range_start}) |> filter(fn:(r) => r._measurement == \"{measurement}\") |> sort(columns: [\"_time\"], desc:true) |> limit(n:1)",
                             bucket=&active_config.bucket,
                             measurement=MEASUREMENT,
                             //range_start=RANGE_START,
                             range_start=&config.template.flux.query_verify_record_range_start,
                             
    );

    //println!("FLUX_QUERY: {flux_query}");

    let token = tuple_formater(&config.template.curl.influx_auth[1],
                               &vec![
                                   ("token", &active_config.token),
                               ],
                               false,
    );

    // NEW
    let influx_call = influxdb_client::connect::InfluxCall::new(
        &uri_write,
        &uri_query,
        
        vec![&config.template.curl.influx_auth[0],
             &token,
        ],

        vec![&config.template.curl.influx_accept[0],
             &config.template.curl.influx_accept[1],
        ],

        vec![&config.template.curl.influx_content[0],
             &config.template.curl.influx_content[1],
        ],
    );

    println!("\n@InfluxCall: {influx_call:#?}");

    // REQW
    let request: Result<RequestBuilder, Box< dyn std::error::Error>>
        = influxdb_client::connect::read_flux_query(
            influx_call,
            flux_query,
            config.flag.debug_flux_query,
        );

    println!("\nREQUEST: {request:?}");

    let response = request
        .unwrap()
        .send()? // reqwest::Error
        .text()?; // -> String

    let response_len = response
        .split("\r\n,") // ',' not to catch last line \r\n\r\n
        .collect::<Vec<_>>()
        .len() - 1; // -HEADER

    println!("\nRESPONSE[{len}]: {response:#?}",
             len = response_len,
    );
    
    /* CSV
    let csv_status = parse_csv(&response);
    println!("\nCSV_STATUS: {csv_status:?}");
    */

    Ok(())
}
