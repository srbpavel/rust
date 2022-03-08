use crate::{
    handler::{AppState,
    }, 
    status,
};
use log::debug;
use actix_web::{get,
                post,
                web,
                Result,
                HttpResponse,
                HttpRequest,
                Responder,
};
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use serde::{Serialize,
            Deserialize,
};
use std::collections::HashMap;
use uuid::Uuid;
use bytes::{BytesMut,
            BufMut,
};


pub const SCOPE: &str = "/video";

/// types for hash_maps
pub type VideoKey = String;
pub type VideoValue = Video;
pub type BinaryValue = Binary;

/// binary data
#[derive(Debug, Clone)]
pub struct Binary {
    pub data: BytesMut,
    pub filename: String,
    pub mime: String,
}

type VideoType = String; 

/// video
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Video {
    id: VideoType,
    group: VideoType,
    name: VideoType,
}

impl Video {
    /// default
    pub fn default() -> Self {
        Self {
            id: String::from(""),
            group: String::from(""),
            name: String::from(""),
        }
    }
}

/// parse url pattern error msg
#[derive(Serialize, Deserialize)]
struct ParseError {
    status: String,
}

/// all videos
#[derive(Serialize, Debug)]
pub struct IndexResponse {     
    result: Option<HashMap<VideoKey, VideoValue>>,
    status: String,
}

/// detail
#[derive(Serialize, Debug)]                               
pub struct DetailResponse {                                   
    result: Option<Video>,
    status: String,
}

/// delete info
#[derive(Serialize, Debug)]
pub struct DeleteResponse {     
    status: String,
}


