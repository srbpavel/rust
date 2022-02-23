/// HANDLER_VIDEO
///
#[macro_use]             
extern crate actix_web;

use actix_web::{
    //dev::{Body,
    //      ServiceResponse,
    //},
    get,
    post,
    //guard,
    web,
    //http::StatusCode,
    middleware,
    //middleware::{
        //errhandlers::{
            //ErrorHandlerResponse,
            //ErrorHandlers,
        //},
	//Logger,
    //},

    App,
    HttpResponse,
    HttpServer,
    Responder,

    Error,
    Result,

    error::{InternalError,
            JsonPayloadError,
    },             
};
    
use serde::{Serialize,
            Deserialize,
};

use std::cell::Cell;                
use std::sync::atomic::{AtomicUsize,
                        Ordering,   
};                                  
                                    
use std::sync::{Arc,                
                Mutex,              
};


const NAME: &'static str = "HANDLER_VIDEO";
const SERVER: &'static str = "127.0.0.1";
const PORT: u64 = 8081;

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);            
const LOG_FORMAT: &'static str = r#""%r" %s %b "%{User-Agent}i" %D"#;

/// this is for each WORKER thread     
#[derive(Debug)]                       
struct AppState {                      
    server_id: usize,                  
    request_count: Cell<usize>,        
    // Atomic reference counted pointer
    // Arc can be shared across threads
    messages: Arc<Mutex<Vec<String>>>, 
}                                      

#[derive(Serialize, Debug)]
struct IndexResponse {     
    server_id: usize,      
    request_count: usize,  
    messages: Vec<String>, 
}


#[derive(Deserialize, Debug)]
struct PostInput {
    message: String,
}

#[derive(Serialize, Debug)]
struct PostResponse {
    server_id: usize,
    request_count: usize,
    message: String,
}


#[derive(Serialize, Debug)]                               
struct LookupResponse {                                   
    server_id: usize,                                     
    request_count: usize,                                 
    result: Option<String>, // None in JSON will be "null"
    //position: String,                                   
    position: Option<String>,                             
    path: String,                                         
}                                                         


#[derive(Deserialize, Debug)]
struct Horse {
    name: String,
    sex: String,
    age: u8,
}

/// MAIN
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // VERBOSE
    std::env::set_var("RUST_BACKTRACE", "1");
    // EVEN LOG -> stdout
    //std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    
    println!("{}",
             welcome_msg()?,
    );

    // shared msg vector for each worker 
    let messages =                       
        Arc::new(                        
            Mutex::new(                  
                vec![]                   
            )                            
        );                               

    // SERVER
    HttpServer::new(move || {
        App::new()
            .data(AppState {                                        
                server_id: SERVER_COUNTER.fetch_add(1,              
                                                    Ordering::SeqCst,
                ),                                                  
                // this is owned by each thread                     
                request_count: Cell::new(0), // with initial value 0
                // create a new pointer for each thread             
                messages: messages.clone(),                         
            })                                                      
            //.wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new(LOG_FORMAT))
            /*
            .wrap(
                middleware::NormalizePath::trim()
            )
            */
            .service(index)

            .service(                                                        
                web::resource("/send") // path <- instead #[..("/send")      
                    .data(web::JsonConfig::default()                         
                          .limit(4096)                                       
                    )                                                         
                    .route(web::post() // HTTP POST <- instead #[post("/..")]
                           .to(post_msg) // -> fn post_msg                   
                    ),                                                       
            )                                                                
            /*
            .service(clear) // -> fn clear #[post("/clear")]
            .service(lookup) // -> fn lookup                               
            .service(                                                        
                web::resource("/lookup/{index}")                             
                    .route(web::get() // HTTP GET                            
                           .to(lookup)                                       
                    ),                                                       
            )
            */

            //.service(id_name_path) // GET -> greedy so i took /horse/{name}
            .service(horse) // GET Struct <- /horse/wonka
            .service(echo) // POST <- /echo ... -d '{"user": "bijac"}'
            .service( // SCOPE
                web::scope("/video")
                    .service(all_video) // <- /video/all
                    .service(video_detail), // <- /video/123
            )
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
    })
        .bind(
            format!("{}:{}",
                    SERVER,
                    PORT,
            )             
        )?
        .run()
        .await
}


/// welcome msg
//fn welcome_msg(value: &str) -> std::io::Result<String> {
fn welcome_msg() -> std::io::Result<String> {
    //Ok(String::from(value))
    Ok(format!("FoOoKuMe -> {NAME}"))
}


#[get("/")]
//async fn index() -> Result<HttpResponse, Error> {
async fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);          
    
    let msg = state                                  
        .messages                                    
        .lock()                                      
        .unwrap();                                   
    
    Ok(                                              
        web::Json(                                   
            IndexResponse {                          
                server_id: state.server_id,          
                request_count: request_count,        
                messages: msg.clone(),               
            }                                        
        )                                            
    )                                                

    /*
    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(
                format!("--> / || /index.html >>> {} \n",
                        NAME,
                )
            )
    )
    */
}


/// route.to()
///
/// add +1 to AppState.request_count / update via .set()
/// lock messages + push .clone()
/// build json via Struct
///
async fn post_msg(msg: web::Json<PostInput>,
                  state: web::Data<AppState>) -> actix_web::Result<web::Json<PostResponse>> {
    // Cell
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    
    // we lock and have access to Vec messages
    let mut ms = state
        .messages
        .lock() // get access to data inside Mutex + blocks until another thread
        .unwrap(); // -> MutexGuard<Vec<String>> // will panic on Err !!!
    
        // and we push are new MSG to Vec
        ms.push(msg.message.clone()); // clone as Vec owns each element
    
    Ok(web::Json(
        PostResponse {
            server_id: state.server_id, // here is our messages: Vec
            request_count: request_count,
            message: msg.message.clone(), // because it is shared
        }
    ))
}



/// id + name DISPLAY
///
/// curl -v 'http://127.0.0.1:8081/1/bijac/index.html'
///
//#[get("/{id}/{name}/index.html")]
#[get("/{id}/{name}")] // Path parameters
async fn id_name_path(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("--> /{id}/{name} >>> nazdar name: <{name}> id: <{id}>\n")
}

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
