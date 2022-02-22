#[macro_use]
extern crate actix_web;

use actix_web::{middleware,
                web,
                App,
                //HttpRequest, ch1
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


/* ch1
#[derive(Serialize, Debug)]
struct IndexResponse {
    message: String,
    datetime: String,
}
*/
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
    /// NEW
    pub fn new(port: u16) -> Self {
        MessageApp {
            port: port
        }
    }

    /// RUN
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
                // this makes -> web::Data<AppState>
                .data(AppState {
                    server_id: SERVER_COUNTER.fetch_add(1,
                                                        Ordering::SeqCst,
                    ),
                    // this is owned by each thread
                    request_count: Cell::new(0), // with initial value 0
                    // create a new pointer for each thread
                    messages: messages.clone(),
                })
                .wrap(middleware::Logger::default())
                .service(index)
                .service(
                    // POST handler resourse
                    web::resource("/send") // path
                        .data(web::JsonConfig::default() // json extractor
                              .limit(4096) // 4096 bytes 
                        )
                        .route(web::post() // HTTP POST REQ
                               .to(post_msg) //our handler function
                        ),
                )
                .service(clear)
                
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


#[get("/")] // GET HANDLER
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    /*
    AppState {
        server_id: usize,
        request_count: Cell<usize>,
        messages: Arc<Mutex<Vec<String>>>,
    }
    */

    // Cell
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let msg = state
        .messages
        .lock() // get access to data inside Mutex + blocks until another thread
        .unwrap(); // -> MutexGuard<Vec<String>> // will panic on Err !!!

    /*
    IndexResponse {
        server_id: usize,
        request_count: usize,
        messages: Vec<String>,
    }
    */
    
    Ok(
        web::Json(
            IndexResponse {
                server_id: state.server_id,
                request_count: request_count,
                messages: msg.clone(), // because it is shared 
            }
        )
    )
}


/// format msg to send
fn post_msg(msg: web::Json<PostInput>,
            state: web::Data<AppState>) -> Result<web::Json<PostResponse>> {

    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    // we lock and have access to Vec messages
    let mut ms = state
        .messages
        .lock()
        .unwrap();

    // and we push are new MSG to Vec
    ms.push(msg.message.clone()); // clone as Vec owns each element
    
    Ok(web::Json(
        PostResponse {
            server_id: state.server_id, // here is our messages: Vec
            request_count: request_count,
            message: msg.message.clone(), // clone 
        }
    ))
}


#[post("/clear")] // POST HANDLER
fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    let mut ms = state.messages.lock().unwrap();
    ms.clear();

    Ok(web::Json(
        IndexResponse {
            server_id: state.server_id,
            request_count: request_count,
            messages: vec![],
        }
    ))
}
