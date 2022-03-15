use crate::{
    handler::AppState,
    status::Status,
    error::VideoError,
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
use std::fmt::{self,
               Debug,
               Display,
};
use std::cmp::PartialOrd;

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
//#[derive(Serialize, Debug, Clone, Deserialize)]
#[derive(Serialize, Clone, Deserialize, PartialEq)]
pub struct Video {
    id: String,
    group: String,
    name: String,
}


impl AsRef<str> for Video {
    fn as_ref(&self) -> &str {
        &self.id
    }
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


///
/// this also get us .to_string()
///
impl fmt::Display for Video {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "video >>> name: '{}' with id: '{}' is in group: '{}'",
               self.name,
               self.id,
               self.group,
               )
    }
}


///
/// for this you need to remove Debug from Struct derive
///
impl fmt::Debug for Video {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Video")
            .field("name", &self.name)
            .field("id", &self.id)
            .field("group", &self.group)
            .finish()
    }
}


trait MyDebug {
    fn my_debug(&self) -> String;
}

impl MyDebug for Video {
    fn my_debug(&self) -> String {
        format!("Video >>> \n id: <{}>\n name: <{}>\n group: <{}> }}",
                self.id,
                self.name,
                self.group,
        )
    }
}


/// all videos
#[derive(Serialize, Debug)]
pub struct IndexResponse {     
    result: Option<HashMap<VideoKey, VideoValue>>,
    status: String,
}

/// detail
#[derive(Serialize, Debug, Deserialize)]                               
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
// properties in handler, as with macro i cannot perform testing
//#[get("/detail/{video_id}")]
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
fn resp_json<T: Serialize>(response: T) -> HttpResponse {
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


/// header error example
fn verify_header_result(key: HeaderKey,
                        headers: &HeaderMap) -> Result<String, VideoError> {

    match headers.get(key.as_string()) {
        Some(id) => {
            match id.to_str() {
                Ok(i) => Ok(String::from(i)),
                Err(_) => Err(VideoError::HeadersError(
                    format!("headers <{}> not found",
                             key.as_string(),
                    )
                        .into(),
                )),
            }
        },
        None => Err(VideoError::HeadersError(
            format!("headers <{}> not found",
                    key.as_string(),
            )
                .into(),
        )),
    }
}


/// path tester
///
#[get("/data/{id}/{group}/{name}")]
pub async fn data(state: web::Data<AppState>,
                  path: web::Path<Video>) -> impl Responder {

    let v = path.clone();
    
    debug!("data_IN: \n#Debug:\n {:#?}",
           &v
    );

    print_it(path
             .group
             .clone()
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

        debug!("data_OUT: \n#Debug:\n {:?}",
               v,
        );

        debug!("data_OUT: \n#MyDebug:\n {}",
               v.my_debug(),
        );

        print_it(v);
        print_it(v.as_ref());

        print_it("str");
        print_it(String::from("String"));
        
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
/// if parent dir handled via service + show_files, file not via this fn
///
/// curl -v "http://127.0.0.1:8081/video/now.txt"|cat -n|less
///
pub async fn single_file(req: HttpRequest) -> Result<NamedFile> {

    let path: std::path::PathBuf = req
        .match_info()
        .query("filename")
        .parse()?;
    
    debug!("single_file_handler: {path:?}");

    Ok(
        NamedFile::open(path)?
    )
}


///
fn print_it<T>(input: T)
where
    T: AsRef<str> + Debug + Display,
{
    debug!("PRINT_IT: {}", input)
}


///
fn gives_higher<T>(one: T, two: T) -> Option<T>
where
    T: PartialOrd + Display + Debug + Copy
{
    let result = if one > two {
        one
    } else if one.eq(&two) {
        debug!("{one} VS {two} -> are equal");
        
        return None
    } else {
        two
    };
    
    debug!("{one} VS {two} -> {:?} is higher", result);
    
    Some(result)
}


///
#[get("/compare/float/{first}/{second}")]
pub async fn compare(path: web::Path<(f64, f64)>) -> impl Responder {
    
    let (aaa, bbb) = path.into_inner();
    
    let h = gives_higher(
        aaa,
        bbb,
    );
    
    HttpResponse::Ok()
        .body(
            format!("{h:?}\n")
        )
}


/// header error handle
///
pub async fn insert_header(mut _payload: Multipart,
                           _state: web::Data<AppState>,
                           req: HttpRequest) -> Result<HttpResponse, VideoError> {

    match verify_header_result(HeaderKey::VideoId,
                               req.headers(),
    ) {
        Ok(h) => {
            //debug!("HEADER_ok: {h}");

            Ok(
                HttpResponse::Ok()
                    .body(
                        format!("Ok header --> video_id: <{h}>\n")
                    )
            )
        },
        
        Err(why) => {
            //debug!("HEADER_err: {why:?}");

            Err(why)
        },
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test,
        App,
    };

    use crate::handler::AppState;
    
    use std::{
        sync::{Arc,                
               Mutex,              
        },
        collections::HashMap,
    };

    async fn resp_ok(_req: HttpRequest) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
    
    async fn resp_bad(_req: HttpRequest) -> HttpResponse {
        HttpResponse::BadRequest().finish()
        // simulate FAILED
        //HttpResponse::Ok().finish()
    }
    
    #[actix_web::test]
    // cargo test test_index_ok -- /home/conan/soft/rust/handler_video/src/handler_video_config.toml
    async fn test_index_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();

        let resp = resp_ok(req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    // cargo test test_not_ok -- /home/conan/soft/rust/handler_video/src/handler_video_config.toml
    async fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();

        let resp = resp_bad(req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
    
    #[actix_web::test]
    // cargo test verify_detail -- /home/conan/soft/rust/handler_video/src/handler_video_config.toml --nocapture
    async fn verify_detail() {

        let video_id = "123456";
        
        // appstate with single record
        let all_state = web::Data::new(
            AppState {
                video_map:
                Arc::new(                        
                    Mutex::new(
                        HashMap::from([
                            (String::from(video_id),
                             Video {
                                 id: String::from(video_id),
                                 group: String::from("da_tester"),
                                 name: String::from("in_test_video"),
                             },
                            )
                        ])
                    )
                ),
                binary_map:
                Arc::new(                        
                    Mutex::new(
                        HashMap::new()
                    )
                ),
            }
        );

        // initial service with get method and fn detail()
        let app = test::init_service(
            App::new()
                .app_data(all_state.clone())
                .route("/video/detail/{video_id}",
                       web::get().to(detail),
                ),
        )
        .await;

        // build get url with video_id
        let req = test::TestRequest::get()
            // /*
            .uri(&format!(
                "/video/detail/{}",
                video_id,
            ))
            // */
            // test FAIL
            //.uri("/video/detail/789")
            .to_request();
        
        let resp = test::call_and_read_body(&app,
                                            req,
        ).await;

        // http_response bytes to json
        let obj = serde_json::from_slice::<DetailResponse>(&resp)
            .unwrap()
            .result
            .unwrap();

        //println!("T_resp: {resp:?}\n {obj:#?}\n id: {:?}", obj.id);

        assert_eq!(
            video_id,
            // test FAIL
            //"789",
            obj.id,
        );
    }
}

