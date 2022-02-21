//use template_formater::tuple_formater;
use template_formater::tuple_formater_safe;

///
/// API call settings
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

        if !key.eq("hostname") {
            self.uri_write = self.uri_write.replace(
                &format!("{key}={old_value}"),
                &format!("{key}={new_value}"),
            )
        } else {
            self.uri_write = self.uri_write.replace(
                &format!("//{old_value}"),
                &format!("//{new_value}"),
            )
        };

        self
    }

    /// display curl READ FLUX_QUERY
    ///
    /// /usr/bin/curl --insecure --request POST "http://jozefina:8086/api/v2/query?org=foookin_paavel" --header "Authorization: Token jbD0MXwVzetW6r6TFSQ5xIAzSFxwl3rD8tJVvzWr_Ax7ZNBJH1A0LHu38PR8WFWEpy0SuDlYpMyjYBB52riFrA==" --header "Accept: application/csv" --header "Content-type: application/vnd.flux" --data-raw 'from(bucket:"backup_ds_test") |> range(start:-12h) |> filter(fn:(r) => r._measurement=="dallas") |> sort(columns: ["_time"], desc:true) |> limit(n:1)'
    ///
    pub fn curl_read(&self,
                     template: &str,
                     flux_query: &str,
                     //debug: bool) -> Result<String, Box<dyn std::error::Error>> {
                     debug: bool) -> Result<String, strfmt::FmtError> {

        let curl_call = tuple_formater_safe(
            template,
            &vec![
                ("url", &self.uri_query),
                ("auth", &self.auth.join(": ")),    
                ("accept", &self.accept.join(": ")),  
                ("content", &self.content.join(": ")),
                ("data", flux_query),        
            ],
            debug,
        );

        //Ok(curl_call)
        curl_call
    }
}
