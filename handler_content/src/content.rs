/// CONTENT
use crate::handler::AppState;
use actix_web::{
    web::{self, BufMut, Bytes, BytesMut},
    Error,
    HttpResponse,
    Responder,
    Result,
};
//use log::debug;
use futures_util::StreamExt;

const PATH_DELIMITER: char = '/';

/// types for maps
pub type ContentKey = String;
pub type BinaryValue = BytesMut;

/// content
#[derive(Debug, Clone)]
pub struct Content {
    id: String,
}

/* FUTURE USE
impl Content {
    /// default
    pub fn default() -> Self {
        Self {
            id: String::from(""),
        }
    }
}
*/

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
/// cat /home/conan/video/youtube/lines_twenty_thousand_leagues_under_the_sea_by_jules_verne.txt | curl -v -X PUT -H "Transfer-Encoding: chunked" -H "Content-type: multipart/form-data" "http://127.0.0.1:8081/jules/verne/twenty" -F "ts=@-;type=text/plain" -H "video_id: verne_piped" -H "group: chunk_tester" --no-buffer --limit-rate 100K
///
pub async fn put_content_p(
    mut payload: web::Payload,
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {

    let AppState { binary_map } = &*state.into_inner();

    let new_content = Content {
        id: path.into_inner(),
        // FUTURE USE if more fields later
        //..Content::default()
    };

    let mut buf = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        /*
        // NOT THERE YET
        // the trait `FromResidual<Result<Infallible, PayloadError>>`
        // is not implemented for `BytesMut`
        //
        buf = web::block(move || {
            buf.put(&*chunk?);

            buf
        }).await?;
        */

        // BLOCKING
        //buf.extend_from_slice(&chunk?);
        buf.put(&*chunk?);

        let binary_hashmap = &binary_map;

        binary_hashmap.insert(new_content.id.clone(), buf.clone());
    }

    Ok(HttpResponse::Ok().body("Status::UploadOk"))
}

/// get_content
///
/// watch curl --silent --verbose --no-buffer http://localhost:8081/foo/bar
///
/// curl --silent --verbose --no-buffer http://localhost:8081/jules/verne/twenty
///
/// curl --silent --verbose --no-buffer http://localhost:8081/foo/
/// curl --silent --verbose --no-buffer http://localhost:8081/foo
///
pub async fn get_content(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> impl Responder {

    let mut content_id = path.into_inner();

    content_id = match content_id.strip_suffix(PATH_DELIMITER) {
        Some(c) => String::from(c),
        None => content_id,
    };

    //debug!("ID: {content_id}");

    let all_content = &state.binary_map;

    let result = all_content.get(&content_id);

    let data = match result {
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
/// curl -X DELETE "http://127.0.0.1:8081/foo/bar/"
/// curl -X DELETE "http://127.0.0.1:8081/foo/bar"
///
pub async fn delete_content(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> impl Responder {

    let mut content_id = path.into_inner();

    content_id = match content_id.strip_suffix(PATH_DELIMITER) {
        Some(c) => String::from(c),
        None => content_id,
    };

    let result = match &mut state.binary_map.remove(&content_id) {
        Some(_) => "Status::DeleteOk",
        None => "Status::DeleteBinaryError",
    };

    HttpResponse::Ok().body(result)
}
