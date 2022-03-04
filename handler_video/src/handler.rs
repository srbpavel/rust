/// HANDLER
///
use crate::handler_video_toml_config_struct::{TomlConfig};
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

use std::cell::Cell;                
use std::sync::atomic::{AtomicUsize,
                        Ordering,   
};                                  
                                    
use std::sync::{Arc,                
                Mutex,              
};

use std::collections::HashMap;

/// counters
//static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);
//static SERVER_ORD: Ordering = Ordering::SeqCst;

/// this is for each WORKER thread     
#[derive(Debug)]                       
pub struct AppState {                      
    //pub server_id: usize,                  
    //pub request_count: Cell<usize>,        
    pub video_map: Arc<Mutex<HashMap<video::VideoKey, video::VideoValue>>>,
    pub binary_map: Arc<Mutex<HashMap<video::VideoKey, video::BinaryValue>>>,
    pub groups: Arc<Mutex<Vec<String>>>,
}                                      

/// RUN
pub async fn run(config: TomlConfig) -> std::io::Result<()> {
    std::env::set_var(
        "RUST_BACKTRACE",
        "1",
    );
    
    std::env::set_var(
        "RUST_LOG",
        "handler_video=debug,actix_web=debug,actix_server=info",
    );

    env_logger::init();

    info!("{}", welcome_msg(&config)?,);

    // detail
    let video_map =
        Arc::new(                        
            Mutex::new(
                HashMap::new()
            )
        );

    // data
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

    // SERVER
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                /*
                // persistent server counter
                server_id: SERVER_COUNTER.fetch_add(1,              
                                                    SERVER_ORD,
                ),
                
                // this is owned by each thread                     
                request_count: Cell::new(0), // initial value
                */
                video_map: video_map.clone(),
                binary_map: binary_map.clone(),
                groups: groups.clone(),
            }))
            .wrap(middleware::Logger::new(&config.log_format))
            // ROOT ###
            // index DISABLED so ROOT return 404
            //.service(index)
            // NO HANDLER FOR 404 yet
            // SCOPE for ####################### VIDEO
            .service(
                web::scope(video::SCOPE)
                    .service(
                        web::resource("/put")
                            .app_data(
                                Data::new(
                                    JsonConfig::default()
                                        .limit(config.video_size_limit as usize)
                                )
                            )
                            .route(web::put()
                                   .to(video::insert_video)
                            )
                    )
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
                    .service(video::download)
                    .service(video::detail)
                    .service(video::clear)
                    .service(                            
                        web::resource("/delete/{index}") 
                            .route(web::delete()
                                   .to(video::delete)           
                            ),                           
                    )
                    .service(video::list_group)
                    .service(
                        web::resource("/groups")
                            .route(web::get()
                                   .to(video::list_groups)
                            )
                    )
                    .service(
                        web::resource("/update/group")
                            .app_data(
                                Data::new(
                                    JsonConfig::default()
                                        .limit(4096)
                                )
                            )
                            .route(web::post()
                                   .to(video::update_group)
                            ),
                    )
            )
    }
    )
        .bind(
            format!("{}:{}",
                    &config.server,
                    config.port,
            )             
        )?
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
