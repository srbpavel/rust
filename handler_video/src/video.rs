use crate::{
    handler::AppState,
    util,
    status,
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
use uuid::Uuid;

/// scope
pub const SCOPE: &str = "/video";

/// types for video hash_map
pub type VideoKey = String;
pub type VideoValue = Video;

/// video
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Video {
    id: String,
    group: String,
    path: PathBuf,
}

impl Video {
    /// default
    pub fn default() -> Self {
        Self {
            id: String::from(""),
            group: String::from(""),
            path: PathBuf::new(),
        }
    }
}

/// file error // FUTURE USE
#[derive(Serialize, Deserialize)]
struct File {
    err: String,
}

/// all videos
#[derive(Serialize, Debug)]
pub struct IndexResponse {     
    server_id: usize,      
    request_count: usize,  
    video_map: HashMap<VideoKey, VideoValue>,
    status: String,
}

/// group members
#[derive(Serialize, Debug)]
pub struct ListResponse {     
    server_id: usize,      
    request_count: usize,  
    result: Option<HashMap<VideoKey, VideoValue>>,
    status: String,
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
    status: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateInput {
    video_id: String,
    group_id: String,
}

/// delete info
#[derive(Serialize, Debug)]
pub struct DeleteResponse {     
    result: String,
}

/// GET index list all videos
///
/// curl 'http://localhost:8081/video/'
///
/// for debug purpose as tested with dozen records not milions yet
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
                //FUTURE USE
                status: status::Status::StatusOk.as_string(),
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
/// already existing record is overwritten -> we do not verify/confirm 
/// if the same id but different file, this will create zombie file
///
/// later: via config flags: verify id/overwrite existing + delete file/send msg
///
pub async fn insert_video(mut payload: Multipart,
                          state: web::Data<AppState>,
                          req: HttpRequest) -> Result<web::Json<PostResponse>> {

    // decide sequence -> verify storage or headers/form?
    // just in single dir for now, will seed to various dirs later
    //VERIFY STORAGE
    let path_to_verify = PathBuf::from(&*state.config.static_dir);

    match util::verify_dir(&path_to_verify,
                           state.config.verify_dir_per_video,
    ) {
        Ok(_) => {},
        Err(err) => {
            // curl: (55) Send failure: Connection reset by peer
            // but still we receive JSON response with status
            return Ok(
                web::Json(
                    PostResponse {
                        result: None,
                        status: err.to_string(),
                    }
                )
            )
        },
    };
    
    // Cell
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    // we lock and have access to HashMap messages
    let mut video_hashmap = state
        .video_map
        .lock() // get access to data inside Mutex + blocks until another thread
        .unwrap(); // -> MutexGuard<Vec<String>> // will panic on Err !!!

    let mut status = status::Status::Init;//.as_string();
    let mut new_video = Video::default();

    //println!("HEADERS: {:?}", req.headers());
    match req.headers().get("video_id") {
        Some(id) => {  // HeaderValue
            new_video.id = id
                .to_str()
                //.unwrap() // NOT SAFE
                // we will rather return Err msg instead generate uuid
                .unwrap_or(&Uuid::new_v4()
                           .to_string()
                )
                .to_string();
        },
        None => {
            // curl: (55) Send failure: Connection reset by peer
            // but still we receive JSON response with status
            return Ok(
                web::Json(
                    PostResponse {
                        result: None,
                        status: status::Status::EmptyVideoId.as_string(),
                    }
                )
            )
        },
    }

    // we should add group to vec/hash just to list groups without iter video
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
                        status: status::Status::EmptyGroupId.as_string(),
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

                    status = status::Status::StatusOk;
                    
                    // verify if filename was in form
                    match dis.get_filename() {
                        Some(filename) => {
                            // FOR DOWNLOAD/PLAYER or ... url
                            new_video.path = Path::new(
                                // STORAGE via config
                                // find better way then in .data() !!!
                                &state.config.static_dir)
                                .join(
                                    format!("{}_{}",
                                            new_video.id,
                                            filename,
                                    )
                                );

                            /*
                            println!("\nFULL_PATHs: {:?}",
                                     new_video.path,
                            );
                            */
                            
                            // another clone but WE NEED AT THE very END
                            let filepath = new_video.path.clone();
                            
                            // HASH record
                            video_hashmap
                                .insert(
                                    new_video.id.clone(), // KEY: video.id
                                    new_video.clone(), // VALUE: Video {}
                                );

                            /* // ### BUF
                            let mut f = web::block(||
                                                   std::fs::File::create(filepath)
                                                   /*
                                                   std::result::Result::Ok(
                                                   )
                                                   */
                            ).await?;
                            //println!("F: {f:?}");
                            
                            while let Some(chunk) = field.try_next().await? {
                                f = web::block(move ||
                                               f
                                               .write_all(&chunk)
                                               .map(|_| f)
                                ).await?;
                            };
                            // #_ */
                            
                            // ### FILE
                            // block -> future to result
                            let mut f = web::block(||
                                                   // we should also verify:
                                                   // access/write/disc not full
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
                            status = status::Status::EmptyFilename
                        },
                    }
                };
            } else {
                status = status::Status::TooManyForms
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
                status: status.as_string(),
            }
        )
    )
}


