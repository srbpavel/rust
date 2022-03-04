/// HANDLER
///
use crate::handler_video_toml_config_struct::{TomlConfig};
use crate::video;

use actix_web::{
    web::{self,
          Data,
          JsonConfig,
    },
    middleware,
    App,
    HttpServer,
};

use log;
use std::{
          sync::{
              Arc,                
              Mutex,              
          },
          collections::HashMap,
};


/// for each WORKER thread     
#[derive(Debug)]                       
pub struct AppState {                      
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

    // groups
    let groups =
        Arc::new(
            Mutex::new(
                Vec::new()
            )
        );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                video_map: video_map.clone(),
                binary_map: binary_map.clone(),
                groups: groups.clone(),
            }))
            .wrap(middleware::Logger::new(&config.log_format))
            .service(
                web::scope(video::SCOPE)
                    .service(video::index)
                    .service(video::download)
                    .service(video::detail)
                    .service(video::clear)
                    .service(video::list_group)
                    .service(video::show_groups)
                    .service(
                        web::resource("/put")
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
                        web::resource("/delete/{index}") 
                            .route(web::delete()
                                   .to(video::delete)           
                            ),                           
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
