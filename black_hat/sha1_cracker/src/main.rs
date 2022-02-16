use sha1::Digest;

use std::{
    env,
    fs::File,
    io::{BufRead,
         BufReader,
    },
};


const SHA1_HEX_STRING_LENGTH: usize = 40;


fn main() {
    // CMD ARGS to Vec
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:\nsha1_cracker: <wordlist.txt> <sha1_hash>");
        
        std::process::exit(1)
    }

    // HASH
    let hash = args[2].trim();

    if hash.len() != SHA1_HEX_STRING_LENGTH {
        println!("sha1 hash is not valid");
        
        std::process::exit(1)
    }

    // TABLE
    let arg_file = &args[1];

    let wordlist_file = match File::open(&arg_file) {
        Ok(wf) => wf,

        Err(err) => {
            eprintln!(
                "EXIT: <{}> not found or accessible\nREASON >>> {err:?}",
                &arg_file,
            );

            std::process::exit(1)
        }
    };
    
    // SEARCH
    BufReader::new(wordlist_file)
        .lines()
        .for_each(|line| match line {

            Ok(common_password) => {
            
                if hash.to_lowercase()
                    == hex::encode(
                        sha1::Sha1::digest(
                            common_password
                                .trim()
                                .as_bytes()
                        )
                    )
                {
                    println!("password found: {}", common_password);
                }
            },

            Err(_) => {},
        });
}
