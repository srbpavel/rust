//! Simple echo websocket server.
//!
//! Open `http://localhost:8080/` in browser to test.

use actix_files::NamedFile;
use actix_web::{
    middleware,
    web,
    App,
    Error,
    HttpRequest,
    HttpResponse,
    HttpServer,
    Responder,
};
use actix_web_actors::ws;

// our server
mod server;
use self::server::MyWebSocket;


// via browser
// as the path is relative, take care as if you start:
// cargo run -> inside ./src --> index.html will be not found
async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html")
        .await
        .unwrap()
}


// start connect + send msg
/// WebSocket handshake and start `MyWebSocket` actor.
async fn echo_ws(req: HttpRequest,
                 stream: web::Payload) -> Result<HttpResponse, Error> {

    // here is REQ HEaders is:
    // "sec-websocket-key": "kRu9FdtEUjiglnK4gINBiA=="
    log::debug!("New SOCKET [aka client]\n{:?}",
                req,
    );

    // https://docs.rs/actix-web-actors/latest/actix_web_actors/ws/fn.start.html
    //
    // + builder
    // https://docs.rs/actix-web-actors/latest/actix_web_actors/ws/struct.WsResponseBuilder.html
    //
    // -> HttpResponse
    //
    // so we send back actor ws start with new MyWebSocket with hb Instant
    // this is Actor start and in initiate HB
    ws::start(MyWebSocket::new(), // our Struct with HB
              &req, // req
              stream, // payload
    )
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "websocket_server=debug,actix_web=debug,actix_server=info",
    );

    env_logger::init();
    
    /*
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("info")
    );
    */

    log::info!("MAIN >>> starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // this is for our browser client
            // WebSocket UI HTML file
            .service(web::resource("/")
                     .to(index)
            )
            // websocket route
            .service(web::resource("/ws")
                     .route(
                         web::get()
                             .to(echo_ws)
                     )
            )
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
