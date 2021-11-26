use std::collections::HashMap;

use crate::measurement::{PreRecord};


#[derive(Debug)]
pub struct Horse {
    pub name: String,
    pub color: String,
    pub age: u8,
    pub valid: bool,
}


impl Default for Horse {
    fn default() -> Horse {
        Horse {
            name: "NAME".to_string(),
            color: "COLOR".to_string(),
            age: 0,
            valid: false,
        }
    }
}


#[allow(dead_code)]
pub fn update_vector() {
    let color_2_barva = HashMap::from([
        ("black", "cerny"),
        ("brown", "hnedak"),
        ("chest_nut", "ryzak"),
        ("white", "siml"),
    ]);

    let mut horses = vec![
        Horse {name: "metynka".to_string(),
               color: "black".to_string(),
               age: 12,
               valid: false,
        },

        Horse {name: "wonka".to_string(),
               color: "brown".to_string(),
               age: 6,
               valid: false,
        },

        // default -> EMPTY
        Horse::default(),

        // default -> SOME FIELDS
        Horse {name: "metik".to_string(),
               ..Horse::default()
        },

        // default -> SOME FIELDS
        Horse {name: "lupo".to_string(),
               color: "White".to_string(),
               age: 7,
               ..Horse::default() // no need here
        },
        
        // default -> ALL_FIELDS 
        Horse {name: "lord".to_string(),
               color: "chest nut".to_string(),
               age: 19,
               valid: false,
        },
    ];

    //println!("\n#HORSES_eng:\n{:?}", horses);
    
    for horse in & mut horses {
        // update -> by fields
        horse.name = horse.name.to_uppercase();

        horse.age = match horse.age {
            0 => 0,
            _ => (horse.age + 1) as u8,
        };

        // update -> whole Struct
        *horse = Horse {
            name: horse.name.to_uppercase(),

            age: horse.age,

            color: String::from(
                match color_2_barva.get(&horse.color[..]) {
                    Some(barva) => barva,
                    //None => &horse.color[..],
                    None => "COLOR", // color not found in hash, so default for FALSE flag verification
                }
            ),

            valid: false,
        };

        // valid flag via verification
        horse.valid = if horse.age != 0 && // default age not updated
            horse.name != "NAME" && // name not updated
            horse.color !="COLOR" { // color not updated or not found in hash_map
                true
            } else {
                false
            };
    }

    let all = horses.len();
    // iter -> filter -> collect back to Vec
    let valid = horses.iter().filter(|horse| horse.valid).collect::<Vec<&Horse>>();
    // iter to be able to count
    let valid_count = &valid.iter().count();
    
    println!("\n#HORSES >>> valid: {v} / invalid: {i} / all: {a}\n{f:#?}",
             a = all,
             v = valid_count,
             i = all - valid_count,
             f = valid,
    );
}
    

#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
pub fn update_struct() {
    let mut list: Vec<&PreRecord> = Vec::new();

    let mut first = &PreRecord {
        key: "key".to_string(),
        ts: 1234567890,
        value: "".to_string(),
        id: "id".to_string(),
        measurement: "measurement".to_string(),
        host: "host".to_string(),
        machine: String::from(""),
        carrier: String::from(""),
        valid: String::from(""),
    };

    list.push(first);

    println!("first: {:#?}",
             first,
    );
    
    let mut second = &PreRecord {
        key: "KEY".to_string(),
        ts: 9876543210,
        value: "".to_string(),
        id: "ID".to_string(),
        measurement: "MEASUREMENT".to_string(),
        host: "HOST".to_string(),
        machine: String::from(""),
        carrier: String::from(""),
        valid: String::from(""),
    };

    list.push(second);

    println!("second: {:#?}",
             second,
    );

    for l in &mut list {
        //let mut l = &PreRecord {

        //*&l.key = String::from("k_k_k")


        /*
        let l = &PreRecord {
            key: l.key.to_string(),
            
            ts: l.ts,
            
            id: l.id.to_string(),
            measurement: l.measurement.to_string(),
            host: l.host.to_string(),
            machine: "machine".to_string(), //l.machine.to_string(),
            carrier: "carrier".to_string(), //l.carrier.to_string(),
            valid: "valid".to_string(), //l.valid.to_string(),
            
            //id: "II".to_string(), measurement: "MMM".to_string(), host: "HHH".to_string(), machine: String::from("MMM"), carrier: String::from("CCC"),
            //valid: String::from("VVV"),
            
            value: "spongebob".to_string(),
        };
        */

        /*
        println!("l: {:#?}",
                 &l,
        );
        */
    }

    println!("list: {:?}",
             list,
    );
    
}


#[allow(dead_code)]
pub fn parse_sentence(s: &str) -> usize {
    println!("s: {s} / debug >>> [s]: {s:#?} / s[..]: {ss:#?}",
             s=&s,
             ss=&s[..]
    );
    
    let bytes = s.as_bytes(); // array of bytes
    
    for (i, &item) in bytes.iter().enumerate() { // iterator
        println!("{} -> {} <- <{}>",
                 format!("{:03}", i),
                 format!("{:03}", &item),
                 *&item as char, // dereference
        );
        
        if item == b' ' { // space test
            println!("\nspace at position: {} -> string before space: <{}> and after: <{}>\n",
                     i,
                     &s[0..i],
                     &s[i+1..],
                     )
        }
    }

    s.len()
}


