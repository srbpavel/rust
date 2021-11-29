use std::collections::HashMap;


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
               color: "WHITE".to_string(),
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
                match color_2_barva.get(&horse.color.to_lowercase()[..]) {
                    Some(barva) => barva,
                    None => "COLOR", // color not found in hash_map, so default for FALSE flag verification
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


