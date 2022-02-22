use template_formater::{
    tuple_formater,
    tuple_formater_safe,
};

mod options {
    pub const TEMPLATE_URI: &str = "{secure}://{server}:{port}{api}";
}
    
/// initial config properties
///
#[derive(Debug, Clone)]
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


impl ToString for InfluxConfig<'_> {
    fn to_string(&self) -> String {
        format!("{}://{}:{}",
                &self.secure,
                &self.server,
                &self.port,
        )
    }
}


pub trait ToUri {
    fn to_uri(&self,
              //template: &str,
              api: &str,
              debug: bool) -> String;
}

impl ToUri for InfluxConfig<'_> {
    fn to_uri(&self,
                 //template: &str,
                 api: &str,
                 debug: bool) -> String {

        // this one is not SAFE
        tuple_formater(
            //template,
            options::TEMPLATE_URI,
            &vec![
                ("secure", &self.secure),
                ("server", &self.server),    
                ("port", &self.port.to_string()),
                ("api", api),  
            ],
            debug,
        )
        
        /*
        format!("{}://{}:{}",
                &self.secure,
                &self.server,
                &self.port,
        )
        */
    }
}

// FUTURE USE
pub trait ToUriWrite {
    fn to_uri_write(&self,
                    template: &str,
                    debug: bool) -> Result<String, strfmt::FmtError>;
}

impl ToUriWrite for InfluxConfig<'_> {
    fn to_uri_write(&self,
                    template: &str,
                    debug: bool) -> Result<String, strfmt::FmtError> {

        tuple_formater_safe(
            template,
            &vec![
                ("org", &self.org),
                ("bucket", &self.bucket),    
                ("precision", &self.precision),  
            ],
            debug,
        )
        
        /*
        format!("{}api/v2/write?org={}bucket={}precision={}",
                &self.to_string(),
                &self.org,
                &self.bucket,
                &self.precision,
        )
        */
    }
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
        
        Self {
            name,
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

    pub fn default() -> Self {
        Self {
            name: "NAME",
            status: false,
            secure: "http", // https
            server: "localhost",
            port: 8086,
            bucket: "BUCKET",
            token: "TOKEN",
            org: "ORG",
            precision: "ms", // len()=13 -> 1645110902036
            machine_id: "MACHINE_ID",
            carrier: "CARRIER",
            flag_valid_default: false,
        }
    }
}
