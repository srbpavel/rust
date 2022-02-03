use std::{fs::{self,
               DirEntry,
               ReadDir},

          path::{Path,
                 PathBuf},

          io::{Error, ErrorKind},

};

mod command_args;
use command_args::{
    Settings,
};


#[derive(Debug)]
pub struct SingleFile<'p> {
    pub path: &'p Path,
    pub output: String,
    pub rename_status: bool,
}


pub trait SfTrait {
    fn debug(&self);
    
    fn print(&self);

    fn parse(& mut self,
             args: &Settings);
    
    fn char_duplicity(& mut self,
                      substitute_char: char);
    
    fn update_rename_status(& mut self);

    fn rename(&self,
              args: &Settings) -> Result<(), Error>;
}


impl SfTrait for SingleFile<'_> {
    fn debug(&self) {
        println!("\n  {:?}", self);
    }

    fn print(&self) {
        println!("\n  {p:?}\n  {o}\n  {r}",
                 p=self.path,
                 o=self.output,
                 r=self.rename_status,
        );
    }

    fn parse(& mut self,
             args: &Settings) {

        self.output = parse_file(&self.path,
                                 args.substitute,
        );
    }
    
    fn char_duplicity(& mut self,
                      substitute_char: char) {

        if self.output
            .contains(
                &format!("{ch}{ch}",
                         ch=substitute_char,
                )) {
                
                self.output = remove_duplicity(&self.output,
                                               substitute_char,
                );
            }
    }

    fn update_rename_status(& mut self) {
        self.rename_status = self.output != format!("{}",
                                                    match self.path
                                                    .file_name()
                                                    .and_then(|s| s.to_str()) {
                                                        Some(n) => n,
                                                        
                                                        None => "",
                                                    }
        );
    }
    
    fn rename(&self,
              args: &Settings) -> Result<(), Error> {

        match create_output_path_buf(&self) {
            Some(output) => {
                rename_file(self.path.to_path_buf(), // IN
                            output, // OUT
                            &args,
                )
            },
            
            None => {

                Err(
                    Error::new(
                        ErrorKind::Other,
                        format!("\n#ERROR: {:#?}\nREASON >>> rename not started as not valid PathBuf",
                                self.path)
                    )
                )
            }
        }
    }
}


impl <'p> SingleFile<'_> {
    pub fn new(
        path: &'p Path, 
        output: String,
        rename_status: bool) -> SingleFile<'p> {
        
        SingleFile {
            path,
            output,
            rename_status,
            ..SingleFile::default()
        }
    }

    pub fn default() -> SingleFile<'p> {
        SingleFile {
            path: Path::new(""),
            output: String::from(""),
            rename_status: false,
        }
    }
}


fn create_output_path_buf(file: &SingleFile) -> Option<PathBuf> {

    let dir = match file
        .path
        .parent() {
            Some(parent) => parent,
            
            None => { return None },
        };

    Some([dir, // ANCESTOR PATH DIR
          Path::new(&file.output), // FILENAME 
    ]
         .iter()
         .collect()
    )
}


fn verify_path_buf(args: &Settings,
                   path: &Path) -> Option<PathBuf> {

    if !path.exists() {
        eprintln!("\nERROR: PATH does not exists or no access {:?}", path);

        return None
    };

    let mut debug_data = format!("\n#ABSOLUTE PATH: {:?}",
                                 path,
    );
    
    // IF RELATIVE WE CHANGE TO ABSOLUTE AND UPDATE
    let full_path_buf: PathBuf = if path.is_relative() {

        let fp = match path.canonicalize() {
            Ok(full_path) => full_path,

            Err(err) => {
                eprintln!("\nERROR: {:?} relative to full_path conversion\nREASON >>> {}",
                          path,
                          err,
                );
                
                return None
            },
        };

        debug_data = format!("\n#RELATIVE: {:?} -> ABSOLUTE {:?}",
                             path,
                             fp,
        );
        
        fp
            
    } else {
        path.to_path_buf()
    };

    // DEBUG
    if args.verbose {
        println!("{}", debug_data);
    };

    Some(full_path_buf)
}


