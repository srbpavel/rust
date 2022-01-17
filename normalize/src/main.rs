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
pub struct MyFile {

    //pub input: DirEntry,

    //pub path: &Path,
    
    //pub name: String,
    //pub extension: String,

    pub output: String,
    
    pub uniq_output: String,
    pub rename_status: bool,
    
    //pub new_file: PathBuf,
}


pub trait Print {
    fn print(&self);
}


impl Print for MyFile {
    fn print(&self) {
        println!("\n  TRAIT Print>>> {:?}", self);
    }
}

impl MyFile {
    pub fn new(
        //input: DirEntry,

        //path: &Path, 

        //name: String,
        //extension: String

        output: String,
        
        uniq_output: String,
        rename_status: bool,
    ) -> MyFile {
        
        MyFile {
            //input,

            //path,

            //name,
            //extension,

            output,
            
            uniq_output,
            rename_status,
            //..MyFile::default()
        }
    }

    /*
    pub fn default() -> MyFile {
        MyFile {
            //input: String::from(""),

            path: Box<Path::new()>,
            
            //name: String::from(""),
            //extension: String::from(""),
            
            uniq_output: String::from(""),
            rename_status: false,
            
            //new_file: PathBuf,
        }
    }
    */

    /*
    pub fn import_lp<'a>(&self,
                         config: &TomlConfig) {
        
        import_lp_via_curl(config,
                           &self)
    }
    */

}


fn parse_file(path: &Path,
              replace_character: char) -> String {
    
    // FILE: NAME -> diacritics + replace
    let name = under_score(& match &path
                           .file_stem()
                           .and_then(|s| s.to_str()) {
                               Some(n) => n.to_string(),
                               None => String::from("")
                           },
                           
                           replace_character,
    );
    
    // FILE: EXTENSION -> only diacritics
    let extension = match &path
        .extension()
        .and_then(|s| s.to_str()) {
            Some(e) => {
                format!(".{}", remove_diacritics(e))
            },
            None => String::from("")
        };

    // JOIN FILENAME FROM name + extension
    let output = format!("{}{}",
                         name,
                         extension,
    );

    output
}



fn parse_dir(dir: ReadDir,
             args: command_args::CmdArgs,
             replace_character: char) {
    
    for element in dir {
        match element // DirEntry
            .as_ref()
            .unwrap()
            .metadata()
            .unwrap()
            .is_file() {
                
                // FILE
                true => {
                    element
                        .map(|f| replace_char(&f,
                                              args.simulate,
                                              replace_character,
                        ))
                        .unwrap(); // rather verify
                },
                
                // DIR -> FUTURE USE
                false => {
                    /* 
                    element
                    .map(|d| dir_work(&d));
                     */
                }
            }
    }
    
    /*
    dir_list
        /* // DEBUG
        .map(|e| e)
        .collect::<Vec<_>>()
        */
        
        .map(|element| element
             .as_ref().unwrap() // when need access to ELEMENT in another map()
             .metadata()
             .map(|m| match m.is_file() {
                 true => {
                     element
                         .map(|name| replace_char(&name,
                                                  args.simulate,
                                                  replace_character,
                         ))
                         .unwrap(); // rather verify
                 },
                 
                 // DIR -> FUTURE USE
                 false => {
                     /* 
                     element
                         .map(|name| dir_work(&name));
                     */
                 }
             })
        )
        .collect::<Vec<_>>();
    */
}


fn verify_path(path: &str) {
    if !Path::new(path).exists() {
        eprintln!("\nEXIT: PATH does not exists or no access {}", path);
        
        process::exit(1);
    };
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


fn rename_file(input: PathBuf,
               output: PathBuf,
               simulate: bool) {
    
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
                    character: char,
                    path: &Path) -> (String, bool) {
    
    let mut uniq = String::from("");
    let mut character_counter = 0;

    for byte in text.as_bytes().iter() {
        let cha = &String::from(*byte as char);

        if cha == &String::from(character) {
            character_counter += 1
        } else {
            character_counter = 0
        }

        if character_counter <= 1 {
            uniq += cha
        }
    }

    // STATUS if rename is needed
    let rename_status = format!("{}", path.display()) != uniq;
    
    (uniq, rename_status)
}


fn under_score(text: &str,
               character: char) -> String {

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
            
            _ => character,
        })
        .collect::<String>()
}


fn replace_char(entry: &DirEntry,
                flag_simulate: bool,
                replace_character: char) {

    // BARE FILE_NAME
    let file_original = entry.file_name(); //OsString

    let path = Path::new(&file_original);

    let output = parse_file(path,
                            replace_character);

    // REMOVE DUPLICITY of multiple replace_character
    let (uniq_output, rename_status) = remove_duplicity(&output,
                                                        replace_character,
                                                        path,
    );

    let new_file: PathBuf = [
        // PATH
        entry
            .path()
            .parent()
            .unwrap(),

        // FILENAME
        Path::new(&uniq_output),
    ]
        .iter()
        .collect();

    
    // Struct
    let my_file = MyFile {output: output,

                          uniq_output: uniq_output,
                          rename_status: rename_status,
    };
    
    my_file.print();
    
    // DEBUG just to see -> change to silent / verbose
    println!("\n #FILENAME:\n  {} -> file_name: {}\n  out: NAME.EXT ->  {}\n  uniq:{}{}\n  rename_status:{}{}  {}",
             // FULL_PATH
             format!("entry full_path: {:>1}{:?}",
                     "",
                     entry.path(),
                     ),
             
             // FILENAME
             path.display(),

             // NAME.EXT
             //&output,
             &my_file.output,
             
             // UNIQ
             format!("{:>13}", ""),
             //uniq_output,
             &my_file.uniq_output,

             format!("{:>4}", ""),

             // STATUS
             //rename_status,
             &my_file.rename_status,

             // INPUT -> OUTPUT
             //if rename_status {
             if my_file.rename_status {
                 format!("\n  target: {:>10}{:?}",
                         "",
                         new_file,
                 )
             } else { String::from("") },
             
    );

    // RENAME TASK
    //if rename_status {
    if my_file.rename_status {
        rename_file(entry.path(), // IN
                    new_file, // OUT
                    flag_simulate // SIMULATE
        );
    };
}


// FUTURE USE
/*
fn dir_work(dir: &DirEntry) {
    println!("\n #DIR: {:?}",
             dir,
    );
}
*/


fn main() {
    let replace_character = '_';

    /*
    // CWD instead ARG `pwd`
    let path_dir = env::current_dir().unwrap();
    */

    // CMD ARGS: simulate flag + work dir path
    let args = command_args::CmdArgs::new(env::args()).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err);
        
        process::exit(1);
    });

    // VERIFY PATH
    verify_path(&args.full_path);
    
    // DEBUG
    println!("\n#WORKING DIR: {}\n#SIMULATE FLAG: {}",
             args.full_path,
             args.simulate,
    );

    // READ DIR
    let dir = fs::read_dir(&args.full_path).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem reading directory\nREASON >>> {}", err);
        
        process::exit(1);
    });

    // DIR WORK
    parse_dir(dir,
              args,
              replace_character);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_diacritics(){
        assert_eq!(remove_diacritics("tRPaÍČek"),

                   String::from("tRPaslICek"),
        )
    }

    // OBSOLETE as file_name and file_extension are diveded
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
    fn replace_character_name(){
        assert_eq!(under_score(&String::from("múčho můřká.áchjó.škýt"),
                               '*'),

                   String::from("mucho*murka*achjo*skyt"),
        )
    }
}
