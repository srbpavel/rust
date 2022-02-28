use crate::handler::{
    AppState,
};

use actix_web::{
    get,
    post,
    web,
    Result,
    HttpResponse,
    HttpRequest,
};

use actix_multipart::Multipart;
use futures_util::TryStreamExt;

use std::io::Write;

use serde::{Serialize,
            Deserialize,
};

use std::collections::HashMap;

use std::path::{Path,
                PathBuf,
};

/// storage as fullpath
static STATIC_DIR: &str = "/home/conan/soft/rust/handler_video/storage/";

/// scope
pub const SCOPE: &str = "/video";

/// types for video hash_map
pub type VideoKey = String;
pub type VideoValue = Video;


/// video status
#[derive(Debug)]
pub enum VideoStatus {
    Init,
    Ok,
    EmptyVideoId,
    EmptyGroupId,
    // in case we want to have video_id as filename name -F "video_id=@video.mp4"
    //EmptyName
    EmptyFilename,
    TooManyForms,
}

/// video_status -> msg
///
impl VideoStatus {
    // can have as &str but then full of lifetime, time will proof
    //pub fn as_str(&self) -> &str {
    pub fn as_string(&self) -> String {
        match *self {
            VideoStatus::Init => String::from("Init"),
            VideoStatus::Ok => String::from("Ok"),
            //VideoStatus::EmptyName => String::from("STREAM for filename not provided"),
            VideoStatus::EmptyVideoId => String::from("header 'video_id' not provided"),
            VideoStatus::EmptyGroupId => String::from("header 'group' not provided"),
            VideoStatus::EmptyFilename => String::from("form 'filename' not provided"),

            VideoStatus::TooManyForms => String::from("TOO MANY FORMS we accept only ONE"),
            // curl with now form -F -> Multipart boundary is not found
            // status code 400
            //VideoStatus::EmptyForms => String::from("'form' not provided"),
         }
    }
}


/// video
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Video {
    id: String,
    group: String,
    //path: String,
    path: PathBuf,
}

impl Video {
    /// default
    pub fn default() -> Self {
        Self {
            group: String::from(""),
            id: String::from(""),
            //path: String::from(""),
            path: PathBuf::new(),
        }
    }
}

/// file error // FUTURE USE
#[derive(Serialize, Deserialize)]
struct File {
    //group: String,
    err: String,
}

/// all videos
#[derive(Serialize, Debug)]
pub struct IndexResponse {     
    server_id: usize,      
    request_count: usize,  
    video_map: HashMap<VideoKey, VideoValue>, 
}

/// group members
#[derive(Serialize, Debug)]
pub struct ListResponse {     
    server_id: usize,      
    request_count: usize,  
    result: Option<HashMap<VideoKey, VideoValue>>, 
}

/// upload 
#[derive(Serialize, Debug)]
pub struct PostResponse {
    result: Option<PostOk>,
    status: String,
}

/// valid upload
#[derive(Serialize, Debug)]
pub struct PostOk {
    server_id: usize,
    request_count: usize,
    video: Video,
}

/// invalid upload
#[derive(Serialize)]
struct PostError {
    msg: String,
}

/// detail
#[derive(Serialize, Debug)]                               
pub struct DetailResponse {                                   
    server_id: usize,                                     
    request_count: usize,                                 
    result: Option<Video>, // None in JSON will be "null"
    url: String,
}


/// GET index list all videos
///
/// curl 'http://localhost:8081/video/'
///
pub async fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);          
    
    let video = state                                  
        .video_map
        .lock()                                      
        .unwrap();                                   
    
    Ok(                                              
        web::Json(                                   
            IndexResponse {                          
                server_id: state.server_id,          
                request_count,        
                video_map: video.clone(),
            }                                        
        )                                            
    )                                                
}


