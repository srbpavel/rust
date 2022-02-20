///
/// https://docs.influxdata.com/flux/v0.x/spec/data-model/#match-parameter-names
///

use template_formater::tuple_formater;


const DEFAULT_EMPTY: &str = "";
pub const DEFAULT_COUNT: &str = " |> count()";


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
            FQError::EmptyBucket => "EMPTY: bucket, use bucket or bucket_id",
            FQError::EmptyRangeStart => "EMPTY: range_start",
            FQError::EmptyFilter => "EMPTY: filter", // WE ALWAYS FILTER
         }
    }
}


/// flux query struct
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    pub debug: bool,
    pub bucket: String,
    pub bucket_id: String,
    pub range_start: String,
    pub range_stop: String,
    pub filter: String,
    pub sort: String,
    pub limit: String,
    pub drop: String,
    pub keep: String,
    pub group: bool,
    pub count: bool,
    pub count_column: String,
    
}

/// query builder + validation + template formating from variables
impl QueryBuilder {

    /// new
    pub fn new(debug: bool,
               bucket: String,
               bucket_id: String,
               range_start: String,
               range_stop: String,
               filter: String,
               sort: String,
               drop: String,
               keep: String,
               limit: String,
               group: bool,
               count: bool,
               count_column: String) -> Self {
        
        Self {
            debug,
            bucket,
            bucket_id,
            range_start,
            range_stop,
            filter,
            sort,
            drop,
            keep,
            limit,
            group,
            count,
            count_column,
        }
    }

    /// default
    ///
    /// 
    ///
    pub fn default() -> Self {
        Self {
            debug: true,
            bucket: String::from(DEFAULT_EMPTY),
            bucket_id: String::from(DEFAULT_EMPTY),
            range_start: String::from(DEFAULT_EMPTY),
            range_stop: String::from(DEFAULT_EMPTY),
            filter: String::from(DEFAULT_EMPTY),
            sort: String::from(DEFAULT_EMPTY),
            keep: String::from(DEFAULT_EMPTY),
            drop: String::from(DEFAULT_EMPTY),
            limit: String::from(DEFAULT_EMPTY),
            group: false,
            count: false,
            count_column: String::from(DEFAULT_COUNT),
        }
    }

    /// debug tuple_formater pairs + build
    ///
    /// 
    ///
    pub fn debug(&mut self,
                 value: bool) -> &mut Self {
        
        self.debug = value;

        self
    }

    /// enable/disable count results
    ///
    /// without group() -> return verbose result
    /// Ok(",result,table,_start,_stop,Machine,SensorCarrier,SensorId,SensorValid,_field,_measurement,host,_value\r\n,_result,0,2022-02-19T20:42:55Z,2022-02-20T08:42:55.238989405Z,spongebob,cargo,1052176647976,true,TemperatureDecimal,temperature,spongebob,20\r\n\r\n") <- here we have count=20 for SensorId=1052176647976
    ///
    /// with group() -> just result count
    /// Ok(",result,table,_value\r\n,_result,0,20\r\n\r\n")
    pub fn count(&mut self,
                 value: bool) -> &mut Self {
        
        self.count = value;

        self
    }

    /// enable/disable group results
    ///
    /// use together with count() -> return just count result as _value
    ///
    pub fn group(&mut self,
                 value: bool) -> &mut Self {
        
        self.group = value;

        self
    }
    
    /// bucket name
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/influxdata/influxdb/from/
    ///
    /// at moment using just bucket in from() function
    ///
    /// name: Horses or ID: 66f7f3f74b11c188
    ///
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

    /// bucket ID
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/influxdata/influxdb/from/
    ///
    /// at moment using just bucket in from() function
    ///
    /// ID: 66f7f3f74b11c188
    ///
    pub fn bucket_id(&mut self,
                     value: &str) -> &mut Self {

        self.bucket_id = String::from(
            tuple_formater("from(bucketID:\"{bucket}\")",
                           &vec![
                               ("bucket", value.trim()),
                           ],
                           self.debug,
            )
        );
        
        self
    }

