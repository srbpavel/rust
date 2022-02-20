///
/// API write
/// https://docs.influxdata.com/influxdb/v2.1/write-data/developer-tools/api/
/// API query
/// https://docs.influxdata.com/influxdb/v2.1/query-data/execute-queries/influx-api/
/// API CSV
/// https://docs.influxdata.com/influxdb/v2.1/reference/syntax/annotated-csv/
///
///

use crate::call::InfluxCall;

use reqwest::blocking::{
    Client,
    RequestBuilder,
};

use std::error::Error;


/// POST READ flux_query
pub fn read_flux_query(client: &Client,
                       influx: &InfluxCall,
                       query: String,
                       debug: bool) -> Result<RequestBuilder, Box<dyn Error + 'static>> {

    if debug {
        println!("\n@READ_FLUX_QUERY:\n+ {query}");
    }

    let request = client.post(influx.uri_query)
        // TOKEN
        .header(influx.auth[0],
                influx.auth[1]
        )
        .header(influx.accept[0],
                influx.accept[1],
        )
        .header(influx.content[0],
                influx.content[1],
        )
        // TIMEOUT -> FUTURE USE
        .timeout(
            std::time::Duration::from_secs(
                10
            )
        )
        // DATA
        .body(query); // -> RequestBuilder
    
    Ok(request)
}


/// POST WRITE LP
pub fn write_lp(client: &Client,
                influx: &InfluxCall,
                lp: String,
                debug: bool) -> Result<RequestBuilder, Box<dyn Error + 'static>> {
    
    if debug {
        println!("\n#WRITE_REQUEST:\n+ {influx:?}\n+ {lp:?}");
    }

    let request = client.post(&influx.uri_write)
        // TOKEN
        .header(influx.auth[0],
                influx.auth[1]
        )
        // TIMEOUT -> FUTURE USE
        .timeout(
            std::time::Duration::from_secs(
                10
            )
        )
        // DATA
        .body(lp); // -> RequestBuilder

    Ok(request)
}
