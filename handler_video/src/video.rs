use crate::AppState;

use actix_web::{
    get,
    //post,
    web,
    //Result,

    HttpResponse,
    Responder,

    /* SAVE EXAMPLE
    middleware,
    HttpServer,
    */
    //Error,
};


use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use std::io::Write;
use uuid::Uuid;

use serde::{Serialize,
            Deserialize,
};

use std::collections::HashMap;

use std::sync::atomic::{AtomicUsize,
                        Ordering,   
};                                  


static VIDEO_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);            
static VIDEO_ID_ORD: Ordering = Ordering::SeqCst;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Video {
    id: usize,
    body: String,
    //path: String,
}

#[derive(Serialize, Debug)]
pub struct IndexResponse {     
    server_id: usize,      
    request_count: usize,  
    video_map: HashMap<usize, String>, 
}

#[derive(Deserialize, Debug)]
pub struct PostInput {
    // FUTURE USE
    //video: String, 
}

#[derive(Serialize, Debug)]
pub struct PostResponse {
    server_id: usize,
    request_count: usize,
    video: Video,
}

#[derive(Serialize, Debug)]                               
pub struct SearchResponse {                                   
    server_id: usize,                                     
    request_count: usize,                                 
    result: Option<Video>, // None in JSON will be "null"
    path: String,
}

/// json ECHO
///
/// curl -X POST 'http://127.0.0.1:8081/echo' -H "Content-Type: application/json" -d '{"video": "123456"}'
///
//#[post("/echo")] // specify at at App + resource + route
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok()
        .body(req_body)
}

/// list all VIDEO's
///
/// curl 'http://127.0.0.1:8081/video/all'
///
#[get("/all")]
async fn all() -> HttpResponse {
    HttpResponse::Ok()
        .body("list: all video\n")
}

/*
/// single VIDEO detail
///
/// curl 'http://127.0.0.1:8081/video/{id}'
///
#[get("/detail/{id}")]
async fn detail(path: web::Path<u32>) -> HttpResponse {
    HttpResponse::Ok()
        .body(
            format!("video_detail: {:?}\n",
                    path
                    .into_inner(),
                    //.0,
            ),
        )
}
*/

/// SEARCH via hash
/// 
/// path as String
/// i did not make it work for usize because do no fing way to verify valid usize?
///
/// curl 'http://localhost:8081/video/detail/{idx}'
///
#[get("/detail/{id}")]
pub async fn detail(state: web::Data<AppState>,
                    idx: web::Path<String>) -> actix_web::Result<web::Json<SearchResponse>> {

    //println!("IDX: {idx:?}");
    
    // deconstruct to inner value
    let to_parse_idx = idx.into_inner();

    let path = format!("/video/detail/{}", // take this from req
                       to_parse_idx,
    );

    // let's try parse
    let parsed_idx = match to_parse_idx.parse::<usize>() {
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

    // we lock msg vec
    let video = state
        .video_map
        .lock()
        .unwrap();

    //println!("MS: {ms:?}");

    let result = match parsed_idx {
        Some(i) =>  
            match video.get(&i) {
                Some(v) => Some(
                    Video {
                        id: i,
                        body: v.to_string(),
                    }
                ),
                None => None,
            },
        None => None,
    };
    
    //println!("RESULT: {result:?}");
    
    Ok(
        web::Json(
            // let's build struct for json
            SearchResponse {
                server_id: state.server_id,
                request_count:request_count,
                result: result,
                path: path,
            }
        )
    )
}

/*
/// INDEX get info
///
///
pub async fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>video upload test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}
*/

/// index list all videos
///
/// curl 'http://localhost:8081/video/'
///
//#[get("/")]
pub async fn index(state: web::Data<AppState>) -> actix_web::Result<web::Json<IndexResponse>> {
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
                request_count: request_count,        
                video_map: video.clone(),
            }                                        
        )                                            
    )                                                
}

