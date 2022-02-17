use strfmt::strfmt;
use std::collections::HashMap;


///
/// fill template with key/value pairs
///
///INPUT:
///
///let tp_path = tuple_formater(                     
///    "query?org={org}", // TEMPLATE                
///    &vec![                                        
///        ("org", "foookin_paavel"), // KEY, VALUE  
///    ],                                            
///    true, // DEBUG flag                           
///);
///
///println!("TP_PATH: {tp_path:?}");
///
///OUTPUT:
///
///#TUPLE_FORMATER:
/// org <- foookin_paavel
///TP_PATH: "query?org=foookin_paavel"
///
pub fn tuple_formater(template: &str,
                      pair: &Vec<(&str, &str)>,
                      debug: bool) -> String {
    
    let tp_template = String::from(template);
    let mut hash_map = HashMap::new();
    
    if debug { println!("\n#TUPLE_FORMATER:") }

    for (key,value) in pair {

        if debug {
            println!("template: {template} key: <{key}> value: <{value}>"); 
        }
        
        hash_map.insert(key.trim().to_string(),
                        value.trim().to_string(),
        );
    }
    
    strfmt(&tp_template, &hash_map).unwrap()
}
