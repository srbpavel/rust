extern crate easy_config;
extern crate serde;

mod example_config;
mod example_fill_toml_config_struct;
mod util;

use example_fill_toml_config_struct::TomlConfig;
use template_formater::tuple_formater;
use std::{
    io::{
        Write,
        Error,
    },
    path::{
        Path,
        PathBuf,
    },
    fs::File,
};

use uuid::Uuid;


struct Video {
    name: String,
    filename: String,
    player_url: String,
    upload_url: String,
    video_id: uuid::Uuid,
}


/// list dir and files
fn get_files(config: &TomlConfig) -> Result<Vec<PathBuf>, Error> {

    // VIDEO_DIR
    let video_dir_path = match Path::new(&*config.video_dir)
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
    let read_dir = match video_dir_path.read_dir() {
        Ok(d) => Some(d),
        Err(_) => None,
    };
        
    let mut video_files = Vec::new();

    // GET READY ALL VIDEO FILES
    if let Some(dir) = read_dir {
        for element in dir {
            match element
                .as_ref()
                .map(|e| match e.metadata() {
                    Ok(m) => format!("{}", m.is_file()),
                    Err(err) => {
                        eprintln!("ERROR: element METADATA: {:?}\nREASON >>> {}",
                                  element,
                                  err,
                        );
                        
                        String::from("")
                    },
                }) {
                    Ok(n) => {
                        // FILE
                        if n.as_str().eq("true") {
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
    };

    Ok(video_files)
}


/// single curl video upload
async fn run_upload(config: &TomlConfig,
                    video: Video) -> Result<String, Box<dyn std::error::Error>> {

    // HTML
    let video_path = format!("{}/{}",
                             &video.player_url,
                             video.video_id
    );

    let video_tag = tuple_formater(
        &config.video_tag,
        &vec![
            ("name", &video.name),
            ("width", &config.player_width),
            ("src", &video_path),
        ],
        config.flag.debug_template,
    );
    
    let html_code = tuple_formater(
        &config.html_template,
        &vec![
            ("all_videos", &video_tag),
        ],
        config.flag.debug_template,
    );

    let mut html_file = File::create(
        format!("{}{}_{}.html",
                &*config.html_path,
                &video.video_id,
                &video.name,
        )
    ).unwrap();
    
    html_file.write_all(
        html_code.as_bytes()
    )?;
    
    // UPLOAD
    let mut cmd = async_process::Command::new("curl");
    
    cmd.arg("-X")
        .arg("PUT")
        .arg("-H")
        .arg("Content-type: multipart/form-data")
        .arg(video.upload_url)
        .arg("-F")
        // type hardcoded as all mp4
        .arg(
            &format!("{name}=@{filename};type=video/mp4",
                     name = video.name,
                     filename = video.filename,
            )
        )
        .arg("-H")
        .arg(
            &format!("video_id: {}",
                     video.video_id,
            )
        )
        .arg("-H")
        .arg(
            &format!("group: {}",
                     &config.video_group,
            )
        );
    
    println!("#CMD: {:?}", cmd);

    let _output = cmd.output().await?;
    //println!("#OUTPUT: {_output:#?}");
    println!("#UPLOADED: {}", video.filename);   

    Ok(video_tag)
}


#[async_std::main]
async fn main() -> std::io::Result<()> {
    // COMMAND ARGS
    let config_file = util::prepare_config(std::env::args()).unwrap_or_else(|err| {
        eprintln!("\nEXIT: Problem parsing cmd arguments\nREASON >>> {}", err);
        std::process::exit(1);
    });
    
    // CONFIG
    let config = example_config::sample_config(&config_file);
    
    let machine = format!("{secure}://{server}:{port}",
                          secure=config.secure,
                          server=config.server,
                          port=config.port,
                          );

    let upload_url = format!("{machine}{path}",
                             path=config.upload_path,
    );

    let player_url = format!("{machine}{path}",
                             path=config.player_path,
    );

    let video_files = get_files(&config)?;
    
    let video_files_sample = match &config.sample_limit_end {
        -1 => video_files,
        n @ 1.. => {
            video_files[
                (config.sample_limit_start as usize)..(*n as usize)
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
    
    let binary_paths: Vec<_> = video_files_sample
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
            
            let video = Video {
                name: name.to_string(),
                filename: filename.to_string(),
                player_url: player_url.to_string(),
                upload_url: upload_url.to_string(),
                video_id: Uuid::new_v4(),
            };

            run_upload(&config,
                       video,
            )
        })
        .collect();

    let cmd_results = futures::future::join_all(binary_paths).await;

    cmd_results
        .iter()
        .for_each(|r|
                  if r.is_err() {
                      println!("{r:?}");
                  }
        );
    
    Ok(())
}

