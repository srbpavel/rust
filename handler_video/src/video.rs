use crate::handler::AppState;

use actix_web::{
    get,
    post,
    web,
    Result,
    HttpResponse,
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


#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Video {
    id: usize,
    name: String,
    path: String,
}


// FUTURE USE
#[derive(Serialize, Deserialize)]
struct File {
    name: String,
    time: u64,
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
                          state: web::Data<AppState>) -> Result<web::Json<PostResponse>> {

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
    let mut video_name = String::from("");
    let mut full_path = String::from("");
    
    // iterate over multipart stream
    // https://actix.rs/actix-web/actix_multipart/struct.Field.html
    while let Some(mut field) = payload
        .try_next()
        .await? {

            let content_disposition = field.content_disposition();

            /*
            println!("headers: {:?} type: {:?}",
                     field.headers(),
                     field.content_type(),
                     
            );
            */

            if let Some (dis) = content_disposition {

                // verify filename was in form
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

                        video_name = match dis.get_name() {
                            Some(name) => {
                                String::from(name)
                            },
                            // curl --form "=@smack.mp4;type=video/mp4" ...
                            None => {
                                //println!("name for filename not provided");
                                String::from("VIDEO_NAME")
                            },
                        };
                        
                        // HASH
                        video.insert(
                            // KEY
                            video_id,
                            // VALUE
                            Video {
                                id:video_id,
                                name:video_name.clone().to_string(),
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
                    },
                }
            };
        }

    Ok(web::Json(
        PostResponse {
            server_id: state.server_id,
            request_count,
            video: Video {
                name: video_name.clone(),
                id: video_id,
                path: full_path,
            },
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
                name: v.name.to_string(),
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

    let result = match parsed_idx {
        Some(i) => {
            video.get(&i).map(|v| Video {
                id: i,
                name: v.name.to_string(),
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
                    name: String::from("name"),
                    time: 1234567890,
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
