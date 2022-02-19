use template_formater::tuple_formater;


/// empty str for default struct
/// durring validation used to compare if member was updated
pub const DEFAULT: &str = "";
/// delimiter between tag/field records
const DELIMITER: char = ','; // 44


/// line_protocol error
#[derive(Debug)]
pub enum LpError {
    TimeStamp,
    EmptyMeasurement,
    EmptyHost,
    EmptyTags,
    EmptyFields,
    EmptyTimeStamp,
}


/// line_protocol error -> msg
impl LpError {
    pub fn as_str(&self) -> &str {
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


/// line_protocol struct
#[derive(Debug, Clone)]
pub struct LineProtocolBuilder {
    pub template: String,
    pub measurement: String,
    pub host: String,
    pub tags: String,
    pub fields: String,
    pub ts: String,
}

/// line_protocol builder + validation + template formating from variables
impl LineProtocolBuilder {

    /// new
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

    /// default
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

    /// data validation
    /// DEFAULT values raise Error
    /// TS sec/ms/ns len verifaction
    pub fn validate(&self) -> Result<(), LpError> {

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
                              **s = String::from(&s[0..s.len() - 1])
                          }
                      }
            );
    }
    
    /// finalize construction from all members
    /// ok if valid otherwise raise error
    pub fn build(&mut self,
                 debug: bool) -> Result<String, LpError> {

        // VALIDATE that all was updated and not DEFAULT
        match self.validate() {
            Ok(_) => {

                // REMOVE trailing delimiter
                self.remove_last_comma();

                // fill LP template with data
                let tp = tuple_formater(&self.template,
                           
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
                );

                Ok(tp)

            },
            
            Err(why) => { Err(why) }
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

    // TRY TO HAVE ONE ONE fn -> learn GENERIC
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
