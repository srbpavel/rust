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
        horse.valid = if
            horse.age != 0 && // default age not updated
            horse.name != "NAME" && // name not updated
            horse.color !="COLOR" { // color not updated or not found in hash_map
                true
            } else {
                false
            };
    }

    let all = horses.len();

    // https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html
    //let valid: Vec<_> = horses
    let valid = horses
        .into_iter()
        .filter(|horse| horse.valid)
        .map(|horse| {
            Horse {
                color:horse.color.to_uppercase(),
                ..horse
            }
        })
        .collect::<Vec<Horse>>();

    let valid_count = &valid
        .iter()
        .count();

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


    // QUICK SAMPLE CODE TEST
    /*
    use toml::Value;
    let fookume = "foookin = 'paavel'".parse::<Value>().unwrap();
    println!("\nTOML: {:#?} <- {:?}",
             fookume["foookin"],
             fookume,);
    */

    /*
    let some_number = Some(false);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("\n<{:?}>\n<{:?}>\n<{:?}>",
             some_number,
             some_string,
             absent_number,
             );
    */
    
    /*
    let _s_string = String::from("foookin paavel");
    //let some_words = various::parse_sentence(&_s_string); // String
    
    let _s = "foookin paavel";
    let some_words = various::parse_sentence(_s); // str

    println!("\n#SOME_WORDS: {}", some_words);
    */

    /*
    let line = "1\n2\n3\n4\nBUCKET";
    
    for num in line.lines() {
        match num.parse::<u32>().map_or(666, |i| i * 2) {
            n => println!("num: {} / <n: {:#?}>", num, n),
        }
        /*
        match num.parse::<u8>().map(|i| i * 2) {
            Ok(n) => println!("num: {} / <n: {:#?}>", num, n),
            Err(why) => eprintln!("\n#ERROR: num: <{}> \n>>> REASON: {}", num, why),
            //Err(..) => {},
        }
        */
    }
    */

    
    /*
    enum Example {
        Data(i32),
    }
    
    let x = Example::Data(123); // wrap the data
    let Example::Data(y) = x;   // unwrap the data via a pattern
    
    dbg!(y); // prints 'y = 123'
 */