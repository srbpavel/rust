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

/*
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
*/
#[get("/")]
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
