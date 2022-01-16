use std::{env,
          fs::{self,
               DirEntry},
          path::{Path,
                 PathBuf},
};


#[allow(unused_must_use)]
fn rename_filename(input: PathBuf,
                   output: PathBuf) {
    println!("  @ WILL RENAME: {:?} -> {:?}",
             input,
             output,
    );

    if !output.as_path().exists() {
        // RENAME
        //fs::rename(input, output);
    } else {
        println!("  WARNING: exists {:?}", output);
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
    s
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

    // NAME
    let name = under_score( & match &path
                              .file_stem()
                              .and_then(|s| s.to_str()) {
                                  Some(n) => n.to_string(),
                                  None => "".to_string()
                              }
    );

    // EXTENSION
    let extension = match &path
        .extension()
        .and_then(|s| s.to_str()) {
            Some(e) => {
                format!(".{}", e)
            },
            None => "".to_string()
        };

    let output = format!("{}{}",
                         name,
                         extension,
    );

    let uniq_output = remove_duplicity(&output);

    let rename_status = format!("{}", path.display()) != uniq_output;

    // PathBuf
    let new_file: PathBuf = [filename.path().parent().unwrap(),
                             Path::new(&uniq_output),
    ].iter().collect();

    /*
    let new_file = format!("{:?}",
                           filename
                           .path()
                           .parent()
                           .unwrap()
                           .join(&uniq_output),
    );
    */

    println!("\n #FILENAME:\n  in: {:?} -> {}\n  out: NAME.EXT -> {}\n  uniq:{}{}\n  rename_status:{}{}\n   {:?}",
             // IN
             filename.path(), // FULL_PATH
             path.display(), // JUST FILENAME

             // OUT
             &output,

             format!("{:>12}", ""),
             uniq_output,

             format!("{:>3}", ""),
             //format!("{}", path.display()) == uniq_output,
             rename_status,

             new_file,
             
             /*
             Path::new(&filename.file_name())
             .extension()
             .and_then(|s| s.to_str())
             //.unwrap()
             .unwrap_or_else(|| {
                 println!("\nERROR: {c}",
                           c=Path::new(&filename.file_name()).display()
                 );
                           
                 ""
             })
             */
             

             // OUT -> EXTENSION
             /*
             under_score(
                 Path::new(&filename.file_name())
                     .extension()
                     .and_then(|s| s.to_str()) //.and_then(OsStr::to_str)
                     .unwrap()
                     .to_string()
             ),
             */

             /*
             filename // split to "name" "." "extension" and join back
             //.file_type()
             .file_name()
             */

             /* OK
             .into_string()
             .unwrap()

             .as_bytes()
             .iter()
             .map(|b| match b {
                 low @ 97..=122 => (*low as char)
                     .to_string(),

                 high @ 65..=90 => (high.to_ascii_lowercase() as char)
                     .to_string(),
                 
                 _ => "_".to_string()
             }
             )
             .collect::<String>()
             */
             
             /*
             .chars()
             .into_iter()
             .map(|ch| ch
                  .to_string()
                  //.to_uppercase()
                  //.map(|m| match )
             )
             .collect::<String>()
             */
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

    dir_list // Result
        .unwrap()
    //.count() // 4

    /* // Ok(DirEntry)
        .map(|e| e)
        .collect::<Vec<_>>()
    */
        
        .map(|element| element
             .as_ref().unwrap() // when i need to access ELEMENT in another map()
             //.path() // full_path
             //.file_name() // just file_name
             .metadata()
             .map(|m| match m.is_dir() {
                 // DIR
                 true =>
                     
                     "".to_string(),
                 
                 /*
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
                                  //"false",
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
