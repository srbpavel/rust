use reqwest::blocking::{
    ClientBuilder,
    RequestBuilder,
};

use std::error::Error;

use template_formater::tuple_formater;


const DEFAULT: &str = "";
const DELIMITER: char = ','; // 44

#[derive(Debug)]
pub enum LpError {
    TimeStamp,
    EmptyMeasurement,
    EmptyHost,
    EmptyTags,
    EmptyFields,
    EmptyTimeStamp,
}

impl LpError {
    fn as_str(&self) -> &str {
        match *self {
            LpError::TimeStamp => "WRONG timestamp format/len",
            LpError::EmptyMeasurement => "EMPTY: measurement",
            LpError::EmptyHost => "EMPTY: host",
            LpError::EmptyTags => "EMPTY: tags",
            LpError::EmptyFields => "EMPTY: fields",
            LpError::EmptyTimeStamp => "EMPTY: ts",
        }
    }
}

/// line_protocol_builder
#[derive(Debug, Clone)]
pub struct LineProtocolBuilder {
    pub template: String,
    pub measurement: String,
    pub host: String,
    pub tags: String,
    pub fields: String,
    pub ts: String,
}

impl LineProtocolBuilder {
    pub fn new(template: String,
               measurement: String,
               host: String,
               tags: String,
               fields: String,
               ts: String) -> Self {
        
        Self {
            template,
            measurement,
            host,
            tags,
            fields,
            ts,
        }
    }

    pub fn default() -> Self {
        Self {
            template: String::from(DEFAULT),
            measurement: String::from(DEFAULT),
            host: String::from(DEFAULT),
            tags: String::from(DEFAULT),
            fields: String::from(DEFAULT),
            ts: String::from(DEFAULT),
        }
    }

    /// DATA validation
    pub fn validate(&self) -> Result<(), LpError> {

        // DEFUALT not updated
        if self.measurement.eq(DEFAULT) {
            return Err(LpError::EmptyMeasurement)
        }

        if self.host.eq(DEFAULT) {
            return Err(LpError::EmptyHost)
        }
        
        if self.tags.eq(DEFAULT) {
            return Err(LpError::EmptyTags)
        }
        
        if self.fields.eq(DEFAULT) {
            return Err(LpError::EmptyFields)
        }
        if self.ts.eq(DEFAULT) {
            return Err(LpError::EmptyTimeStamp)
        }
        
        // WRONG timestamp len/format -> need config !!!
        // is correct to verify millis via len ?
        // VALIDATION WILL be performed before BUILD in future
        if format!("{}", self.ts).len() != 13 { //13MS 10SEC {
            return Err(LpError::TimeStamp)
        }
        
        Ok(())
    }

    /// remove last DELIMITER char in tags/fields values
    pub fn remove_last_comma(&mut self) {
        [&mut self.tags, &mut self.fields]
            .iter_mut()

            .for_each(|s|
                      if let Some(last) = s.as_bytes().last() {
                          if last.eq(&(DELIMITER as u8)) {
                              println!("LAST_OK: <{}>", s);

                              **s = String::from(&s[0..s.len() - 1])
                          }
                      } else {
                          println!("LAST_ERR: <{}>", s);
                      }
            );
            
            /*
            .for_each(|s| match s.as_bytes().last() {

                Some(last) => if last.eq(&(DELIMITER as u8)) {
                    **s = String::from(&s[0..s.len() - 1])
                },

                None => {}
            }
            );
            */
        
            /*
            .for_each(|s| { let last = s.as_bytes()
                            .last()
                            .unwrap(); // NOT SAFE
                            
                            if last.eq(&(DELIMITER as u8)) {
                                **s = String::from(&s[0..s.len() - 1])
                            }
            }
            );
            */
    }
    
    /// BUILD 
    pub fn build(&mut self,
                 debug: bool) -> String {

        // VALIDATE that all was updated and not DEFAULT
        match self.validate() {
            Ok(_) => {

                // REMOVE trailing delimiter
                self.remove_last_comma();

                // fill LP template with data
                tuple_formater(&self.template,
                           
                               &vec![
                                   ("measurement", &self.measurement),
                                   ("host", &self.host),
                                   
                                   ("tags",
                                    &self.tags,
                                   ),
                                   
                                   ("fields",
                                    &self.fields,
                                   ),
                                   
                                   ("ts", &self.ts),
                               ],
                               
                               debug,
                )

            },
            
            Err(why) => {

                eprintln!("\n###ERROR: {:#?}\nREASON >>> {:?}",
                          self,
                          why.as_str(),
                );
            
                String::from("")
            }
        }
    }

    /// template
    pub fn template(&mut self, value: &str) -> &mut Self {
        self.template = String::from(value.trim());

        self
    }

    /// measurement
    pub fn measurement(&mut self, value: &str) -> &mut Self {
        self.measurement = String::from(value.trim());

        self
    }

    /// host
    pub fn host(&mut self, value: &str) -> &mut Self {
        self.host = String::from(value.trim());

        self
    }

    // TRY TO HAVE ONE ONE fn
    /// update tag
    pub fn tag(&mut self,
                name: &str,
                value: &str) -> &mut Self {
        
        self.tags += &format!("{name}={value}{delimiter}",
                              name = name.trim(),
                              value = value.trim(),
                              delimiter = DELIMITER,
                              //delimiter = ';', // ERROR handle
        );

        self
    }

    /// update field
    pub fn field(&mut self,
                 name: &str,
                 value: &str) -> &mut Self {

        self.fields += &format!("{name}={value}{delimiter}",
                                name = name.trim(),
                                value = value.trim(),
                                delimiter = DELIMITER,
        );

        self
    }

    pub fn ts(&mut self, value: &str) -> &mut Self {
        self.ts = String::from(value.trim());
        
        self
    }
}


/// data to write
#[derive(Debug)]
pub struct InfluxData<'d> {
    pub config: InfluxConfig<'d>,
    pub call: InfluxCall<'d>,
    pub lp: String,
}


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
                /*
                uri_write: "",
                uri_query: "",
                */
                uri_write: DEFAULT,
                uri_query: DEFAULT,
                
                auth: vec![],
                accept: vec![],
                content: vec![],
            },
            
            //lp: "".to_string(),
            lp: String::from(DEFAULT)
        }
    }
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


/*
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
}
*/

