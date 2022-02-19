use crate::influxdb_toml_config_struct::{TomlConfig};

use influxdb_client::{
    config::InfluxConfig,
    call::InfluxCall,
    data::InfluxData,
    lp::LineProtocolBuilder,
    flux_query::QueryBuilder,
};

use reqwest::blocking::{
    Client,
    RequestBuilder,
};

use csv::StringRecord;

use serde::Deserialize;

use std::collections::HashMap;

use chrono::{DateTime,
             Utc,
};


/// use Struct or Hash for CSV parsing
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
pub fn parse_datetime(datetime: &str) -> Result<DateTime<Utc>, chrono::format::ParseError> {
    
    match datetime.parse() {
        Ok(t) => return Ok(t),
        Err(why) => {
            //eprintln!("ERROR: conversion &str -> DateTime \nREASON >>>{why}");

            //None
            Err(why)
        }
    }
}
/*
pub fn parse_datetime(datetime: &str) -> Option<DateTime<Utc>> {
    
    match datetime.parse() {
        Ok(t) => Some(t),
        Err(why) => {
            eprintln!("ERROR: conversion &str -> DateTime \nREASON >>>{why}");

            None
        }
    }
}
*/


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
pub fn parse_csv(client: &Client,
                 config: &TomlConfig,
                 influx_config: &InfluxConfig,
                 influx_call: &mut InfluxCall,
                 response: &str) -> Result<(), Box<dyn std::error::Error>> {

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
    //println!("\nHEADER: {headers:?}");

    let mut record_counter = 0;

    // ALL_RECORDS WALK
    for single_record in reader.records() { // .records() -> iterator

        record_counter += 1;
        
        match single_record {
            
            Ok(rec) => { // StringRecord

                // record as STRUCT
                if RECORD_STRUCT {
                    let s_record = record_via_struct(&rec,
                                                     &headers,
                    )?;

                    println!("\nRECORD[{record_counter}]:\n+ {s_record:?}");                    
                    // LP via METRIC
                    let metric = &config.metrics["temperature"];

                    let mut line_protocol_builder = LineProtocolBuilder::default();
                   //println!("\n@LP_B DEFAULT: {line_protocol_builder:?}");

                    let result_lpb = line_protocol_builder
                        .template(&metric.generic_lp)

                        .measurement(&metric.measurement)
                        .host(&config.host)

                        // TAG: NAME, VALUE
                        .tag(&metric.tag_machine, &influx_config.machine_id)
                        .tag(&metric.tag_carrier, &influx_config.carrier)
                        .tag(&metric.tag_id, &s_record.ds_id)

                        // record is valid -> true
                        .tag(&metric.tag_valid, &format!("{}", true))

                        // FIELD: NAME, VALUE
                        .field(&metric.field,
                               &s_record.value,
                        )

                        .ts(&format!("{}",
                                     match parse_datetime(s_record.time) {
                                         Ok(dt) => dt.timestamp_millis(),
                                         Err(why) => {
                                             return Err(Box::new(why))
                                         },
                                     }
                        ))

                        .build(config.flag.debug_tuple_formater);

                    // VERIFY LP parsing
                    match result_lpb {
                        Ok(data) => {
                            let updated_data = InfluxData {
                                config: influx_config.clone(),
                                // .clone() -> deMut
                                // expected struct `InfluxCall`,
                                // found `&mut InfluxCall<'_>`
                                //call: updated_call.clone(),
                                call: influx_call.clone(),
                                lp: data,
                            };
                            
                            //println!("\n@INFLUX_DATA_UPDATE_KEY:\n+ {influx_data:?}");
                            
                            // /* // WRITE
                            let write_result = write(client,
                                                     config,
                                                     //&influx_data);
                                                     &updated_data);
                            
                            if write_result.is_err() {
                                println!("WRITE_RESULT: {write_result:?}");
                            }
                            // */

                            // FLUX BUILDER

                            let mut flux_query_builder = QueryBuilder::default();
                            //println!("\n@FLUX_B DEFAULT: {flux_query_builder:?}");

                            let result_fqb = flux_query_builder
                                .debug(false)
                                .bucket("reqwest_backup_ds_test")
                                .range_start("-24h")
                                //.range_stop("-12h")
                                .filter("_measurement", "temperature")
                                .filter("host", "spongebob")
                                .filter("SensorId", &s_record.ds_id)
                                .sort("_time", "true")
                                .limit("1")
                                //.group(true)
                                //.count(true)
                                //.count_column("_value")
                                .build()
                                ;

                            match result_fqb {
                                Ok(data) => {
                                    println!("\n@RESULT_FQB: {data}");

                                    let verify_result = verify(client,
                                                               config,
                                                               &updated_data,
                                                               &data);
                                    
                                    println!("VERIFY_RESULT:\n+ {verify_result:?}");
                                },

                                Err(why) => {
                                    
                                    //eprintln!("\n###ERROR FQB:\n+ {:?}\nREASON >>> {:?}",
                                    eprintln!("\n###ERROR FQB:\nREASON >>> {:?}",
                                              //result_fqb,
                                              why.as_str(),                        
                                    );                                             
                                },
                            }
                            
                            /* // VERIFY 
                            let flux_query = format!("from(bucket:\"{bucket}\") |> range(start:{range_start}) |> filter(fn:(r) => r._measurement == \"{measurement}\") |> filter(fn:(r) => r.SensorId == \"{ds_id}\") |> sort(columns: [\"_time\"], desc:true) |> limit(n:1)",
                                                     //bucket=&active_config.bucket,
                                                     bucket="reqwest_backup_ds_test",
                                                     ds_id=&s_record.ds_id,
                                                     measurement="temperature",
                                                     range_start=&config.template.flux.query_verify_record_range_start,
                                                     
                            );

                            let verify_result = verify(client,
                                                       config,
                                                       &updated_data,
                                                       &flux_query);

                            println!("VERIFY_RESULT:\n+ {verify_result:?}");
                            */
                            
                            /*
                            if verify_result.is_err() {
                                println!("VERIFY_RESULT: {verify_result:?}");
                            }
                            */
                            //
                            
                        },
                        Err(why) => {

                            eprintln!("\n###ERROR RECORD:\n+ {:?}\n+ {:?}\nREASON >>> {:?}",
                                      s_record,
                                      line_protocol_builder,
                                      why.as_str(),                        
                            );                                             
                        },
                    }
                    
                // FUTURE_USE -> record HASH
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
                eprintln!("ERROR: record\nREASON >>>: {:?}", why);

            },
        };

        /*
        println!("\n break");
        break
        */
    }

    Ok(())
}


/// START
pub fn start(config: TomlConfig) -> Result<(), reqwest::Error> {
    // DUMMY measurement + influx instance
    const MEASUREMENT: &str = "dallas";
    const WRITE_BUCKET: &str = "reqwest_backup_ds_test";
    let active_config = &config.all_influx.values[1];

    
    // init InfluxConfig
    let influx_config = InfluxConfig::new(
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
    
    // WRITE
    let uri_write = influxdb_client::format::uri_write(
        &format!("{}{}",
                 &config.template.curl.influx_uri_api,
                 &config.template.curl.influx_uri_write),
        
        &influx_config,
        
        config.flag.debug_tuple_formater,
    );

    // READ
    let uri_query = influxdb_client::format::uri_query(
        &format!("{}{}",
                 &config.template.curl.influx_uri_api,
                 &config.template.curl.influx_uri_query),
        
        &influx_config,
        
        config.flag.debug_tuple_formater,
    );
    
    // FLUX
    let flux_query = format!("from(bucket:\"{bucket}\") |> range(start:{range_start}) |> filter(fn:(r) => r._measurement == \"{measurement}\") |> sort(columns: [\"_time\"], desc:true) |> limit(n:1)",
                             bucket=&active_config.bucket,
                             measurement=MEASUREMENT,
                             range_start=&config.template.flux.query_verify_record_range_start,
                             
    );

    // TOKEN
    let token = influxdb_client::format::token(
        &config.template.curl.influx_auth[1],

        &influx_config,
        
        config.flag.debug_tuple_formater,
    );

    // CALL
    let mut influx_call = InfluxCall::new(
        uri_write,
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

    // REQW Client
    let client: reqwest::blocking::Client = influxdb_client::client::client()?;
    
    // REQW READ RequestBuilder
    let request_read: Result<RequestBuilder, Box< dyn std::error::Error>>
        = influxdb_client::connect::read_flux_query(
            &client,
            &influx_call,
            flux_query,
            config.flag.debug_flux_query,
        );

    if config.flag.debug_reqwest {
        println!("\nREQUEST: {request_read:?}");
    }

    // WE HAVE flux query DATA
    let response = request_read
        .unwrap()
        .send()? // reqwest::Error
        .text()?; // -> String

    if config.flag.debug_reqwest {
        let response_len = response
            .split("\r\n,") // ',' not to catch last line \r\n\r\n
            .collect::<Vec<_>>()
            .len() - 1; // -HEADER
        
        println!("\nRESPONSE[{len}]: {response:#?}",
                 len = response_len,
        );
    }

    // UPDATE InfluxCall for WRITE
    let updated_call = influx_call
        .update_key(
            "bucket",
            "backup_ds_test",
            //"reqwest_backup_ds_test",
            WRITE_BUCKET,
        );
    
    println!("\n@UPDATE_CALL -> WRITE:\n+ {:?}",
             updated_call.uri_write,
    );    
    
    
    //CSV
    let csv_status = parse_csv(&client,
                               &config,
                               &influx_config,
                               updated_call,
                               &response,
    );

    if csv_status.is_err() {
        println!("CSV_STATUS: {csv_status:?}");
    }

    Ok(())
}


/// WRITE
pub fn write(client: &Client,
             config: &TomlConfig,
             influx_data: &InfluxData) -> Result<(), reqwest::Error> {

    // REQW WRITE RequestBuilder
    let request_write: Result<RequestBuilder, Box< dyn std::error::Error>>
        = influxdb_client::connect::write_lp(
            &client,
            &influx_data.call,
            String::from(&influx_data.lp),
            config.flag.debug_reqwest,
        );

    if config.flag.debug_reqwest {
        println!("\nREQUEST_WRITE: {request_write:?}");
    }

    let response = request_write
        .unwrap()
        .send()? // reqwest::Error
        .text()?; // -> String

    if config.flag.debug_reqwest {
        println!("\nRESPONSE: {response:#?}");
    }
    
    Ok(())
}


/// VERIFY
pub fn verify(client: &Client,
              config: &TomlConfig,
              influx_data: &InfluxData,
              flux_query: &str) -> Result<String, reqwest::Error> {


    println!("\nVERIFY_flux_Q:\n+ {flux_query:?}");
    
    // REQW READ RequestBuilder
    let request_read: Result<RequestBuilder, Box< dyn std::error::Error>>
        = influxdb_client::connect::read_flux_query(
            &client,
            &influx_data.call,
            String::from(flux_query),
            config.flag.debug_flux_query,
        );

    if config.flag.debug_reqwest {
        println!("\nREQUEST: {request_read:?}");
    }

    // WE HAVE flux query DATA
    let response = request_read
        .unwrap()
        .send()? // reqwest::Error
        .text()?; // -> String

    if config.flag.debug_reqwest {
        let response_len = response
            .split("\r\n,") // ',' not to catch last line \r\n\r\n
            .collect::<Vec<_>>()
            .len() - 1; // -HEADER
        
        println!("\nRESPONSE[{len}]: {response:#?}",
                 len = response_len,
        );
    }
    
    //Ok(())
    Ok(response)
}
