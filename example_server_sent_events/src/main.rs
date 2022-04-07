use actix_web::{
    http::header::{self, ContentType},
    middleware,
    web::{self, Data, Path},
    App, HttpResponse, HttpServer, Responder,
};

mod broadcast;
use broadcast::Broadcaster;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let data = Broadcaster::create();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/events", web::get().to(new_client))
            .route("/broadcast/{msg}", web::get().to(broadcast))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    let index_html = include_str!("index.html");

    HttpResponse::Ok()
        .append_header(ContentType::html())
        .body(index_html)
}

//CLIENT -> our curl GET listener
async fn new_client(broadcaster: Data<Broadcaster>) -> impl Responder {
    let rx = broadcaster.new_client();

    println!("start new client\n -> {:?}",
             broadcaster
             .inner
             .lock()
             .clients,
    );
    
    HttpResponse::Ok()
        .append_header((header::CONTENT_TYPE, "text/event-stream"))
        .streaming(rx)
}

//BROADCASTER -> our video upload + bytes updater
async fn broadcast(msg: Path<String>, broadcaster: Data<Broadcaster>) -> impl Responder {
    println!("boardcast msg:\n -> <{msg:?}>\n  --> {:?}",
             broadcaster // our Struct
             .inner
             .lock()
             .clients,
    );

    broadcaster 
        .send(&msg.into_inner() // method to iter all clients + send MSG
        );

    HttpResponse::Ok().body("msg sent")
}
