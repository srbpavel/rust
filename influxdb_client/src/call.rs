use template_formater::tuple_formater_safe;

/*
const SECURE: &str = "secure";
const HOSTNAME: &str = "hostname";
const ORG: &str = "org";
const BUCKET: &str = "bucket";
const PRECISION: &str = "precision";
*/

mod options {
    pub const SECURE: &str = "secure";
    pub const HOSTNAME: &str = "hostname";
    pub const ORG: &str = "org";
    pub const BUCKET: &str = "bucket";
    pub const PRECISION: &str = "precision";
    
    /*
    pub static SECURE: &str = "secure";
    pub static HOSTNAME: &str = "hostname";
    pub static ORG: &str = "org";
    pub static BUCKET: &str = "bucket";
    pub static PRECISION: &str = "precision";
    */
}

///
/// settings for influxdb api call
///
///InfluxCall {
/// uri_write: "SECURE://SERVER:PORT/api/v2/write?org=ORG&bucket=BUCKET&precision=PRECISION",
/// uri_query: "SECURE://SERVER:PORT/api/v2/query?org=ORG",
/// auth: [ "Authorization", "Token ..."],
/// accept: ["Accept", "application/csv"],
/// content: ["Content-type", "application/vnd.flux"],
///}
///
#[derive(Debug, Clone)]
pub struct InfluxCall<'i> {
    pub uri_write: String,
    pub uri_query: &'i str,
    pub auth: Vec<&'i str>,
    pub accept: Vec<&'i str>,
    pub content: Vec<&'i str>,
}

impl <'i>InfluxCall<'i> {
    /// new
    pub fn new(uri_write: String,
               uri_query: &'i str,
               auth: Vec<&'i str>,
               accept: Vec<&'i str>,
               content: Vec<&'i str>) -> Self {
        
        Self {
            uri_write,
            uri_query,
            auth,
            accept,
            content,
        }
    }

    /// object update
    pub fn update_key(&mut self,
                      key: &str,
                      old_value: &str,
                      new_value: &str) -> &mut Self {

        // try with closure
        // maybe not good to make it smaller as bucket can be HORSES / Horses
        // 
        let pattern = key.trim();//.to_lowercase();
        let old = old_value.trim();//.to_lowercase();
        let new = new_value.trim();//.to_lowercase();
        
        //match pattern.as_str() {
        match pattern {
            // http://HOSTNAME:8086/
            //
            //"hostname" => {
            options::HOSTNAME => {
            //HOSTNAME => {
                self.uri_write = self.uri_write.replace(
                    &format!("://{old}"),
                    &format!("://{new}"),
                );
            },
            // HTTP://hostname:8086/
            // HTTPS://hostname:8086/
            //
            //"secure" => {
            options::SECURE => {
                self.uri_write = self.uri_write.replace(
                    &format!("{old}://"),
                    &format!("{new}://"),
                );
            },
            // /api/v2/write?org=ORG&bucket=BUCKET&precision=MS
            //
            // keys: org / bucket / precicion
            //
            //k @ "org" | k @ "bucket" | k @ "precision" => {
            //k @ ("org" | "bucket" | "precision") => {
            k @ (options::ORG | options::BUCKET | options::PRECISION) => {
                self.uri_write = self.uri_write.replace(
                    &format!("{k}={old}"),
                    &format!("{k}={new}"),
                );   
            },
            // not found
            _ => {
                eprintln!("\n!!! ERROR: InfluxCall.update_key() -> pattern <{}> not found",
                          key,
                )
            }
        }
        
        /*
        if !key.eq("hostname") && !key.eq("secure") {
            // /api/v2/write?org=ORG&bucket=BUCKET&precision=MS
            //
            // keys: org / bucket / precicion
            //
            self.uri_write = self.uri_write.replace(
                &format!("{key}={old_value}"),
                &format!("{key}={new_value}"),
            )
        } else {
            // http://HOSTNAME:8086/
            //
            self.uri_write = self.uri_write.replace(
                &format!("://{old_value}"),
                &format!("://{new_value}"),
            )
        };
        */

        self
    }

    /// display curl post read flux_query
    ///
    /// /usr/bin/curl --insecure --request POST "http://jozefina:8086/api/v2/query?org=foookin_paavel" --header "Authorization: Token jbD0MXwVzetW6r6TFSQ5xIAzSFxwl3rD8tJVvzWr_Ax7ZNBJH1A0LHu38PR8WFWEpy0SuDlYpMyjYBB52riFrA==" --header "Accept: application/csv" --header "Content-type: application/vnd.flux" --data-raw 'from(bucket:"backup_ds_test") |> range(start:-12h) |> filter(fn:(r) => r._measurement=="dallas") |> sort(columns: ["_time"], desc:true) |> limit(n:1)'
    ///
    pub fn curl_read(&self,
                     template: &str,
                     flux_query: &str,
                     debug: bool) -> Result<String, strfmt::FmtError> {

        tuple_formater_safe(
            template,
            &vec![
                ("url", &self.uri_query),
                ("auth", &self.auth.join(": ")),    
                ("accept", &self.accept.join(": ")),  
                ("content", &self.content.join(": ")),
                ("data", flux_query),        
            ],
            debug,
        )
    }
}
