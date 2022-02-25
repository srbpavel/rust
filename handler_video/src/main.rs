/// HANDLER_VIDEO
///
use actix_web::{
    web,
    middleware,
    App,
    HttpServer,
};

mod message;
mod video;

use std::cell::Cell;                
use std::sync::atomic::{AtomicUsize,
                        Ordering,   
};                                  
                                    
use std::sync::{Arc,                
                Mutex,              
};

use std::collections::HashMap;


const NAME: &'static str = "HANDLER_VIDEO";
const SERVER: &'static str = "127.0.0.1";
const PORT: u64 = 8081;

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);
static SERVER_ORD: Ordering = Ordering::SeqCst;

const LOG_FORMAT: &'static str = r#""%r" %s %b "%{User-Agent}i" %D"#;


/// this is for each WORKER thread     
#[derive(Debug)]                       
pub struct AppState {                      
    // via thread / worker
    server_id: usize,                  
    //
    request_count: Cell<usize>,        
    // Atomic reference counted pointer
    // Arc can be shared across threads
    hash_map: Arc<Mutex<HashMap<usize, String>>>, 
}                                      


/// MAIN
#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    let hash_map =
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
            })                                                      
            // LOG
            .wrap(middleware::Logger::new(LOG_FORMAT))
            // ROOT INDEX OUTSIDE scopes !!! 
            // DISABLE so ROOT return 404
            //.service(index)
            // SCOPE for MESSAGES
            .service(
                web::scope("/msg")
                    // INDEX INSIDE scopes !!!
                    .service(message::index)
                    // ADD msg
                    .service(
                        // path <- instead #[..("/send")
                        web::resource("/send")
                            .data(web::JsonConfig::default()
                                  .limit(4096)
                            )
                            // HTTP POST <- instead #[post("/..")]
                            .route(web::post()
                                   // -> fn post_msg
                                   .to(message::post_msg)
                            ),
                    )
                    // FLUSH all msg from Hash
                    // -> fn clear #[post("/clear")]
                    .service(message::clear)
                    // SEARCH msg via Hash key: id
                    .service(                            
                        web::resource("/search/{index}") 
                            // HTTP GET
                            .route(web::get()
                                   .to(message::search)           
                            ),                           
                    )
                    // LAST
                    // -> fn last #[get("/last")]
                    .service(message::last)
                    // DELETE via id
                    .service(                            
                        web::resource("/delete/{index}") 
                            // HTTP GET
                            //.route(web::get()
                            // HTTP DELETE
                            .route(web::delete()
                                   .to(message::delete)           
                            ),                           
                    )
            )
            // SCOPE for VIDEO
            .service(
                web::scope("/video")
                    //.service(video::index)
                    .service(video::all) // <- /video/all
                    .service(video::detail) // <- /video/123
                    .service(video::echo), // <- /video/echo
            )
    }
    )
        .bind(
            format!("{}:{}",
                    SERVER,
                    PORT,
            )             
        )?
        .workers(8) // study more
        .run()
        .await
            //.service(id_name_path) // GET -> greedy so i took /horse/{name}
            /*
            .service(horse) // GET Struct <- /horse/wonka
            */
            /* GREEDY, solve later
            .route(
                    "/{filename:.*}", // '/' /index.html /index.txt ...
                    web::get()
                        .to(index)
            )
            */
            /*
            .service(
                web::resource("/index.html")
                    .route(
                        web::get()
                            .to(index)
                    )
            )
            */
            /* NOT THERE YET
            .service( // RESOURCE
                web::resource("/user/{name}")
                    .name("user_detail")
                    .guard(
                        guard::Post())
                    .guard(
                        guard::Header("content-type", "application/json"))
                    .to(|| HttpResponse::Ok()),
                    /*
                    .route(
                        web::get().to(|| HttpResponse::Ok()))
                    .route(
                        web::put().to(|| HttpResponse::Ok())),
                    */
            )
            */
}


/// welcome msg
//fn welcome_msg(value: &str) -> std::io::Result<String> {
fn welcome_msg() -> std::io::Result<String> {
    //Ok(String::from(value))
    Ok(format!("FoOoKuMe -> {NAME}"))
}


// FUTURE USE -> CODE parts
/*
/// id + name DISPLAY
///
/// curl -v 'http://127.0.0.1:8081/1/bijac/index.html'
///
//#[get("/{id}/{name}/index.html")]
#[get("/{id}/{name}")] // Path parameters
async fn id_name_path(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("--> /{id}/{name} >>> nazdar name: <{name}> id: <{id}>\n")
}
*/

/*
/// horse Struct
///
/// curl 'http://127.0.0.1:8081/horse/matylda'
///
#[get("/horse/{name}/{sex}/{age}")]
async fn horse(info: web::Path<Horse>) -> Result<String> {
    Ok(
        format!("{:?}\n",
                info,
        )
    )
}
*/



