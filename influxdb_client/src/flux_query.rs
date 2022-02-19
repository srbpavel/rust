use template_formater::tuple_formater;

const FLUX_DELIMITER: &str = " |> ";
const DEFAULT_EMPTY: &str = "";
const DEFAULT_SORT: &str = "sort(columns: [\"_time\"], desc:true)";
const DEFAULT_LIMIT: &str = "limit(n:1)";


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

            //measurement,
            //host,

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

            //measurement: String::from(DEFAULT_EMPTY),
            //host: String::from(DEFAULT_EMPTY),

            sort: String::from(DEFAULT_SORT),
            limit: String::from(DEFAULT_LIMIT),
        }
    }

    /// bucket
    pub fn bucket(&mut self,
                  value: &str) -> &mut Self {

        self.bucket = String::from(
            tuple_formater("from(bucket:\"{bucket}\"",
                           &vec![
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
            tuple_formater("range(start:{range_start}) ",
                           &vec![
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

        self.filter += &format!("{}{FLUX_DELIMITER}",
                                tuple_formater(
                                    "filter(fn:(r) => r.{key} == \"{value}\")",
                                    &vec![
                                        ("key", key.trim()),
                                        ("value", value.trim()),
                                    ],
                                    self.debug,
                                ));
        
        /*
        self.filter += &String::from(
            tuple_formater(
                "filter(fn:(r) => r.{key} == \"{value}\")",
                &vec![
                    ("key", key.trim()),
                    ("value", value.trim()),
                ],
                self.debug,
            )
        );
        */
        
        self
    }
}
