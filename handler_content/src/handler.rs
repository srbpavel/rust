/// HANDLER
///
use crate::handler_content_toml_config_struct::{TomlConfig};
use crate::content;

use actix_web::{
    web::
    {self,
     //Data,
     //JsonConfig,
    },
    //guard,
    middleware,
    App,
    HttpServer,
    //HttpResponse,
};

/*
use std::{
    sync::{Arc,                
           Mutex,              
    },
    collections::HashMap,
};

use actix_files::Files;
*/

//use serde::Deserialize;

/*
/// for each WORKER thread     
//#[derive(Debug, Deserialize)]
#[derive(Debug)]                       
pub struct AppState {
    pub video_map: Arc<Mutex<HashMap<video::VideoKey, video::VideoValue>>>,
    pub binary_map: Arc<Mutex<HashMap<video::VideoKey, video::BinaryValue>>>,
}                                      
*/

/// RUN
pub async fn run(config: TomlConfig) -> std::io::Result<()> {
    std::env::set_var(
        "RUST_BACKTRACE",
        "1",
    );
    
    std::env::set_var(
        "RUST_LOG",
        "handler_content=debug,actix_web=debug,actix_server=info",
    );

    env_logger::init();

    log::info!("{}", welcome_msg(&config)?,);

    /*
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
    */
    
    let mut server = HttpServer::new(move || {
        App::new()
            /*
            .app_data(Data::new(AppState {
                video_map: video_map.clone(),
                binary_map: binary_map.clone(),
            }))
            */
            .wrap(middleware::Logger::new(&config.log_format))
            /*
            .default_service(
                web::route()
                    .guard(
                        guard::Not(
                            guard::Get()
                        )
                    )
                    .to(HttpResponse::MethodNotAllowed),
            )
            */
            .service(                            
                web::resource(vec![
                    "/{id:.*}",
                    "/{id:.*}/",
                    //"/{prefix:.*}/{id:.*}",
                    //"/{prefix:.*}/{id:.*}/",
                    //"/{one}/{two:.*}/{three:.*}",
                ])
                    .route(web::get().to(content::get_content))
                    .route(web::put().to(content::put_content))
                    .route(web::delete().to(content::delete_content))
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
