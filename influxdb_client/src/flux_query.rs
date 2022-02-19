use template_formater::tuple_formater;

//const FLUX_DELIMITER: &str = "|>";
const DEFAULT_EMPTY: &str = "";
const DEFAULT_COUNT: &str = " |> count()";

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
    pub sort: String,
    pub limit: String,
    pub group: bool,
    pub count: bool,
    pub count_column: String,
    
}

/// query builder + validation + template formating from variables
impl QueryBuilder {

    /// new
    pub fn new(debug: bool,
               bucket: String,
               range_start: String,
               range_stop: String,
               filter: String,
               sort: String,
               limit: String,
               group: bool,
               count: bool,
               count_column: String) -> Self {
        
        Self {
            debug,
            bucket,
            range_start,
            range_stop,
            filter,
            sort,
            limit,
            group,
            count,
            count_column,
        }
    }

    /// default
    pub fn default() -> Self {
        Self {
            debug: true,
            bucket: String::from(DEFAULT_EMPTY),
            range_start: String::from(DEFAULT_EMPTY),
            range_stop: String::from(DEFAULT_EMPTY), // FUTURE USE
            filter: String::from(DEFAULT_EMPTY),
            sort: String::from(DEFAULT_EMPTY),
            limit: String::from(DEFAULT_EMPTY),

            group: false,
            count: false,
            count_column: String::from(DEFAULT_COUNT),
        }
    }

    /// debug tuple_formater pairs + build
    pub fn debug(&mut self,
                 value: bool) -> &mut Self {
        
        self.debug = value;

        self
    }

    /// enable/disable count results
    pub fn count(&mut self,
                 value: bool) -> &mut Self {
        
        self.count = value;

        self
    }

    /// enable/disable group results
    pub fn group(&mut self,
                 value: bool) -> &mut Self {
        
        self.group = value;

        self
    }
    
    /// bucket
    pub fn bucket(&mut self,
                  value: &str) -> &mut Self {

        self.bucket = String::from(
            tuple_formater("from(bucket:\"{bucket}\")",
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

        self.range_start = String::from(value);
        
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

        self
    }

    /// sort
    pub fn sort(&mut self,
                key: &str,
                value: &str) -> &mut Self {
        
        self.sort = tuple_formater(
            " |> sort(columns: [\"{key}\"], desc:{value})",
            &vec![
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
            " |> limit(n:{value})",
            &vec![
                ("value", value.trim()),
            ],
            self.debug,
        );
        
        self
    }

    /// count
    pub fn count_column(&mut self,
                 value: &str) -> &mut Self {

        self.count_column = format!(" |> count(column: \"{}\")",
                                    value.trim(),
        );
        
        /*
        self.count_column = if !value.eq("") {
            format!(" |> count(column: \"{}\")",
                    value.trim(),
            )
        } else {
            String::from(" |> count()")
        };
        */

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
        
        let mut fqb = vec![&self.bucket,
                           &range,
                           &self.filter,
                           &self.sort,
                           &self.limit,
        ]
            .into_iter()
            .map(|v| v.as_str())
            .collect::<Vec<_>>()
            .concat();

        if self.group {
            println!("group: true");
            
            fqb = format!("{}{}",
                          fqb,
                          " |> group()",
            );
        } else { println!("group: false");  }
        
        if self.count {
            println!("count: true");
            
            fqb = format!("{}{}",
                          fqb,
                          self.count_column,
            );
        } else { println!("count: false");  }

        Ok(fqb)
    }
}
