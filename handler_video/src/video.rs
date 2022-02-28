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

/// storage path // rather fullpath? as systemd or ... can break? test it
static STATIC_DIR: &str = "./tmp/";

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
            VideoStatus::EmptyVideoId => String::from("VIDEO_ID not provided"),
            VideoStatus::EmptyGroupId => String::from("GROUP_ID not provided"),
            VideoStatus::EmptyFilename => String::from("FILENAME not provided"),

            VideoStatus::TooManyForms => String::from("TOO MANY FORMS we accept only ONE"),
         }
    }
}


/// video
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Video {
    id: String,
    group: String,
    path: String,
}

/// file error // FUTURE USE
#[derive(Serialize, Deserialize)]
struct File {
    group: String,
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


/// index list all videos
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
/// curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F "now_text=@now.txt;type=text/plain"
///
/// curl -X PUT -H 'Content-type: multipart/form-data' http://localhost:8081/video/put -F 'munch_roses_extended_remix=@/home/conan/video/youtube/munch_roses_extended_remix.mp4;type=video/mp4'
///
/// WE DO NOT get JSON here as we get data via PayLOAD, will be enough?
pub async fn insert_video(mut payload: Multipart,
                          state: web::Data<AppState>,
                          req: HttpRequest) -> Result<web::Json<PostResponse>> {
                          
    // Cell
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    // we lock and have access to HashMap messages
    let mut video = state
        .video_map
        .lock() // get access to data inside Mutex + blocks until another thread
        .unwrap(); // -> MutexGuard<Vec<String>> // will panic on Err !!!


    let mut status = VideoStatus::Init.as_string();

    // IMPLEMENT new+default
    let mut new_video = Video {
        group: String::from(""),
        id: String::from(""),
        path: String::from(""),
    };

    /* // HEADERS
    println!("REQ: {:?}\n\nid: {:?}\ngroup: {:?}",
             req.headers(),
             req.headers().get("video_id"),
             req.headers().get("group"),
    );
    */
    match req.headers().get("video_id") {
        Some(id) => {  // HeaderValue
            new_video.id = id
                .to_str()
                .unwrap() // NOT SAFE
                .to_string();
        },
        None => {
            //println!("###ERROR: no VIDEO_ID");
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
                .unwrap()
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
    
    println!("NEW_VIDEO: {:?}",
             new_video,
    );

    let mut content_counter = 0;
    
    // iterate over multipart stream
    // https://actix.rs/actix-web/actix_multipart/struct.Field.html
    // decide if we want just one ore more forms?
    while let Some(mut field) = payload
        .try_next()
        .await? {
            content_counter += 1;

            if content_counter == 1 {
            
            let content_disposition = field.content_disposition();

            /*
            println!("headers: {:?}\ntype: {:?}\ncounter: {:?}",
                     field.headers(),
                     field.content_type(),
                     content_counter,
                     
            );
            */

            if let Some (dis) = content_disposition {
                // OBSOLETE
                //status = VideoStatus::Ok.as_string();


                // /*
                println!("\ndis: {:?}",
                         dis,
                         //field.headers(),
                         //field.content_type(),
                         //content_counter,
                         
                );
                // */
                
                // verify if filename was in form
                match dis.get_filename() {
                    Some(filename) => {
                        // FOR DOWNLOAD/PLAYER or ... url
                        //let filepath = format!("{}{}_{}",
                        new_video.path = format!("{}{}_{}",
                                                 STATIC_DIR,
                                                 //video_id,
                                                 new_video.id,
                                                 filename,
                        );
                        
                        // another clone but WE NEED AT THE very END
                        //full_path = filepath.clone();
                        //full_path = new_video.path.clone();
                        let filepath = new_video.path.clone();

                        /* OBSOLETE
                        video_stream = match dis.get_name() {
                            Some(stream) => {
                                String::from(stream)
                            },
                            // simulate:
                            // curl --form "=@smack.mp4;type=video/mp4" ...
                            None => {
                                status = VideoStatus::EmptyName.as_string();
                                
                                String::from("")
                            },
                        };
                        */
                        
                        // HASH
                        video.insert(
                            // KEY
                            new_video.id.clone(),
                            // VALUE: Video
                            new_video.clone(),
                        );
                        
                        // ### FILE
                        // block -> future to result
                        let mut f = web::block(||
                                               std::fs::File::create(filepath)
                        ).await?;
                        
                        /*
                        println!("F: {f:?}");
                        */
                        
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
                        // FUTURE USE
                        // here we can verify that 'name' can be like:
                        // stream=stream_001 so we will have group_id
                        // and filter/list videos via that
                        //
                        // there will be more error handling

                        // now stream is name for filename
                        // OBSOLETE
                        status = VideoStatus::EmptyFilename.as_string()
                    },
                }
            };
            } else {
                status = VideoStatus::TooManyForms.as_string()
            }
        }

    /* OBSOLETE
    if content_counter > 1 {
        status = VideoStatus::TooManyForms.as_string()
    }
    */

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
#[get("/detail/{idx}")]
pub async fn detail(state: web::Data<AppState>,
                    idx: web::Path<String>) -> Result<web::Json<DetailResponse>> {

    let to_parse_idx = idx.into_inner();

    let path = format!("{}/detail/{}",
                       SCOPE,
                       to_parse_idx,
    );

    //let parsed_idx = match to_parse_idx.parse::<usize>() {
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
            //video.find_equiv(&i).map(|v| Video {
                id: i,
                group: v.group.to_string(),
                path: v.path.to_string(),
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

    //let parsed_idx = match to_parse_idx.parse::<usize>() {
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
                //id: i,
                id: i.to_string(),
                group: v.group.to_string(),
                path: v.path.to_string(),
            })
        },
        None => None,
    };

    match result {
        Some(v) => {
            let content = format!("form-data; filename={}",
                                  v.path,
            );

            let data = std::fs::read(v.path)
                .unwrap(); // NOT SAFE will panic!

            HttpResponse::Ok()
                .header("Content-Disposition",
                        content,
                )
                .body(data)
        },

        None => {
            HttpResponse::NotFound().json(
                &File {
                    group: String::from("stream"),
                    err: "error".to_string(),
                    
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
                //video_map: ms.clone(), // ok but still expenssive?
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

    let result = if !to_parse_idx.eq("") {
            let mut group_map = HashMap::new();

            video
                .iter()
                .for_each(|(key,value)| 
                     if value.group.eq(&to_parse_idx) {
                         // as &, but expensive
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