/// list all videos
///
/// for debug purpose as tested with dozen records not milions yet
///
#[get("/all")]
pub async fn all(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {

    let all_videos = state                                  
        .video_map
        .lock()                                      
        .unwrap();                                   

    let result = if all_videos.is_empty() {
        None
    } else {
        Some(all_videos.clone())
    };
    
    Ok(                                              
        web::Json(                                   
            IndexResponse {                          
                result,
                status: status::Status::ListAll.as_string(),
            }                                        
        )                                            
    )                                                
}


/// detail
/// 
#[get("/detail/{video_id}")]
pub async fn detail(state: web::Data<AppState>,
                    idx: web::Path<String>) -> Result<web::Json<DetailResponse>> {

    let to_parse_idx = idx.into_inner();

    let parsed_idx = match to_parse_idx.parse::<String>() {
        Ok(i) => {
            Some(i)
        },
        Err(why) => {
            return Ok(
                web::Json(
                    DetailResponse {                          
                        result: None,
                        status: format!("{why}"),
                    }
                )
            )
        },
    };

    let mut status;
    
    let result = match parsed_idx {
        Some(i) => {
            status = status::Status::VideoIdNotFound;

            let video = state
                .video_map
                .lock()
                .unwrap();

            video.get(&i).map(|v| {
                status = status::Status::VideoIdFound;

                let binary = state
                    .binary_map
                    .lock()
                    .unwrap();

                binary
                    .get(&i)
                    .map(|b| {
                        debug!("B: {} -> {}", b.filename, b.mime);
                    });
                
                Video {
                    id: i,
                    group: v.group.to_string(),
                    name: v.name.clone(),
                }
            })
        },
        None => {
            status = status::Status::VideoIdWrongFormat;
            
            None
        },
    };

    Ok(
        web::Json(
            DetailResponse {
                result,
                status: status.as_string(),
            }
        )
    )
}


/// flush video_map + binary_map
///
#[post("/clear")]
pub async fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {

    let mut all_videos = state
        .video_map
        .lock()
        .unwrap();

    all_videos.clear();
    
    let mut all_binary = state
        .binary_map
        .lock()
        .unwrap();

    all_binary.clear();

    Ok(
        web::Json(
            IndexResponse {
                result: None,
                status: status::Status::ClearOk.as_string(),
            }
        )
    )
}


/// DELETE video_id
/// 
pub async fn delete(state: web::Data<AppState>,
                    idx: web::Path<String>) -> Result<web::Json<DeleteResponse>> {

    //debug!("IDX: {idx:?}");
    
    let to_parse_idx = idx.into_inner();

    let parsed_idx = match to_parse_idx.parse::<String>() {
        Ok(i) => {
            Some(i)
        },
        Err(why) => {
            return Ok(
                web::Json(
                    DeleteResponse {                          
                        status: format!("{why}"),
                    }                                        
                )
            )
        },
    };

    //debug!("PARSED_IDX: {parsed_idx:?}");

    // even we get string, we still parse as later we will get id as usize/...
    let result = match parsed_idx {
        Some(i) => {
            let mut video_hashmap = state
                .video_map
                .lock()
                .unwrap();
            
            match video_hashmap.get_mut(&i) {
                Some(_) => {
                    match video_hashmap.remove(&i) {
                        Some(_) => {
                                let mut binary_hashmap = state
                                .binary_map
                                .lock()
                                .unwrap();
                            
                            match binary_hashmap.remove(&i) {
                                Some(_) => {
                                    status::Status::DeleteOk
                                },
                                None => status::Status::DeleteError,
                            }
                        },
                        None => status::Status::DeleteError,
                    }
                },
                None => status::Status::VideoIdNotFound,
            }
        },
        None => status::Status::DeleteInvalidId,
    };
    
    Ok(
        web::Json(
            DeleteResponse {                          
                status: result.as_string(),
            }                                        
        )
    )
}


/// list group members
///
#[get("/list/{group_id}")]
pub async fn list_group(state: web::Data<AppState>,
                        idx: web::Path<String>) -> Result<web::Json<IndexResponse>> {

    let to_parse_idx = idx.into_inner();

    let video = state                                  
        .video_map
        .lock()                                      
        .unwrap();                                   

    let group_map: HashMap<VideoKey, VideoValue> = video.clone()
        .into_iter()
        .filter(|(_,value)|
                    value.group.eq(&to_parse_idx)
        )
        .collect();

    let status;
    
    let result = if group_map.is_empty() {
        status = status::Status::GroupNotFound;

        None
    } else {
        status = status::Status::GroupFound;
            
        Some(group_map)
    };
    
    Ok(                                              
        web::Json(                                   
            IndexResponse {                          
                result,
                status: status.as_string(),
            }                                        
        )                                            
    )                                                
}


/// upload video
///
pub async fn insert_video(mut payload: Multipart,
                          state: web::Data<AppState>,
                          req: HttpRequest)  -> Result<web::Json<DetailResponse>> {

    //debug!("REQ: {req:?}");

    let AppState { video_map, binary_map } = &*state.into_inner();

    let mut status = status::Status::Init;
    let mut new_video = Video::default();

    //debug!("HEADERS: {:?}", req.headers());

    // VIDEO_ID
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
                    DetailResponse {                          
                        result: None,
                        status: status::Status::EmptyVideoId.as_string(),
                    }                                        
                )
            )
        },
    }

    // GROUP
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
                    DetailResponse {                          
                        result: None,
                        status: status::Status::EmptyGroupId.as_string(),
                    }
                )
            )
        },
    }
    
    // iterate over multipart stream
    // https://actix.rs/actix-web/actix_multipart/struct.Field.html
    let mut content_counter = 0;

    while let Some(mut field) = payload
    // // while let Some(item) = payload
        .try_next()
        .await? {
            content_counter += 1;

            // // let mut field = item;
            
            // we only accept one form with file
            if content_counter == 1 {
                let content_disposition = field.content_disposition();

                match content_disposition.get_name() {
                    Some(name) => {
                        if name.trim().eq("") {
                            return Ok(
                                web::Json(
                                    DetailResponse {                          
                                        result: None,
                                        status: status::Status::EmptyFormName.as_string(),
                                    }
                                )
                            )
                        }
                        
                        new_video.name = String::from(name);
                    },
                    None =>{
                        return Ok(
                            web::Json(
                                DetailResponse {                          
                                    result: None,
                                    status: status::Status::EmptyFormName.as_string(),
                                }
                            )
                        )
                    },
                }

                // FORM
                match content_disposition.get_filename() {
                    Some(filename) => {

                        let content_type = field.content_type().essence_str();
                        debug!("CONTENT_TYPE: {:?}", content_type);
                        
                        // /* // https://actix.rs/docs/server/
                        let mut buf = Binary {
                            data: BytesMut::with_capacity(1024),
                            filename: filename.to_string(),
                            mime: String::from(content_type),

                        };
                        // */
                        
                        /*
                        let mut buf = web::block(move || {
                            Binary {
                                data: BytesMut::with_capacity(1024),
                                filename: filename.to_string(),
                            }
                        }).await?;
                        */

                        let mut chunk_counter = 0;
                        while let Some(chunk) = field.try_next().await? {
                            chunk_counter += 1;

                            // FIRST CHUNK
                            if chunk_counter == 1 {
                                // LOCK DETAIL
                                let mut video_hashmap = video_map.lock().unwrap();
                                video_hashmap
                                    .insert(
                                        new_video.id.clone(), // KEY: video.id
                                        new_video.clone(), // VALUE: Video {}
                                    );
                            }

                            buf.data = web::block(move || {
                                let ch = &*chunk;
                                debug!("CHUNK: {chunk_counter}");

                                buf
                                    .data
                                    //.put(&*chunk);
                                    .put(ch);

                                buf.data
                            }).await?;

                            // /*
                            // LOCK DATA
                            let mut binary_hashmap = binary_map.lock().unwrap();
                            binary_hashmap
                                .insert(
                                    new_video.id.clone(), // KEY: video.id
                                    buf
                                        .clone(), // VALUE: Binary {}
                                );
                            // */
                        };

                        status = status::Status::UploadDone;
                    },
                    None => {
                        return Ok(
                            web::Json(
                                DetailResponse {                          
                                    result: None,
                                    status: status::Status::EmptyFormFilename.as_string(),
                                }
                            )
                        )
                    },
                }
            } else {
                return Ok(
                    web::Json(
                        DetailResponse {                          
                            result: None,
                            status: status::Status::TooManyForms.as_string(),
                        }
                    )
                )
            }
        }

    Ok(
        web::Json(
            DetailResponse {                          
                result: Some(
                    new_video
                ),
                status: status.as_string(),
            }
        )
    )
}


