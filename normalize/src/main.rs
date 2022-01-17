use std::{env,
          fs::{self,
               DirEntry},
          path::{Path,
                 PathBuf},
};


// https://github.com/YesSeri/diacritics/blob/main/src/lib.rs
fn remove_diacritics(text: &str) -> String {
    let no_diacritics: String = text
        
        .chars()

        .map(|ch| String::from(ch))
        .collect::<Vec<String>>()
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
                   output: PathBuf) {
    println!("  @ WILL RENAME:    {:?} -> {:?}",
             input,
             output,
    );

    if !output.as_path().exists() {
        // RENAME
        //fs::rename(input, output);
    } else {
        println!("  @ WARNING: exists {:?}", output);
    }
    
}


fn remove_duplicity(s: &String) -> String {
    let mut uniq = String::from("");
    let mut under_score_counter = 0;
    
    for ch in s.as_bytes().iter() {
        let cha = &(*ch as char).to_string();

        if cha == &String::from("_") {
            under_score_counter += 1
        } else {
            under_score_counter = 0
        }

        //println!("CHA: {}", under_score_counter);

        if under_score_counter <= 1 {
            uniq += cha
        }
    }
    
    uniq
}


//fn under_score(s: String) -> String {
fn under_score(s: &String) -> String {
//fn under_score(s: &str) -> &str {

    // REMOVE DIACRITICS
    let no_dia = remove_diacritics(s);

    // REPLACE other then AZaz09
    no_dia
        .as_bytes()
        .iter()
        .map(|b| match b {
            low @ 97..=122 => (*low as char),
            
            high @ 65..=90 => (high.to_ascii_lowercase() as char),

            num @ 48..=57 => (*num as char),
            
            _ => '_',
        }
        )
        // .map(|s| s.to_string()) // no need 
        //.map(|s| s as Box<str>)

        .collect::<String>()
        //.collect::<&str>()
}


//fn replace_char(filename: &String) {
fn replace_char(filename: &DirEntry) {
    let file = filename.file_name();

    let path = Path::new(&file);

    // FILE: NAME
    let name = under_score( & match &path
                              .file_stem()
                              .and_then(|s| s.to_str()) {
                                  Some(n) => n.to_string(),
                                  None => String::from("")
                              }
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

    let uniq_output = remove_duplicity(&output);

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
        rename_filename(filename.path(),
                        new_file,
        );
    };
}


// FUTURE USE
#[allow(dead_code)]
fn dir_work(dir: &DirEntry) {
    println!("#DIR: {:?}",
             dir,
    );
}


#[allow(unused_must_use)]
fn main() {
    println!("#NORMALIZE\n");

    let path_dir = env::current_dir().unwrap();

    println!("#CURRENT DIR: {}\n",
             path_dir
             .display()
    );


    let dir_list = fs::read_dir(path_dir);

    dir_list
        .unwrap()

        //.count()
        
        /*
        .map(|e| e)
        .collect::<Vec<_>>()
        */
        
        .map(|element| element

             .as_ref().unwrap() // when need access to ELEMENT in another map()

             .metadata()
             .map(|m| match m.is_dir() {
                 // DIR
                 true => String::from(""),

                 /* // FUTURE USE 
                 format!("DIR: {:#?}",
                 //"true",
                 element
                 //.unwrap()
                 //.path(),
                 .map(|name| dir_work(&name))
                 ),
                 */

                 // FILE
                 false => format!("FILE: {:?}",
                                  element
                                  //.unwrap()
                                  //.file_name()
                                  //.into_string()
                                  //.unwrap()
                                  .map(|name| replace_char(&name))
                                  //.to_uppercase(),
                 ),
             }
             )
             .unwrap()
             
        )
        .collect::<Vec<_>>()
        ;
}