/// PUT new video
/// 
/// curl -X PUT 'http://localhost:8081/video/put
///
/// curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F "video_name=@/home/conan/video/youtube/munch_roses_extended_remix.mp4;type=video/mp4" -H "video_id: 789" -H "group: stream_002" 2>/dev/null | jq
///
pub async fn insert_video(mut payload: Multipart,
                          state: web::Data<AppState>,
                          req: HttpRequest) -> Result<web::Json<PostResponse>> {
                          
    // Cell
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    // we lock and have access to HashMap messages
    let mut video_hashmap = state
        .video_map
        .lock() // get access to data inside Mutex + blocks until another thread
        .unwrap(); // -> MutexGuard<Vec<String>> // will panic on Err !!!

    let mut status = VideoStatus::Init.as_string();
    let mut new_video = Video::default();

    //println!("HEADERS: {:?}", req.headers());
    match req.headers().get("video_id") {
        Some(id) => {  // HeaderValue
            new_video.id = id
                .to_str()
                .unwrap() // NOT SAFE
                .to_string();
        },
        None => {
            // curl: (55) Send failure: Connection reset by peer
            // but still we receive JSON response with status
            return Ok(
                web::Json(
                    PostResponse {
                        result: None,
                        status: VideoStatus::EmptyVideoId.as_string(),
                    }
                )
            )
        },
    }

    match req.headers().get("group") {
        Some(group) => {
            new_video.group = group
                .to_str()
                .unwrap() // NOT SAFE
                .to_string()
                
        },
        None => {
            return Ok(
                web::Json(
                    PostResponse {
                        result: None,
                         status: VideoStatus::EmptyGroupId.as_string(),
                    }
                )
            )
        },
    }
    
    //println!("NEW_VIDEO: {new_video:?}");

    // iterate over multipart stream
    // https://actix.rs/actix-web/actix_multipart/struct.Field.html
    let mut content_counter = 0;

    while let Some(mut field) = payload
        .try_next()
        .await? {
            content_counter += 1;

            // we only accept one form with file
            if content_counter == 1 {
                
                let content_disposition = field.content_disposition();
                
                if let Some (dis) = content_disposition {
                    //println!("\ndis: {dis:?}");

                    status = VideoStatus::Ok.as_string();
                    
                    // verify if filename was in form
                    match dis.get_filename() {
                        Some(filename) => {
                            // FOR DOWNLOAD/PLAYER or ... url
                            /*
                            new_video.path = format!("{}{}_{}",
                                                     STATIC_DIR,
                                                     new_video.id,
                                                     filename,
                            );
                            */
                            new_video.path = Path::new(STATIC_DIR)
                                .join(
                                    format!("{}_{}",
                                            new_video.id,
                                            filename,
                                    )
                                );

                            println!("\nFULL_PATHs: {:?}",
                                     new_video.path,
                            );
                            
                            // another clone but WE NEED AT THE very END
                            let filepath = new_video.path.clone();
                            
                            // HASH
                            video_hashmap.insert(
                                new_video.id.clone(), // KEY: video.id
                                new_video.clone(), // VALUE: Video {}
                            );
                            
                            // ### FILE
                            // block -> future to result
                            let mut f = web::block(||
                                                   // we should also verify:
                                                   // we can write
                                                   // have access
                                                   std::fs::File::create(filepath)
                            ).await?;
                            //println!("F: {f:?}");
                            
                            // stream of *Bytes* object
                            while let Some(chunk) = field.try_next().await? {
                                //println!("CHUNK: {:#?}", chunk);
                                f = web::block(move ||
                                               f
                                               .write_all(&chunk)
                                               .map(|_| f)
                                ).await?;
                            };
                        },
                        None => {
                            status = VideoStatus::EmptyFilename.as_string()
                        },
                    }
                };
            } else {
                status = VideoStatus::TooManyForms.as_string()
            }
        }

    Ok(
        web::Json(
            PostResponse {
                result: Some(
                    PostOk {
                        server_id: state.server_id,
                        request_count,
                        video: new_video,
                    }
                ),
                status
            }
        )
    )
}


/// DETAIL via hash
/// 
/// curl 'http://localhost:8081/video/detail/{video_id}'
///
#[get("/detail/{video_id}")]
pub async fn detail(state: web::Data<AppState>,
                    idx: web::Path<String>) -> Result<web::Json<DetailResponse>> {

    let to_parse_idx = idx.into_inner();

    let path = format!("{}/detail/{}",
                       SCOPE,
                       to_parse_idx,
    );

    let parsed_idx = match to_parse_idx.parse::<String>() {
        Ok(i) => {
            Some(i)
        },
        Err(why) => {
            eprintln!("foookin INDEX: {to_parse_idx}\nREASON >>> {why}");

            None
        },
    };

    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let video = state
        .video_map
        .lock()
        .unwrap();

    let result = match parsed_idx {
        Some(i) => {
            video.get(&i).map(|v| Video {
                id: i,
                group: v.group.to_string(),
                //path: v.path.to_string(),
                path: v.path.clone(),
            })
        },
        None => None,
    };
    
    Ok(
        web::Json(
            DetailResponse {
                server_id: state.server_id,
                request_count,
                result,
                url: path,
            }
        )
    )
}


