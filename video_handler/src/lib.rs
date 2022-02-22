#[macro_use]
extern crate actix_web;

use actix_web::{middleware,
                web,
                App,
                HttpServer,
                Result,
};

use serde::{Deserialize,
            Serialize,
};

use std::cell::Cell;
use std::sync::atomic::{AtomicUsize,
                        Ordering,
};

use std::sync::{Arc,
                Mutex,
};


static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);


#[derive(Deserialize, Debug)]
struct PostInput {
    message: String,
}

#[derive(Serialize, Debug)]
struct PostResponse {
    server_id: usize,
    request_count: usize,
    message: String,
}


/// this is for each WORKER thread
#[derive(Debug)]
struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    // Atomic reference counted pointer
    // Arc can be shared across threads
    messages: Arc<Mutex<Vec<String>>>,
}


#[derive(Serialize, Debug)]
struct IndexResponse {
    server_id: usize,
    request_count: usize,
    messages: Vec<String>,
}


#[derive(Debug)]
pub struct MessageApp {
    port: u16,
}

impl MessageApp {
    /// NEW -> initial setup port + later server/...
    pub fn new(port: u16) -> Self {
        MessageApp {
            port: port
        }
    }

    /// RUN -> start service
    pub fn run(&self) -> std::io::Result<()> {
        // shared msg vector for each worker
        let messages =
            Arc::new(
                Mutex::new(
                    vec![]
                )
            );

        println!("starting http server: 127.0.0.1:{}",
                 self.port,
        );
        
        HttpServer::new(move || {
            App::new()
                // this creates -> web::Data<AppState> and later called
                // via .service(...)
                .data(AppState {
                    // 0..7
                    server_id: SERVER_COUNTER.fetch_add(1,
                                                        Ordering::SeqCst,
                    ),
                    // this is owned by each thread
                    request_count: Cell::new(0), // with initial value 0
                    // create a new pointer for each thread
                    messages: messages.clone(),
                })
                .wrap(middleware::Logger::default())
                .service(index) // -> fn index + #[get"/"]
                .service( 
                    web::resource("/send") // path <- instead #[..("/send")
                        .data(web::JsonConfig::default() // json extractor
                              .limit(4096) // 4096 bytes 
                        )
                        .route(web::post() // HTTP POST <- instead #[post("/..")]
                               .to(post_msg) // -> fn post_msg 
                        ),
                )
                .service(clear) // -> fn clear #[post("/clear")]
                
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


/// service: handler
///
/// we just add +1 and publish json
///
#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let msg = state
        .messages
        .lock()
        .unwrap();

    Ok(
        web::Json(
            IndexResponse {
                server_id: state.server_id,
                request_count: request_count,
                messages: msg.clone(),
            }
        )
    )
}


/// route.to()
///
/// add +1 to AppState.request_count / update via .set()
/// lock messages + push .clone()
/// build json via Struct
///
fn post_msg(msg: web::Json<PostInput>,
            state: web::Data<AppState>) -> Result<web::Json<PostResponse>> {

    // Cell
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    // we lock and have access to Vec messages
    let mut ms = state
        .messages
        .lock() // get access to data inside Mutex + blocks until another thread
        .unwrap(); // -> MutexGuard<Vec<String>> // will panic on Err !!!

    // and we push are new MSG to Vec
    ms.push(msg.message.clone()); // clone as Vec owns each element
    
    Ok(web::Json(
        PostResponse {
            server_id: state.server_id, // here is our messages: Vec
            request_count: request_count,
            message: msg.message.clone(), // because it is shared 
        }
    ))
}


/// service: handler
///
/// add +1
/// flush messages
/// json via Struct but with empty Vec
///
#[post("/clear")]
fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1; // we still count
    state.request_count.set(request_count);

    let mut ms = state.messages.lock().unwrap();
    ms.clear(); // messages are flushed

    // actualy this is nearly the same as after start with no messages
    // but few server_id and counter count
    Ok(web::Json(
        IndexResponse {
            server_id: state.server_id,
            request_count: request_count,
            messages: vec![], // no messages for json
        }
    ))
}