/// GET detail via hash
/// 
/// curl 'http://localhost:8081/video/detail/{video_id}'
///
/// good enough for dozen id's, tune for millions
///
#[get("/detail/{video_id}")]
pub async fn detail(state: web::Data<AppState>,
                    idx: web::Path<String>) -> Result<web::Json<DetailResponse>> {

    let to_parse_idx = idx.into_inner();

    let url = format!("{}/detail/{}",
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

    let status;
    
    let result = match parsed_idx {
        Some(i) => {
            let detail = video.get(&i).map(|v| { 
                Video {
                    id: i,
                    group: v.group.to_string(),
                    path: v.path.clone(),
                }
            });

            match detail {
                Some(d) => {
                    status = status::Status::VideoIdFound;

                    Some(d)
                },
                None => {
                    status = status::Status::VideoIdNotAvailable;

                    None
                },
            }
                
            /*
            video.get(&i).map(|v| 
                              Video {
                                  id: i,
                                  group: v.group.to_string(),
                                  path: v.path.clone(),
                              }
            )
             */

            
        },
        None => {
            status = status::Status::VideoIdNotFound;
            
            None
        },
    };
    
    Ok(
        web::Json(
            DetailResponse {
                server_id: state.server_id,
                request_count,
                result,
                url,
                //FUTURE USE
                status: status.as_string(),
            }
        )
    )
}


/// GET DOWNLOAD via hash
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
                path: v.path.clone(),
            })
        },
        None => None,
    };

    match result {
        Some(v) => {
            match std::fs::read(v.path.clone()) {
                Ok(data) => {
                    // just filename without path?
                    let content = format!("form-data; filename={}",
                                          match v.path.to_str() {
                                              Some(p) => p,
                                              // should not occure as verified?
                                              None => "FILENAME",
                                          },
                    );

                    //println!("CONTENT: {content:?}");
                    
                    HttpResponse::Ok()
                        .header("Content-Disposition",
                                content,
                        )
                        .body(data)
                },
                Err(why) => {
                    HttpResponse::NotFound().json(
                        &File {
                            err: format!("{why:?}")
                        }
                    )
                },
            }
        },

        None => {
            HttpResponse::NotFound().json(
                &File {
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
                // FUTURE USE
                status: status::Status::StatusOk.as_string(),
            }
        )
    )
}


