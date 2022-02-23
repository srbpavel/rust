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

    // VERBOSE
    std::env::set_var("RUST_BACKTRACE", "1");

    // EVEN LOG -> stdout
    // display default datetime in Utc
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // LET IT ROOL
    let app = MessageApp::new(8080);
    app.run()
}


/// FUTURE USE
fn sss(value: &str) -> std::io::Result<String> {
    Ok(String::from(value))
}
