/// HANDLER
///
use crate::handler_video_toml_config_struct::{TomlConfig};
//use crate::message;
use crate::video;

use log::{
    //debug,
    //error,
    info,
};

use actix_web::{
    web::{
        self,
        Data,
        JsonConfig
    },
    middleware,
    App,
    HttpServer,
    HttpResponse,
    Responder,

    //FromRequest,
    //HttpRequest,
    //Error,
    //dev::Payload,
    //dev::ServiceRequest,
    //error::ErrorBadRequest,
};

//use futures::future::{Ready, ok, err};

use std::cell::Cell;                
use std::sync::atomic::{AtomicUsize,
                        Ordering,   
};                                  
                                    
use std::sync::{Arc,                
                Mutex,              
};

use std::collections::HashMap;

/// counters
static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);
static SERVER_ORD: Ordering = Ordering::SeqCst;

//const SURNAME: &'static str = "sRb"; 

/*
/// person
#[derive(Debug, Clone)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
}

impl FromRequest for Person {
    type Error = Error;
    type Future = Ready<Result<Person, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> futures_util::future::Ready<Result<Person, actix_web::Error>> {
        //req.app_data::<web::Data<Person>>()
        match req.app_data::<web::Data<Person>>() {
            Some(p) => return ok(p.into_inner()),
            None => err(ErrorBadRequest("fail")),
        }
        //err(ErrorBadRequest("fail"))
    }
}
*/

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
    //pub hash_map: Arc<Mutex<HashMap<usize, String>>>,
    // Video
    pub video_map: Arc<Mutex<HashMap<video::VideoKey, video::VideoValue>>>,
    // DataConfig for SCOPE
    // since this, the app is very very slow with video upload
    // find better solution!!!
    // pub config: DataConfig,
    // data storage in RAM
    pub binary_map: Arc<Mutex<HashMap<video::VideoKey, video::BinaryValue>>>,
    // list of groups
    pub groups: Arc<Mutex<Vec<String>>>,
}                                      

/*
#[derive(Debug, Clone)]
pub struct DataConfig {
    pub static_dir: String,
    pub verify_dir_per_video: bool,
}
*/

