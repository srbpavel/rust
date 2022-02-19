use template_formater::tuple_formater;

const FLUX_DELIMITER: &str = "|>";
const DEFAULT_EMPTY: &str = "";
//const DEFAULT_SORT: &str = "sort(columns: [\"_time\"], desc:true)";
//const DEFAULT_LIMIT: &str = "limit(n:10)";


/// flux_query error
#[derive(Debug)]
pub enum FQError {
    EmptyBucket,
    EmptyRangeStart,
    EmptyFilter,
}


/// flux_query error -> msg
impl FQError {
    pub fn as_str(&self) -> &str {
        match *self {
            FQError::EmptyBucket => "EMPTY: bucket",
            FQError::EmptyRangeStart => "EMPTY: range_start",
            FQError::EmptyFilter => "EMPTY: filter",
         }
    }
}


/// flux query struct
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    pub debug: bool,

    pub bucket: String,

    pub range_start: String,
    pub range_end: String,

    pub filter: String,
    
    //pub measurement: String,
    //pub host: String,

    pub sort: String,
    pub limit: String,
}

/// query builder + validation + template formating from variables
impl QueryBuilder {

    /// new
    pub fn new(debug: bool,

               bucket: String,

               range_start: String,
               range_end: String,

               filter: String,
               
               //measurement: String,
               //host: String,

               sort: String,
               limit: String) -> Self {
        
        Self {
            debug,
            bucket,
            range_start,
            range_end,
            filter,
            sort,
            limit,
        }
    }

    /// default
    pub fn default() -> Self {
        Self {
            debug: true,
            
            bucket: String::from(DEFAULT_EMPTY),

            range_start: String::from(DEFAULT_EMPTY),
            range_end: String::from(DEFAULT_EMPTY),

            filter: String::from(DEFAULT_EMPTY),

            sort: String::from(DEFAULT_EMPTY),
            limit: String::from(DEFAULT_EMPTY),
            /*
            sort: String::from(DEFAULT_SORT),
            limit: String::from(DEFAULT_LIMIT),
            */
        }
    }

    /// debug
    pub fn debug(&mut self,
                 value: bool) -> &mut Self {
        
        self.debug = value;

        self
    }
    
    /// bucket
    pub fn bucket(&mut self,
                  value: &str) -> &mut Self {

        self.bucket = String::from(
            tuple_formater("from(bucket:\"{bucket}\") {FD} ",
                           &vec![
                               ("FD", FLUX_DELIMITER), 
                               ("bucket", value.trim()),
                           ],
                           self.debug,
            )
        );
        
        self
    }

    /// range_start
    pub fn range_start(&mut self,
                       value: &str) -> &mut Self {

        self.range_start = String::from(
            tuple_formater("range(start:{range_start}) {FD} ",
                           &vec![
                               ("FD", FLUX_DELIMITER), 
                               ("range_start", value.trim()),
                           ],
                           self.debug,
            )
        );
        
        self
    }

    /// measurement
    pub fn filter(&mut self,
                  key: &str,
                  value: &str) -> &mut Self {

        self.filter += &format!("{} {FLUX_DELIMITER} ",
                                tuple_formater(
                                    "filter(fn:(r) => r.{key} == \"{value}\")",
                                    &vec![
                                        ("key", key.trim()),
                                        ("value", value.trim()),
                                    ],
                                    self.debug,
                                ));
        self
    }

    /// sort
    pub fn sort(&mut self,
                key: &str,
                value: &str) -> &mut Self {
        
        self.sort = tuple_formater(
            "sort(columns: [\"{key}\"], desc:{value}) {FD} ",
            &vec![
                ("FD", FLUX_DELIMITER), 
                ("key", key.trim()),
                ("value", value.trim()),
            ],
            self.debug,
        );
        
        self
    }

    /// limit
    pub fn limit(&mut self,
                 value: &str) -> &mut Self {
        
        self.limit = tuple_formater(
            //"limit(n:{value}) {FD}",
            "limit(n:{value})",
            &vec![
                ("FD", FLUX_DELIMITER), 
                ("value", value.trim()),
            ],
            self.debug,
        );
        
        self
    }

    /// finalize construction from all members
    /// ok if valid otherwise raise error
    pub fn build(&mut self) -> Result<String, FQError> {

        //let fqb = tuple_formater("{bucket} {FD} {range_start} {FD} {filter} {sort} {FD} {limit}",
        let fqb = tuple_formater("{bucket} {range_start} {filter} {sort} {limit}",
                                 
                                 &vec![
                                     //("FD", FLUX_DELIMITER),
                                     ("bucket", &self.bucket),

                                     ("range_start", &self.range_start),
                                     //("range_end", &self.range_end),
                                     
                                     ("filter", &self.filter),
                                     ("sort", &self.sort),
                                     ("limit", &self.limit),
                                 ],
                                
                                 self.debug,
        );

        Ok(fqb)
        
        // REMOVE trailing delimiter -> FUTURE USE
        //Ok(remove_last_delimiter(fqb))
    }
}


/*
pub fn remove_last_delimiter(text: &str) -> String {
    /*
    [&mut self.tags, &mut self.fields]
        .iter_mut()
        
        .for_each(|s|
                  if let Some(last) = s.as_bytes().last() {
                      if last.eq(&(DELIMITER as u8)) {
                          **s = String::from(&s[0..s.len() - 1])
                      }
                  }
        );
    */
}
*/
