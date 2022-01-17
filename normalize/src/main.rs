use std::{env,
          fs::{self,
               DirEntry},
          path::{Path,
                 PathBuf},
};

use std::process;

mod command_args;


fn verify_path(path: &String) {
    if !Path::new(&path).exists() {
        eprintln!("\nEXIT: PATH does not exists {}", path);
        
        process::exit(1);
    };
}


// https://github.com/YesSeri/diacritics/blob/main/src/lib.rs
fn remove_diacritics(text: &str) -> String {
    let no_diacritics: String = text
        
        .chars()

        .map(|ch| String::from(ch))
        .collect::<Vec<_>>() // <Vec<String>>
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
        .collect();
        
    no_diacritics
}


#[allow(unused_must_use)]
fn rename_filename(input: PathBuf,
                   output: PathBuf,
                   simulate: bool) {

    println!("  @ WILL RENAME:    {:?} -> {:?}",
             input,
             output,
    );

    if !output.as_path().exists() {
        if !simulate {
            // RENAME
            fs::rename(input, output);
        }
    } else {
        println!("  @ WARNING: exists {:?}", output);
    }
    
}


fn remove_duplicity(text: &String,
                    character: char) -> String {
    
    let mut uniq = String::from("");
    let mut character_counter = 0;
    
    for ch in text.as_bytes().iter() {
        let cha = &(*ch as char).to_string();

        if cha == &String::from(character) {
            character_counter += 1
        } else {
            character_counter = 0
        }

        if character_counter <= 1 {
            uniq += cha
        }
    }
    
    uniq
}


fn under_score(text: &String,
               character: char) -> String {

    // REMOVE DIACRITICS
    let no_dia = remove_diacritics(text);

    // REPLACE other then 09AZaz
    no_dia
        .as_bytes()
        .iter()
        .map(|b| match b {
            low @ 97..=122 => (*low as char),
            
            high @ 65..=90 => (high.to_ascii_lowercase() as char),

            num @ 48..=57 => (*num as char),
            
            _ => character,
        })
        .collect::<String>()
}


fn replace_char(filename: &DirEntry,
                flag_simulate: bool) {
    
    let file = filename.file_name();

    let path = Path::new(&file);

    // FILE: NAME
    let name = under_score( & match &path
                              .file_stem()
                              .and_then(|s| s.to_str()) {
                                  Some(n) => n.to_string(),
                                  None => String::from("")
                              },

                              '_',
    );

    // FILE: EXTENSION
    let extension = match &path
        .extension()
        .and_then(|s| s.to_str()) {
            Some(e) => {
                format!(".{}", remove_diacritics(e))
            },
            None => String::from("")
        };

    let output = format!("{}{}",
                         name,
                         extension,
    );

    let uniq_output = remove_duplicity(&output, '_');

    let rename_status = format!("{}", path.display()) != uniq_output;

    // PathBuf
    let new_file: PathBuf = [
        filename
            .path()
            .parent()
            .unwrap(),
        
        Path::new(&uniq_output),
    ]
        .iter()
        .collect();

    println!("\n #FILENAME:\n  {} -> file_name: {}\n  out: NAME.EXT ->  {}\n  uniq:{}{}\n  rename_status:{}{}  {}",
             // IN
             format!("full_path: {:>7}{:?}",
                     "",
                     filename.path(), // FULL_PATH
                     ),
             
             
             path.display(), // JUST FILENAME

             // OUT
             &output,

             format!("{:>13}", ""),
             uniq_output,

             format!("{:>4}", ""),

             rename_status,

             if rename_status {
                 format!("\n  target: {:>10}{:?}",
                         "",
                         new_file,
                 )
             } else { String::from("") },
             
    );

    if rename_status {
        rename_filename(filename.path(), // IN
                        new_file, // OUT

                        //true // false // SIMULATE
                        flag_simulate // false // SIMULATE
        );
    };
}


// FUTURE USE
#[allow(dead_code)]
fn dir_work(dir: &DirEntry) {
    println!("\n #DIR: {:?}",
             dir,
    );
}


#[allow(unused_must_use)]
fn main() {
    /*
    // CWD instead ARG `pwd`
    let path_dir = env::current_dir().unwrap();
    */

    // CMD ARG
    let args = command_args::CmdArgs::new(env::args()).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem parsing arguments\nREASON >>> {}", err);
        
        process::exit(1);
    });

    // VERIFY PATH IS VALID
    verify_path(&args.full_path);
    
    println!("\n#WORKING DIR: {}\n#SIMULATE: {}",
             args.full_path,
             args.simulate,
    );

    // DIR LIST
    let dir_list = fs::read_dir(args.full_path);

    dir_list
        .unwrap() // is safe ?

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
                         ))
                         .unwrap(); // make it safe !!!
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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diacritics_with_no_extension(){
        assert_eq!(remove_diacritics("tRPaslÍČek"),

                   String::from("tRPaslICek"),
        )
    }

    #[test]
    fn diacritics(){
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
    fn under_score_name(){
        assert_eq!(under_score(&String::from("múčho můřká.áchjó.škýt"),
                               '_'),

                   String::from("mucho_murka_achjo_skyt"),
        )
    }
}
