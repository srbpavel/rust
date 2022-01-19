use std::{fs::{self,
               DirEntry,
               ReadDir},

          path::{Path,
                 PathBuf},

          process,
};

mod command_args;
use command_args::{Args};


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
             args: &Args);
    
    fn char_duplicity(& mut self,
                      substitute_char: char);
    
    fn update_rename_status(& mut self);

    fn rename(&self,
              args: &Args);
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
             args: &Args) {

        self.output = parse_file(&self.path,
                                 args.substitute_char,
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
              args: &Args) {

        rename_file(self.path.to_path_buf(), // IN
                    create_output_path_buf(&self), // OUT
                    &args,
        );
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


fn create_output_path_buf(file: &SingleFile) -> PathBuf {
    [
        // ANCESTOR PATH DIR
        match file.path
            .parent() {
                Some(d) => d,
                
                None => {
                    // EXIT not nice but how to skip ?
                    process::exit(1); 
                },
            },
        
        // FILENAME 
        Path::new(&file.output),
    ]
        .iter()
        .collect()
}


fn verify_path_buf(args: &Args) -> PathBuf {

    if !args.path.exists() {
        eprintln!("\nEXIT: PATH does not exists or no access {:?}", args.path);

        // EXIT as path is essential
        process::exit(1);
    };

    let mut debug_data = format!("\n#PATH: {:?}",
                                 args.path,
    );

    // IF RELATIVE WE CHANGE TO ABSOLUTE AND UPDATE
    let full_path: PathBuf = if args.path.is_relative() {
        
        let fp = args.path
            .canonicalize()
            .unwrap(); // should never fail as verified so safe ?

        debug_data = format!("\n#PATH: {:?} -> {:?}",
                             args.path,
                             fp,
        );
        
        fp
            
    } else {
        args.path.to_path_buf()
    };

    // DEBUG
    if args.verbose {
        println!("{}", debug_data);
    };

    full_path
}


fn rename_file(input: PathBuf,
               output: PathBuf,
               args: &Args) {

    let debug_data = format!("{:?} -> {:?}",
                           input,
                           output,
    );
    
    // DEBUG
    if args.verbose {
        println!("  @ WILL RENAME: {}", debug_data);
    };

    // VERIFY NOT TO OVERWRITE if any
    if !output.as_path().exists() {
        if !args.simulate {
            // RENAME
            match fs::rename(input, output) {
                Ok(_) => {
                    println!("  RENAME: {}", debug_data);
                },
                
                Err(err) => {
                    eprintln!("\nERROR: RENAME\nREASON >>> {}", err);
                }
            }
        }
    } else {
        println!("  @ WARNING: destination file exists >>> {}", debug_data);
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


fn normalize_chars(entry: &DirEntry,
                   args: &Args) {

    // FULL_PATH
    let mut file = SingleFile {path: &entry.path(),

                               ..SingleFile::default()
    };

    // SPLIT NAME + EXT -> remove dia + replace non 09AZaz <- join back
    file.parse(&args);

    // TEST FOR substitute_char duplicity and replace if any
    file.char_duplicity(args.substitute_char);
    
    // if RENAME is needed
    file.update_rename_status();

    // DEBUG
    if args.verbose {
        file.debug();
    };

    // RENAME
    if file.rename_status {
        file.rename(&args)
    };
}


// too long !!!
fn parse_dir(dir: ReadDir,
             args: &Args) {
    
    for element in dir {
        match element // DirEntry
            .as_ref() // FOR INNER ELEMENT USAGE
            .and_then(|e| Ok(
                // BOOL
                /*
                e
                    .metadata()
                    .unwrap() // should have Metadata always as path valid ?
                    .is_file())
            ) {
                */

                // String
                /*
                format!("{}", e
                        .metadata()
                        .unwrap() // should have Metadata always as path valid ?
                        .is_file())
            )) {
                */

                // /*
                match e.metadata() {
                    Ok(m) => format!("{}", m.is_file()),
                    Err(_) => String::from("ERROR METADATA"),
                }
            )) {
                // */
                
                //Ok(n) => match n {
                Ok(n) => match n.as_ref() {
                    // FILE
                    //true => {
                    "true" => {
                        match &element {
                            Ok(file) => {
                                normalize_chars(&file,
                                                &args,
                                )
                            },
                            
                            Err(err) => {
                                eprintln!("\nERROR: FILE element: {:?}\n>>> Reason: {}",
                                          element,
                                          err,
                                );
                            }
                        }
                    },

                    // DIR
                    //false => {
                    "false" => {
                        // RECURSE PARSE DESCENDANT DIR
                        if args.recursive {
                            match &element {
                                Ok(dir) =>
                                    parse_dir(
                                        list_dir(Path::new(&dir.path()),
                                                 &args,
                                        ),
                                        &args,
                                    ),
                                
                                Err(err) => {
                                    eprintln!("\nERROR: DIR element: {:?}\n>>> Reason: {}",
                                              element,
                                              err,
                                    );
                                }
                            };
                        }
                    },

                    _ => {},
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


fn list_dir(path: &Path,
            args: &Args) -> ReadDir {

    // DEBUG
    if args.verbose {
        println!("\n>>> DIR_PATH: {}", path.display());
    }

    let dir = path
        .read_dir() // INSTEAD fs::read_dir(&path)
        .unwrap_or_else(|err| { // -> match
            eprintln!("\nEXIT: Problem reading directory\nREASON >>> {}", err);
        
            process::exit(1); // do i need this or just want it ?
        });
    
    dir
}


fn main() {
    //CMD ARGS
    let args = command_args::read();

    // DEBUG CMD
    if args.verbose {
        println!("\n#CMD: {:#?}", args);
    };
    
    // START WITH ARG DIR
    // not nice to have &args 3x times
    parse_dir(
        list_dir(
            &verify_path_buf(&args),
            &args,
        ),
        &args,
    );
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn relative_path_to_absolute(){

        let args = Args {
            path: Path::new(".").to_path_buf(),
            simulate: false,
            recursive: false,
            verbose: false,
            substitute_char: '_',
        };

        let cwd = env::current_dir().unwrap();

        assert_eq!(verify_path_buf(&args),
                   Path::new(&cwd).to_path_buf(),
        )
        
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
                 
                 //relative
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
    }
    
    #[test]
    fn replace_diacritics(){
        assert_eq!(remove_diacritics("tRPaslÍČek"),

                   String::from("tRPaslICek"),
        )
    }

    // OBSOLETE as file_name and file_extension are seperate
    #[test]
    fn replace_diacritics_with_dots(){
        assert_eq!(remove_diacritics("ŽÍžaLa.jŮlie"),

                   String::from("ZIzaLa.jUlie"),
        )
    }

    #[test]
    fn char_duplicity(){
        assert_eq!(remove_duplicity(&String::from("a---B-c--D"),
                                    '-'),

                   String::from("a-B-c-D"),
        )
    }

    #[test]
    fn replace_char_name_with_dia(){
        assert_eq!(
            replace_char_to_substitute(
                &remove_diacritics(
                    &String::from("múčho můřká.áchjó.škýt")),
                
                '*'),
            
            String::from("mucho*murka*achjo*skyt"),
        )
    }

    #[test]
    fn replace_char_name_without_dia(){
        assert_eq!(
            replace_char_to_substitute(
                &String::from("mucho*murka*achjo*skyt"),
                
                '_'),
            
            String::from("mucho_murka_achjo_skyt"),
        )
    }
}
