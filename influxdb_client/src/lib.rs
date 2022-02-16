use reqwest::blocking::{
    //Client,
    ClientBuilder,
    Response,
    RequestBuilder,
};

use std::error::Error;


/// POST
pub fn post_query(uri: &str,
                  //data: &'static str, // until template {bucket}...
                  data: String,
                  token: &str) -> Result<RequestBuilder, Box<dyn Error + 'static>> {
    
    println!("post: {uri:?}");

    /*
    let der = std::fs::read("my-cert.der")?;
    let cert = reqwest::Certificate::from_der(&der)?;
    .add_root_certificate(cert)
    */

    //let client = ClientBuilder::new() // -> Client
    let client = ClientBuilder::new() // -> ClientBuilder
        .danger_accept_invalid_certs(true) // HTTPS with no certificate
        .build()?;
    
    let request = client.post(uri) // -> RequestBuilder
        .header("Authorization",
                &format!("Token {token}"),
        )
        .header("Accept",
                "application/csv"
        )
        .header("Content-type",
                "application/vnd.flux"
        )
        .timeout(std::time::Duration::from_secs(10)) // .connect_timeout()
        .body(data); // -> RequestBuilder
        
    Ok(request)
}


/// GET
pub fn get(uri: &str) -> Result<Response, Box<dyn Error>> {

    println!("get: {uri:?}");

    let resp = reqwest::blocking::get(uri)?;

    println!("response: {:#?}", resp);

    Ok(resp)
}
