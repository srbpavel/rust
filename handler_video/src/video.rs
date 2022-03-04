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

/// scope
pub const SCOPE: &str = "/video";

/// types hash_maps
pub type VideoKey = String;
pub type VideoValue = Video;
pub type BinaryValue = Binary;

/// binary
#[derive(Debug, Clone)]
pub struct Binary {
    pub data: BytesMut,
    pub filename: String,
}

/// detail
#[derive(Serialize, Debug, Clone, PartialEq)]
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

/// file error // FUTURE USE
#[derive(Serialize, Deserialize)]
struct File {
    err: String,
}

/// all videos
#[derive(Serialize, Debug)]
pub struct IndexResponse {     
    video_map: HashMap<VideoKey, VideoValue>,
    status: String,
}

/// group members
#[derive(Serialize, Debug)]
pub struct ListResponse {     
    result: Option<HashMap<VideoKey, VideoValue>>,
    status: String,
}

/// all groups
#[derive(Serialize, Debug)]
pub struct GroupsResponse {     
    result: Option<Vec<String>>,
    status: String,
}

/// upload 
#[derive(Serialize, Debug)]
pub struct UploadResponse {
    result: Option<PostOk>,
    status: String,
}

/// valid upload
#[derive(Serialize, Debug)]
pub struct PostOk {
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
    result: Option<Video>,
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


/// index list all videos
///
/// for debug purpose as tested with dozen records not milions yet
///
#[get("")]
pub async fn index(state: web::Data<AppState>,
                   _req: HttpRequest) -> Result<web::Json<IndexResponse>> {

    let all_videos = state                                  
        .video_map
        .lock()                                      
        .unwrap();                                   

    //debug!("ALL_VIDEOS: {all_videos:?}");
    
    Ok(                                              
        web::Json(                                   
            IndexResponse {                          
                video_map: all_videos.clone(),
                status: status::Status::StatusOk.as_string(),
            }                                        
        )                                            
    )                                                
}


/// detail via hash
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

    let video = state
        .video_map
        .lock()
        .unwrap();

    let mut status;
    
