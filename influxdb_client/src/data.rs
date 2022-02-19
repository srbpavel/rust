use crate::config::InfluxConfig;
use crate::lp::DEFAULT;
use crate::call::InfluxCall;


/// data to write
#[derive(Debug)]
pub struct InfluxData<'d> {
    pub config: InfluxConfig<'d>,
    pub call: InfluxCall<'d>,
    pub lp: String,
}

impl <'d>InfluxData<'d> {
    /// new
    pub fn new(config: InfluxConfig<'d>,
               call: InfluxCall<'d>,
               lp: String) -> Self {
        
        Self {
            config,
            call,
            lp,
        }
    }

    /// default
    pub fn default() -> Self {
        Self {
            config: InfluxConfig { ..InfluxConfig::default()

            },

            call: InfluxCall {
                uri_write: String::from(DEFAULT),
                uri_query: DEFAULT,
                auth: vec![],
                accept: vec![],
                content: vec![],
            },
            
            lp: String::from(DEFAULT)
        }
    }
}
