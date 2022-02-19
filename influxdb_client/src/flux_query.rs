use template_formater::tuple_formater;

//const FLUX_DELIMITER: &str = "|>";
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
    pub range_stop: String,

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
               range_stop: String,

               filter: String,
               
               //measurement: String,
               //host: String,

               sort: String,
               limit: String) -> Self {
        
        Self {
            debug,
            bucket,
            range_start,
            range_stop,
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
            range_stop: String::from(DEFAULT_EMPTY),

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
            //tuple_formater("from(bucket:\"{bucket}\") {FD} ",
            tuple_formater("from(bucket:\"{bucket}\")",
                           &vec![
                               //("FD", FLUX_DELIMITER), 
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

        self.range_start = String::from(value);
        
        /*
        self.range_start = String::from(
            tuple_formater("range(start:{range_start}) {FD} ",
                           &vec![
                               ("FD", FLUX_DELIMITER), 
                               ("range_start", value.trim()),
                           ],
                           self.debug,
            )
        );
        */
        
        self
    }

    /// range_end
    pub fn range_stop(&mut self,
                     value: &str) -> &mut Self {

        self.range_stop = String::from(value);
        
        self
    }

    /// measurement
    pub fn filter(&mut self,
                  key: &str,
                  value: &str) -> &mut Self {

        self.filter += &tuple_formater(
            " |> filter(fn:(r) => r.{key} == \"{value}\")",
            &vec![
                ("key", key.trim()),
                ("value", value.trim()),
            ],
            self.debug,
        );
        
        /*
        self.filter += &format!("{} {FLUX_DELIMITER} ",
                                tuple_formater(
                                    "|> filter(fn:(r) => r.{key} == \"{value}\")",
                                    &vec![
                                        ("key", key.trim()),
                                        ("value", value.trim()),
                                    ],
                                    self.debug,
                                ));
        */
        self
    }

    /// sort
    pub fn sort(&mut self,
                key: &str,
                value: &str) -> &mut Self {
        
        self.sort = tuple_formater(
            " |> sort(columns: [\"{key}\"], desc:{value})",
            &vec![
                //("FD", FLUX_DELIMITER), 
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
            " |> limit(n:{value})",
            &vec![
                //("FD", FLUX_DELIMITER), 
                ("value", value.trim()),
            ],
            self.debug,
        );
        
        self
    }

    /// finalize construction from all members
    /// ok if valid otherwise raise error
    pub fn build(&mut self) -> Result<String, FQError> {

        let range = if self.range_stop.eq("") {
            format!(" |> range(start:{})", &self.range_start)
                    
        } else {
            format!(" |> range(start: {}, stop: {})",
                    &self.range_start,
                    &self.range_stop,
                    )
        };
        
        let fqb = vec![&self.bucket,
                       &range,
                       &self.filter,
                       &self.sort,
                       &self.limit,
        ]
            .into_iter()
            .map(|v| v.as_str())
            .collect::<Vec<_>>()
            .concat();
       
        /*
        //let fqb = tuple_formater("{bucket} {FD} {range_start} {FD} {filter} {sort} {FD} {limit}",
        let fqb = tuple_formater("{bucket} {range_start} {filter} {sort} {limit}",
                                 
                                 &vec![
                                     //("FD", FLUX_DELIMITER),
                                     ("bucket", &self.bucket),

                                     ("range_start", &self.range_start),
                                     //("range_stop", &self.range_stop),
                                     
                                     ("filter", &self.filter),
                                     ("sort", &self.sort),
                                     ("limit", &self.limit),
                                 ],
                                
                                 self.debug,
        );
        */

        Ok(fqb)
    }
}

