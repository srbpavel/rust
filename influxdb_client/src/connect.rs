use reqwest::blocking::{
    ClientBuilder,
    Response,
    RequestBuilder,
};

use std::error::Error;

/// config properties
///
#[derive(Debug)]
pub struct InfluxConfig<'c> {
    pub name: &'c str,
    pub status: bool,

    pub secure: &'c str,

    pub server: &'c str,
    pub port: u16,

    pub bucket: &'c str,
    pub token: &'c str,
    pub org: &'c str,
    pub precision: &'c str,

    pub machine_id: &'c str,
    pub carrier: &'c str,
    pub flag_valid_default: bool,
}

impl <'c>InfluxConfig<'c> {
    pub fn new(name: &'c str,
               status: bool,

               secure: &'c str,

               server: &'c str,
               port: u16,
               
               bucket: &'c str,
               token: &'c str,
               org: &'c str,
               precision: &'c str,
               
               machine_id: &'c str,
               carrier: &'c str,
               flag_valid_default: bool) -> Self {
        
        Self {name,
              status,

              secure,
              
              server,
              port,
              
              bucket,
              token,
              org,
              precision,
              
              machine_id,
              carrier,
              flag_valid_default,
        }
    }
}


///
/// properties for API call
///
///InfluxCall {
/// uri_write: "SECURE://SERVER:PORT/api/v2/write?org=ORG&bucket=BUCKET&precision=PRECISION",
/// uri_query: "SECURE://SERVER:PORT/api/v2/query?org=ORG",
/// auth: [ "Authorization", "Token ..."],
/// accept: ["Accept", "application/csv"],
/// content: ["Content-type", "application/vnd.flux"],
///}
///
#[derive(Debug)]
pub struct InfluxCall<'i> {
    pub uri_write: &'i str,
    pub uri_query: &'i str,
    pub auth: Vec<&'i str>,
    pub accept: Vec<&'i str>,
    pub content: Vec<&'i str>,
}

impl <'i>InfluxCall<'i> {
    pub fn new(uri_write: &'i str,
               uri_query: &'i str,
               auth: Vec<&'i str>,
               accept: Vec<&'i str>,
               content: Vec<&'i str>) -> Self {
        
        Self {uri_write,
              uri_query,
              auth,
              accept,
              content,
        }
    }
}


/// POST flux_query
pub fn read_flux_query(influx: InfluxCall,
                       query: String,
                       debug: bool) -> Result<RequestBuilder, Box<dyn Error + 'static>> {

    if debug {
        println!("\n#READ_FLUX_QUERY: {query}");
    }

    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true) // HTTPS with no certificate
        .build()?;

    let request = client.post(influx.uri_query)
        .header(influx.auth[0],
                influx.auth[1]
        )
        .header(influx.accept[0],
                influx.accept[1],
        )
        .header(influx.content[0],
                influx.content[1],
        )
        .timeout(
            std::time::Duration::from_secs(
                10
            )
        )
        .body(query); // -> RequestBuilder
    
    Ok(request)
}


/*
/// GET
pub fn get(uri: &str) -> Result<Response, Box<dyn Error>> {

    println!("get: {uri:?}");

    let resp = reqwest::blocking::get(uri)?;

    println!("response: {:#?}", resp);

    Ok(resp)
}
*/
