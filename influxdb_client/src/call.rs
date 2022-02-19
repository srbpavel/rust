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

    /* OBSOLETE ?
    /// swap key bucket/org/precision
    ///
    /// fookin, i will need to return Self not String
    ///
    pub fn swap_key(&self,
                    key: &str,
                    old_value: &str,
                    new_value: &str) -> String {

        if key != "hostname" {
            self.uri_write.replace(
                &format!("{key}={old_value}"),
                
                &format!("{key}={new_value}"),
            )
        } else {
            self.uri_write.replace(
                &format!("//{old_value}"),
                
                &format!("//{new_value}"),
            )
        }
    }
    */
}