    let result = match parsed_idx {
        Some(i) => {
            status = status::Status::VideoIdNotFound;
            
            video.get(&i).map(|v| {
                status = status::Status::VideoIdFound;

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
                url,
                status: status.as_string(),
            }
        )
    )
}


/// flush video_map Hash
///
#[post("/clear")]
pub async fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {

    let mut all_videos = state
        .video_map
        .lock()
        .unwrap();

    let mut all_groups = state
        .groups
        .lock()
        .unwrap();
    
    all_videos.clear();
    all_groups.clear();
    
    Ok(
        web::Json(
            IndexResponse {
                video_map: HashMap::new(),
                status: status::Status::StatusOk.as_string(),
            }
        )
    )
}


/// DELETE via id
/// 
pub async fn delete(state: web::Data<AppState>,
                    idx: web::Path<String>) -> Result<web::Json<DeleteResponse>> {

    //debug!("IDX: {idx:?}");
    
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

    //debug!("PARSED_IDX: {parsed_idx:?}");
    
    let mut video_hashmap = state
        .video_map
        .lock()
        .unwrap();

    let mut binary_hashmap = state
        .binary_map
        .lock()
        .unwrap();

    // even we get string, we still parse as later we will get id as usize/...
    let result = match parsed_idx {
        Some(i) => {
            match video_hashmap.get_mut(&i) {
                Some(_) => {
                    match video_hashmap.remove(&i) {
                        Some(_) => {
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
                result: result.as_string(),
            }                                        
        )
    )
}


/// UPDATE group_id for video
/// 
///
pub async fn update_group(update: web::Json<UpdateInput>,
                          state: web::Data<AppState>) -> actix_web::Result<web::Json<DetailResponse>> {
    
    //debug!("UPDATE_VIDEO: {update:?}");    

    let url = format!("{}/update/group",
                      SCOPE,
    );
    
    let mut video_hashmap = state
        .video_map
        .lock()
        .unwrap();

    let mut groups_list = state
        .groups
        .lock()
        .unwrap();
    
    let status;

    let clone_video_hashmap = video_hashmap.clone();
    
    let result = match video_hashmap.get_mut(&update.video_id) {
        Some(video) => {
            let original_group = video.group.clone();

            // update group
            video.group = update.group_id.to_string();

            // new group to vec
            if !groups_list.contains(&video.group) {
                groups_list.push(video.group.clone());
            }

            // delete group if this was last member
            let group_map: HashMap<VideoKey, VideoValue> = clone_video_hashmap
            //let group_map: HashMap<VideoKey, VideoValue> = video_hashmap
                .into_iter()
                .filter(|(_,value)|
                        value.group.eq(&original_group)
                )
                .collect();

            // this should not work? as from clone
            if group_map.is_empty() {
                groups_list
                    .retain(|g|
                            g.eq(&original_group)
                    );
            }
            
            status = status::Status::UpdateOk;
            
            Some(
                Video {
                    id: video.id.clone(),
                    group: update.group_id.to_string(),
                    name: video.name.clone(),
                }
            )
        },
        None => {
            status = status::Status::UpdateError;

            None}
        ,
    };
    
    Ok(
        web::Json(
            DetailResponse {
                result,
                url,
                status: status.as_string(),
            }
        )
    )
}


/// list group members
///
#[get("/list/{group_id}")]
pub async fn list_group(state: web::Data<AppState>,
                        idx: web::Path<String>) -> Result<web::Json<ListResponse>> {

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
                result,
                status: status.as_string(),
            }                                        
        )                                            
    )                                                
}


/// list all groups
///
#[get("/groups")]
pub async fn show_groups(state: web::Data<AppState>) -> Result<web::Json<GroupsResponse>> {

    let all_groups = state                                  
        .groups
        .lock()                                      
        .unwrap();                                   

    let status;
    
    let result = if all_groups.is_empty() {
        status = status::Status::NoGroupsAvailable;

        None
    } else {
        status = status::Status::GroupsAvailable;
            
        Some(all_groups.to_vec())
    };
    
    Ok(                                              
        web::Json(                                   
            GroupsResponse {                          
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
                          req: HttpRequest)  -> Result<web::Json<UploadResponse>> {

    //debug!("REQ: {req:?}");

    
    let mut video_hashmap = state
        .video_map
        .lock()
        .unwrap();

    let mut binary_hashmap = state
        .binary_map
        .lock()
        .unwrap();

    let mut groups_list = state
        .groups
        .lock()
        .unwrap();

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
                    UploadResponse {                          
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
                    UploadResponse {                          
                        result: None,
                        status: status::Status::EmptyGroupId.as_string(),
                    }
                )
            )
        },
    }
    
    //debug!("{new_video:#?}");

    let new_group = new_video.group.clone();

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

                match content_disposition.get_name() {
                    Some(name) => {
                        //debug!("\ndis_name: {name:?}");
                        new_video.name = String::from(name);
                    },
                    None =>{
                        return Ok(
                            web::Json(
                                UploadResponse {
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
                        //debug!("\ndis_filename: {filename:?}");

                        if !groups_list.contains(&new_group) {
                            groups_list.push(new_group.clone());
                        }

                        let mut buf = Binary {
                            data: BytesMut::with_capacity(1024),
                            filename: filename.to_string(),
                        };

                        /*
                        binary_hashmap
                            .insert(
                                new_video.id.clone(), // KEY: video.id
                                buf.clone(), // VALUE: Binary {}
                            );
                        */
                        
                        let mut chunk_counter = 0;
                        
                        while let Some(chunk) = field.try_next().await? {
                            chunk_counter += 1;
                            //debug!("chunk_counter: {chunk_counter}");
                            
                            if chunk_counter == 1 {
                                debug!("hash_create: {chunk_counter}");
                                //status = status::Status::UploadStarted;
                                
                                video_hashmap
                                    .insert(
                                        new_video.id.clone(), // KEY: video.id
                                        new_video.clone(), // VALUE: Video {}
                                    );
                            }
                            
                            buf.data.put(&*chunk);

                            binary_hashmap
                                .insert(
                                    new_video.id.clone(), // KEY: video.id
                                    buf.clone(), // VALUE: Binary {}
                                );
                        };

                        status = status::Status::UploadDone;
                    },
                    None =>{
                        return Ok(
                            web::Json(
                                UploadResponse {
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
                        UploadResponse {
                            result: None,
                            status: status::Status::TooManyForms.as_string(),
                        }
                    )
                )
            }
        }

    Ok(
        web::Json(
            UploadResponse {
                result: Some(
                    PostOk {
                        video: new_video,
                    }
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
            eprintln!("foookin INDEX: {to_parse_idx}\nREASON >>> {why}");

            None
        },
    };

    let binary = state
        .binary_map
        .lock()
        .unwrap();
    
    // join these two together
    let result = match parsed_idx {
        Some(i) => {
            binary.get(&i).map(|v|
                               Binary {
                                   data: v.data.clone(),  // niet goed !!!
                                   filename: v.filename.clone(),
                               }
            )
        },
        None => None,
    };
    
    match result {
        Some(v) => {
            let content = format!("form-data; filename={}",
                                  v.filename,
            );
            
            //debug!("CONTENT: {content:?}");
            
            HttpResponse::Ok()
                .append_header(
                    ("Content-Disposition",
                     content,
                    )
                )
                .body(v.data)
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