/// DOWNLOAD via hash
/// 
/// curl 'http://localhost:8081/video/download/{id}'
///
#[get("/download/{idx}")]
pub async fn download(state: web::Data<AppState>,
                      idx: web::Path<String>) -> HttpResponse {

    let to_parse_idx = idx.into_inner();

    let parsed_idx = match to_parse_idx.parse::<String>() {
        Ok(i) => {
            Some(i)
        },
        Err(why) => {
            eprintln!("foookin INDEX: {to_parse_idx}\nREASON >>> {why}");

            None
        },
    };

    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let video = state
        .video_map
        .lock()
        .unwrap();

    // join these two together
    let result = match parsed_idx {
        Some(i) => {
            video.get(&i).map(|v| Video {
                id: i.to_string(),
                group: v.group.to_string(),
                //path: v.path.to_string(),
                path: v.path.clone(),
            })
        },
        None => None,
    };

    match result {
        Some(v) => {
            /*
            let content = format!("form-data; filename={:?}",
                                  //v.path,
                                  v.path.to_str(),
            );
            */

            // same as .exists() -> bool but here -> Result
            // also durring reading io::ErrorKind::Interrupted
            match std::fs::read(v.path.clone()) {
                Ok(data) => {
                    let content = format!("form-data; filename={}",
                                          match v.path.to_str() {
                                              Some(p) => p,
                                              // should not occure?
                                              None => "",
                                          },
                    );

                    println!("CONTENT: {content:?}");
                    
                    HttpResponse::Ok()
                        .header("Content-Disposition",
                                content,
                        )
                        .body(data)
                },
                Err(why) => {
                    HttpResponse::NotFound().json(
                        &File {
                            //err: "path does not exists".to_string(),
                            err: format!("{why:?}")
                        }
                    )
                },
            }
            
            /*
            let video_path = std::path::Path::new(&v.path); 

            if video_path.exists() {
                let data = std::fs::read(video_path)
                    .unwrap(); // NOT SAFE will panic!

                let content = format!("form-data; filename={}",
                                      v.path,
                );
                
                HttpResponse::Ok()
                    .header("Content-Disposition",
                            content,
                    )
                    .body(data)
            } else {
                HttpResponse::NotFound().json(
                    &File {
                        err: "path does not exists".to_string(),
                        
                    }
                )
            }
            */

            //match 

            /*
            let data = std::fs::read(v.path)
                .unwrap(); // NOT SAFE will panic!

            HttpResponse::Ok()
                .header("Content-Disposition",
                        content,
                )
                .body(data)
            */
        },

        None => {
            HttpResponse::NotFound().json(
                &File {
                    //group: String::from("stream"),
                    err: "id does not exist".to_string(),
                    
                }
            )
        },
    }
}


/// flush video_map Hash
///
/// curl -X POST 'http://127.0.0.1:8081/video/clear'
///
#[post("/clear")]
pub async fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let mut all_videos = state
        .video_map
        .lock()
        .unwrap();
    
    all_videos.clear();
    
    Ok(
        web::Json(
            IndexResponse {
                server_id: state.server_id,
                request_count,
                video_map: HashMap::new(), // no need to create new as we have old
                //video_map: ms.clone(), // ok but expenssive?
            }
        )
    )
}


/// list group members
///
/// curl 'http://localhost:8081/list/{group_id}'
///
#[get("/list/{group_id}")]
pub async fn list_group(state: web::Data<AppState>,
                        idx: web::Path<String>) -> Result<web::Json<ListResponse>> {

    let to_parse_idx = idx.into_inner();

    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);          
    
    let video = state                                  
        .video_map
        .lock()                                      
        .unwrap();                                   

    /*
    // this should not occure ?
    let result = if !to_parse_idx.eq("") {
            let mut group_map = HashMap::new();

            video
                .iter()
                .for_each(|(key,value)| 
                     if value.group.eq(&to_parse_idx) {
                         // because &, but expensive
                         group_map.insert(
                             key.clone(),
                             value.clone(),
                         );
                     }
                );
            
        Some(group_map)
    } else {
        None
    };
    */

    // /*
    let mut group_map = HashMap::new();

    video
        .iter()
        .for_each(|(key,value)| 
                  if value.group.eq(&to_parse_idx) {
                      // because &, but expensive
                      group_map.insert(
                          key.clone(),
                          value.clone(),
                      );
                  }
        );
    // */

    // as to have empty result as Json 'null' not {}
    let result = if group_map.is_empty() {
        None
    } else {
        Some(group_map)
    };
    
    Ok(                                              
        web::Json(                                   
            ListResponse {                          
                server_id: state.server_id,          
                request_count,        
                result,
            }                                        
        )                                            
    )                                                
}
