use clap::{App,
           AppSettings,
           Arg,
           ArgMatches,
};

use std::path::{Path,
                PathBuf,
};


const ABOUT: &str = "\nRename FILES [not DIRS] with non alpha-numeric characters + remove diacritics + lowercase in given path or current directory and descendant directories";

const USAGE: &str = "\nnormalize DIR [OPTION].. \n\nnormalize /temp/ -s -v -r -c=# [dry-run + verbose + recursive rename files in /temp/ with usage of # substitute]";


// `pwd`
const DEFAULT_PATH: &str = ".";
const DEFAULT_CHAR: char = '_';


mod options {
    pub static PATH: &str = "path";
    pub static SIMULATE: &str = "simulate";
    pub static RECURSIVE: &str = "recursive";
    pub static VERBOSE: &str = "verbose";
    pub static SUBSTITUTE: &str = "substitute";
}


#[derive(Debug, Default)]
pub struct Settings {
    pub path: PathBuf,
    pub simulate: bool,
    pub recursive: bool,
    pub verbose: bool,
    pub substitute: char,
}


impl Settings {
    // need's to be PUB for TESTS
    pub fn default() -> Settings {
        Settings {
            path: Path::new(&DEFAULT_PATH)
                .canonicalize()
                .unwrap()
                .to_path_buf(),
            
            simulate: false,
            recursive: false,
            verbose: false,
            substitute: DEFAULT_CHAR,
        }
    }

    
    fn new(matches: ArgMatches) -> Settings {
        // start with DEFAULT
        let mut settings = Settings { ..Settings::default() };

        // ALL bool FLAG's
        settings.simulate = matches.is_present(options::SIMULATE);
        settings.recursive = matches.is_present(options::RECURSIVE);
        settings.verbose = matches.is_present(options::VERBOSE);
        
        // PATH
        let path = Path::new(matches.value_of(options::PATH)
                             .unwrap()
        );

        settings.path = if path.exists() { path
                                           .canonicalize() // relative -> full_path
                                           .unwrap()
                                           .to_path_buf()
        } else {
            eprintln!("\nERROR: ARG path: <{path:?}> not valid -> we will stop");

            std::process::exit(1)
        };

        // SUBSTITUTE_CHAR
        settings.substitute = parse_substitute_char(
            matches.value_of(options::SUBSTITUTE),
            &settings,
        );

        // UPDATED
        settings
    }
}


/// workflow for substitute char
///
/// use DEFAULT_CHAR if not provided
///
/// exit if provided but empty like -c= or -c=""
///
/// exit if not single char, but multiple as -c=---
///
fn parse_substitute_char(sub: Option<&str>,
                         settings: &Settings) -> char {

    match sub {
        Some(value) => {
            if settings.verbose {
                println!("substitute_char value: <{value}>");
            }
            
            let bytes = value.as_bytes();
            
            match bytes.len() {
                // CORRECT
                1 => { 
                    let substitute = bytes[0] as char;
                    
                    if settings.verbose {
                        println!("char: '{substitute}'");
                    }
                    
                    substitute
                },
                
                // PROVIDED but EMPTY
                0 => {
                    println!("EXIT: empty char: '{}'",
                             
                             value,
                    );
                    
                    std::process::exit(1);
                },

                // TO MANY CHARS
                _ => {
                    println!("EXIT: too many chars: '{:}'",
                             
                             value,
                    );
                    
                    std::process::exit(1);
                },
            }
        },
        
        None => {
            
            // DEFAULT char
            settings.substitute
        },
    }
}


/// clap App param's
fn new_clap_app<'a>() -> App<'a> {

    App::new("NORMALIZE")
        .about(ABOUT)
        .override_usage(USAGE)
        .setting(AppSettings::InferLongArgs)
        .arg(
            Arg::new(options::PATH)
                // ONLY single path
                .multiple_occurrences(false)
                .takes_value(true)
                .index(1)
                .default_value(DEFAULT_PATH),
        )
        .arg(
            Arg::new(options::SIMULATE)
                .short('s')
                .long(options::SIMULATE)
                .help("dry run"),
        )
        .arg(
            Arg::new(options::RECURSIVE)
                .short('r')
                .long(options::RECURSIVE)
                .help("apply also for descendant dirs"),
        )
        .arg(
            Arg::new(options::VERBOSE)
                .short('v')
                .long(options::VERBOSE)
                .help("display debug info"),
        )
        .arg(
            Arg::new(options::SUBSTITUTE)
                .short('c')
                .long(options::SUBSTITUTE)
                .takes_value(true)
                // BECAUSE char's as: -c # or --substitute * BREAKS IT
                // -c=* or -c="*" works's OK
                .require_equals(true)
                // SET IN Settings::default()
                //.default_value("_")
                .help("substitute char"),
        )
}


/// main call to get Settings from cmd_args
pub fn get_settings() -> Settings {
    // CREATE App
    let app = new_clap_app();

    // GET_MATCHES
    let matches = app.get_matches();

    /* // DEBUG
    println!("MATCHES: {matches:#?}\n\nMATCH_TEST: {:?} / {:?} / {:?}",

             // value of supplied arg at runtime 
             matches.value_of(options::SUBSTITUTE),
             
             // presence of arg
             matches.is_present(options::SUBSTITUTE),
             
             // if any args present
             matches.args_present(),
    );
    */

    // CMD ARGS to Struct
    Settings::new(matches)
}
