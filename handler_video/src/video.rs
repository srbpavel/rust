use crate::handler::AppState;

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

use std::sync::atomic::{AtomicUsize,
                        Ordering,   
};                                  


static VIDEO_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);            
static VIDEO_ID_ORD: Ordering = Ordering::SeqCst;

static STATIC_DIR: &str = "./tmp/";

pub const SCOPE: &str = "/video";


/// flux_query error
#[derive(Debug)]
pub enum VideoStatus {
    Init,
    Ok,
    EmptyName,
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
            VideoStatus::EmptyName => String::from("STREAM for filename not provided"),
            VideoStatus::EmptyFilename => String::from("FILENAME not provided"),

            VideoStatus::TooManyForms => String::from("TOO MANY FORMS we accept only one"),
         }
    }
}


#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Video {
    id: usize,
    stream: String,
    path: String,
}

// FUTURE USE
#[derive(Serialize, Deserialize)]
struct File {
    stream: String,
    err: String,
}

#[derive(Serialize, Debug)]
pub struct IndexResponse {     
    server_id: usize,      
    request_count: usize,  
    video_map: HashMap<usize, Video>, 
}

/* // FUTURE USE -> this was for json message post_msg / OBSOLETE here ?
#[derive(Deserialize, Debug)]
pub struct PostInput {
    video: String, 
}
*/

#[derive(Serialize, Debug)]
pub struct PostResponse {
    server_id: usize,
    request_count: usize,
    video: Video,
    status: String,
}

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

    println!("REQ: {:?}\n\nid: {:?}\ngroup: {:?}",
             req.headers(),
             req.headers().get("video_id"),
             req.headers().get("group"),
    );
    
    
    // Cell
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    // we lock and have access to HashMap messages
    let mut video = state
        .video_map
        .lock() // get access to data inside Mutex + blocks until another thread
        .unwrap(); // -> MutexGuard<Vec<String>> // will panic on Err !!!

    // CLEAR do not reset counter yet, as needed for Curl testing
    let video_id = VIDEO_ID_COUNTER.fetch_add(1,              
                                              VIDEO_ID_ORD,
    );

    // as for content which is not file until we will have error msg
    let mut video_stream = String::from("init_stream");
    let mut full_path = String::from("init_path");
    let mut status = VideoStatus::Init.as_string();

    let mut content_counter = 0;
    
    // iterate over multipart stream
    // https://actix.rs/actix-web/actix_multipart/struct.Field.html
    // decide if we want just one ore more forms?
    while let Some(mut field) = payload
        .try_next()
        .await? {

            content_counter += 1;
            
            let content_disposition = field.content_disposition();

            /*
            println!("headers: {:?}\ntype: {:?}\ncounter: {:?}",
                     field.headers(),
                     field.content_type(),
                     content_counter,
                     
            );
            */

            if let Some (dis) = content_disposition {
                status = VideoStatus::Ok.as_string();


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
                        let filepath = format!("{}{}_{}",
                                               STATIC_DIR,
                                               video_id,
                                               filename,
                        );
                        
                        // another clone but WE NEED AT THE very END
                        full_path = filepath.clone();

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
                        
                        // HASH
                        video.insert(
                            // KEY
                            video_id,
                            // VALUE
                            Video {
                                id:video_id,
                                stream:video_stream.clone().to_string(),
                                path:filepath.clone().to_string(),
                            },
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
                        status = VideoStatus::EmptyFilename.as_string()
                    },
                }
            };
        }

    if content_counter > 1 {
        status = VideoStatus::TooManyForms.as_string()
    }
    
    Ok(web::Json(
        PostResponse {
            server_id: state.server_id,
            request_count,
            video: Video {
                stream: video_stream.clone(),
                id: video_id,
                path: full_path,
            },
            status,
        }
    ))
}


/// DETAIL via hash
/// 
/// path as String
/// i did not make it work for usize because do no find way to verify valid usize?
///
/// curl 'http://localhost:8081/video/detail/{idx}'
///
#[get("/detail/{id}")]
pub async fn detail(state: web::Data<AppState>,
                    idx: web::Path<String>) -> Result<web::Json<DetailResponse>> {

    let to_parse_idx = idx.into_inner();

    let path = format!("{}/detail/{}",
                       SCOPE,
                       to_parse_idx,
    );

    let parsed_idx = match to_parse_idx.parse::<usize>() {
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
                stream: v.stream.to_string(),
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

    let parsed_idx = match to_parse_idx.parse::<usize>() {
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
                id: i,
                stream: v.stream.to_string(),
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
                    stream: String::from("stream"),
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
