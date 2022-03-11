use crate::{
    handler::AppState,
    status::Status,
};
// /*
use log::{debug,
          //error,
};
// */
use actix_web::{get,
                post,
                web::{self,
                      Bytes,
                      BytesMut,
                      BufMut,
                },
                Result,
                HttpResponse,
                HttpRequest,
                Responder,
                http::header::HeaderMap,
                
};
use actix_multipart::Multipart;
use actix_files::NamedFile;
use futures_util::TryStreamExt;
use serde::{Serialize,
            Deserialize,
};
use std::collections::HashMap;
/*
use bytes::{BytesMut,
            BufMut,
};
*/

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
#[derive(Serialize, Debug, Clone, Deserialize)]
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

/// info
#[derive(Serialize, Debug)]
pub struct StatusResponse {     
    status: String,
}

/// req header keys
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
pub async fn all(state: web::Data<AppState>) -> impl Responder {

    let status;

    let all_videos = state                                  
        .video_map
        .lock()
        .unwrap();
    
    let result = if all_videos.is_empty() {
        status = Status::ListNone;
        
        None
    } else {
        status = Status::ListAll;
        
        Some(all_videos.clone())
    };

    resp_json(
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
                    idx: web::Path<String>) -> impl Responder {

    let to_parse_idx = inner_trim(idx);

    let mut status;
    status = Status::VideoIdNotFound;
    
    let video = state                                  
        .video_map
        .lock()
        .unwrap();

    let result = video.get(&to_parse_idx).map(|v| {
        status = Status::VideoIdFound;

        v.clone()
    });

    resp_json(
        DetailResponse {
            result,
            status: status.as_string(),
        }
    )
}


/// flush video_map + binary_map
///
#[post("/clear")]
pub async fn clear(state: web::Data<AppState>) -> impl Responder {

    let _vm = state
        .video_map
        .lock()
        .map(|mut v| v.clear()
        );

    let _bm = state
        .binary_map
        .lock()
        .map(|mut b| b.clear()
        );

    resp_json(
        IndexResponse {                          
            result: None,
            status: Status::ClearOk.as_string(),
        }
    )
}


/// DELETE video_id
/// 
pub async fn delete(state: web::Data<AppState>,
                    idx: web::Path<String>) -> impl Responder {

    let to_parse_idx = inner_trim(idx);

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
                            Status::DeleteOk
                        },
                        None => Status::DeleteBinaryError,
                    }
                },
                None => Status::DeleteDetailError,
            }
        },
        None => Status::VideoIdNotFound,
    };

    resp_json(
        StatusResponse {
            status: result.as_string(),
        }                                        
    )
}


/// list group members
///
#[get("/list/{group_id}")]
pub async fn list_group(state: web::Data<AppState>,
                        idx: web::Path<String>) -> impl Responder {

    let to_parse_idx = inner_trim(idx);

    let status;
    
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
   
    let result = if group_map.is_empty() {
        status = Status::GroupNotFound;

        None
    } else {
        status = Status::GroupFound;
            
        Some(group_map)
    };

    resp_json(
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

    let mut new_video = Video::default();

    let mut response = DetailResponse {                          
        result: None,
        status: Status::Init.as_string(),
    };
    
    new_video.id = match verify_header(HeaderKey::VideoId,
                                       req.headers(),
    ) {
        Some(value) => String::from(value),
        None => {
            response.status = Status::EmptyVideoId.as_string();

            return ok_json(response)
        },
    };

    new_video.group = match verify_header(HeaderKey::Group,
                                          req.headers(),
    ) {
        Some(value) => String::from(value),
        None => {
            response.status = Status::EmptyGroup.as_string();

            return ok_json(response)
        },
    };
    
    let mut content_counter = 0;

    let AppState { video_map, binary_map } = &*state.into_inner();

    // https://docs.rs/actix-web/latest/actix_web/web/struct.Payload.html
    // 
    // via actix_web::web::Payload instead actix_multipart::Multipart
    //
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
                            response.status = Status::EmptyFormName.as_string(); 

                            return ok_json(response)
                        }

                        new_video.name = String::from(name);
                    },
                    None => {
                        response.status = Status::EmptyFormName.as_string();

                        return ok_json(response)
                    },
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
                            let mut binary_hashmap = binary_map
                                .lock()
                                .unwrap();

                            binary_hashmap
                                .insert(
                                    new_video.id.clone(), // K: video.id
                                    buf
                                        .clone(), // V: Binary {}
                                );
                        };

                        response.status = Status::UploadDone.as_string();
                    },
                    None => {
                        response.status = Status::EmptyFormFilename.as_string();

                        return ok_json(response)
                    },
                }
            } else {
                response.status = Status::TooManyForms.as_string();
                
                return ok_json(response)
            }
        }

    response.result = Some(new_video);
    
    ok_json(
        response
    )
}


