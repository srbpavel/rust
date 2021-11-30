use strfmt::strfmt;
use std::collections::HashMap;


pub fn tuple_formater<'sf>(template: &String,
                           pair: &Vec<(&str, &str)>,
                           debug: bool) -> String {
    
    let template = String::from(template);
    let mut hash_map = HashMap::new();
    
    if debug { println!("\n#TUPLE_FORMATER:") }

    for t in pair {
        if debug {
            println!(" {} <- {}", t.0, t.1); 
        }
        
        hash_map.insert(t.0.to_string(), String::from(t.1)); // not testing t.0 and t.1 presence
    }
    
    strfmt(&template, &hash_map).unwrap()
}


/*
fn formater<'sf>(template: &String,
                    fake_key: &Vec<&str>, 
                    fake_value: &Vec<&str>) -> String{

    let print_template = String::from(template);
    let mut print_hash = HashMap::new();
    
    for p in 0..*&fake_key.len() as u8 {
        print_hash.insert(
            fake_key[p as usize].to_string(),
            String::from(fake_value[p as usize])
        );
    }
    
    strfmt(&print_template, &print_hash).unwrap()
}
*/


/*
pub struct Pair {
    pub key: String,
    pub value: String,
}


fn struct_formater<'sf>(template: &String,
                          pair: &Vec<Pair>) -> String {

    let print_template = String::from(template);
    let mut print_hash = HashMap::new();
    
    for t in pair {
        println!("Struct: {} -> {}", t.key, &t.value);
        print_hash.insert(t.key.to_string(), String::from(&t.value));
    }

    strfmt(&print_template, &print_hash).unwrap()
}
*/

            /*
            let print_formated =  formater(&print_template, 
                                              &vec!["m", "d", "e"],
                                              &vec![&metric.measurement,
                                                    &config.work_dir,
                                                    &err.to_string(),
                                              ]
            );
            */

            /*
            let print_formated = struct_formater(
                &print_template, 
                &vec![
                    Pair {key:"m".to_string(), value:metric.measurement.to_string()},
                    Pair {key:"d".to_string(), value:config.work_dir.to_string()},
                    Pair {key:"e".to_string(), value:err.to_string()},
                ],
            );
            */