/// DELETE via id
/// 
/// path as String
///
/// curl -X DELETE 'http://localhost:8081/video/delete/{id}'
///
/// this tells server that client expect JSON data in response
/// -H "Accept: application/json"
///
pub async fn delete(state: web::Data<AppState>,
                    idx: web::Path<String>) -> Result<web::Json<DeleteResponse>> {

    //println!("IDX: {idx:?}");
    
    // deconstruct to inner value
    let to_parse_idx = idx.into_inner();

    // let's try parse
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

    //println!("PARSED_IDX: {parsed_idx:?}");
    
    // we still add to this thread counter
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    // we lock msg vec, but now as MUT because we delete
    let mut video_hashmap = state
        .video_map
        .lock()
        .unwrap();

    // even we get string, we still parse as later we will get id as usize/...
    let result = match parsed_idx {
        Some(i) => {
            // search for video_id in hashmap
            match video_hashmap.get_mut(&i) {
                Some(record) => {
                    // validate path not permission
                    if Path::new(&record.path).exists() {
                        // first we remove video Struct
                        match video_hashmap.remove(&i) {
                            Some(response_record) => {
                                // then file -> we use response data path
                                match std::fs::remove_file(&response_record.path) {
                                    Ok(_) => status::Status::DeleteOk.as_string(),
                                    //Ok(_) => status::Status::Delete::DeleteOk.as_string(),
                                    // in case dir ownership or perm has changed
                                    // -rw------- 1 root root 8320394 Mar  1 11:56
                                    // /home/conan/soft/rust/handler_video/storage/456_love_tonight_extended_mix.mp4
                                    //id: 456,
                                    //path: \"/home/conan/soft/rust/handler_video/storage/456_love_tonight_extended_mix.mp4\",
                                    // reason: Permission denied (os error 13)"
                                    Err(why) => {
                                        format!("id: {}, path: {:?}, reason: {}",
                                                i,
                                                response_record.path,
                                                why,
                                        )
                                    }
                                }
                            },
                            None => status::Status::DeleteError.as_string(),
                            //None => status::Status::Delete::DeleteError.as_string(),
                        }
                    } else {
                        status::Status::FileNotFound.as_string()
                    }
                },
                None => status::Status::VideoIdNotFound.as_string(),
            }
        },
        None => status::Status::DeleteInvalidId.as_string(),
        //None => status::Status::Delete::DeleteInvalidId.as_string(),
    };
    
    Ok(
        web::Json(
            DeleteResponse {                          
                result
            }                                        
        )
    )
}


/// UPDATE group_id for video
/// 
/// curl -X POST "http://localhost:8081/video/update/group" -H "Content-Type: application/json" -d '{"video_id": "123", "group_id": "video_on_demand"}' 2>/dev/null | jq
///
///
pub async fn update_group(update: web::Json<UpdateInput>,
                          state: web::Data<AppState>) -> actix_web::Result<web::Json<DetailResponse>> {
    
    //println!("UPDATE_VIDEO: {update:?}");    

    let url = format!("{}/update/group",
                      SCOPE,
    );
    
    // Cell
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    
    // we lock and have access to HashMap messages
    let mut video_hashmap = state
        .video_map // HASH
        .lock() // get access to data inside Mutex + blocks until another thread
        .unwrap(); // -> MutexGuard<Vec<String>> // will panic on Err !!!

    let result = match video_hashmap.get_mut(&update.video_id) {
        Some(video) => {
            //println!("Video: {video:#?}");

            // update
            video.group = update.group_id.to_string();

            Some(
                Video {
                    id: video.id.clone(),
                    group: update.group_id.to_string(),
                    path: video.path.clone(),
                }
            )
        },
        None => None,
    };
    
    Ok(
        web::Json(
            DetailResponse {
                server_id: state.server_id,
                request_count,
                result,
                url,
                //FUTURE USE
                status: status::Status::UpdateOk.as_string(),
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

    /* // via NEW HASHMAP
    let mut group_map = HashMap::new();
    video
        .iter() // &
        .for_each(|(key,value)| 
                  if value.group.eq(&to_parse_idx) {
                      // because &, but expensive
                      group_map.insert(
                          key.clone(),
                          value.clone(),
                      );
                  }
        );
    */

    // /* // via CLONE
    let group_map: HashMap<VideoKey, VideoValue> = video.clone()
        .into_iter()
        .filter(|(_,value)|
                    value.group.eq(&to_parse_idx)
        )
        .collect();
    // */

    let status;
    
    // as to have empty result as Json 'null' not {}
    let result = if group_map.is_empty() {
        status = status::Status::GroupNotFound;

        None
    } else {
        status = status::Status::GroupFound;
            
        Some(group_map)
    };
    
    Ok(                                              
        web::Json(                                   
            ListResponse {                          
                server_id: state.server_id,          
                request_count,        
                result,
                //FUTURE USE
                status: status.as_string(),
            }                                        
        )                                            
    )                                                
}
