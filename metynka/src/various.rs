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


trait Display {
    fn display(&self) -> String;

    fn display_body(&self) -> String;
}


impl Display for Message {
    fn display(&self) -> String {
        format!("\n#MESSAGE:\n\n##HEADERS\n{h:?}\n\n##ENVELOPE\n{e:?}", 
                h=&self.headers(),
                e=&self.envelope(),
        )
    }

    fn display_body(&self) -> String {
        let body = self.formatted().into_iter()
            .map(|u| u as char)
            .map(|ch| String::from(ch))
            .collect::<Vec<String>>()
            .concat();

            /* instead concat if some other char needed
            .join("");
            */
        
        format!("\n##BODY\n{b}", 
                b=body,
        )
    }
}


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
        .unwrap(); // safe ? 

    if config.flag.debug_email {
        println!("{}",
                 email.display(),
        );

        if config.flag.debug_email_body {
            println!("{}",
                     email.display_body(),
            );
        }
    }

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

    // /*
    match mailer.send(&email) {
        Ok(_) => {
            println!("\n#EMAIL: ok")
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
    // */
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


#[allow(dead_code)]
pub fn bin_ruler(width: usize,
                 len: usize) -> String {

    let ruler_template = "9876543210";

    let ruler = ruler_template.repeat(width); // ruler series

    ruler[ruler.len() - len ..].to_string() // ruler slice -> maybe by 8 bit steps ???
}


#[allow(dead_code)]
pub fn bin_shift(int: u64,
                 shift_direction: &str,
                 shift_steps: u8,
                 display_ruler: bool) {

    // shift direction + sign
    let (shift, shift_str) = match shift_direction {
        "left" => (int << shift_steps, "<<"),
        "right" => (int >> shift_steps, ">>"),
        _ => panic!("wrong shift direction")
    };

    let bin_int_len = format!("{:0b}", int).len();
    let bin_shift_len = format!("{:0b}", shift).len();

    // initial :b width + ruler len
    let (width, len) = match bin_int_len >= bin_shift_len {
        true => ((bin_int_len / 8) + 1, bin_int_len),
        false => ((bin_shift_len / 8) + 1, bin_shift_len),
    };

    // render ruler
    let ruler = &bin_ruler(width,
                           len);

    // final :b len
    let bin_int = format!("{:0width$b}", int, width=len);
    let bin_shift = format!("{:0width$b}", shift, width=len);
    
    let shift_sentence = format!("{}{} {} {} is {} / ",
                                 format!("{} shift: ", shift_direction),
                                 int,
                                 shift_str,
                                 shift_steps,
                                 shift);

    let space_str = " -> ";

    println!("\n#BIN:\n");
    
    if display_ruler {
        println!("{}",
                 format!("{front}{r}{space}{r}",
                         front=" ".repeat(shift_sentence.len()),
                         r=ruler,
                         space=" ".repeat(space_str.len()),
                 )
        );
    }
    
    println!("{shift_str}{bin_int}{space_str}{bin_shift}",
             shift_str=shift_sentence,
             space_str=space_str,
             bin_int=bin_int,
             bin_shift=bin_shift);
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

/*
fn main() {
    println!("{}",
    create_phone_number(&[0,1,2,3,4,5,6,7,8,9]),
    );
}

fn slice_to_string(s: &[String]) -> String {
    s.concat().to_string()
}                                                  
            
fn create_phone_number(numbers: &[u8]) -> String { 
    let v = numbers.into_iter()                    
    .map(|u| u.to_string())               
    .collect::<Vec<String>>();                     
                                                   
    format!("({}) {}-{}",                          
    slice_to_string(&v[0..3]),                     
    slice_to_string(&v[3..6]),                     
    slice_to_string(&v[6..]),                      
    )                                              
}
*/
