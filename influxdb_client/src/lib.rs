use reqwest::blocking::{
    ClientBuilder,
    Response,
    RequestBuilder,
};

use std::error::Error;


/// POST
pub fn post_query(uri: &str,
                  data: String,
                  token: &str) -> Result<RequestBuilder, Box<dyn Error + 'static>> {
    
    println!("post: {uri:?}");

    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true) // HTTPS with no certificate
        .build()?;
    
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
        
    Ok(request)
}


/// GET
pub fn get(uri: &str) -> Result<Response, Box<dyn Error>> {

    println!("get: {uri:?}");

    let resp = reqwest::blocking::get(uri)?;

    println!("response: {:#?}", resp);

    Ok(resp)
}