/// 
/// curl -X PUT 'http://localhost:8081/video/put
///
/// curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F ahoj=vole -F yeah=baby
///
/// curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F ahoj=vole -F yeah=baby -F "image=@info.txt;type=text/plain"
///
/// curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F "now_text=@now.txt;type=text/plain"
///
/// WE DO NOT get JSON here as we get data via PayLOAD, will be enough?
pub async fn insert_video(mut payload: Multipart,
                          //state: web::Data<AppState>) -> Result<HttpResponse, Error> {
                          //state: web::Data<AppState>) -> Result<web::Json<PostResponse, Error>> {
                          state: web::Data<AppState>) -> actix_web::Result<web::Json<PostResponse>> {
    println!("PUT:");

    // Cell
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    // we lock and have access to HashMap messages
    let mut video = state
        .video_map // HASH
        .lock() // get access to data inside Mutex + blocks until another thread
        .unwrap(); // -> MutexGuard<Vec<String>> // will panic on Err !!!
    // /CLEAR do not reset counter, yet.
    let video_id = VIDEO_ID_COUNTER.fetch_add(1,              
                                              VIDEO_ID_ORD,
    );

    let mut video_name = String::from("VIDEO_NAME");
    let dir = "./tmp/";
    //let mut filepath = format!("{dir}{video_id}_{video_name}");
    
    /*
    // HASH
    video.insert(
        video_id,
        //msg.video.clone(), // we do no get json here
        String::from("VIDEO_BODY")
    );
    */

    
    // iterate over multipart stream
    while let Some(mut field) = payload
        .try_next()
        .await? {

            let content_disposition = field.content_disposition();
            
            let _fff = match content_disposition {
                Some(dis) => {
                    video_name = match dis.get_name() {
                        Some(name) => String::from(name),
                        None => String::from("VIDEO_NAME"),
                    };

                    println!("DIS: {:?}\nfilename: {:?}\nname: {:?}",
                             dis,
                             dis.get_filename(),
                             //dis.get_name(),
                             video_name,
                    );

                    // HASH
                    video.insert(
                        video_id,
                        //msg.video.clone(), // we do no get json here
                        //String::from("VIDEO_BODY")
                        //video_name.clone(),
                        video_id.clone().to_string(),
                    );
                    
                    let filename = dis
                        .get_filename()
                        // if not filename -> generate uuid as new filenames
                        .map_or_else(||
                                     Uuid::new_v4().to_string(),
                                     sanitize_filename::sanitize,
                        );

                    let filepath = format!("{}{}_{}",
                                           dir,
                                           video_id,
                                           filename,
                    );
                    
                    /*
                    filepath = format!("{}{}_{}",
                                       dir,
                                       video_id,
                                       filename,
                    );
                    */

                    println!("FILENAME:{:?}\nPATH:{:?}",
                             filename,
                             filepath,
                    );

                    // block -> future to result
                    //https://docs.rs/actix-web/latest/actix_web/web/fn.block.html
                    let mut f = web::block(||
                                           //std::fs::File::create(filepath)
                                           std::fs::File::create(filepath)
                    ).await?;

                    println!("F:{:?}",
                             f,
                    );

                    // stream of *Bytes* object
                    while let Some(chunk) = field.try_next().await? {
                        //println!("CHUNK: {:#?}", chunk);

                        f = web::block(move ||
                                       f
                                       .write_all(&chunk)
                                       .map(|_| f)
                        )
                            .await?//?
                            ;
                    };
                },

                None => {},
            };
        }

    /*
    Ok(
        HttpResponse::Ok()
            .into()
    )
    */

    Ok(web::Json(
        PostResponse {
            server_id: state.server_id, // here is our messages: Vec
            request_count: request_count,
            video: Video {
                //body: msg.video.clone(),
                //body: String::from("VIDEO_BODY"),
                body: video_name,
                id: video_id,
                //path: filepath.clone(),
            },
        }
    ))
}

