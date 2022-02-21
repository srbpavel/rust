use template_formater::tuple_formater_safe;

mod options {
    pub const SECURE: &str = "secure";
    pub const HOSTNAME: &str = "hostname";
    pub const ORG: &str = "org";
    pub const BUCKET: &str = "bucket";
    pub const PRECISION: &str = "precision";
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
                      pattern: &str,
                      old: &str,
                      new: &str) -> &mut Self {

        let pattern = pattern.trim();
        let old = old.trim();
        let new = new.trim();
        
        match pattern {
            options::HOSTNAME => {
                self.uri_write = self.uri_write.replace(
                    &format!("://{old}"),
                    &format!("://{new}"),
                );
            },
            options::SECURE => {
                self.uri_write = self.uri_write.replace(
                    &format!("{old}://"),
                    &format!("{new}://"),
                );
            },
            k @ (options::ORG | options::BUCKET | options::PRECISION) => {
                self.uri_write = self.uri_write.replace(
                    &format!("{k}={old}"),
                    &format!("{k}={new}"),
                );   
            },
            _ => {
                eprintln!("\n!!! ERROR: InfluxCall.update_key() -> pattern <{}> not found",
                          pattern,
                )
            }
        }

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
