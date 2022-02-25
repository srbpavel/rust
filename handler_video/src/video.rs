//use crate::AppState;

use actix_web::{
    get,
    post,
    web,
    //Result,

    HttpResponse,
    Responder,
};

/*
use serde::{Serialize,
            Deserialize,
};
*/

//use std::collections::HashMap;

//static VIDEO_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);            
//static VIDEO_ID_ORD: Ordering = Ordering::SeqCst;

/*
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Video {
    body: String,
    id: usize,
}
*/


/// json ECHO
///
/// curl -X POST 'http://127.0.0.1:8081/echo' -H "Content-Type: application/json" -d '{"video": "123456"}'
///
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
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

