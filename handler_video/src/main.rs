/// HANDLER_VIDEO
///
use actix_web::{
    //get,
    //post,
    web,
    middleware,
    App,
    //HttpResponse,
    HttpServer,
    //Responder,
    // Error, // covered ?
    //Result,
};


mod message;

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

static MSG_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);            
static MSG_ID_ORD: Ordering = Ordering::SeqCst;

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
    //messages: Arc<Mutex<Vec<String>>>,
    //messages: Arc<Mutex<Vec<Message>>>,
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

    // shared msg Vector for each worker
    //
    // test via Hash to accces not by index but id
    //
    /* // VEC
    let messages =                       
        Arc::new(                        
            Mutex::new(                  
                vec![]                   
            )                            
        );                               
    */

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
                //
                // use the same for msg/video id
                //
                server_id: SERVER_COUNTER.fetch_add(1,              
                                                    SERVER_ORD,
                ),                                                  
                // this is owned by each thread                     
                request_count: Cell::new(0), // with initial value 0
                // create a new pointer for each thread             
                // VEC
                //messages: messages.clone(),
                // HASH
                hash_map: hash_map.clone(),
            })                                                      
            // LOG
            .wrap(middleware::Logger::new(LOG_FORMAT))
            // ROOT
            // MAIN INDEX OUTSIDE scopes !!! 
            //.service(index) // DISABLE so ROOT return 404
            // SCOPE for MESSAGES
            .service(
                web::scope("/msg")
            // ADD msg
            .service(                                                        
                web::resource("/send") // path <- instead #[..("/send")      
                    .data(web::JsonConfig::default()                         
                          .limit(4096)                                       
                    )                                                         
                    .route(web::post() // HTTP POST <- instead #[post("/..")]
                           .to(message::post_msg) // -> fn post_msg                   
                    ),                                                       
            )
            .service(message::index) // INDEX INSIDE scopes !!!
            // FLUSH all msg from Hash
            .service(message::clear) // -> fn clear #[post("/clear")]
            /* VEC
            // READ msg via index VEC only !!!
            .service(                            
                web::resource("/lookup/{index}") 
                    .route(web::get() // HTTP GET
                           .to(lookup)           
                    ),                           
            ) 
            */
            // SEARCH msg via Hash key: id
            .service(                            
                web::resource("/search/{index}") 
                    .route(web::get() // HTTP GET
                           .to(message::search)           
                    ),                           
            )
            // LAST
            .service(message::last) // -> fn last #[get("/last")]
            // DELETE via id
            .service(                            
                web::resource("/delete/{index}") 
                    .route(web::get() // HTTP GET
                           .to(message::delete)           
                    ),                           
            )
    )}) // scope_end at start
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
            .service(echo) // POST <- /echo ... -d '{"user": "bijac"}'
            .service( // SCOPE
                web::scope("/video")
                    .service(all_video) // <- /video/all
                    .service(video_detail), // <- /video/123
            )
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

/*
/// json ECHO
///
/// curl -X POST 'http://127.0.0.1:8081/echo' -H "Content-Type: application/json" -d '{"msg": "msg_body"}'
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
async fn all_video() -> HttpResponse {
    HttpResponse::Ok().body("all_videos\n")
}

/// single VIDEO detail
///
/// curl 'http://127.0.0.1:8081/video/2'
///
#[get("/{id}")]
async fn video_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok()
        .body(
            format!("video_detail: {}\n",
                    path
                    .into_inner()
                    .0,
            ),
        )
}
*/
