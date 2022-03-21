use std::{
    sync::{
        atomic::{AtomicUsize,
                 Ordering,
        },
        Arc,
    },
    time::Instant,
};

use actix::*;
use actix_files::{Files,
                  NamedFile,
};
use actix_web::{
    middleware::Logger,
    web,
    App,
    Error,
    HttpRequest,
    HttpResponse,
    HttpServer,
    Responder,
};
use actix_web_actors::ws;

mod server;
mod session;


// our browser chat
async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

// 
/// Entry point for our websocket route
async fn chat_route(req: HttpRequest,
                    stream: web::Payload,
                    srv: web::Data<Addr<server::ChatServer>>) -> Result<HttpResponse, Error> {

    ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "Main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

/// Displays state
async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {}", current_count)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "websocket_chat_server=debug,actix_web=debug,actix_server=info",
    );

    env_logger::init();

    //env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // our state to share 
    // set up applications state
    // keep a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // start chat server actor
    let server = server::ChatServer::new(
        app_state
            .clone()
    )
        .start();

    log::info!("CHAT starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // data: state to count visitors
            .app_data(web::Data::from(app_state.clone()))
            // data: chat server
            .app_data(web::Data::new(server.clone()))
            // chat room
            .service(web::resource("/").to(index))
            // visitor counter
            .route("/count", web::get().to(get_count))
            // ws main route
            .route("/ws", web::get().to(chat_route))
            // no need for this
            //.service(Files::new("/static", "./static"))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
