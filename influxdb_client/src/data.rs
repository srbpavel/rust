///
/// all data needed to call influx
///

use crate::config::InfluxConfig;
use crate::call::InfluxCall;

use template_formater::tuple_formater;


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
    pub fn curl_write(&self,
                      template: &str) -> String {

        //println!("@CURL_WRITE_TEMPLATE:\n +{template}");

        tuple_formater(
            template,
            &vec![
                ("url", &self.call.uri_write),
                ("auth", &self.call.auth.join(": ")),    
                ("data", &self.lp),        
                /*
                ("program", &config.template.curl.program),
                ("param_insecure", &config.template.curl.param_insecure),
                ("param_request", &config.template.curl.param_request),
                ("param_post", &config.template.curl.param_post),  
                
                ("url", updated_data.call.uri_write),
                
                ("param_header", &config.template.curl.param_header),
                
                ("auth", updated_data.call.auth.join(": ")),    
                
                ("param_data", &config.template.curl.param_data),       
                ("data", updated_data.lp),        
                */
            ],
            //self.debug,
            true,
            )
        
        /*
        //String::from("curl")
        format!("{program}{uri_write}{auth:?}",
                program="ccc",
                uri_write=&self.call.uri_write,
                auth=&self.call.auth,
        )
        */
    }
}
