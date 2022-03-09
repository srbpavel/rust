use crate::{
    handler::{AppState,
    }, 
    status,
};
//use log::debug;
use actix_web::{get,
                post,
                web,
                Result,
                HttpResponse,
                HttpRequest,
                Responder,
                http::header::HeaderMap,
};
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use serde::Serialize;
use std::collections::HashMap;
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


/// video
#[derive(Serialize, Debug, Clone)]
pub struct Video {
    id: String,
    group: String,
    name: String,
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
pub struct StatusResponse {     
    status: String,
}


enum HeaderKey {
    VideoId,
    Group,
}

impl HeaderKey {
    pub fn as_string(&self) -> String {
        match *self {
            Self::VideoId => String::from("video_id"),
            Self::Group => String::from("group"),
        }
    }
}


/// list all videos
///
/// for debug purpose as tested with dozen records not milions yet
///
#[get("/all")]
pub async fn all(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {

    let status;

    let all_videos = match state                                  
        .video_map
        .lock() {
            Ok(video_map) => video_map,
            Err(_) => 
                return ok_json(
                    IndexResponse {                          
                        result: None,
                        status: status::Status::InvalidVideoMap.as_string(),
                    }
                ),
        };

    /*
    let all_videos = state                                  
        .video_map
        .lock()
        .unwrap();
    */

    let result = if all_videos.is_empty() {
        status = status::Status::ListNone;
        
        None
    } else {
        status = status::Status::ListAll;
        
        Some(all_videos.clone())
    };

    ok_json(
        IndexResponse {                          
            result,
            status: status.as_string(),
        }
    )
}


/// detail
/// 
#[get("/detail/{video_id}")]
pub async fn detail(state: web::Data<AppState>,
                    idx: web::Path<String>) -> Result<web::Json<DetailResponse>> {

    let to_parse_idx = inner_trim(idx);
    
    let video = state                                  
        .video_map
        .lock()
        .unwrap();

    let mut status;
    status = status::Status::VideoIdNotFound;
    
    let result = video.get(&to_parse_idx).map(|v| {
        status = status::Status::VideoIdFound;
        
        Video {
            id: to_parse_idx,
            group: v.group.clone(),
            name: v.name.clone(),
        }
    });

    ok_json(
        DetailResponse {
            result,
            status: status.as_string(),
        }
    )
}


/// flush video_map + binary_map
///
#[post("/clear")]
pub async fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {

    let _ = state                                  
        .video_map
        .lock()
        .map(|mut v| v.clear()
        );

    let _ = state                                  
        .binary_map
        .lock()
        .map(|mut b| b.clear()
        );

    ok_json(
        IndexResponse {                          
            result: None,
            status: status::Status::ClearOk.as_string(),
        }
    )
}


/// DELETE video_id
/// 
pub async fn delete(state: web::Data<AppState>,
                    idx: web::Path<String>) -> Result<web::Json<StatusResponse>> {

    let to_parse_idx = inner_trim(idx);

    // if deleted detail durring upload,
    // we have to wait until last chunk to remove binary
    let mut video_hashmap = state
        .video_map
        .lock()
        .unwrap();
    
    let result = match video_hashmap.get_mut(&to_parse_idx) {
        Some(_) => {
            match video_hashmap.remove(&to_parse_idx) {
                Some(_) => {
                    let mut binary_hashmap = state
                        .binary_map
                        .lock()
                        .unwrap();
                    
                    match binary_hashmap.remove(&to_parse_idx) {
                        Some(_) => {
                            status::Status::DeleteOk
                        },
                        None => status::Status::DeleteBinaryError,
                    }
                },
                None => status::Status::DeleteDetailError,
            }
        },
        None => status::Status::VideoIdNotFound,
    };

    ok_json(
        StatusResponse {
            status: result.as_string(),
        }                                        
    )
}


/// list group members
///
#[get("/list/{group_id}")]
pub async fn list_group(state: web::Data<AppState>,
                        idx: web::Path<String>) -> Result<web::Json<IndexResponse>> {

    let to_parse_idx = inner_trim(idx);
    
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

    ok_json(
        IndexResponse {                          
            result,
            status: status.as_string(),
        }
    )
}


/// upload video
///
pub async fn insert_video(mut payload: Multipart,
                          state: web::Data<AppState>,
                          req: HttpRequest)  -> Result<web::Json<DetailResponse>> {

    let AppState { video_map, binary_map } = &*state.into_inner();

    let mut status = status::Status::Init;
    let mut new_video = Video::default();

    new_video.id = match verify_header(HeaderKey::VideoId,
                                       req.headers(),
    ) {
        Some(value) => value,
        None => return ok_json(
            DetailResponse {                          
                result: None,
                status: status::Status::EmptyVideoId.as_string(),
            }
        ),
    };

    new_video.group = match verify_header(HeaderKey::Group,
                                          req.headers(),
    ) {
        Some(value) => value,
        None => return ok_json(
            DetailResponse {                          
                result: None,
                status: status::Status::EmptyGroup.as_string(),
            }
        ),
    };
    
    let mut content_counter = 0;

    while let Some(mut field) = payload
        .try_next()
        .await? {
            content_counter += 1;

            // we only accept one form with file
            if content_counter == 1 {
                let content_disposition = field.content_disposition();

                match content_disposition.get_name() {
                    Some(name) => {
                        if name.trim().eq("") {
                            return ok_json(
                                DetailResponse {                          
                                    result: None,
                                    status: status::Status::EmptyFormName.as_string(),                     
                                }
                            )
                        }

                        new_video.name = String::from(name);
                    },
                    None =>
                        return ok_json(
                            DetailResponse {                          
                                result: None,
                                status: status::Status::EmptyFormName.as_string(),                     
                            }
                        ),
                }

                match content_disposition.get_filename() {
                    Some(filename) => {

                        let content_type = field
                            .content_type()
                            .essence_str();
                        
                        let mut buf = Binary {
                            data: BytesMut::with_capacity(1024),
                            filename: filename.to_string(),
                            mime: String::from(content_type),

                        };
                        
                        let mut chunk_counter = 0;

                        while let Some(chunk) = field.try_next().await? {
                            chunk_counter += 1;

                            // FIRST CHUNK
                            if chunk_counter == 1 {
                                // LOCK DETAIL
                                let mut video_hashmap = video_map
                                    .lock()
                                    .unwrap();

                                video_hashmap
                                    .insert(
                                        new_video.id.clone(), // K: video.id
                                        new_video.clone(), // V: Video {}
                                    );
                            }

                            buf.data = web::block(move || {
                                //let ch = &*chunk;
                                //debug!("CHUNK: {chunk_counter}");

                                buf
                                    .data
                                    .put(&*chunk);
                                    //.put(ch);

                                buf.data
                            }).await?;

                            // LOCK DATA
                            let mut binary_hashmap = binary_map.lock().unwrap();

                            binary_hashmap
                                .insert(
                                    new_video.id.clone(), // K: video.id
                                    buf
                                        .clone(), // V: Binary {}
                                );
                        };

                        status = status::Status::UploadDone;
                    },
                    None =>
                        return ok_json(
                            DetailResponse {                          
                                result: None,
                                status: status::Status::EmptyFormFilename.as_string(),
                            }
                        ),
                }
            } else {

                return ok_json(
                    DetailResponse {                          
                        result: None,
                        status: status::Status::TooManyForms.as_string(),
                    }
                )
            }
        }


    ok_json(
        DetailResponse {                          
            result: Some(new_video),
            status: status.as_string(),
        }
    )
}


/// download
/// 
#[get("/download/{idx}")]
pub async fn download(state: web::Data<AppState>,
                      idx: web::Path<String>) -> HttpResponse {

    let to_parse_idx = inner_trim(idx);

    let binary = state
        .binary_map
        .lock()
        .unwrap();
    
    let result = binary
        .get(&to_parse_idx)
        .map(|v|
             v.clone()
        );

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
                .body(v.data)
        },
        None => {
            HttpResponse::NotFound().json(
                &StatusResponse {
                    status: status::Status::VideoIdNotFound.as_string(),
                }
            )
        },
    }
}


/// PLAYER -> data in bytes
/// 
#[get("/play/{idx}")]
pub async fn play(state: web::Data<AppState>,
                  idx: web::Path<String>) -> impl Responder {

    let to_parse_idx = inner_trim(idx);

    let binary = state
        .binary_map
        .lock()
        .unwrap();
    
    let result = binary
        .get(&to_parse_idx)
        .map(|v|
             v.clone()
        );

    match result {
        Some(v) => {
            web::Bytes::from(v.data)
        },
        None => {
            web::Bytes::from(
                status::Status::PlayerBinaryNotFound
                    .as_string()
            )
        },
    }
}


/// wrap struct inside json
fn ok_json<T>(response: T) -> Result<web::Json<T>> {
    Ok(
        web::Json(
            response
        )
    )
}


/// unpack pattern and trim
fn inner_trim(idx: web::Path<String>) -> String {
    idx
        .into_inner()
        .trim()
        .to_string()
}


/// search for header key
fn verify_header(key: HeaderKey,
                 headers: &HeaderMap) -> Option<String> {

    match headers.get(key.as_string()) {
        Some(id) => {
            match id.to_str() {
                Ok(i) => Some(String::from(i)),
                Err(_) => None,
            }
        },
        None => None,
    }
}
