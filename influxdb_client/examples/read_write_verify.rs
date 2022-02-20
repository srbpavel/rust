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
    Client,
    RequestBuilder,
};

use csv::StringRecord;

use serde::Deserialize;

use chrono::{DateTime,
             Utc,
};


// used for all operations read/write/verify
//const MEASUREMENT: &str = "dallas";
// this will be bucket to write data to and also verify
//const WRITE_BUCKET: &str = "reqwest_backup_ds_test";


/// START
pub fn start(config: TomlConfig) -> Result<(), reqwest::Error> {
    // CONFIG
    // influx instance to read data from
    let read_config = &config.all_influx.values[1];
    // metric variables as measurement, tag/field names, 
    let metric = &config.metrics["temperature"];
    // influx instance to write data to
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

    // FLUX_QUERY_BUILDER
    let mut flux_query_builder = QueryBuilder::default();
    
    let result_fqb = flux_query_builder
        .debug(false) // display tuple_format key/value pairs
        
        .bucket(influx_config.bucket)
        //.bucket_id("66f7f3f74b11c188")
        
        .range_start("-12h") // RELATIVE
        //.range_start("2022-02-19T09:00:00Z") //EXACT
        //.range_start(&format!("{}", minus_12h.to_rfc3339()))
        //.range_start(&format!("{}", (chrono::Utc::now() - chrono::Duration::hours(12)).to_rfc3339()))
        //.range_start("1645302362") //TS
        //.range_start(&format!("{}", minus_12h_ts)) //TS
        //.range_start(&format!("{}", (chrono::Utc::now() - chrono::Duration::hours(12)).timestamp()))
        
        //.range_stop("-12h")
        //.range_stop("now()")
        
        .filter("_measurement", &metric.measurement)
        //.filter("host", "spongebob")
        //.filter(&metric.tag_id, &s_record.ds_id)
        //.filter(&metric.tag_id, &s_record.DsId)
        
        //.drop(vec!["_start", "_stop"])
        
        //.keep(vec!["_time"])
        //.keep(vec!["_time", &metric.tag_id])
        
        .sort("_time", "true")
        .limit("1")

        //.group(true) // + count -> return count result as _value

        //.count(true) // - group -> result + _value: count
        //.count_column("_value") // specify column
        
        .build();
    
    let flux_query = match result_fqb {
        Ok(data) => {
            //println!("\n@RESULT_FQB:\n+ {data}");

            data
        },
        Err(why) => {
            eprintln!("\n###ERROR FQB:\nREASON >>> {:?}",
                      why.as_str(),
            );

            std::process::exit(1)
        },
    };
            
    // construct CALL for read/write
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
    
    Ok(())
}
