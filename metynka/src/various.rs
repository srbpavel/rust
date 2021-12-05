use std::collections::HashMap;

use metynka::{TomlConfig};

// lettre = "0.10.0-rc.4"
// https://github.com/lettre/lettre/tree/master/examples
use lettre::{
    Message, SmtpTransport, Transport,
    transport::smtp::{
        authentication::Credentials,
        client::{
            Tls, TlsParameters},
    },
};

/*
use std::fmt;
impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //fmt.debug_struct("Message")
        self.fmt.debug_struct("Message")
            .field("headers", &self.headers)
            //.field("body", &self.body)
            .field("body", "NOT DISPLAYED")
            .field("envelope", &self.envelope)
            .finish()
    }
}
*/


#[allow(dead_code)]
pub fn easy_email(config: &TomlConfig,
                  subject: &str,
                  body: &str,
                  sms: bool) {

    /*
    .from("influx_db <influxdb@srbpavel.cz>".parse().unwrap())
        
    .to("<prace@srbpavel.cz>".parse().unwrap())

    .reply_to(config.email.source_email.parse().unwrap())
    */

    // MSG
    let mut email = Message::builder() // -> MessageBuilder
        .from(config.email.source_email.parse().unwrap())
        .to(config.email.target_email.parse().unwrap())
        .subject(subject);

    // append SMS warning
    email = match sms {
        true => email.cc(config.email.sms_email.parse().unwrap()),
        false => email,
    };

    let email = email
        .body(String::from(body)) // -> Message
        .unwrap();

    if config.flag.debug_email {
        println!("\n#EMAIL\n{:#?}", email);
    }



    
    
    /* DEBUG MSG <- no CC
    #EMAIL
    Message {
        headers: Headers {
            headers: [
                (
                    HeaderName(
                        "From",
                    ),
                    "influxdb@srbpavel.cz",
                ),
                (
                    HeaderName(
                        "To",
                    ),
                    "prace@srbpavel.cz",
                ),
                (
                    HeaderName(
                        "Subject",
                    ),
                    "spongebob -> metynka:: temperature",
                ),
                (
                    HeaderName(
                        "Content-Transfer-Encoding",
                    ),
                    "7bit",
                ),
                (
                    HeaderName(
                        "Date",
                    ),
                    "Sun, 05 Dec 2021 07:46:44 -0000",
                ),
            ],
        },
        body: Raw(
        [
            91,
            ..
        ],
        ),
        envelope: Envelope {
            forward_path: [
                Address {
                    serialized: "prace@srbpavel.cz",
                    at_start: 5,
                },
            ],
            reverse_path: Some(
                Address {
                    serialized: "influxdb@srbpavel.cz",
                    at_start: 8,
                },
            ),
        },
    }
    */

    // USER + PASS
    let creds = Credentials::new(config.email.source_email.parse().unwrap(),
                                 config.email.v_pass.parse().unwrap());

    // DEFAULT port 587
    /*
    let mailer = SmtpTransport::starttls_relay(&config.email.smtp_server)
        .unwrap()
        .credentials(creds)
        .build();
    */

    // CONFIG port
    // new TLS via smtp_server_address
    let tls_parameters = TlsParameters::new(config.email.smtp_server.to_string()).unwrap();
   
    let mailer = SmtpTransport::builder_dangerous(&config.email.smtp_server)
        .port(config.email.port)
        .tls(Tls::Required(tls_parameters))
        .credentials(creds)
        .build();

        /* WRONG port -> long long time_out

         5  REASON >>>lettre::transport::smtp::Error {
         6      kind: Connection,
         7      source: Error {
         8          kind: TimedOut,
         9          message: "connection timed out",
        10      },
        11  }
        */

    match mailer.send(&email) {
        Ok(_) => {
            //println!("email sent")
        },
        
        Err(e) => {
            eprintln!("ERROR: email not send\nREASON >>>{:#?}", e)
        },

        /* wrong UESR_EMAIL or PASSSWORD

        14  ERROR: email not send
        15  REASON >>>lettre::transport::smtp::Error {
        16      kind: Permanent(
        17          Code {
        18              severity: PermanentNegativeCompletion,
        19              category: Unspecified3,
        20              detail: Five,
        21          },
        22      ),
        23  }
         */

        /* wrong SMTP server
        
        11  ERROR: email not send
        12  REASON >>>lettre::transport::smtp::Error {
        13      kind: Connection,
        14      source: Custom {
        15          kind: Uncategorized,
        16          error: "failed to lookup address information: Name or service not known",
        17      },
        18  }
        */
    }
}


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

/*
let s = format!("{:#?}", (1, 2));
    match s.contains("\","){
        true => {
            let parts = s.matches("\",").collect::<Vec<&str>>();
            parts.len()
        },
        _ => 1
    }

*/
