use clap::{Parser};


#[derive(Debug)]
#[derive(Parser)]
#[clap(version = "0.1.0", name="NORMALIZE", about = "\nReplace non alpha-numeric characters + remove diacritics + lowercase in given path or current directory files / descendant directories", author = "\nPavel SRB <prace@srbpavel.cz>")]
pub struct Args {
    #[clap(parse(from_os_str))]
    #[clap(default_value=".", help="working path / `pwd`", index=1)]
    pub path: std::path::PathBuf,

    #[clap(parse(try_from_str))]
    #[clap(short = 's', long = "simulate", help="dry run")]
    pub simulate: bool,

    #[clap(parse(try_from_str))]
    #[clap(short = 'r', long = "recursive", help="apply also for descendant dirs")]
    pub recursive: bool,

    #[clap(short = 'v', long = "verbose", help="display debug info")]
    pub verbose: bool,

    #[clap(name="SUBSTITUTE CHAR", short = 'c', long = "substitute", default_value="_", help="substitute char")]
    pub substitute_char: char,
}

/* // OBSOLETE
use std::{env,
          process,
};

//#[derive(Debug, Clone)]
#[derive(Debug)]
pub struct CmdArgs {
    pub simulate: bool,
    pub full_path: String,
}


impl CmdArgs {
    pub fn new(mut args: env::Args) -> Result<CmdArgs, &'static str> {
        // DEBUG -> HARDCODED as config with debug_flag's not parsed yet
        println!("\n#COMMAND {:#?}",
                 args);

        // CLAP TEST
        /*
        const ARG_COUNT: usize = 2 + 1; // sum of struct CmdArgs members + 1 PROGRAM

        if args.len() < ARG_COUNT {
            return Err("not enough arguments")
        } else if args.len() > ARG_COUNT {
            return Err("too many arguments")
        }
        */

        let _program = match args.next() { // FUTURE_USE

            Some(arg) => arg,

            None => return Err("should never fail"),
        };

        let simulate = match args.next() {

            Some(arg) => arg
                .to_lowercase()
                .parse::<bool>()
                .unwrap_or_else(|err| {
                    eprintln!("\nEXIT: SIMULATE argument not true/false\nREASON: >>> {}", err);
                    process::exit(1);
                }),

            None => {
                eprintln!("no SIMULATE cmd_argument");
                process::exit(1);
            }
        };

        
        let full_path = match args.next() {

            Some(arg) => arg,

            None => return Err("no [full_path] cmd_argument"),
        };

        /* ENV
        // terminal: $ export CASE_INSENSITIVE=1
        // terminla: $ unset CASE_INSENSITIVE
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
         */

        return Ok(CmdArgs {simulate,
                           full_path,
        });
    }
}
*/
