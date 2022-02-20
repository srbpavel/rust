///
/// https://docs.influxdata.com/flux/v0.x/spec/data-model/#match-parameter-names
///

use template_formater::tuple_formater;


const DEFAULT_EMPTY: &str = "";
pub const DEFAULT_COUNT: &str = " |> count()";


/// flux_query error
#[derive(Debug)]
pub enum FluxQueryError {
    EmptyBucket,
    EmptyRangeStart,
    EmptyFilter,
}


/// flux_query error -> msg
impl FluxQueryError {
    pub fn as_str(&self) -> &str {
        match *self {
            FluxQueryError::EmptyBucket => "EMPTY: bucket, use bucket or bucket_id",
            FluxQueryError::EmptyRangeStart => "EMPTY: range_start",
            FluxQueryError::EmptyFilter => "EMPTY: filter", // WE ALWAYS FILTER
         }
    }
}


type QBStr = String;

/// flux query struct
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    pub debug: bool,
    pub bucket: QBStr,
    pub bucket_id: QBStr,
    pub range_start: QBStr,
    pub range_stop: QBStr,
    pub filter: QBStr,
    pub sort: QBStr,
    pub limit: QBStr,
    pub drop: QBStr,
    pub keep: QBStr,
    pub group: bool,
    pub count: bool,
    pub count_column: QBStr,
    
}

/// query builder + validation + template formating from variables
impl QueryBuilder {

