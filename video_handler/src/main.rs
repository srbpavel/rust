/// VIDEO_HANDLER
///
use video_handler::MessageApp;
//use crate::MessageApp;


///
//fn main() -> Result<(), ()> {
//fn main() -> Result<(), std::io::Error> {
fn main() -> std::io::Result<()> {
    println!("{}",
             sss("FoOoKuMe -> video_handler")?,
    );
    

    // EVEN LOG -> stdout
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let app = MessageApp::new(8080);
    app.run()
}


/// FUTURE USE
fn sss(value: &str) -> std::io::Result<String> {
    Ok(String::from(value))
}
