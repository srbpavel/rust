/// HANDLER_VIDEO
///
use actix_web::{
    //get,
    web,
    middleware,
    App,
    HttpServer,
    //HttpRequest,
};

//use actix_files as fs;
//use actix_files::NamedFile;
//use std::path::PathBuf;

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
    // Message
    hash_map: Arc<Mutex<HashMap<usize, String>>>,
    // Video
    //video_map: Arc<Mutex<HashMap<usize, String>>>,
    video_map: Arc<Mutex<HashMap<usize, video::Video>>>, 
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
            // STATIC files @@@@@@@@@@@@@@
            //.service(tmp_files)
            /*
            .service(
                fs::Files::new("/static", ".")
                    .show_files_listing()
            )
            */
            // SCOPE for ####################### MESSAGES
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
            // SCOPE for ####################### VIDEO
            .service(
                web::scope("/video")
                    //.service(video::index)
                    // PUT
                    .service(
                        //web::resource("/put")
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
                        // COMBINE BOTH as LEARN
                        // has to be call as: /video/
                        web::resource("/")
                        // has to be call as: /video
                        //web::resource("") 
                            .route(web::get()
                                   .to(video::index)
                            )
                            /* SAVE EXAMPLE
                            .route(web::post()
                                   .to(video::save_file)
                                   //.to(video::echo) // remove #[post...
                            ),
                            */
                    )
                    .service(video::download) // <- /video/download/123
                    //.service(video::play) // <- /video/play/123
                    .service(video::detail) // <- /video/detail/123
                    // FLUSH all msg from Hash
                    // -> fn clear #[post("/clear")]
                    .service(video::clear)
                    //.service(video::all) // <- /video/all
                    //.service(video::echo), // <- /video/echo
                    /*
                    .service(
                        web::resource("/echo")
                            .route(web::post()
                                   .to(video::echo)
                            )
                    )
                    */
                    /*
                    .service(
                        web::resource("/save_file")
                            .route(web::post()
                                   .to(video::save_file)
                            ),
                    )
                    */
            )
    }
    )
        .bind(
            format!("{}:{}",
                    SERVER,
                    PORT,
            )             
        )?
        //.workers(8) // study more
        .workers(1) // study more
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


/* STATIC
/// https://actix.rs/docs/static-files/
#[get("/temp")]
async fn tmp_files(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = "./tmp/"
        .parse()
        .unwrap();
    Ok(
        NamedFile::open(path)?
    )
}
 */
