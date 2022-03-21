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
async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html")
        .await
        .unwrap()
}


// start connect + send msg
/// WebSocket handshake and start `MyWebSocket` actor.
async fn echo_ws(req: HttpRequest,
                 stream: web::Payload) -> Result<HttpResponse, Error> {

    log::debug!("New SOCKET\n{:?}",
                req,
    );

    // https://docs.rs/actix-web-actors/latest/actix_web_actors/ws/fn.start.html
    //
    // + builder
    // https://docs.rs/actix-web-actors/latest/actix_web_actors/ws/struct.WsResponseBuilder.html
    //
    // -> HttpResponse
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