fn rename_file(input: PathBuf,
               output: PathBuf,
               args: &Settings) -> Result<(), Error> {

    let debug_data = format!("{:?} -> {:?}",
                           input,
                           output,
    );
    
    // DEBUG
    println!("  {}", debug_data);
    /*
    if args.verbose {
        println!("  @ WILL RENAME: {}", debug_data);
    };
    */

    // VERIFY NOT TO OVERWRITE if any
   if !output.as_path().exists() {
        if !args.simulate {
            // RENAME
            fs::rename(input, output) // -> Ok()

        } else {
            // SIMULATE
            Err(
                Error::new(
                    ErrorKind::Other,
                    format!("# INFO: simulate {:?}",
                            input,
                    )
                )
            )
        }
       
    } else {

       Err(
           Error::new(
               ErrorKind::Other,
               format!("@ Err ERROR: destination file exists >>> {}",
                       debug_data)
           )
       )
    }
}


fn remove_duplicity(text: &str,
                    character: char) -> String {
    
    let mut uniq = String::from("");
    let mut character_counter = 0;

    for byte in text.as_bytes().iter() {
        let ch = &String::from(*byte as char);

        if ch == &String::from(character) {
            character_counter += 1
        } else {
            character_counter = 0
        }

        if character_counter <= 1 {
            uniq += ch
        }
    }

    uniq
}


// MATCH TABLE FROM https://github.com/YesSeri/diacritics/blob/main/src/lib.rs
fn remove_diacritics(text: &str) -> String {

    text
        .chars()
        /*
        .inspect(|ch| {
            println!("dia_char: {ch}");
        })
        */
        .map(|ch| String::from(ch))
        .collect::<Vec<_>>()
        .iter()
        .map(|dia| match dia.as_ref() {

            "À" | "Á" | "Â" | "Ã" | "Ä" | "Å" | "Æ" => "A",
            "Þ" => "B",
            "Ç" | "Č" => "C",
            "Ď" | "Ð"  => "D",
            "Ě" | "È" | "É" | "Ê" | "Ë" => "E",
            "Ƒ" => "F",
            "Ì" | "Í" | "Î" | "Ï" => "I",
            "Ň" | "Ñ" => "N",
            "Ò" | "Ó" | "Ô" | "Õ" | "Ö" | "Ø" => "O",
            "Ř" =>"R",
            "ß" => "ss",
            "Š" => "S",
            "Ť" => "T",
            "Ů" | "Ù" | "Ú" | "Û" | "Ü" => "U",
            "Ý" => "Y",
            "Ž" => "Z",

            "à" | "á" | "â" | "ã" | "ä" | "å" | "æ" => "a",
            "þ" => "b",
            "ç" | "č" => "c",
            "ď" | "ð"  => "d",
            "ě" | "è" | "é" | "ê" | "ë" => "e",
            "ƒ" => "f",
            "ì" | "í" | "î" | "ï" => "i",
            "ñ" | "ň" => "n",
            "ò" | "ó" | "ô" | "õ" | "ö" | "ø" => "o",
            "ř" => "r",
            "š" => "s",
            "ť" => "t",
            "ů" | "ù" | "ú" | "û" | "ü" => "u",
            "ý" | "ÿ" => "y",
            "ž" => "z",

            _ => dia,
        })
        .collect::<String>()
}


fn replace_char_to_substitute(text: &str,
                              substitute: char) -> String {

    // REPLACE OTHER THEN: 09AZaz AND upper to lower_case
    text
        .as_bytes()
        .iter()
        .map(|b| match b {
            // az
            low @ 97..=122 => (*low as char),

            // AZ -> az
            high @ 65..=90 => (high.to_ascii_lowercase() as char),

            // 09
            num @ 48..=57 => (*num as char),
            
            _ => substitute,
        })
        .collect::<String>()
}


fn parse_file(path: &Path,
              substitute_char: char) -> String {
    
    // FILE: NAME -> replace + diacritics + lowercase
    let name = replace_char_to_substitute(& match &path
                                          .file_stem()
                                          .and_then(|s| s.to_str()) {
                                              Some(n) => remove_diacritics(n),
                                              
                                              None => String::from("")
                                          },
                                          substitute_char,
    );
    
    // FILE: EXTENSION -> diacritics + lower_case
    let extension = match &path
        .extension()
        .and_then(|s| s.to_str()) {
            Some(e) => {
                format!(".{}", remove_diacritics(&e.to_lowercase()))
            },
            
            None => String::from("")
        };

    // COLLECT FILENAME
    let output = format!("{}{}",
                         name,
                         extension,
    );

    output
}


