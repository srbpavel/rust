use crate::config::InfluxConfig;

use template_formater::tuple_formater;


/// prepare uri_write for InfluxCall
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


/// prepare uri_query for InfluxCall
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

/// prepare auth with TOKEN for InfluxCall
pub fn token(template: &str,
             config: &InfluxConfig,
             debug: bool) -> String {
    
    tuple_formater(template,
                   
                   &vec![                              
                       ("token", config.token),
                   ],                                  
                   
                   debug,
    )
}
