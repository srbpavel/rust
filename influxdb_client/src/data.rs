///
/// all data needed to call influx
///

use crate::config::InfluxConfig;
use crate::call::InfluxCall;


pub const DEFAULT: &str = "";


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

    /// display curl version of request
    ///
    /// $curl --cacert /home/conan/.ssh/ruth/influxdb-selfsigned.crt --request POST "https://ruth:8086/api/v2/write?org=foookin_paavel&bucket=bbb&precision=ms" --header "Authorization: Token ....." --data-raw "battery_adc,host=spongebob,BatSenKey=14,BatUlId=50x29196a980,BatCarrier=ttn,BatValid=true BatDecimal=7.64,BatKey=20909 1621759946147"
    ///
    pub fn curl(&self) -> String {
        println!("@CURL:\n +");

        //String::from("curl")
        format!("{program}{uri_write}{auth:?}",
                program="ccc",
                uri_write=&self.call.uri_write,
                auth=&self.call.auth,
        )
    }
}
