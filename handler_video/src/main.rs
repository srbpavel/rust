/// HANDLER_VIDEO
///
use actix_web::{
    //dev::{Body,
    //      ServiceResponse,
    //},
    get,
    post,
    //http::StatusCode,
    //middleware,
    //middleware::{
        //errhandlers::{
            //ErrorHandlerResponse,
            //ErrorHandlers,
        //},
	//Logger,
    //},
    web,

    App,
    HttpResponse,
    HttpServer,
    Responder,

    Error,
    Result,
};
    

//use serde_json::json;


const NAME: &'static str = "HANDLER_VIDEO";
const SERVER: &'static str = "127.0.0.1";
const PORT: u64 = 8081;


/// MAIN
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // VERBOSE
    std::env::set_var("RUST_BACKTRACE", "1");
    // EVEN LOG -> stdout
    //std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    println!("{}",
             welcome_msg("FoOoKuMe -> {NAME}")?,
    );
    
    env_logger::init();

    // SERVER
    HttpServer::new(|| {
        App::new()
            //.service(index)
            .route(
                    "/{filename:.*}", // takes / /index.html /index.txt ...
                    web::get()
                        .to(index)
            )
            /*
            .service(
                web::resource("/index.html")
                    .route(
                        web::get()
                            .to(index)
                    )
            )
            */
            .service(id_name_path)
            .service(echo) // POST
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


//#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(
                format!("--> / || /index.html >>> {} \n",
                        NAME,
                )
            )
    )
}


/// curl -v 'http://127.0.0.1:8081/1/bijac/index.html'
///
#[get("/{id}/{name}/index.html")]
async fn id_name_path(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("--> /{id}/{name}/ || index.html >>> nazdar name: {name} id: {id}\n")
}

/// curl -X POST 'http://127.0.0.1:8081/echo' -H "Content-Type: application/json" -d '{"msg": "msg_body"}'
///
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok()
        .body(req_body)
}
