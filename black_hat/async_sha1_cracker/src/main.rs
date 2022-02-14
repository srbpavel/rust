use sha1::Digest;

use std::{
    io::{BufRead,
    },
};


const SHA1_HEX_STRING_LENGTH: usize = 40;


/// str -> hash + pass eq
async fn parse(pass: &str,
               hash: &str) -> Option<String> {
    
    if hash.to_lowercase()
        == hex::encode(
            sha1::Sha1::digest(
                pass.trim().as_bytes()
            )
        )
    {
        println!("\npassword: {pass}\n");

        Some(String::from(pass))
            
    } else { None }
}


#[async_std::main]
async fn main() {
    
    // CMD ARGS to Vec
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Usage:\nasync_sha1_cracker: <wordlist.txt> <sha1_hash>");
        std::process::exit(1)
    }

    // HASH
    let hash = &args[2].trim();

    if hash.len() != SHA1_HEX_STRING_LENGTH {
        println!("sha1 hash is not valid");
        
        std::process::exit(1)
    }

    // TABLE
    let arg_file = &args[1];

    // SYNC: FILENAME -> File
    let wordlist_file = match std::fs::File::open(&arg_file) {
        Ok(wf) => wf,
        
        Err(err) => {
            eprintln!(
                "EXIT: <{}> not found or accessible\nREASON >>> {err:?}",
                &arg_file,
            );
            
            std::process::exit(1)
        }
    };
    
    ///////////////////////////////////////////////////////////////////////////

    // /* // IF COMMENTED -> BUG_PANIC_START: rustc 1.58.1 
    // SOLVED with: rustc 1.60.0-nightly
    // SYNC FILE -> lines // no need for this to be async
    let lines = std::io::BufReader::new(wordlist_file)
        .lines()
        .filter_map(|line| match line {
            Ok(_) => Some(line.unwrap()),
            Err(_) => None,
        })
        //.inspect(|l| println!("inspect: {l:?}"))
        .collect::<Vec<String>>();

    // ALL QUERIES prepared for FUTURE
    let queries: Vec<_> = lines
        .iter()
        .map(|line|
             parse(&line, hash)
        )
        .collect();

    // LET IT ROOL
    let _ = futures::future::join_all(queries).await;
    // */ // BUG_PANIC_END

    // PLAYGROUND
    /*
    vec!["wonka", "metik", "welly"]
        .into_iter()
        .for_each(|horse| {
            println!("semik: {horse}");
        });
    */
    
    println!("\n{:?}",
             std::env::var("USER")
             .ok()
             .and_then(|user| list_groups(&user)),
    );
    
}


fn list_groups(user: &str) -> Option<String> {

        Some(
            std::process::Command::new("groups")
                .arg(user)
                .output()
                .expect("user groups")
                .stdout // Vec<u8>

                .into_iter()
                .map(|ch| String::from(ch as char))
                .collect::<Vec<_>>()
                .concat()
        )
}
