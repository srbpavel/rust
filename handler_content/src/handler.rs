/// HANDLER
///
use crate::content;
use crate::handler_content_toml_config_struct::TomlConfig;
use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer, Responder};
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub binary_map: Arc<DashMap<content::ContentKey, content::BinaryValue>>,
}

/// RUN
pub async fn run(config: TomlConfig) -> std::io::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");

    std::env::set_var(
        "RUST_LOG",
        "handler_content=debug,actix_web=debug,actix_server=info",
    );

    env_logger::init();

    log::info!("{}", welcome_msg(&config)?,);

    let binary_map = Arc::new(DashMap::new());

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                binary_map: binary_map.clone(),
            }))
            .wrap(middleware::Logger::new(&config.log_format))
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(HttpResponse::MethodNotAllowed),
            )
            .service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource(vec!["/{url_path:.*}", "/{url_path:.*}/"])
                    .route(web::get().to(content::get_content))
                    .route(web::put().to(content::put_content_p))
                    .route(web::delete().to(content::delete_content))//,
                    .route(web::route()
                           .method(
                               http::Method::from_bytes(b"LIST")
                                   .unwrap() //not safe
                           )
                           .to(content::list_content)
                    ),
            )
    })
    .bind(format!("{}:{}", &config.server, config.port,))?;

    server = match &config.workers {
        -1 => server,
        n @ 1.. => server.workers(*n as usize),
        _ => {
            eprintln!(
                "\nEXIT: set correct number of workers:\n default: -1\n user defined: 1/2/4/.."
            );
            std::process::exit(1);
        }
    };

    server.run().await
}

/// welcome msg
fn welcome_msg(config: &TomlConfig) -> std::io::Result<String> {
    Ok(format!(
        "start -> {} at {} / {}",
        &config.name, &config.host, &config.server,
    ))
}

/// INDEX
/// just to verify resource regex is not greedy
///
async fn index() -> impl Responder {
    HttpResponse::Ok().body("index")
}