/// RUN
pub async fn run(config: TomlConfig) -> std::io::Result<()> {
    // DEBUG VERBOSE
    // https://actix.rs/docs/errors/
    std::env::set_var("RUST_BACKTRACE", "1");
    // EVEN LOG -> stdout
    std::env::set_var("RUST_LOG", "handler_video=debug,actix_web=debug,actix_server=info");
    env_logger::init();
    //debug!("debug");
    //error!("error");
    info!("{}", welcome_msg(&config)?,);
    
    // shared msg HashMap for each worker
    // as we want to find via id not index
    /*
    // Message
    let hash_map =
        Arc::new(                        
            Mutex::new(
                HashMap::new()
            )
        );
    */

    // Video
    let video_map =
        Arc::new(                        
            Mutex::new(
                HashMap::new()
            )
        );

    // Binary
    let binary_map =
        Arc::new(                        
            Mutex::new(
                HashMap::new()
            )
        );

    // groups
    let groups =
        Arc::new(
            Mutex::new(
                Vec::new()
            )
        );

    /*
    // tester to app data
    let person = Person {
        first_name: String::from("foookin"),
        last_name: String::from("paavel"),
        age: 43,
    };

    let person_2 = person.clone();
    */

    // SERVER
    HttpServer::new(move || {
        App::new()
            // https://docs.rs/actix-web/3.3.3/actix_web/struct.App.html
            // ext: U
            //.app_data(person_2.clone())
            // data: U
            //.data(person_2.clone())
            // TESTER person data vs app_data
            //.app_data(Data::new(person_2.clone()))
            //.app_data(Data::new(AppState {
            //.data(AppState {
            .app_data(Data::new(AppState {
                // persistent server counter
                server_id: SERVER_COUNTER.fetch_add(1,              
                                                    SERVER_ORD,
                ),                                                  
                // this is owned by each thread                     
                request_count: Cell::new(0), // initial value
                // create a new pointer for each thread             
                // message
                //hash_map: hash_map.clone(),
                // video
                video_map: video_map.clone(),
                // binary
                binary_map: binary_map.clone(),
                /*
                // config
                config: DataConfig {
                    static_dir: String::from(config.static_dir.clone()),
                    verify_dir_per_video: config.flag.verify_dir_per_video,
                },
                */
                // groups
                groups: groups.clone(),
            }))
            // LOG
            //.wrap(middleware::Logger::default())
            //.wrap(middleware::Logger::new(LOG_FORMAT))
            .wrap(middleware::Logger::new(&config.log_format))
            // ROOT ###
            // index DISABLED so ROOT return 404
            //.service(index)
            // NO HANDLER FOR 404 yet
            /*
            // HEALTH // FUTURE USE for tests
            .route("/health_check",
                   web::get()
                   .to(health_check)
            )
            */
            // SCOPE for ####################### VIDEO
            .service(
                web::scope(video::SCOPE)
                    // /*
                    // UPLOAD
                    .service(
                        web::resource("/put")
                            .app_data(
                                Data::new(
                                    JsonConfig::default()
                                    // NO LIMIT for VIDEO
                                    //.limit(4096)
                                )
                            )
                            /*
                            .data(web::JsonConfig::default()
                                  // NO LIMIT for VIDEO
                                  //.limit(4096)
                            )
                            */
                            .route(web::put()
                                   .to(video::insert_video)
                            )
                    )
                    // */
                    // INDEX
                    .service(
                        // TRY COMBINE BOTH as you LEARN
                        // has to be call as: /video/
                        web::resource("/")
                        // has to be call as: /video
                        //web::resource("") 
                            .route(web::get()
                                   .to(video::index)
                            )
                    )
                    // FS -> RAM 
                    .service(video::download) // <- /video/download/123
                    .service(video::detail) // <- /video/detail/123
                    // FLUSH all msg from Hash
                    .service(video::clear)
                    // DELETE via id
                    .service(                            
                        web::resource("/delete/{index}") 
                        // HTTP DELETE
                            .route(web::delete()
                                   .to(video::delete)           
                            ),                           
                    )
                    // LIST group members
                    .service(video::list_group)
                    // ALL GROUPS
                    .service(
                        web::resource("/groups")
                            .route(web::get()
                                   .to(video::list_groups)
                            )
                    )
                    // UPDATE group_id for single video
                    //.service(video::update_group)
                    .service(
                        web::resource("/update/group")
                            .app_data(
                                Data::new(
                                    JsonConfig::default()
                                        .limit(4096)
                                )
                            )
                            /*
                            .data(web::JsonConfig::default()
                                  .limit(4096)
                            )
                            */
                            .route(web::post()
                                   .to(video::update_group)
                            ),
                    )
            )
            // SCOPE for ####################### MESSAGES
            /*
            .service(
                web::scope("/msg")
                    // INDEX INSIDE scopes !!!
                    .service(message::index)
                    // ADD msg
                    .service(
                        web::resource("/send")
                            .app_data(
                                Data::new(
                                    JsonConfig::default()
                                        .limit(4096)
                                )
                            )
                            /*
                            .data(web::JsonConfig::default()
                                  .limit(4096)
                            )
                            */
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
            */
    }
    )
        // https://actix.rs/docs/server/
        .bind(
            format!("{}:{}",
                    //SERVER,
                    &config.server,
                    //PORT,
                    config.port,
            )             
        )?
        // number of logical CPUs in the system
        // each thread process is blocking
        // non-cpu-bound operation should be expressed as futures or asynchronous
        .workers(config.workers)
        .run()
        .await
}


/// welcome msg
fn welcome_msg(config: &TomlConfig) -> std::io::Result<String> {
    Ok(
        format!("start -> {} at {} / {}",
                &config.name,
                &config.host,
                &config.server,
        )
    )
}


/*
///
/// curl -v http://localhost:8081/health_check
///
/// HTTP/1.1 200 OK
///
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
*/
