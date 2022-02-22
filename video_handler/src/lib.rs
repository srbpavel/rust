#[macro_use]
extern crate actix_web;

use actix_web::{middleware,
                web,
                App,
                HttpRequest,
                HttpServer,
                Result,
};

use serde::Serialize;


#[derive(Serialize, Debug)]
struct IndexResponse {
    message: String,
    datetime: String,
}


#[derive(Debug)]
pub struct MessageApp {
    port: u16,
}

impl MessageApp {
    /// NEW
    pub fn new(port: u16) -> Self {
        MessageApp {
            port: port
        }
    }

    /// RUN
    pub fn run(&self) -> std::io::Result<()> {
        println!("starting http server: 127.0.0.1:{}",
                 self.port,
        );
        
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(index)
        })
            .bind(
                ("127.0.0.1",
                 self.port,
                )
            )?
            .workers(8)
            .run()
    }
}

#[get("/")]
fn index(req: HttpRequest) -> Result<web::Json<IndexResponse>> {
    let hello = req
        .headers()
        .get("hello") // -> we read data from HEADER 'hello: ...'
        .and_then(|v| v.to_str().ok()) // Result -> Option
        .unwrap_or_else(|| "world"); // -> no HEADER 'hello: ...'

    Ok(
        web::Json(
            IndexResponse {
                //message: hello.to_owned(),
                message: String::from(hello),
                //message: format!("{}", hello),
                datetime: format!("{}",
                                  chrono::Utc::now(),
                ),
            }
        )
    )
}