/// download
/// 
#[get("/download/{idx}")]
pub async fn download(state: web::Data<AppState>,
                      idx: web::Path<String>) -> impl Responder {

    let to_parse_idx = inner_trim(idx);

    let binary = state
        .binary_map
        .lock()
        .unwrap();

    let result = binary
        .get(&to_parse_idx)
        .cloned();
    
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
                    status: Status::VideoIdNotFound.as_string(),
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
        .cloned();

    match result {
        Some(v) => {
            //web::Bytes::from(v.data)
            Bytes::from(v.data)
        },
        None => {
            //web::Bytes::from(
            Bytes::from(
                Status::PlayerBinaryNotFound
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


/// json as http response
fn resp_json<T: serde::Serialize>(response: T) -> HttpResponse {
    HttpResponse::Ok()
        .json(
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


/// path tester
///
#[get("/{id}/{group}/{name}")]
pub async fn data(state: web::Data<AppState>,
                  path: web::Path<Video>) -> impl Responder {

    let v = path.clone();
    
    debug!("data_IN: {:#?}",
           &v
    );

    match path.group.as_str() {
        "raise_error" => {
            debug!("should raise 404");
            return HttpResponse::BadRequest().finish()
        },
        _ => {},
    }
    
    let mut status;
    status = Status::VideoIdNotFound;
    
    let video = state                                  
        .video_map
        .lock()
        .unwrap();

    let result = video.get(&path.id).map(|v| {
        status = Status::VideoIdFound;

        debug!("data_OUT: {:#?}",
               v,
        );
        
        v.clone()
    });

    resp_json(result)
}


/// stream experiment
/// 
#[get("/stream/{idx}")]
pub async fn stream(state: web::Data<AppState>,
                    idx: web::Path<String>) -> impl Responder {

    let to_parse_idx = inner_trim(idx);

    let binary = state
        .binary_map
        .lock()
        .unwrap();

    let result = binary
        .get(&to_parse_idx)
        .cloned();
    
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
                .append_header(
                    ("Content-Type",
                     v.mime,
                    )
                )
                .body(
                    v.data
                )
        },
        None => {
            resp_json(
                &StatusResponse {
                    status: Status::VideoIdNotFound.as_string(),
                }
            )
        },
    }
}


/*
/// favicon static
///
///
#[get("/favicon")]
pub async fn favicon() -> Result<actix_files::NamedFile> {
    Ok(
        NamedFile::open(
            "static/favicon.ico"
        )?
    )
}
*/


/// return single file
///
/// curl -v "http://127.0.0.1:8081/video/static/ts.txt"|cat -n|less
/// curl -v "http://127.0.0.1:8081/video/now.txt"|cat -n|less
///
pub async fn single_file(req: HttpRequest) -> Result<NamedFile> {

    let path: std::path::PathBuf = req
        .match_info()
        .query("filename")
        .parse()?;
    //.unwrap();
    
    Ok(
        NamedFile::open(path)?
    )
}
