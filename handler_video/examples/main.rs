extern crate easy_config;
extern crate serde;

mod example_config;
mod example_fill_toml_config_struct;
mod util;

use template_formater::tuple_formater;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {
    // COMMAND ARGS
    let config_file = util::prepare_config(std::env::args()).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem parsing cmd arguments\nREASON >>> {}", err);
        std::process::exit(1);
    });
    
    // CONFIG
    let config = example_config::sample_config(&config_file);
    //println!("EXAMPLE_CONFIG: {:#?}", config);

    let upload_url = format!("{secure}://{server}:{port}{path}",
                             secure=config.secure,
                             server=config.server,
                             port=config.port,
                             path=config.upload_path,
    );

    println!("UPLOAD_URL: {upload_url}");
    
    let player_url = format!("{secure}://{server}:{port}{path}",
                             secure=config.secure,
                             server=config.server,
                             port=config.port,
                             path=config.player_path,
    );

    println!("PLAYER_URL: {player_url}");

    println!("HTML: {}",
             &config.html_file,
    );
        
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

    //println!("VIDEO DIR PATH: {video_dir_path:?}");

    // LIST
    let read_dir = match video_dir_path.read_dir() { // ReadDir
        Ok(d) => Some(d),
        Err(_) => None,
    };

    let mut video_files = Vec::new();

    // GET READY ALL VIDEO FILES
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

    //println!("FILES: {:?}", video_files);
    println!("SAMPLE_LIMIT: {}..{}",
             &config.sample_limit_start,
             &config.sample_limit_end,
    );
    
    let video_files_sample = match &config.sample_limit_end {
        -1 => video_files,
        n @ 1.. => {
            video_files[
                (*&config.sample_limit_start as usize)..(*n as usize)
            ].to_vec()
        },
        _ => {
            eprintln!("ERROR: wrong sample limit: start: {} end: {}",
                      &config.sample_limit_start,
                      &config.sample_limit_end,
            );
            std::process::exit(1)
        }
    };
    
    let binary_paths = video_files_sample
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

            let mut cmd = std::process::Command::new("curl");

            cmd.arg("-X")
                .arg("PUT")
                .arg("-H")
                .arg("Content-type: multipart/form-data")
                .arg(&upload_url)
                .arg("-F")
                // type hardcoded as all mp4
                .arg(
                    &format!("{name}=@{filename};type=video/mp4")
                )
                .arg("-H")
                .arg(
                    &format!("video_id: {video_id}")
                )
                .arg("-H")
                .arg("group: youtube");

            println!("\n\n#CMD: {:?}", &cmd);

            let output = &cmd.output();

            println!("#OUTPUT: {output:#?}");

            let video_path = format!("{}/{}",
                                     player_url,
                                     video_id
            );

            let video_tag = tuple_formater(
                &config.video_tag,
                &vec![
                    ("name", &name),
                    ("width", &config.player_width),
                    ("src", &video_path),
                ],
                config.flag.debug_template,
            );

            let single_html_code = tuple_formater(
                &config.html_template,
                &vec![
                    ("all_videos", &video_tag),
                ],
                config.flag.debug_template,
            );      

            let mut single_html_file = std::fs::File::create(
                format!("{}{}_{}.html",
                        &*config.html_path,
                        &video_id,
                        &name,
                )
            ).unwrap();

            single_html_file.write_all(
                &single_html_code.as_bytes()
            ).unwrap();
            
            video_tag
        })
        .collect::<Vec<_>>()
        .concat();

    let html_code = tuple_formater(
        &config.html_template,
        &vec![
            ("all_videos", &binary_paths),
        ],
        config.flag.debug_template,
    );

    let mut html_file = std::fs::File::create(
        format!("{}{}",
                &*config.html_path,
                &*config.html_file,
        )
    )?;

    html_file.write_all(html_code.as_bytes())?;

    Ok(())
}

