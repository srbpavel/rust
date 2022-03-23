/// CONTENT
use crate::handler::AppState;
use actix_web::{
    web::{self, BufMut, Bytes, BytesMut},
    Error,
    //HttpRequest,
    HttpResponse,
    Responder,
    Result,
};
//use log::debug;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};

const PATH_DELIMITER: char = '/';

/// types for hash_maps
pub type ContentKey = String;
//pub type ContentValue = Content;
pub type BinaryValue = BytesMut;
//pub type BinaryValue = Vec<u8>;

/// content
#[derive(Debug, Serialize, Clone, Deserialize, PartialEq)]
pub struct Content {
    id: String,
}

impl Content {
    /// default
    pub fn default() -> Self {
        Self {
            id: String::from(""),
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
    //req: HttpRequest,
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    //debug!("PUT_P:\n{req:?}\n{path:?}");

    let AppState {
        //content_map,
        binary_map,
    } = &*state.into_inner();

    let new_content = Content {
        id: path.into_inner(),
        ..Content::default()
    };

    let mut buf = web::BytesMut::new();
    //let mut buf = web::BytesMut::with_capacity(1024);
    //let mut buf = Vec::new();

    let mut chunk_counter = 0;

    while let Some(chunk) = payload.next().await {
        chunk_counter += 1;

        // FIRST CHUNK
        if chunk_counter == 1 {
            //debug!("FIRST CHUNK\n{:?}", new_content);

            /*
            // LOCK DATA
            let mut content_hashmap = content_map.lock().unwrap();
            content_hashmap.insert(new_content.id.clone(), new_content.clone());
            */
        }

        /*
        // NOT THERE YET
        // the trait `FromResidual<Result<Infallible, PayloadError>>`
        // is not implemented for `BytesMut`
        //
        buf = web::block(move || {
            //buf.extend_from_slice(&*chunk?);
            buf.put(&*chunk?);

            buf
        }).await?;
        */

        /*
        let th = std::thread::spawn(move || {
            buf.put(&*chunk?);
        });

        th.join().unwrap();
        */

        // BLOCKING
        //buf.extend_from_slice(&chunk?);
        buf.put(&*chunk?);

        // LOCK DATA
        //let mut binary_hashmap = binary_map.lock().unwrap();

        let binary_hashmap = &binary_map;
        binary_hashmap.insert(new_content.id.clone(), buf.clone());
        //binary_hashmap.insert(new_content.id.clone(), buf.to_vec().clone());
    }

    Ok(HttpResponse::Ok().body(new_content.id))
}

/// get_content
///
pub async fn get_content(
    //req: HttpRequest,
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> impl Responder {
    //debug!("GET: {req:?}\n{path:?}");

    let mut content_id = path.into_inner();

    content_id = match content_id.strip_suffix(PATH_DELIMITER) {
        Some(c) => String::from(c),
        None => content_id,
    };

    //debug!("ID: {content_id}");

    //let all_content = state.binary_map.lock().unwrap();
    let all_content = &state.binary_map;
    
    //let result = all_content.get(&content_id).cloned();
    let result = all_content.get(&content_id);

    let data = match result {
        //Some(v) => Bytes::from(v),
        Some(v) => Bytes::from(v.clone()),
        None => Bytes::from("GET data error"),
    };

    HttpResponse::Ok()
        .insert_header(("content-type", "application/octet-stream"))
        .insert_header(("content-encoding", "chunked"))
        .body(data)
}

/// delete_content
///
pub async fn delete_content(
    //req: HttpRequest,
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> impl Responder {
    //debug!("DELETE: {req:?}");

    let mut content_id = path.into_inner();

    content_id = match content_id.strip_suffix(PATH_DELIMITER) {
        Some(c) => String::from(c),
        None => content_id,
    };

    //let mut binary_hashmap = state.binary_map.lock().unwrap();
    //let mut binary_hashmap = &mut *state.binary_map;

    //let result = match binary_hashmap.remove(&content_id) {
    let result = match &mut state.binary_map.remove(&content_id) {
        Some(_) => "Status::DeleteOk",
        None => "Status::DeleteBinaryError",
    };

    /*
    let mut content_hashmap = state.content_map.lock().unwrap();

    let result = match content_hashmap.get_mut(&content_id) {
        Some(_) => match content_hashmap.remove(&content_id) {
            Some(_) => {
                let mut binary_hashmap = state.binary_map.lock().unwrap();

                match binary_hashmap.remove(&content_id) {
                    Some(_) => "Status::DeleteOk",
                    None => "Status::DeleteBinaryError",
                }
            }
            None => "Status::DeleteContentError",
        },
        None => "Status::ContentIdNotFound",
    };
    */

    HttpResponse::Ok().body(result)
}

/*
/// list_content
///
pub async fn list_content(
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {

    debug!("LIST:\n{req:?}\n{path:?}");

    HttpResponse::Ok().body("LIST")
}
*/
