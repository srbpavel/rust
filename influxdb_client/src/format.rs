use template_formater::tuple_formater;

//use crate::connect::InfluxConfig;
use crate::config::InfluxConfig;


/// prepare api write uri
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


/// prepare api read/query uri
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

/// prepare token header
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