/// RAM download
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
            return HttpResponse::NotFound()
                .json(
                    &ParseError {
                        status: format!("{why}"),
                }
            )
        },
    };

    let result = match parsed_idx {
        Some(i) => {
            let binary = state
                .binary_map
                .lock()
                .unwrap();
            
            binary.get(&i).map(|v|
                               Binary {
                                   data: v.data.clone(),  // niet goed
                                   filename: v.filename.clone(),
                                   //mime: String::from("MIME"),
                                   mime: v.mime.clone(),
                               }
            )
        },
        None => None,
    };

    match result {
        Some(v) => {
            HttpResponse::Ok()
                .append_header(
                    ("Content-Disposition",
                     format!("form-data; filename={}",
                             v.filename,
                     ),
                    )
                )
                /*
                .content_type(
                    actix_web::http::header::ContentType::octet_stream()
                )
                .status(actix_web::http::StatusCode::OK)
                */
                .body(v.data)
        },
        None => {
            HttpResponse::NotFound().json(
                &ParseError {
                    status: status::Status::VideoIdNotFound.as_string(),
                }
            )
        },
    }
}


/// PLAYER
/// 
#[get("/play/{idx}")]
pub async fn play(state: web::Data<AppState>,
                  idx: web::Path<String>) -> impl Responder {

    let to_parse_idx = idx.into_inner();

    let parsed_idx = match to_parse_idx.parse::<String>() {
        Ok(i) => {
            Some(i)
        },
        Err(_why) => {
            // Infallible
            return web::Bytes::from_static(b"player: index String parse error")
        },
    };

    let result = match parsed_idx {
        Some(i) => {
            let binary = state
                .binary_map
                .lock()
                .unwrap();
            
            binary.get(&i).map(|v|
                               Binary {
                                   data: v.data.clone(),  // niet goed jochie
                                   filename: v.filename.clone(),
                                   //mime: String::from("MIME"),
                                   mime: v.mime.clone(),
                               }
            )
        },
        None => return web::Bytes::from_static(b"player: parsed None")
    };

    match result {
        Some(v) => {
            web::Bytes::from(v.data)
        },
        None => {
            // we should rather return 404 
            return web::Bytes::from_static(b"player: binary_id not found")
        },
    }
}
