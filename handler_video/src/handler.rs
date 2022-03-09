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

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                video_map: video_map.clone(),
                binary_map: binary_map.clone(),
            }))
            .wrap(middleware::Logger::new(&config.log_format))
            .default_service(
                web::route()
                    .guard(
                        guard::Not(
                            guard::Get()
                        )
                    )
                    .to(HttpResponse::MethodNotAllowed),
            )
            .service(
                web::scope(video::SCOPE)
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
        )?;

    server = match &config.workers {
        -1 => server,
        n @ 1.. => server
            .workers(*n as usize),
        _ => {
            eprintln!("\nEXIT: set correct number of workers:\n default: -1\n user defined: 1/2/4/..");
            std::process::exit(1);
        },
    };

    server
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
