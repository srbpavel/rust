/// HANDLER
///
use crate::message;
use crate::video;

use actix_web::{
    web,
    middleware,
    App,
    HttpServer,
    HttpResponse,
    Responder,
};

use std::cell::Cell;                
use std::sync::atomic::{AtomicUsize,
                        Ordering,   
};                                  
                                    
use std::sync::{Arc,                
                Mutex,              
};

use std::collections::HashMap;

const NAME: &str = "HANDLER_VIDEO";
const SERVER: & str = "127.0.0.1";
const PORT: u64 = 8081;

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);
static SERVER_ORD: Ordering = Ordering::SeqCst;

const LOG_FORMAT: & str = r#""%r" %s %b "%{User-Agent}i" %D"#;


/// this is for each WORKER thread     
#[derive(Debug)]                       
pub struct AppState {                      
    // via thread / worker
    pub server_id: usize,                  
    //
    pub request_count: Cell<usize>,        
    // Atomic reference counted pointer
    // Arc can be shared across threads
    // Message
    pub hash_map: Arc<Mutex<HashMap<usize, String>>>,
    // Video
    pub video_map: Arc<Mutex<HashMap<usize, video::Video>>>, 
}                                      


/// RUN
pub async fn run() -> std::io::Result<()> {
    // DEBUG VERBOSE
    std::env::set_var("RUST_BACKTRACE", "1");
    // EVEN LOG -> stdout
    //std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    
    println!("{}",
             welcome_msg()?,
    );

    // shared msg HashMap for each worker
    // as we want to find via id not index
    // Message
    let hash_map =
        Arc::new(                        
            Mutex::new(
                HashMap::new()
            )
        );

    // Video
    let video_map =
        Arc::new(                        
            Mutex::new(
                HashMap::new()
            )
        );
    
    // SERVER
    HttpServer::new(move || {
        App::new()
            .data(AppState {                                        
                // persistent server counter
                server_id: SERVER_COUNTER.fetch_add(1,              
                                                    SERVER_ORD,
                ),                                                  
                // this is owned by each thread                     
                request_count: Cell::new(0), // initial value
                // create a new pointer for each thread             
                hash_map: hash_map.clone(),
                video_map: video_map.clone(),
            })                                                      
            // LOG
            .wrap(middleware::Logger::new(LOG_FORMAT))
            // ROOT INDEX OUTSIDE scopes !!! 
            // DISABLE so ROOT return 404
            //.service(index)
            // HEALTH
            .route("/health_check",
                   web::get()
                   .to(health_check)
            )
            // SCOPE for ####################### MESSAGES
            .service(
                web::scope("/msg")
                    // INDEX INSIDE scopes !!!
                    .service(message::index)
                    // ADD msg
                    .service(
                        web::resource("/send")
                            .data(web::JsonConfig::default()
                                  .limit(4096)
                            )
                            .route(web::post()
                                   // -> fn post_msg
                                   .to(message::post_msg)
                            ),
                    )
                    // FLUSH all msg from Hash
                    .service(message::clear)
                    // SEARCH msg via Hash key: id
                    .service(                            
                        web::resource("/search/{index}") 
                            // HTTP GET
                            .route(web::get()
                                   .to(message::search)           
                            ),                           
                    )
                    // LAST_ID
                    // -> fn last #[get("/last_id")]
                    .service(message::last_id)
                    // DELETE via id
                    .service(                            
                        web::resource("/delete/{index}") 
                            // HTTP DELETE
                            .route(web::delete()
                                   .to(message::delete)           
                            ),                           
                    )
            )
            // SCOPE for ####################### VIDEO
            .service(
                //web::scope("/video")
                web::scope(video::SCOPE)
                    // PUT
                    .service(
                        web::resource("/put")
                            .data(web::JsonConfig::default()
                                  // NO LIMIT for VIDEO
                                  //.limit(4096)
                            )
                            .route(web::put()
                                   .to(video::insert_video)
                            )
                    )
                    // index
                    .service(
                        // COMBINE BOTH as you LEARN
                        // has to be call as: /video/
                        web::resource("/")
                        // has to be call as: /video
                        //web::resource("") 
                            .route(web::get()
                                   .to(video::index)
                            )
                    )
                    .service(video::download) // <- /video/download/123
                    .service(video::detail) // <- /video/detail/123
                    // FLUSH all msg from Hash
                    // -> fn clear #[post("/clear")]
                    .service(video::clear)
            )
    }
    )
        // https://actix.rs/docs/server/
        .bind(
            format!("{}:{}",
                    SERVER,
                    PORT,
            )             
        )?
        // number of logical CPUs in the system
        // each thread process is blocking
        // non-cpu-bound operation should be expressed as futures or asynchronous
        .workers(4) //
        .run()
        .await
}


/// welcome msg
fn welcome_msg() -> std::io::Result<String> {
    Ok(format!("FoOoKuMe -> {NAME}"))
}


///
/// curl -v http://localhost:8081/health_check
///
/// HTTP/1.1 200 OK
///
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
