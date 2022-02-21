use crate::influxdb_toml_config_struct::{TomlConfig};

use influxdb_client::{
    config::InfluxConfig,
    call::InfluxCall,
    data::InfluxData,
    lp::LineProtocolBuilder,
    flux_query::QueryBuilder,

    format::{uri_write,
             uri_query,
             token,
    }
};

use reqwest::blocking::{
    RequestBuilder,
};

use serde::Deserialize;

use chrono::{DateTime,
             Utc,
};


/// this depends on flux query result
/// we start with &str,bool,u64 and parse datetime later
/// very nice tutorial rust csv explained https://blog.burntsushi.net/csv/
///
///",result,table,_start,_stop,_time,_value,DsCarrier,DsId,DsPin,DsValid,Machine,_field,_measurement,host\r\n,_result,0,2022-02-16T08:45:43.372462165Z,2022-02-16T20:45:43.372462165Z,2022-02-16T20:45:09.299Z,20.5625,labjack,1052176647976,14,true,mrazak,DsDecimal,dallas,ruth\r\n
///
/// Record { annotation: "", result: "_result", table: 4, _start: "2022-02-20T05:32:49.735230174Z", _stop: "2022-02-20T17:32:49.735230174Z", _time: "2022-02-20T17:30:04.474Z", _value: "19.1875", Machine: "mrazak", DsCarrier: "labjack", DsId: "96928329000", DsPin: "14", DsValid: true, _field: "DsDecimal", _measurement: "dallas", host: "ruth" }
///
/// #[serde(rename = "..." )] Record { annotation: "", result: "_result", table: 0, start: "2022-02-16T08:45:43.372462165Z", stop: "2022-02-16T20:45:43.372462165Z", time: "2022-02-16T20:45:09.299Z", value: "20.5625", machine: "mrazak", ds_carrier: "labjack", ds_id: "1052176647976", ds_pin: "14", ds_valid: true, field: "DsDecimal", measurement: "dallas", host: "ruth" }
///
#[allow(dead_code)]
#[allow(non_snake_case)]
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
    //#[serde(rename = "_start")]
    //start: &'h str,
    _start: &'h str,
    //#[serde(rename = "_stop")]
    //stop: &'h str,
    _stop: &'h str,
    //#[serde(rename = "_time")]
    //time: &'h str,
    _time: &'h str,

    //#[serde(rename = "_value")]
    //value: &'h str,
    _value: &'h str,
    //#[serde(rename = "Machine")]
    //machine: &'h str,
    Machine: &'h str,
    //#[serde(rename = "DsCarrier")]
    //ds_carrier: &'h str,
    DsCarrier: &'h str,
    //#[serde(rename = "DsId")]
    //ds_id: &'h str,
    DsId: &'h str,
    //#[serde(rename = "DsPin")]
    //ds_pin: &'h str,
    DsPin: &'h str,
    //#[serde(rename = "DsValid")]
    //ds_valid: bool,
    DsValid: bool,
    //#[serde(rename = "_field")]
    //field: &'h str,
    _field: &'h str,
    //#[serde(rename = "_measurement")]
    //measurement: &'h str,
    _measurement: &'h str,
    host: &'h str,
}

/// &str -> DateTime instead Serde parsing
pub fn parse_datetime(datetime: &str) -> Result<DateTime<Utc>, chrono::format::ParseError> {
    
    match datetime.parse() {
        Ok(t) => return Ok(t),
        Err(why) => {
            Err(why)
        }
    }
}


