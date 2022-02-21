///
/// all data needed to call influx
///

use crate::config::InfluxConfig;
use crate::call::InfluxCall;

//use template_formater::tuple_formater;
use template_formater::tuple_formater_safe;


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

    /// display curl WRITE LP
    ///
    /// /usr/bin/curl --insecure --request POST 'http://jozefina:8086/api/v2/write?org=foookin_paavel&bucket=reqwest_sunday_backup_ds_test&precision=ms' --header 'Authorization: Token jbD0MXwVzetW6r6TFSQ5xIAzSFxwl3rD8tJVvzWr_Ax7ZNBJH1A0LHu38PR8WFWEpy0SuDlYpMyjYBB52riFrA==' --data-raw 'dallas,host=ruth,DsCarrier=labjack,DsId=1052176647976,DsPin=444,Machine=mrazak,DsValid=true DsDecimal=19.3125 1645429809298'
    ///
    pub fn curl_write(&self,
                      template: &str,
                      //debug: bool) -> Result<String, Box<dyn std::error::Error>> {
                      debug: bool) -> Result<String, strfmt::FmtError> {

        let curl_call = tuple_formater_safe(
            template,
            &vec![
                ("url", &self.call.uri_write),
                ("auth", &self.call.auth.join(": ")),    
                ("data", &self.lp),        
            ],
            debug,
        );

        //Ok(curl_call)
        curl_call
    }
}
