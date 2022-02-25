//use crate::AppState;

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
    Error,
};


use actix_multipart::Multipart;

use futures_util::TryStreamExt;
//use futures_util::stream::try_stream::TryStreamExt;

use std::io::Write;
use uuid::Uuid;

/* SAVE EXAMPLE
use std::io::Write;
use futures_util::TryStreamExt as _;
use uuid::Uuid;
*/

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

/// 
/// curl -X PUT 'http://localhost:8081/video/put
///
/// curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F ahoj=vole -F yeah=baby
///
/// curl -X PUT -H "Content-type: multipart/form-data" 'http://localhost:8081/video/put' -F ahoj=vole -F yeah=baby -F "image=@info.txt;type=text/plain"
///
pub async fn insert_video(mut payload: Multipart) -> Result<HttpResponse, Error> {
    println!("PUT:");

    // iterate over multipart stream
    while let Some(mut field) = payload
        .try_next()
        .await?/*.unwrap()*//*?*/ {
            
            let content_disposition = field.content_disposition();
            
            println!("CONTENT: {:?}",
                     content_disposition,
            );
            
            let fff = match content_disposition {
                Some(f) => {
                    println!("F: {f:?}\nfilename: {:?}\nname: {:?}",
                             f.get_filename(),
                             f.get_name(),
                    );

                    let filename = f
                        .get_filename()
                        // generate uuid as new filenamesss
                        .map_or_else(||
                                     Uuid::new_v4().to_string(),
                                     sanitize_filename::sanitize,
                        );

                    let filepath = format!("./tmp/{}", filename);

                    println!("FILENAME:{:?}\nPATH:{:?}",
                             filename,
                             filepath,
                    );

                    let mut ff = web::block(||
                                            std::fs::File::create(filepath)
                    ).await;

                    println!("F:{:?}",
                             ff,
                    );

                    while let Some(chunk) = field
                        .try_next()
                        .await
                        .unwrap()/*?*/ {
                        // filesystem operations are blocking, we have to use threadpool
                        f = web::block(move ||
                                       f
                                       .write_all(&chunk)
                                       .map(|_| f)
                        )
                                .await
                                .unwrap()
                                .unwrap()/*??*/;
                             
                },

                None => {},
            };
            
                
        /*
        let filename = content_disposition
            .get_filename()
            .map_or_else(||
                         Uuid::new_v4().to_string(),
                         sanitize_filename::sanitize,
            );

        let filepath = format!("./tmp/{}", filename);
        
        // File::create is blocking operation, use threadpool
        let mut f = web::block(||
                               std::fs::File::create(filepath))
            .await.unwrap().unwrap()/*??*/;
        
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await.unwrap()/*?*/ {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move ||
                           f
                           .write_all(&chunk)
                           .map(|_| f)
            )
                .await.unwrap().unwrap()/*??*/;
        }
        */
    }

    Ok(
        HttpResponse::Ok()
            .into()
    )

    
    /*
    let html = r#"<html>
        <head><title>INSERT video</title></head>
        <body>
        </body>
    </html>"#;

    Ok(
        HttpResponse::Ok()
            .body(html)
            )
    */
}

/*
/// SAVE example
///
///
pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {

    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await.unwrap()/*?*/ {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();
        
        let filename = content_disposition
            .get_filename()
            .map_or_else(||
                         Uuid::new_v4().to_string(),
                         sanitize_filename::sanitize,
            );

        let filepath = format!("./tmp/{}", filename);
        
        // File::create is blocking operation, use threadpool
        let mut f = web::block(||
                               std::fs::File::create(filepath))
            .await.unwrap().unwrap()/*??*/;
        
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await.unwrap()/*?*/ {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move ||
                           f
                           .write_all(&chunk)
                           .map(|_| f)
            )
                .await.unwrap().unwrap()/*??*/;
        }
    }

    Ok(
        HttpResponse::Ok()
            .into()
    )
}
*/