/// all in one go, just to follow an example
pub fn start(config: TomlConfig) -> Result<(), Box<dyn std::error::Error>> {
    // influx instance to read data from
    let read_config = &config.all_influx.values[1];
    // metric variables as measurement, tag/field names
    let metric = &config.metrics["temperature"];
    // influx instance to write data to / we only use bucket from here
    let write_config = &config.all_influx.values[0];
    
    // populate InfluxConfig
    let influx_config = InfluxConfig::new(
        &read_config.name,
        read_config.status,
        &read_config.secure,
        &read_config.server,
        read_config.port,
        &read_config.bucket,
        &read_config.token,
        &read_config.org,
        &read_config.precision,
        &read_config.machine_id,
        &read_config.carrier,
        read_config.flag_valid_default,
    );

    if config.flag.debug_influx_instances {
        println!("\n@LIB InfluxConfig: {influx_config:#?}");
    }

    // WRITE_URI: 
    let uri_write = uri_write(
        &format!("{}{}",
                 &config.template.curl.influx_uri_api,
                 &config.template.curl.influx_uri_write),
        
        &influx_config,
        config.flag.debug_template_formater,
    );

    // READ_URI
    let uri_query = uri_query(
        &format!("{}{}",
                 &config.template.curl.influx_uri_api,
                 &config.template.curl.influx_uri_query),
        
        &influx_config,
        config.flag.debug_template_formater,
    );
    
    // TOKEN
    let token = token(
        &config.template.curl.influx_auth[1],
        &influx_config,
        config.flag.debug_template_formater,
    );

    // READ: FLUX_QUERY_BUILDER
    let mut flux_query_builder = QueryBuilder::default();

    let result_fqb = flux_query_builder
        .debug(false) // display tuple_format key/value pairs
        .bucket(influx_config.bucket)
        //.bucket_id("66f7f3f74b11c188")
        .range_start("-12h")
        //.range_start("2022-02-19T09:00:00Z")
        //.range_start(&format!("{}", (chrono::Utc::now() - chrono::Duration::hours(12)).to_rfc3339()))
        //.range_start("1645302362")
        //.range_start(&format!("{}", (chrono::Utc::now() - chrono::Duration::hours(12)).timestamp()))
        //.range_stop("-12h")
        //.range_stop("now()")
        .filter("_measurement", "==", &metric.measurement)
        //.filter("host", "==", "spongebob")
        //.filter(&metric.tag_id, "==", &s_record.ds_id)
        //.filter(&metric.tag_id, "==", &s_record.DsId)
        //.drop(vec!["_start", "_stop"])
        //.keep(vec!["_time"])
        //.keep(vec!["_time", &metric.tag_id])
        .sort(vec!["_time"], "true")
        .limit("1")
        //.group(true)
        //.count(true)
        //.count_column("_value")
        .build();

    let flux_query = match result_fqb {
        Ok(data) => {
            data
        },
        Err(why) => {
            eprintln!("\n###ERROR FQB:\nREASON >>> {:?}",
                      why.as_str(),
            );

            std::process::exit(1)
        },
    };
            
    // construct CALL for READ
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

    if config.flag.debug_influx_instances {
        println!("\n@{influx_call:#?}");
    }

    // CURL READ FLUX QUERY reqwest equivalent
    let curl_read = influx_call.curl_read(&config.template.curl.curl_read,
                                          &flux_query,
                                          config.flag.debug_template_formater,
    );
                                                          
    if config.flag.debug_influx_curl {
        println!("\n@CURL_READ_INPUT\n{}",
                 curl_read?,
        );
    }
    
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
        
        println!("\nRESPONSE[result count: {len}]: {response:#?}",
                 len = response_len,
        );
    }

    /* limit 1 result for each table, but we will receive multiple tables as per tag indexing

    ",result,table,_start,_stop,_time,_value,DsCarrier,DsId,DsPin,DsValid,Machine,_field,_measurement,host\r\n,

    _result,0,2022-02-20T05:45:14.02667451Z,2022-02-20T17:45:14.02667451Z,2022-02-20T17:45:09.547Z,19.6875,labjack,1052176647976,14,true,mrazak,DsDecimal,dallas,ruth\r\n,

    _result,1,2022-02-20T05:45:14.02667451Z,2022-02-20T17:45:14.02667451Z,2022-02-20T17:45:04.757Z,19.0625,labjack,236134354984,8,true,hrnecek_s_ledem,DsDecimal,dallas,ruth\r\n,

    _result,2,2022-02-20T05:45:14.02667451Z,2022-02-20T17:45:14.02667451Z,2022-02-20T17:45:05.628Z,21.1875,labjack,841704586024,8,true,hrnecek_s_ledem,DsDecimal,dallas,ruth\r\n,

    _result,3,2022-02-20T05:45:14.02667451Z,2022-02-20T17:45:14.02667451Z,2022-02-20T17:45:08.67Z,19.75,labjack,910462155048,14,true,mrazak,DsDecimal,dallas,ruth\r\n,

    _result,4,2022-02-20T05:45:14.02667451Z,2022-02-20T17:45:14.02667451Z,2022-02-20T17:45:07.796Z,19.375,labjack,96928329000,14,true,mrazak,DsDecimal,dallas,ruth\r\n\r\n"
    */

    // NOW WE CAN WRITE DATA TO another influx server or just different bucket or ...
    // UPDATE InfluxCall -> same SERVER but different BUCKET
    let updated_call = influx_call
        .update_key(
            "bucket",
            &read_config.bucket,
            &write_config.bucket,
        );

    if config.flag.debug_influx_instances {
        println!("\n@UPDATE_CALL -> we will WRITE data to BUCKET\n+ InfluxCall.uri_write: {:?}",
                 updated_call.uri_write,
        );
    }
    
    //parse response to CSV
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

    if config.flag.debug_metric_record {
        println!("\n#CSV_HEADER:\n+ {headers:?}");
    }

    // WALK through all CSV records
    let mut record_counter = 0;
    for single_record in reader.records() { // .records() -> iterator
        record_counter += 1;
        
        match single_record {
            Ok(rec) => { // StringRecord
                let s_record: Record = rec.deserialize(Some(headers))?;

                if config.flag.debug_metric_record {
                    println!("\n#CSV -> Struct: RECORD[{record_counter}]:\n+ {s_record:?}");
                }

                let mut line_protocol_builder = LineProtocolBuilder::default();

                if config.flag.debug_influx_lp {
                    println!("\n@LineProtocol_Builder DEFAULT: {line_protocol_builder:?}");
                }

                // LINE_PROTOCOL_BUILDER
                let result_lpb = line_protocol_builder
                    .template(&metric.generic_lp)
                    .measurement(&s_record._measurement)
                    .host(&s_record.host)
                    // TAG: KEY, VALUE
                    .tag(&headers[7], &s_record.DsCarrier)
                    .tag(&headers[8], &s_record.DsId)
                    .tag(&headers[9], &s_record.DsPin)
                    .tag(&headers[11], &s_record.Machine)
                    // RECORD is valid -> true
                    .tag(&headers[10], &format!("{}", true))
                    // FIELD: KEY, VALUE
                    .field(&s_record._field,
                           &s_record._value,
                    )
                    .ts(&format!("{}",
                                 match parse_datetime(s_record._time) {
                                     Ok(dt) => dt.timestamp_millis(),
                                     Err(why) => {
                                         return Err(Box::new(why))
                                     },
                                 }
                    ))
                    .build(config.flag.debug_template_formater);
                
                // VALIDATE LINE_PROTOCOL
                match result_lpb {
                    Ok(data) => {
                        let updated_data = InfluxData {
                            config: influx_config.clone(),
                            call: influx_call.clone(),
                            lp: data,
                        };

                        // CURL WRITE LP reqwest equivalent
                        let curl_write = updated_data.curl_write(&config.template.curl.curl_write,
                                                                 config.flag.debug_template_formater,
                        );
                                                          
                        if config.flag.debug_influx_curl {
                            println!("\n@CURL_WRITE\n{}",
                                     curl_write?,
                            );
                        }

                        // WRITE: REQW RequestBuilder
                        let request_write: Result<RequestBuilder, Box< dyn std::error::Error>>
                            = influxdb_client::connect::write_lp(
                                &client,
                                &updated_data.call,
                                String::from(&updated_data.lp),
                                config.flag.debug_reqwest,
                            );
                        
                        if config.flag.debug_reqwest {
                            println!("\n#REQUEST_WRITE:\n+ {request_write:?}");
                        }
                        
                        let response = request_write
                            .unwrap()
                            .send()? // reqwest::Error
                            .text()?; // -> String
                        
                        if config.flag.debug_reqwest {
                            println!("\n#RESPONSE WRITE:\n+ {response:#?}");
                        }

                        // OK, SO NOW DATA HAS BEEN WRITTEN to new place, let's verify it is there
                        // VERIFY FLUX QUERY BUILDER
                        let mut flux_query_builder = QueryBuilder::default();
                        //println!("\n@FLUX_B DEFAULT: {flux_query_builder:?}");
                        
                        let result_fqb = flux_query_builder
                            .debug(false) // display tuple_format pairs
                            .bucket(&write_config.bucket,)
                            .range_start("-12h")
                            .filter("_measurement", "==",&s_record._measurement)
                            .filter("host", "==",&s_record.host)
                            .filter(&headers[8], "==", &s_record.DsId)
                            .filter("_time", "==", &s_record._time)
                            /*
                            .filter("_time",
                                    ">",
                                    &format!("{}",
                                             (chrono::Utc::now() - chrono::Duration::minutes(15))
                                             .to_rfc3339()
                                    ),
                            )
                            */
                            
                            //.filter("_value", ">", "18")
                            //.filter("_value", "<=", "18")
                            //.filter("_value", ">=", "18")
                            .drop(vec!["_start", "_stop"])
                            //.keep(vec!["_time"])
                            //.keep(vec!["_time", &headers[8]])
                            //.sort(vec!["_time", &s_record.DsId], "true")
                            .sort(vec!["_time"], "true")
                            //.limit("1")
                            //.group(true) // + count -> return number
                            //.count(true) // - group -> result + _value: count
                            //.count_column("_value") // specify column
                            .build();
                        
                        match result_fqb {
                            Ok(flux_query) => {
                                //println!("\n@RESULT_FluxQueryBuilder for VERIFY: {flux_query}");

                                // CURL READ FLUX QUERY reqwest equivalent VERIFY
                                let curl_read_verify = influx_call.curl_read(&config.template.curl.curl_read,
                                                                             &flux_query,
                                                                             config.flag.debug_template_formater,
                                );
                                
                                if config.flag.debug_influx_curl {
                                    println!("\n@CURL_READ_VERIFY\n{}",
                                             curl_read_verify?,
                                    );
                                }
                                
                                // VERIFY: REQW READ RequestBuilder
                                let request_read: Result<RequestBuilder, Box< dyn std::error::Error>>
                                    = influxdb_client::connect::read_flux_query(
                                        &client,
                                        &updated_data.call,
                                        String::from(flux_query),
                                        config.flag.debug_flux_query,
                                    );
                                
                                if config.flag.debug_reqwest {
                                    println!("\nREQUEST VERIFY: {request_read:?}");
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
                                    
                                    println!("\nRESPONSE VERIFY[{len}] : {response:#?}",
                                             len = response_len,
                                    );
                                }
                                
                            },
                            Err(why) => {
                        
                                eprintln!("\n###ERROR FluxQueryBuilder VERIFY:\nREASON >>> {:?}",
                                          why.as_str(),                        
                                );                                             
                            },
                        }
                    },
                    Err(why) => {
                        
                        eprintln!("\n###ERROR record LineProtocolBuilder:\n+ {:?}\n+ {:?}\nREASON >>> {:?}",
                                  s_record,
                                  line_protocol_builder,
                                  why.as_str(),                        
                        );                                             
                    },
                }
            },
            Err(why) => {
                eprintln!("ERROR: record\nREASON >>>: {:?}", why);

            },
        };
        
        /* DEBUG just to see first record
        println!("\n break");
        break
        */
    }
    
    Ok(())
}