    /// new
    pub fn new(debug: bool,
               bucket: QBStr,
               bucket_id: QBStr,
               range_start: QBStr,
               range_stop: QBStr,
               filter: QBStr,
               sort: QBStr,
               drop: QBStr,
               keep: QBStr,
               limit: QBStr,
               group: bool,
               count: bool,
               count_column: QBStr) -> Self {
        
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
            bucket: QBStr::from(DEFAULT_EMPTY),
            bucket_id: QBStr::from(DEFAULT_EMPTY),
            range_start: QBStr::from(DEFAULT_EMPTY),
            range_stop: QBStr::from(DEFAULT_EMPTY),
            filter: QBStr::from(DEFAULT_EMPTY),
            sort: QBStr::from(DEFAULT_EMPTY),
            keep: QBStr::from(DEFAULT_EMPTY),
            drop: QBStr::from(DEFAULT_EMPTY),
            limit: QBStr::from(DEFAULT_EMPTY),
            group: false,
            count: false,
            count_column: QBStr::from(DEFAULT_COUNT),
        }
    }

    /// debug tuple_formater pairs + final build
    ///
    pub fn debug(&mut self,
                 value: bool) -> &mut Self {
        
        self.debug = value;

        self
    }

    /// count results
    ///
    /// without group() -> return verbose result
    ///
    /// Ok(",result,table,_start,_stop,Machine,SensorCarrier,SensorId,SensorValid,_field,_measurement,host,_value\r\n,_result,0,2022-02-19T20:42:55Z,2022-02-20T08:42:55.238989405Z,spongebob,cargo,1052176647976,true,TemperatureDecimal,temperature,spongebob,20\r\n\r\n") <- here we have count=20 for SensorId=1052176647976
    ///
    /// with group() -> just result count
    ///
    /// Ok(",result,table,_value\r\n,_result,0,20\r\n\r\n")
    ///
    /// count()
    ///
    pub fn count(&mut self,
                 value: bool) -> &mut Self {
        
        self.count = value;

        self
    }

    /// enable/disable group results
    ///
    /// use together with count() -> return just count result as _value
    ///
    /// group()
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
    /// using just bucket in from() function
    ///
    /// from(bucket:"bucket")
    ///
    pub fn bucket(&mut self,
                  value: &str) -> &mut Self {

        self.bucket = format!("from(bucket:\"{}\")",
                              value.trim(),
        );
        
        /*
        self.bucket = String::from(
            tuple_formater("from(bucket:\"{bucket}\")",
                           &vec![
                               ("bucket", value.trim()),
                           ],
                           self.debug,
            )
        );
        */

        self
    }

    /// bucket ID
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/influxdata/influxdb/from/
    ///
    /// just bucketID in from() function
    ///
    /// ID: 66f7f3f74b11c188
    ///
    /// from(bucketID:"bucket_id")
    ///
    pub fn bucket_id(&mut self,
                     value: &str) -> &mut Self {

        self.bucket = format!("from(bucketID:\"{}\")",
                              value.trim(),
        );

        /*
        self.bucket_id = String::from(
            tuple_formater("from(bucketID:\"{bucket}\")",
                           &vec![
                               ("bucket", value.trim()),
                           ],
                           self.debug,
            )
        );
        */

        self
    }

    /// range_start
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/universe/range/
    ///
    /// relative: -60m, -12h, -1d, -1y 
    ///
    /// exact: 2022-02-19T09:00:00Z
    /// &format!("{}", (chrono::Utc::now() - chrono::Duration::hours(12)).to_rfc3339())
    ///
    /// timestamp: 1645302731 !!! in seconds!!!
    /// &format!("{}", (chrono::Utc::now() - chrono::Duration::hours(12)).timestamp()))
    ///
    /// this has nothing to do with your data, even if stored in precision ms,ns
    ///
    /// https://docs.rs/chrono/latest/chrono/struct.Duration.html
    ///
    /// range(start:-12h)
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
    /// same as for range_start()
    ///
    /// range(start:-12h, stop: now())
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
    /// filter(fn:(r) => r._measurement=="dallas")
    /// filter(fn:(r) => r.host=="ruth")
    ///
    /// filter(fn:(r) => r._time==2022-02-20T16:40:07.708Z)
    /// for TAG _time the value is without double qoutes
    ///
    /// filter(fn:(r) => r._value <= 18
    /// filter(fn:(r) => r._value >= 20.0
    ///
    ///
    /// .filter("_time",                                                      
    ///    ">",                                                          
    ///    &format!("{}",                                                
    ///             (chrono::Utc::now() - chrono::Duration::minutes(15)) 
    ///             .to_rfc3339()                                        
    ///    ),                                                            
    /// )
    ///
    /// filter(fn:(r) => r._time > 2022-02-20T17:12:07.210666375+00:00)
    ///
    pub fn filter(&mut self,
                  key: &str,
                  comparator: &str,
                  value: &str) -> &mut Self {

        self.filter += &format!(
            " |> filter(fn:(r) => r.{}",
            &tuple_formater(
                if !key.trim().eq("_time") && comparator.trim().eq("==") {
                    "{key}{comparator}\"{value}\")"
                } else {
                    "{key} {comparator} {value})"
                },
                &vec![
                    ("key", key.trim()),
                    ("comparator", comparator.trim()),
                    ("value", value.trim()),
                ],
                self.debug,
            )
        );

        self
    }

    /*
    /// filter threshold
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/universe/filter/
    ///
    /// filter(fn: (r) => r._value > 0 and r._value < 10 )
    /// 
    /// key, comp_1, val_1, comp_2, val_2
    ///
    pub fn filter_threshold(&mut self) -> &mut Self {

        // FUTURE USE

        self
    }
    */

    /// sort
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/universe/sort/
    ///
    /// sort(columns: ["_time"], desc:true)
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
    /// drop(columns: ["_start", "_stop"])
    ///
    pub fn drop(&mut self,
                columns: Vec<&str>) -> &mut Self {

        self.drop = format!(" |> drop(columns: {columns:?})");
        
        self
    }
    
    /// keep
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/universe/keep/
    ///
    /// keep(columns: ["_time"])
    ///
    pub fn keep(&mut self,
                columns: Vec<&str>) -> &mut Self {

        self.keep = format!(" |> keep(columns: {columns:?})");
        
        self
    }
    
    /// limit
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/universe/limit/
    ///
    /// limit(n: 10)
    ///
    pub fn limit(&mut self,
                 value: &str) -> &mut Self {

        self.limit = format!(" |> limit(n:{})",
                             value.trim(),
        );
        
        self
    }

    /// count column
    ///
    /// https://docs.influxdata.com/flux/v0.x/stdlib/universe/count/
    ///
    /// count(column: "_value")
    ///
    pub fn count_column(&mut self,
                        value: &str) -> &mut Self {

        // as we set column, count will be activated
        self.count = true;
        
        self.count_column = format!(" |> count(column: \"{}\")",
                                    value.trim(),
        );
        
        self
    }

    /// finalize construction from all members
    ///
    /// ok if valid otherwise raise error
    ///
    pub fn build(&mut self) -> Result<String, FluxQueryError> {

        // VALIDATION just for EMPTY at the moment
        if self.bucket.eq(DEFAULT_EMPTY) && self.bucket_id.eq(DEFAULT_EMPTY) {
            return Err(FluxQueryError::EmptyBucket)
        }

        if self.range_start.eq(DEFAULT_EMPTY) {
            return Err(FluxQueryError::EmptyRangeStart)
        }

        if self.filter.eq(DEFAULT_EMPTY) {
            return Err(FluxQueryError::EmptyFilter)
        }

        // RANGE start or start+stop
        let range = if self.range_stop.eq("") {
            format!(" |> range(start:{})", &self.range_start)
        } else {
            format!(" |> range(start:{}, stop:{})",
                    &self.range_start,
                    &self.range_stop,
                    )
        };

        // JOIN
        //
        // now vec+iter / change to format and verify
        //
        let mut flux_query = vec![
            // BUCKET
            if self.bucket.eq(DEFAULT_EMPTY) {
                &self.bucket_id
            } else {
                &self.bucket
            },

            // DATE
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

        // add GROUP
        if self.group {
            flux_query = format!("{}{}",
                          flux_query,
                          " |> group()",
            );
        }

        // add COUNT
        if self.count {
            flux_query = format!("{}{}",
                          flux_query,
                          self.count_column,
            );
        }

        Ok(flux_query)
    }
}