fn normalize_chars<'f>(entry: &'f Path,
                       args: &'f Settings) -> SingleFile<'f > {

    // FULL_PATH
    let mut single_file = SingleFile {path: &entry,
                                      ..SingleFile::default()
    };

    // SPLIT NAME + EXT -> remove dia + replace non 09AZaz <- join back
    single_file.parse(&args);

    // TEST FOR substitute_char duplicity and replace if any
    single_file.char_duplicity(args.substitute);
    
    // if RENAME is needed
    single_file.update_rename_status();

    single_file
}


fn match_element(n: &str,
                 element: Result<DirEntry, Error>,
                 args: &Settings) {

    match n {
        // FILE
        "true" => {
            match &element {
                Ok(file) => {

                    let path = &file.path();

                    let single_file = normalize_chars(path,
                                                      &args,
                    );
                    
                    // DEBUG
                    if args.verbose {
                        single_file.debug();
                    } else {
                        println!("");
                    };
                    
                    // RENAME
                    if single_file.rename_status {
                        let rename_status = single_file.rename(&args);

                        match rename_status {
                            Ok(_) => {
                                if args.verbose{ 
                                    println!("  RENAME_STATUS: Ok");
                                }
                            },
                            
                            Err(err) => {
                                eprintln!("  RENAME_STATUS: {}", err);
                            }
                        }
                    } else {
                        println!("  RENAME_STATUS: no need to rename {:?}",
                        single_file.path);
                    };
                },
                
                Err(err) => {
                    eprintln!("  ERROR: FILE element: {:?}\n>>> Reason: {}",
                              element,
                              err,
                    );
                }
            }
        },
        
        // DIR
        "false" => {
            // RECURSE PARSE DESCENDANT DIR's
            if args.recursive {
                match &element {
                    Ok(dir) => {
                        prepare_parse_dir(&args,
                                          Path::new(&dir.path()),
                        );
                    },
                        
                    Err(err) => {
                        eprintln!("\nERROR: DIR element: {:?}\n>>> Reason: {}",
                                  element,
                                  err,
                        );
                    }
                };
            }
        },

        // ERROR ARM from DirEntry.metadata()
        _ => { /* nothing to do for now */ },
    }
}


fn parse_dir(dir: ReadDir,
             args: &Settings) {
    
    for element in dir {
        match element // DirEntry
            .as_ref() // FOR INNER ELEMENT USAGE
            .and_then(|e| Ok(match e.metadata() {
                // BOOL -> STRING as need to handle if metadata ERROR
                Ok(m) => format!("{}", m.is_file()),
                
                Err(err) => {
                    eprintln!("\nERROR: element METADATA: {:?}\n>>> Reason: {}",
                              element,
                              err,
                    );
                    
                    String::from("")
                }
            }
                             
            )) {
                
                Ok(n) => {
                    match_element(&n, // n is "true" || "false" || ""
                                  element,
                                  &args,
                    );
                },
                
                Err(err) => {
                    eprintln!("\nERROR: element: {:?}\n>>> Reason: {}",
                              element,
                              err,
                    );
                }
            }
    }
}


fn list_dir(path: &Path) -> Option<ReadDir> {  

    let dir = match path.read_dir() {
        Ok(d) => d,
        
        Err(err) => {
            eprintln!("\nERROR: Problem reading directory: {:?}\nREASON >>> {}",
                      path,
                      err,
            );

            return None
        },
    };
    
    Some(dir)
}


fn prepare_parse_dir(args: &Settings,
                     path: &Path) {
    
    match verify_path_buf(&args,
                          &path,
    ) {
        Some(full_path) => {

            match list_dir(&full_path) {
                Some(dir_data) => {
                    
                    parse_dir(
                        dir_data,
                        &args,
                    )
                },
                
                None => {
                    // PERMISSION DENIED to read dir data
                    // future use
                    eprintln!("Err <list_dir> path: {:?} {:?}\n", path, args);
                }
            }
        },

        None => {
            // NON EXISTING path/dir
            // future use
            eprintln!("Err <verify_path_buf> path: {:?} {:?}\n", path, args);
        }
    }
}


