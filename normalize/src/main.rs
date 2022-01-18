use std::{env,

          fs::{self,
               DirEntry,
               ReadDir},

          path::{Path,
                 PathBuf},

          process,
};


mod command_args;


#[derive(Debug)]
pub struct MyFile<'p> {
    pub path: &'p Path,
    pub output: String,
    pub rename_status: bool,
}


pub trait Print {
    fn print_debug(&self);
    
    fn print(&self);
}


impl Print for MyFile<'_> {
    fn print_debug(&self) {
        println!("\n  {:?}", self);
    }

    fn print(&self) {
        println!("\n  {p:?}\n  {o}\n  {r}",
                 p=self.path,
                 o=self.output,
                 r=self.rename_status,
        );
    }
}


impl <'p> MyFile<'_> {
    pub fn new(
        path: &'p Path, 
        output: String,
        rename_status: bool) -> MyFile<'p> {
        
        MyFile {
            path,
            output,
            rename_status,
            ..MyFile::default()
        }
    }

    pub fn default() -> MyFile<'p> {
        MyFile {
            path: Path::new(""),
            output: String::from(""),
            rename_status: false,
        }
    }
}


fn verify_path(path: &str) {
    if !Path::new(path).exists() {
        eprintln!("\nEXIT: PATH does not exists or no access {}", path);
        
        process::exit(1);
    };
}


fn rename_file(input: PathBuf,
               output: PathBuf,
               simulate: bool) {

    // DEBUG
    println!("  @ WILL RENAME:    {:?} -> {:?}",
             input,
             output,
    );

    // VERIFY NOT TO OVERWRITE if any
    if !output.as_path().exists() {
        if !simulate {
            // RENAME
            match fs::rename(input, output) {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("\nERROR: RENAME\nREASON >>> {}", err);
                }
            }
        }
    } else {
        println!("  @ WARNING: destination file exists {:?}", output);
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

    // REMOVE DIACRITICS
    let no_dia = remove_diacritics(text);

    // REPLACE OTHER THEN: 09AZaz AND upper to lower_case
    no_dia
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
                               Some(n) => n.to_string(),
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
                flag_simulate: bool,
                substitute_char: char) {

    // FULL_PATH
    let path = entry.path();

    // Struct
    let mut file = MyFile {path: &path, ..MyFile::default()};

    // SPLIT NAME + EXT -> remove dia + replace non 09AZaz
    file.output = parse_file(&file.path,
                             substitute_char);

    // TEST FOR substitute_char duplicity
    if file.output
        .contains(
            &format!("{ch}{ch}",
                     ch=substitute_char,
            )) {

            file.output = remove_duplicity(&file.output,
                                           substitute_char,
            );
        }

    // if RENAME is needed
    file.rename_status = file.output != format!("{}",
                                                path
                                                .file_name()
                                                .unwrap()
                                                .to_str()
                                                .unwrap(),
    );
            
    file.print_debug();
    
    let new_file: PathBuf = [file.path // DIR
                             .parent()
                             .unwrap(),
                             
                             Path::new(&file.output), // FILENAME
    ]
        .iter()
        .collect();
    
    // RENAME TASK
    if file.rename_status {
        rename_file(file.path.to_path_buf(), // IN
                    new_file, // OUT
                    flag_simulate // SIMULATE
        );
    };
}


fn parse_dir(dir: ReadDir,
             simulate_flag: bool,
             substitute_char: char) {
    
    for element in dir {
        match element // DirEntry
            .as_ref()
            .unwrap() // NOT SAFE
            .metadata()
            .unwrap() // NOT SAFE
            .is_file() {
                
                // FILE
                true => {
                    match &element {

                        Ok(e) => {
                            normalize_chars(&e,
                                            simulate_flag,
                                            substitute_char,
                            )
                        },

                        Err(err) => {
                            eprintln!("\nERROR: element: {:?}\n>>> Reason: {}",
                                      element,
                                      err,
                            );
                        }
                    
                    }
                    /*
                    element
                        .map(|f| normalize_chars(&f,
                                              simulate_flag,
                                              substitute_char,
                        ));
                    */
                },
                
                // DIR
                false => {
                    // /* // HARDCODER -> FUTURE as CmdArg
                    // RECURSE PARSE DESCENDANT DIR
                    match &element {

                        Ok(e) => parse_dir(list_dir(Path::new(&e.path())),
                                           simulate_flag,
                                           substitute_char),
                        Err(err) => {
                            eprintln!("\nERROR: element: {:?}\n>>> Reason: {}",
                                      element,
                                      err,
                            );
                        }
                    };

                    /*    
                    parse_dir(list_dir(Path::new(&element.unwrap().path())),
                              simulate_flag,
                              substitute_char);
                    */
                    // */
                }
            }
    }
}


fn list_dir(path: &Path) -> ReadDir {
    println!("\n>>> DIR_PATH: {}", path.display());

    /*
    let dir = fs::read_dir(&path).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem reading directory\nREASON >>> {}", err);
        
        process::exit(1); // do i need this or just want it ?
    });
    */

    let dir = path
        .read_dir() // INSTEAD fs::read_dir(&path)
        .unwrap_or_else(|err| {
            eprintln!("\nEXIT: Problem reading directory\nREASON >>> {}", err);
        
            process::exit(1); // do i need this or just want it ?
        });
    
    dir
}


fn main() {
    // HARDCODED for now
    let substitute_char = '_';

    /*
    // CWD instead ARG `pwd`
    let path_dir = env::current_dir().unwrap();
    */

    // CMD ARGS: simulate flag + work dir path
    let args = command_args::CmdArgs::new(env::args()).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err);
        
        process::exit(1);
    });

    // VERIFY WORK PATH
    verify_path(&args.full_path);
    
    // DEBUG
    println!("\n#WORK_DIR: {}\n#SIMULATE FLAG: {}",
             args.full_path,
             args.simulate,
    );

    // START WITH ARG DIR
    parse_dir(list_dir(Path::new(&args.full_path)),
              args.simulate,
              substitute_char);
}


#[cfg(test)]
mod tests {
    use super::*;

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
    fn replace_char_name(){
        assert_eq!(replace_char_to_substitute(&String::from("múčho můřká.áchjó.škýt"),
                               '*'),

                   String::from("mucho*murka*achjo*skyt"),
        )
    }
}
