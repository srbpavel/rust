use reqwest::blocking::{
    ClientBuilder,
    Response,
    RequestBuilder,
};

use std::error::Error;


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
/*
pub fn read_query(uri: &str,
                  data: String,
                  token: &str) -> Result<RequestBuilder, Box<dyn Error + 'static>> {
 */
pub fn read_query(call: InfluxCall,
                  data: String) -> Result<RequestBuilder, Box<dyn Error + 'static>> {

    println!("READ_QUERY: {:?}", call.uri_query);

    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true) // HTTPS with no certificate
        .build()?;

    let request = client.post(call.uri_query)
        .header(call.auth[0],
                call.auth[1]
        )
        .header(call.accept[0],
                call.accept[1],
        )
        .header(call.content[0],
                call.content[1],
        )
        .timeout(
            std::time::Duration::from_secs(
                10
            )
        )
        .body(data); // -> RequestBuilder
    
    /*
    let request = client.post(uri)
        .header("Authorization",
                &format!("Token {token}"),
        )
        .header("Accept",
                "application/csv"
        )
        .header("Content-type",
                "application/vnd.flux"
        )
        .timeout(
            std::time::Duration::from_secs(
                10
            )
        ) // OR .connect_timeout()
        .body(data); // -> RequestBuilder
    */
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
