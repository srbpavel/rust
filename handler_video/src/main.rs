/// HANDLER_VIDEO
///
use actix_web::{
    get,
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder,

    Error,
    Result,
};


const SERVER: &'static str = "127.0.0.1";
const PORT: u64 = 8081;


/// MAIN
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{}",
             welcome_msg("FoOoKuMe -> handler_video")?,
    );

    // VERBOSE
    std::env::set_var("RUST_BACKTRACE", "1");
    // EVEN LOG -> stdout
    //std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(id_name_path)
    })
        .bind(
            format!("{}:{}",
                    SERVER,
                    PORT,
            )             
        )?
        .run()
        .await
}


/// welcome msg
fn welcome_msg(value: &str) -> std::io::Result<String> {
    Ok(String::from(value))
}


#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::Ok()
            .content_type("text/html")
            .body("/index.html"))
}


/// curl -v 'http://127.0.0.1:8081/1/bijac/index.html'
///
#[get("/{id}/{name}/index.html")]
async fn id_name_path(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("nazdar -> name: {} id:{}", name, id)
}