    /// range_start
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/universe/range/
    ///
    /// RELATIVE: -60m, -12h, -1d, -1y 
    ///
    /// EXACT: 2022-02-19T09:00:00Z
    /// &format!("{}", (chrono::Utc::now() - chrono::Duration::hours(12)).to_rfc3339())
    ///
    /// TS: 1645302731 / need to be in SECONDS
    /// &format!("{}", (chrono::Utc::now() - chrono::Duration::hours(12)).timestamp()))
    ///
    /// https://docs.rs/chrono/latest/chrono/struct.Duration.html
    ///
    pub fn range_start(&mut self,
                       value: &str) -> &mut Self {

        self.range_start = String::from(value);
        
        self
    }

    /// range_end
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/universe/range/
    ///
    /// -12h, now()
    ///
    pub fn range_stop(&mut self,
                     value: &str) -> &mut Self {

        self.range_stop = String::from(value);
        
        self
    }

    /// filter
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/universe/filter/
    ///
    pub fn filter(&mut self,
                  key: &str,
                  value: &str) -> &mut Self {

        self.filter += &format!(
            " |> filter(fn:(r) => r.{}",

            &tuple_formater(
                // flux query wants time without double quotes
                if !key.trim().eq("_time") {
                    //" |> filter(fn:(r) => r.{key}==\"{value}\")"
                    "{key}==\"{value}\")"
                } else {
                    //" |> filter(fn:(r) => r.{key}=={value})"
                    "{key}=={value})"
                },
                &vec![
                    ("key", key.trim()),
                    ("value", value.trim()),
                ],
                self.debug,
            )
        );

        self
    }

    /// sort
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/universe/sort/
    ///
    pub fn sort(&mut self,
                columns: Vec<&str>,
                flag: &str) -> &mut Self {

        self.sort = format!(
            " |> sort(columns: {:?}, desc:{})",
            columns,
            flag.trim(),
        );
        
        self
    }

    /// drop
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/universe/drop/
    ///
    pub fn drop(&mut self,
                cols: Vec<&str>) -> &mut Self {

        self.drop = format!(
            " |> drop(columns: {:?})",
            cols,
        );
        
        self
    }
    
    /// keep
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/universe/keep/
    ///
    pub fn keep(&mut self,
                cols: Vec<&str>) -> &mut Self {

        self.keep = format!(
            " |> keep(columns: {:?})",
            cols,
        );
        
        self
    }
    
    /// limit
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/universe/limit/
    ///
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
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/universe/count/
    ///
    pub fn count_column(&mut self,
                        value: &str) -> &mut Self {

        self.count = true;
        
        self.count_column = format!(" |> count(column: \"{}\")",
                                    value.trim(),
        );
        
        self
    }

    /// finalize construction from all members
    /// ok if valid otherwise raise error
    pub fn build(&mut self) -> Result<String, FQError> {

        // VALIDATION just for EMPTY at the moment
        if self.bucket.eq(DEFAULT_EMPTY) && self.bucket_id.eq(DEFAULT_EMPTY) {
            return Err(FQError::EmptyBucket)
        }

        if self.range_start.eq(DEFAULT_EMPTY) {
            return Err(FQError::EmptyRangeStart)
        }

        if self.filter.eq(DEFAULT_EMPTY) {
            return Err(FQError::EmptyFilter)
        }

        // RANGE
        let range = if self.range_stop.eq("") {
            format!(" |> range(start:{})", &self.range_start)
                    
        } else {
            format!(" |> range(start: {}, stop: {})",
                    &self.range_start,
                    &self.range_stop,
                    )
        };

        // JOIN
        let mut fqb = vec![
            // BUCKET
            if self.bucket.eq(DEFAULT_EMPTY) {
                &self.bucket_id
            } else {
                &self.bucket
            },
            
            &range,
            
            &self.filter,
            &self.drop,
            &self.keep,
            &self.sort,
            &self.limit,
        ]
            .iter()
            .map(|v| v.as_str())
            .collect::<Vec<_>>()
            .concat();

        // GROUP
        if self.group {
            fqb = format!("{}{}",
                          fqb,
                          " |> group()",
            );
        }

        // COUNT
        if self.count {
            fqb = format!("{}{}",
                          fqb,
                          self.count_column,
            );
        }

        Ok(fqb)
    }
}
