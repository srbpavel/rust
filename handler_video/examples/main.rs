extern crate easy_config;
extern crate serde;

mod example_config;
mod example_fill_toml_config_struct;
mod util;

fn main() {
    // COMMAND ARGS
    let config_file = util::prepare_config(std::env::args()).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem parsing cmd arguments\nREASON >>> {}", err);
        std::process::exit(1);
    });
    
    // CONFIG
    let config = example_config::sample_config(&config_file);

    println!("EXAMPLE_CONFIG: {:#?}",
             config,
    );

    let upload_url = format!("{secure}://{server}:{port}{path}",
                             secure=config.secure,
                             server=config.server,
                             port=config.port,
                             path=config.upload_path,
    );

    println!("UPLOAD_URL: {upload_url}");
        
    // VIDEO_DIR
    let video_dir_path = match std::path::Path::new(&*config.video_dir)
        .canonicalize() {
            Ok(path) => path.to_path_buf(),
            Err(why) => {
                eprintln!("\nEXIT: path does not exists: {}\nREASON >>> {}",
                          &config.video_dir,
                          why,
                );
                
                std::process::exit(1);
                
            }
        };

    println!("PATH: {video_dir_path:?}");

    // LIST
    let read_dir = match video_dir_path.read_dir() { // ReadDir
        Ok(d) => Some(d),
        Err(_) => None,
    };

    let mut video_files = Vec::new();
    
    match read_dir {
        Some(dir) => {
            for element in dir {
                match element
                    .as_ref()
                    .and_then(|e| Ok(match e.metadata() {
                        Ok(m) => format!("{}", m.is_file()),
                        Err(err) => {
                            eprintln!("ERROR: element METADATA: {:?}\nREASON >>> {}",
                                      element,
                                      err,
                            );

                            String::from("")
                        },
                    })) {
                        Ok(n) => {
                            match n.as_str() {
                                // FILE
                                "true" => {
                                    match &element {
                                        Ok(file) => {
                                            let file_path = file.path();
                                            
                                            //println!("FILE_PATH: {file_path:?}");

                                            video_files.push(file_path);
                                        },
                                        Err(err) => {
                                            eprintln!("ERROR: FILE element: {:?}\n REASON >>> {}",
                                                      element,
                                                      err,
                                            )  
                                        },
                                    }
                                },
                                // DIR
                                "false" => {},
                                _ => {},
                            }
                        },
                        Err(err) => {
                            eprintln!("ERROR: element: {:?}\nREASON >>> {}",
                                      element,
                                      err,
                            );
                        },
                    }
            }
        }
        None => {},
    }

    // 
    //println!("FILES: {:?}", video_files);

    let _curl_list = video_files
        .iter()
        .map(|f| {

            let filename = match &f.to_str() {
                Some(s) => s,
                None => "",
            };

            let name = match &f.file_stem() {
                Some(n) => n.to_str().unwrap(),
                None => "",
            };

            let video_id = uuid::Uuid::new_v4();

            /*
            let curl = format!("curl -X PUT -H \"Content-type: multipart/form-data\" \"{url}\" -F \"{name}=@{filename};type=video/mp4\" -H \"video_id: {video_id}\" -H \"group: {group}\"",
                               url = upload_url, 
                               group = "youtube",
            );

            println!("{:?}", curl);
            */
            
            let mut cmd = std::process::Command::new("curl");

            cmd.arg("-X")
                .arg("PUT")
                .arg("-H")
                .arg("Content-type: multipart/form-data")
                .arg(&upload_url)
                .arg("-F")
                .arg(
                    &format!("{name}=@{filename};type=video/mp4")
                )
                .arg("-H")
                .arg(
                    &format!("video_id: {video_id}")
                )
                .arg("-H")
                .arg("group: youtube")
                //.output()
                //.unwrap()
                ;

            println!("\n\nCMD: {:?}", &cmd);
            let output = &cmd.output();
            //let output_result = &output.unwrap();

            //println!("CMD: {:?}\nSTDOUT: {}\nSTDERR: {}",
            println!("OUTPUT: {:?}",
                     output, //cmd,
                     /*
                     String::from_utf8(cmd
                                       //.unwrap()
                                       .stdout
                                       .to_vec()
                     )
                     .unwrap(),

                     String::from_utf8(cmd
                                       //.unwrap()
                                       .stderr
                                       .to_vec()
                     )
                     .unwrap(),
                     */
            );

            /*
            match output {
                Ok(o) => o.stdout,
                Err(why) => {
                    eprintln!("CURL ERR: {why:?}");

                    Vec::new()
                },
            }
            */
        })
        .collect::<Vec<_>>();

    //println!("CURL_LIST: {:#?}", curl_list);
    //println!("CURL_OUT: {:#?}", curl_list);
}

