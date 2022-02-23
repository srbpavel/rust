#[macro_use]
extern crate actix_web;

use actix_web::{
    middleware,
    web,
    App,
    HttpRequest,
    HttpResponse,
    HttpServer,
    //Result, // full_path at ->
    error::{//Error, // full_path at ->
            InternalError, // helper to wrap any error -> 400, 200, 505
            JsonPayloadError,
    },
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
const LOG_FORMAT: &'static str = r#""%r" %s %b "%{User-Agent}i" %D"#;


#[derive(Serialize, Debug)]
struct LookupResponse {
    server_id: usize,
    request_count: usize,
    result: Option<String>, // None in JSON will be "null"
    position: String,
}


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

#[derive(Serialize)]
struct PostError {
    server_id: usize,
    request_count: usize,
    error: String,
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
                // [2022-02-22T18:05:20Z INFO  actix_web::middleware::logger] 127.0.0.1:43738 "POST /send HTTP/1.1" 400 53 "-" "curl/7.74.0" 0.000799
                //.wrap(middleware::Logger::default())
                // [2022-02-22T18:08:52Z INFO  actix_web::middleware::logger] "POST /send HTTP/1.1" 400 53 "curl/7.74.0" 1.039174
                .wrap(middleware::Logger::new(LOG_FORMAT))
                .service(index) // -> fn index + #[get"/"]
                .service( 
                    web::resource("/send") // path <- instead #[..("/send")
                        .data(web::JsonConfig::default()
                              .limit(4096)
                              //.error_handler(post_error)
                       )
                        /*
                        .data(web::JsonConfig::default() // json extractor
                              .limit(4096) // 4096 bytes
                              // None instead Some() in fn !!!
                              .error_handler(post_error), // -> fn post_error
                        )
                        */
                        .route(web::post() // HTTP POST <- instead #[post("/..")]
                               .to(post_msg) // -> fn post_msg 
                        ),
                )
                .service(clear) // -> fn clear #[post("/clear")]
                .service(lookup) // -> fn lookup
                
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
fn index(state: web::Data<AppState>) -> actix_web::Result<web::Json<IndexResponse>> {
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
            state: web::Data<AppState>) -> actix_web::Result<web::Json<PostResponse>> {

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
fn clear(state: web::Data<AppState>) -> actix_web::Result<web::Json<IndexResponse>> {
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


// /*
/// route.to()
///
fn _post_error(err: JsonPayloadError, // what went wrong
               req: &HttpRequest /* ref to request */) -> actix_web::error::Error {

    println!("REQ: {req:?}\nheader: {:#?}\npath: {:#?}\nquery_string: {:#?}\nmatch_info: {:#?}\napp_config.host(): {:#?}\napp_data: {:#?}\nget_app_data: {:#?}",
             req.headers(),
             req.path(),
             req.query_string(),
             req.match_info(),
             req.app_config().host(),
             req.app_data::<web::Data<AppState>>(),
             req.get_app_data::<web::Data<AppState>>(),
    );
    
    // we access our AppState for @request_count + @@server_id
    let extns = req.extensions();
    println!("EXTNS: {:?}", extns.get::<web::Data<AppState>>());
    
    // state is value inside of the extensions with type web::Data<AppState>
    //
    // extensions have a generic function: fn get<T>(&self) -> Option<&T>
    // we get reference via turbofish
    // get::<web::Data<AppState>> means call get<T>()
    //
    // and it returns Option
    //
    //let o_state = extns.get::<web::Data<AppState>>();
        //.unwrap(); // not safe can panic! and it fookin does !!!

    let contains = extns.contains::<web::Data<AppState>>();
    println!("CONTAINS: {contains:?}");
    
    let o_state: Option<_> = extns.get::<web::Data<AppState>>();
    //let o_state: Option<_> = extns.get(web::Data<AppState>);

    match o_state {
        Some(state) => {
            let request_count = state.request_count.get() + 1; // @
            state.request_count.set(request_count);
            
            // let's build our error Struct
            let post_error = PostError {
                server_id: state.server_id, // @@
                request_count: request_count, // @
                error: format!("{}", err), // use Display trait impl
            };
            
            InternalError::from_response(
                err,
                HttpResponse::BadRequest()
                    .json(post_error))
                // std::convert::From
                // std::convert::Into
                .into() // this is needed as it return Error Struct
        },
        
        None => {
            let post_error = PostError {
                server_id: 111,
                request_count: 222,
                error: format!("{}", "foookin NONE"),
            };

            InternalError::from_response(
                err,
                HttpResponse::BadRequest()
                    .json(post_error))
                .into()
        },
    }
    
    /*
    // we are still counting
    let request_count = state.request_count.get() + 1; // @
    state.request_count.set(request_count);

    // let's build our error Struct
    let post_error = PostError {
        server_id: state.server_id, // @@
        request_count: request_count, // @
        error: format!("{}", err), // use Display trait impl
    };

    InternalError::from_response(
        err,
        HttpResponse::BadRequest()
            .json(post_error))
        // std::convert::From
        // std::convert::Into
        .into() // this is needed as it return Error Struct
    */
}
// */

///
/// potunit kdyz budu chit vice dily Path : /1/2/3
///
/// naucit se at to vraci posladni kdyz dam /-1
///
#[get("/lookup/{index}")]
fn lookup(state: web::Data<AppState>,
          //idx: web::Path<usize>) -> actix_web::Result<web::Json<LookupResponse>> {
          idx: web::Path<String>) -> actix_web::Result<web::Json<LookupResponse>> {

    println!("IDX: {idx:?}");

    let mut position;
    
    // deconstruct to inner value
    let to_parse_idx = idx.into_inner();

    // let try if it i64 or not
    let parsed_idx = match to_parse_idx.parse::<i64>() {
        Ok(i) => {
            position = format!("{}", i);
            
            Some(i)
        },
        Err(why) => {
            eprintln!("fooking INDEX: {to_parse_idx}\nREASON >>> {why}");

            position = String::from("null");
            
            None
        },
    };

    println!("PARSED_IDX: {parsed_idx:?}");
    
    // we still add to this thread counter
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    // we lock msg vec
    let ms = state
        .messages
        .lock()
        .unwrap();

    println!("MS: {ms:?}");

    let result = match parsed_idx {
        // we have positive number
        Some(p @ 0..) => {
            position = format!("{}", p);

            ms
                .get(p as usize) // position need to be usize
                .cloned()
        },

        // we want exactly the last
        Some(-1) => {
            let target = ms.last().cloned(); // ms[ms.len()-1]
            
            position = format!("{:?}",
                               ms
                               .iter()
                               .position(|x| {
                                   println!("x: {:?}", target);

                                   Some(x) == target.as_ref()
                               }),
            );

            target
        },

        // bin all other
        _ => None,
    };
    
    /*
    // we get stored message via index 
    let result = ms
        .get(
            parsed_idx
            /*
            parsed_idx // this is our index position of msg in vec, we start at 0
                .into_inner()
            */
        )
        .cloned(); // Option<&String> -> Option<String>
    */

    println!("RESULT: {result:?}");
    
    Ok(web::Json(
        // let's build struct for json
        LookupResponse {
            server_id: state.server_id,
            request_count:request_count,
            result: result,
            position: position,
        }))
}
