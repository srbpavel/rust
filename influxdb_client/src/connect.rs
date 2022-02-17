use reqwest::blocking::{
    ClientBuilder,
    //Response, // GET
    RequestBuilder,
};

use std::error::Error;

/*
pub trait Print {
    fn print(&self);

    fn print_call(&self);

    fn print_config(&self);
}
*/


/// data to write
#[derive(Debug)]
pub struct InfluxData<'d> {
    pub config: InfluxConfig<'d>,
    pub call: InfluxCall<'d>,
    pub lp: String,
}

/*
impl Print for InfluxData {
    fn print(&self) {
        println!("\nTRAIT >>> {:?}", self);
    }

    fn print_call(&self) {
        self.call.print();
    }

    fn print_config(&self) {
        self.config.print();
    }
}
*/


impl <'d>InfluxData<'d> {
    pub fn new(config: InfluxConfig<'d>,
               call: InfluxCall<'d>,
               lp: String) -> Self {
        
        Self {
            config,
            call,
            lp,
        }
    }
    
    pub fn default() -> Self {
        Self {
            config: InfluxConfig { ..InfluxConfig::default()

            },

            call: InfluxCall {
                uri_write: "",
                uri_query: "",

                auth: vec![],
                accept: vec![],
                content: vec![],
            },
            
            lp: "".to_string(),
        }
    }

    /*
    pub fn import_lp<'a>(&self,
                         config: &TomlConfig) {
        
        import_lp_via_curl(config,
                           &self)
    }
    */
}


/// config properties
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
            
            secure: "http",
            
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


///
/// API call properties
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
        
        Self {
            uri_write,
            uri_query,
            auth,
            accept,
            content,
        }
    }
}


/// POST flux_query
pub fn read_flux_query(influx: &InfluxCall,
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


/// Record
///
/// TemplateSensors
///
pub fn prepare_generic_lp_format(_config: &InfluxConfig) {
                                 //generic_pre_record: &Record,
                                 //metric: &TemplateSensors) -> String {
    
    println!("\n@LP: ");
    
    /*
    tuple_formater(&metric.generic_lp,
                   &vec![
                       ("tag_machine", &metric.tag_machine),
                       ("tag_carrier", &metric.tag_carrier),
                       ("tag_valid", &metric.tag_valid),
                       ("tag_id", &metric.tag_id),
                       ("field", &metric.field),

                       ("measurement", &generic_pre_record.measurement),
                       ("host", &generic_pre_record.host),
                       ("machine_id", &generic_pre_record.machine),
                       
                       ("carrier", &generic_pre_record.carrier),
                       ("valid", &generic_pre_record.valid),
                       
                       ("id", &generic_pre_record.id),
                       ("value", &generic_pre_record.value.to_string()),
                       
                       ("ts", &generic_pre_record.ts.to_string()),
                   ],
                   config.flag.debug_template_formater
    )
    */
}


/*
pub fn prepare_generic_lp_format(config: &InfluxConfig,
                                 generic_pre_record: &Record,
                                 metric: &TemplateSensors)  -> String {

    tuple_formater(&metric.generic_lp,
                   &vec![
                       ("tag_machine", &metric.tag_machine),
                       ("tag_carrier", &metric.tag_carrier),
                       ("tag_valid", &metric.tag_valid),
                       ("tag_id", &metric.tag_id),
                       ("field", &metric.field),

                       ("measurement", &generic_pre_record.measurement),
                       ("host", &generic_pre_record.host),
                       ("machine_id", &generic_pre_record.machine),
                       
                       ("carrier", &generic_pre_record.carrier),
                       ("valid", &generic_pre_record.valid),
                       
                       ("id", &generic_pre_record.id),
                       ("value", &generic_pre_record.value.to_string()),
                       
                       ("ts", &generic_pre_record.ts.to_string()),
                   ],
                   config.flag.debug_template_formater
    )
}
*/
