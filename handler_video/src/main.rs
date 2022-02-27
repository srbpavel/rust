mod handler;
use handler::run;

mod message;
mod video;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}

