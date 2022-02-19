use reqwest::blocking::{
    Client,
    ClientBuilder,
};


/// client init
pub fn client() -> Result<Client, reqwest::Error> {

    ClientBuilder::new()
        .danger_accept_invalid_certs(true) // HTTPS with no certificate
        .build()
}
