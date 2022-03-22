use log::{debug,
          //error,
};
use actix_web::{
    web,
    Result,
    Error,
    HttpResponse,
    HttpRequest,
    Responder,
};

//use actix_multipart::Multipart;
use futures_util::{
    StreamExt,
    //TryStreamExt,
};

const PATH_DELIMITER: char = '/';

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
pub async fn put_content_p(mut payload: web::Payload,
                           req: HttpRequest,
                           path: web::Path<String>) -> Result<HttpResponse, Error> {

    debug!("PUT_P:\n{req:?}\n{path:?}");

    let result = path.into_inner();

    let parts = &result
        .rsplitn(2, PATH_DELIMITER)
        .collect::<Vec<_>>();

    debug!("PARTS[{}]: {:?}",
           parts.len(),
           parts,
    );

    /*
    let parts = result
        .split(DELIMITER)
        .collect::<Vec<_>>();
    
    let mut len = parts.len();
    let mut prefix = String::from("");
    let mut content_id = String::from("");
    
    parts
        .iter()
        .for_each(|p| {
            len -= 1;

            if len.eq(&0) {
                content_id = p.to_string();
            } else {
                prefix = format!("{}{}{}",
                                 prefix,
                                 DELIMITER,
                                 p,
                )
            }
        });
    
    debug!("\nPATH: {}\nPREFIX: {}\nID: {}",
           &result,
           prefix,
           content_id,
    );
    */

    let mut bytes = web::BytesMut::new();

    let mut chunk_counter = 0;
    
    while let Some(item) = payload
        .next()
        .await {
            chunk_counter += 1;
            
            // FIRST CHUNK
            // here we will create record so we can GET content
            if chunk_counter == 1 {
                debug!("FIRST CHUNK");
            }
            
            bytes
                .extend_from_slice(
                    &item?
                );
    }    

    Ok(
        HttpResponse::Ok()
            //.body(result)
            .body(bytes)
    )
}


/// get_content
/// 
pub async fn get_content(req: HttpRequest,
                         path: web::Path<String>) -> impl Responder {

    debug!("GET: {req:?}\n{path:?}");

    let result = path.into_inner();
    
    HttpResponse::Ok()
        .body(result)
}


/// delete_content
/// 
pub async fn delete_content(req: HttpRequest,
                            path: web::Path<String>) -> impl Responder {

    debug!("DELETE: {req:?}");

    let result = path.into_inner();
    
    HttpResponse::Ok()
        .body(result)
}


/* // OBSOLOTE 
/// put_content MULTIPART
/// 
pub async fn put_content_m(mut payload: Multipart,
                           req: HttpRequest,
                           path: web::Path<String>) -> Result<HttpResponse, Error> {

    debug!("PUT_M:\n{req:?}\n{path:?}");

    let result = path.into_inner();
    
    while let Some(mut field) = payload
        .try_next()
        .await? {
            let mut chunk_counter = 0;

            while let Some(chunk) = field
                .try_next()
                .await? {
                    chunk_counter += 1;
                    
                    // FIRST CHUNK
                    // here we will create record so we can GET
                    if chunk_counter == 1 {
                        debug!("FIRST CHUNK");
                    }
                    
                    let _ch = &*chunk;
                    debug!("CHUNK:\n{chunk_counter}");
                };
        }
    
    Ok(
        HttpResponse::Ok()
            //.finish()
            .body(result)
    )
}
*/
