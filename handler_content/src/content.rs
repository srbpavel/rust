/// CONTENT
use crate::handler::AppState;
use actix_web::{
    web::{self,
          Bytes,
          BytesMut,
          //BufMut,
    },
    Error, HttpRequest, HttpResponse, Responder, Result,
};
use log::debug;
//use std::future::Future;
use futures_util::StreamExt;
use serde::{Serialize,
            Deserialize,
};

const PATH_DELIMITER: char = '/';

/// types for hash_maps
pub type ContentKey = String;
pub type ContentValue = Content;
pub type BinaryValue = BytesMut;

/// content
#[derive(Debug, Serialize, Clone, Deserialize, PartialEq)]
pub struct Content {
    id: String,
    //group: Option<String>,
}

impl Content {
    /// default
    pub fn default() -> Self {
        Self {
            id: String::from(""),
            //group: None,
        }
    }
}

/// put_content PAYLOAD
///
/// https://docs.rs/actix-web/latest/actix_web/web/struct.Payload.html
///
/// cat /home/conan/video/youtube/lines_twenty_thousand_leagues_under_the_sea_by_jules_verne.txt | curl -v -X PUT "http://127.0.0.1:8081/foo/bar" --no-buffer --limit-rate 100K -T -
///
///
/// curl --verbose -X PUT http://localhost:8081/foo/bar/456 -d "1234567890"
///
/// ./chunk.sh
/// ./ccc.sh
///
/// cat /home/conan/video/youtube/lines_twenty_thousand_leagues_under_the_sea_by_jules_verne.txt | curl -v -X PUT -H "Transfer-Encoding: chunked" -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/foo/bar" -F "ts=@-;type=text/plain" -H "video_id: verne_piped" -H "group: chunk_tester" --no-buffer --limit-rate 100K
///
pub async fn put_content_p(
    mut payload: web::Payload,
    //mut data: web::Bytes,
    req: HttpRequest,
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    debug!("PUT_P:\n{req:?}\n{path:?}");

    // /*
    let AppState { content_map, binary_map } = &*state.into_inner();

    debug!("ALL_CONTENT: {:?}",
           content_map,
    );
    // */

    let result = path.into_inner();

    let mut new_content = Content::default();
    new_content.id = String::from(result);
    
    //let mut bytes = web::BytesMut::new();
    let mut buf = web::BytesMut::new();
    //let mut buf = BytesMut::with_capacity(1024);    

    let mut chunk_counter = 0;

    // via PAYLOAD
    while let Some(chunk) = payload.next().await {
        chunk_counter += 1;

        // FIRST CHUNK
        if chunk_counter == 1 {
            debug!("FIRST CHUNK\n{:?}",
                   new_content,
            );

            // LOCK DATA
            //let content_hashmap = &mut *state.content_map;
            // /*
            let mut content_hashmap = content_map
                .lock()
                .unwrap();
            // */
            
            content_hashmap
                .insert(
                    new_content.id.clone(),
                    new_content.clone(),
                );
        }

        /* // NOT YET
        buf = web::block(move || {
            //let ch = &*chunk;
            //debug!("CHUNK: {chunk_counter}");
            
            // bytes.extend_from_slice(&item?);
            buf.extend_from_slice(&*item?);
            
            /*
            buf
            .put(&*item?);
            //.put(ch);
            */
            
            buf
        }).await?;
        */

        // BLOCKING 
        buf.extend_from_slice(&chunk?);

        // LOCK DATA
        // let binary_hashmap = &mut *state.binary_map;
        // /*
        let mut binary_hashmap = binary_map
            .lock()
            .unwrap();
        // */

        binary_hashmap
            .insert(
                new_content.id.clone(),
                buf.clone(),
            );
    }
    
    Ok(HttpResponse::Ok().body(new_content.id))
}

/// get_content
///
pub async fn get_content(req: HttpRequest,
                         path: web::Path<String>,
                         state: web::Data<AppState>) -> impl Responder {

    debug!("GET: {req:?}\n{path:?}");
    
    let mut content_id = path.into_inner();

    content_id = match content_id.strip_suffix(PATH_DELIMITER) {
        Some(c) => String::from(c),
        None => content_id,
    };
    
    debug!("ID: {content_id}");

    // let all_content = &state.binary_map;
    // /*
    let all_content = state
        .binary_map
        .lock()
        .unwrap();
    // */

    let result = all_content
        .get(&content_id)
        .cloned();

    /*
    debug!("RESULT: {:?}",
           result,
    );
    */

    let data = match result {
        Some(v) => {
            Bytes::from(v)
        },
        None => {
            Bytes::from(
                "GET data error"
            )
        },
    };

    HttpResponse::Ok()
        .insert_header(("content-type", "application/octet-stream"))
        .insert_header(("content-encoding", "chunked"))
        .body(data)
}

/// delete_content
///
pub async fn delete_content(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    debug!("DELETE: {req:?}");

    let result = path.into_inner();

    HttpResponse::Ok().body(result)
}

