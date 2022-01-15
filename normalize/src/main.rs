use std::{env,
          fs::{self,
               DirEntry},
          path::{Path},
};

//use std::ffi::OsStr;


fn under_score(s: String) -> String {
//fn under_score(s: &str) -> &str {
    s
        .as_bytes()
        .iter()
        .map(|b| match b {
            low @ 97..=122 => (*low as char)
                ,//.to_string(),
            
            high @ 65..=90 => (high.to_ascii_lowercase() as char)
                ,//.to_string(),
            
            _ => '_',//.to_string() // "_"
        }
        )
        // .map(|s| s.to_string()) // no need 
        //.map(|s| s as Box<str>)

        .collect::<String>()
        //.collect::<&str>()
}


//fn replace_char(filename: &String) {
fn replace_char(filename: &DirEntry) {
    println!(" #FILENAME:\n  in: {:?}\n  out: NAME.EXT -> {}.{}",
             // IN
             filename.path(),

             // OUT -> NAME
             under_score(
                 Path::new(&filename.file_name())
                     .file_stem()
                     .and_then(|s| s.to_str())
                     .unwrap()
                     .to_string()
             ),
             
             // OUT -> EXTENSION
             under_score(
                 Path::new(&filename.file_name())
                     .extension()
                     .and_then(|s| s.to_str()) //.and_then(OsStr::to_str)
                     .unwrap()
                     .to_string()
             ),

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
}


#[allow(dead_code)]
fn dir_work(dir: &DirEntry) {
    println!("#DIR: {:?}",
             dir,
    );
}


#[allow(unused_must_use)]
fn main() {
    println!("#NORMALIZE\n");

    // read cmd arg

    // loop for all in dir
     // if file -> replace
     // if dir append to dir list
    
    
    // replace_char_in_str


    // get cwd or ARG
    let path_dir = env::current_dir().unwrap();

    println!("#CURRENT DIR: {}\n",
             path_dir
             //.unwrap()
             .display()
    );


    let dir_list = fs::read_dir(path_dir);

    // println!("dir list: {:#?}", // PRINT
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
                      
                      false =>

                      // /*
                          format!("FILE: {:?}",
                                  //"false",
                                  element
                                  //.unwrap()
                                  //.file_name()
                                  //.into_string()
                                  //.unwrap()
                                  .map(|name| replace_char(&name))
                                  //.to_uppercase(),
                          ),
                      // */
                  }
                  )
                  .unwrap()
                  
             )
             .collect::<Vec<_>>()
             ;
    // );
}
