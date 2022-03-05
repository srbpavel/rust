/// HANDLER
///
use crate::handler_video_toml_config_struct::{TomlConfig};
use crate::video;

use actix_web::{
    web::
    {self,
     Data,
     JsonConfig,
    },
    guard,
    middleware,
    App,
    HttpServer,
    HttpResponse,
};

use std::{
    sync::{Arc,                
           Mutex,              
    },
    collections::HashMap,
};


/// for each WORKER thread     
#[derive(Debug)]                       
pub struct AppState {                      
    pub video_map: Arc<Mutex<HashMap<video::VideoKey, video::VideoValue>>>,
    pub binary_map: Arc<Mutex<HashMap<video::VideoKey, video::BinaryValue>>>,
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

    log::info!("{}", welcome_msg(&config)?,);

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

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                video_map: video_map.clone(),
                binary_map: binary_map.clone(),
            }))
            .wrap(middleware::Logger::new(&config.log_format))
            // 405 instead 404
            .default_service(
                web::route()
                    .guard(
                        guard::Not(
                            guard::Get()
                        )
                    )
                    // -> 405
                    //.to(HttpResponse::MethodNotAllowed),
                    // -> 200
                    .to(|| async { HttpResponse::Ok().body("url not active\n") }),
            )
            .service(
                web::scope(video::SCOPE)
                    // curl "http://127.0.0.1:8081/video/"
                    .service(video::index_trail)
                    // curl "http://127.0.0.1:8081/video"
                    .service(video::index)
                    // curl -X POST "http://127.0.0.1:8081/video"
                    // -H "host: spongebob"
                    .service(
                        web::scope("")
                            .guard(
                                guard::Header(
                                    "host",
                                    "spongebob")
                            )
                            .route("",
                                   web::to(||
                                           async {
                                               HttpResponse::Ok()
                                                   .body("SPONGEBOB")
                                           }
                                   )
                            ),
                    )
                    // curl -X POST "http://127.0.0.1:8081/video"
                    // -H "host: jozefina"
                    .service(
                        web::scope("")
                            .guard(
                                guard::Header(
                                    "host",
                                    "jozefina")
                            )
                            .route("",
                                   web::to(||
                                           async {
                                               HttpResponse::Ok()
                                                   .body("JOZEFINA")
                                           }
                                   )
                            ),
                    )
                    // curl -X POST "http://127.0.0.1:8081/video"
                    // -d '{"video_id": "123", "group_id": "video_on_demand"}'
                    .service(
                        web::scope("")
                            .guard(
                                guard::Post()
                            )
                            .route("",
                                   web::post()
                                   .to(video::index_post)
                            )
                    )
                    /*
                    .route("",
                           web::post()
                           .to(video::index_post)
                    )
                    */
                    .service(video::all)
                    .service(video::download)
                    .service(video::play)
                    .service(video::detail)
                    .service(video::clear)
                    .service(video::list_group)
                    .service(
                        web::resource("/upload")
                            .app_data(
                                Data::new(
                                    JsonConfig::default()
                                )
                            )
                            .route(web::put()
                                   .to(video::insert_video)
                            )
                    )
                    .service(                            
                        web::resource("/delete/{video_id}") 
                            .route(web::delete()
                                   .to(video::delete)           
                            ),                           
                    )
            )
    })
        .bind(
            format!("{}:{}",
                    &config.server,
                    config.port,
            )             
        )?
        // default: number of logical CPUs
        //.workers(config.workers)
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
