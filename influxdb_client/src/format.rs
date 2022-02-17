use template_formater::tuple_formater;

use crate::connect::InfluxConfig;


/// fill WRITE template with config data
pub fn uri_write(template: &str,
                 config: &InfluxConfig,
                 debug: bool) -> String {

    tuple_formater(template,
                   
                   &vec![                                           
                       ("secure", config.secure),           
                       ("server", config.server),           
                       ("port", &config.port.to_string()),   
                       
                       ("org", config.org),                 
                       ("bucket", config.bucket),           
                       ("precision", config.precision),     
                   ],                                               
                   
                   debug,
    )
}


/// fill READ template with config data
pub fn uri_query(template: &str,
                 config: &InfluxConfig,
                 debug: bool) -> String {
    
    tuple_formater(template,
                   
                   &vec![                                           
                       ("secure", config.secure),           
                       ("server", config.server),           
                       ("port", &config.port.to_string()),   
                       
                       ("org", config.org),                 
                   ],                                               

                   debug,
    )
}