fn main() {
    //CMD ARGS as Settings
    let settings = command_args::get_settings();
    
    // DEBUG CMD
    if settings.verbose {
        println!("\n#CMD: {:#?}", settings);
    };

    // START WITH ARG DIR
    prepare_parse_dir(&settings,
                      &settings.path,
    );
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn normalize_filename() {
        use std::fs::{File,
                      remove_file,
        };
    
        let name_in = "ŽÍžaLa.jŮlie.není.Šnek";
        let _create_result = File::create(name_in);
        let path_in = Path::new(name_in);

        let args = Settings {
            verbose: false,
            // DEBUG
            //verbose: true,
            ..Settings::default()
        };

        let normalized_file: SingleFile = normalize_chars(path_in,
                                                          &args,
        );

        // RENAME
        // NOT TESTING STATUS as "name_in" is created to be renamed
        let rename_result = normalized_file.rename(&args);

        // DELETE NORMALIZED
        let remove_result = match remove_file(&normalized_file.output) {
            Ok(()) => true,
            Err(_) => false,
        };

        if args.verbose {
            println!("  @ RENAME_STATUS: {:?}\n  @ DELETE_STATUS: {:?}",
                     rename_result,
                     remove_result,
            )
        }
        
        // to compare Path
        /*
        assert_eq!(Path::new("zizala_julie_neni.snek"),
                   Path::new(&normalized_file.output),
        );
        */

        // to compare Path status
        /*
        assert_eq!(Path::new(&normalized_file.output).exists(),
                   true,
        );
        */

        // to compare DELETE result to bool
        // all previous steps needs to be valid to succed
        assert!(remove_result)
    }

    #[test]
    fn relative_path_to_absolute() {

        let args = Settings {
            path: Path::new(".").to_path_buf(),
            simulate: true,
            recursive: false,
            verbose: false,
            ..Settings::default()
        };

        // TRY TO SIMULATE ERROR !!!
        let cwd = env::current_dir().unwrap();

        // DEBUG TEST
        /* // $cargo test -- --nocapture
        // ugly, but just be sure
        println!("TEST:\n cwd: {:?}\n rel: {:?}",
                 cwd
                 .read_dir()
                 .unwrap()
                 .map(|res| res.map(|e| e
                                    .path()
                 )
                      .unwrap()
                 )
                 .collect::<Vec<_>>(),
                 
                 args.path
                 .read_dir()
                 .unwrap()
                 .map(|res| res.map(|e| e
                                    .path()
                                    .canonicalize()
                                    .unwrap()
                 )
                                    .unwrap()
                 )
                 .collect::<Vec<_>>(),
        );
        */
        
        assert_eq!(verify_path_buf(&args,
                                   &args.path,
        ),
                   Some(Path::new(&cwd).to_path_buf()),
        )
    }
    
    #[test]
    fn replace_diacritics() {
        assert_eq!(remove_diacritics("tRPaslÍČek"),

                   String::from("tRPaslICek"),
        )
    }

    // OBSOLETE as file_name and file_extension are seperate
    #[test]
    fn replace_diacritics_with_dots() {
        assert_eq!(remove_diacritics("ŽÍžaLa.jŮlie"),

                   String::from("ZIzaLa.jUlie"),
        )
    }

    #[test]
    fn char_duplicity() {
        assert_eq!(remove_duplicity(&String::from("a---B-c--D"),
                                    '-'),

                   String::from("a-B-c-D"),
        )
    }

    #[test]
    fn replace_char_name_with_dia() {
        assert_eq!(
            replace_char_to_substitute(
                &remove_diacritics(
                    &String::from("múčho můřká.áchjó.škýt")),
                
                '*'),
            
            String::from("mucho*murka*achjo*skyt"),
        )
    }

    #[test]
    fn replace_char_name_without_dia() {
        assert_eq!(
            replace_char_to_substitute(
                &String::from("mucho*murka*achjo*skyt"),
                
                '_'),
            
            String::from("mucho_murka_achjo_skyt"),
        )
    }
}
