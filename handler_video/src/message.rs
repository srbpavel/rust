use crate::{AppState,
            //SERVER_COUNTER,
            //SERVER_ORD,
            MSG_ID_COUNTER,
            MSG_ID_ORD,
};

use actix_web::{
    get,
    post,
    web,
    //middleware,
    //App,
    //HttpResponse,
    //HttpServer,
    //Responder,
    // Error, // covered ?
    Result,
};


use serde::{Serialize,
            Deserialize,
};

/*
use std::cell::Cell;                
use std::sync::atomic::{AtomicUsize,
                        Ordering,   
};
*/                                  

/*
use std::sync::{Arc,                
                Mutex,              
};
*/

use std::collections::HashMap;


#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Message {
    body: String,
    id: usize,
}

#[derive(Serialize, Debug)]
pub struct IndexResponse {     
    server_id: usize,      
    request_count: usize,  
    //messages: Vec<String>,
    //messages: Vec<Message>,
    hash_map: HashMap<usize, String>, 
}

#[derive(Deserialize, Debug)]
pub struct PostInput {
    message: String,
}

#[derive(Serialize, Debug)]
pub struct PostResponse {
    server_id: usize,
    request_count: usize,
    //message: String,
    //id: usize,
    message: Message,
}

/* // VEC only
#[derive(Serialize, Debug)]                               
struct LookupResponse {                                   
    server_id: usize,                                     
    request_count: usize,                                 
    //result: Option<String>, // None in JSON will be "null"
    result: Option<Message>, // None in JSON will be "null"
    //position: String,                                   
    //position: Option<String>, // only for VEC
    path: String,
    //id: usize,
}                                                         
*/

#[derive(Serialize, Debug)]                               
pub struct SearchResponse {                                   
    server_id: usize,                                     
    request_count: usize,                                 
    //result: Option<String>, // None in JSON will be "null"
    result: Option<Message>, // None in JSON will be "null"
    //position: String,                                   
    //position: Option<String>, // only for VEC
    path: String,
    //id: usize,
}


#[derive(Serialize, Debug)]                               
pub struct LastResponse {                                   
    server_id: usize,                                     
    request_count: usize,                                 
    //result: Option<String>, // None in JSON will be "null"
    result: Option<Message>, // None in JSON will be "null"
    //position: String,                                   
    //position: Option<String>, // only for VEC
    //path: String,
    //id: usize,
}                                                         



#[get("/")]
pub async fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);          
    
    let msg = state                                  
        // VEC
        //.messages
        // HASH
        .hash_map
        .lock()                                      
        .unwrap();                                   
    
    Ok(                                              
        web::Json(                                   
            IndexResponse {                          
                server_id: state.server_id,          
                request_count: request_count,        
                //messages: msg.clone(),
                hash_map: msg.clone(),
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
pub async fn post_msg(msg: web::Json<PostInput>,
                      state: web::Data<AppState>) -> actix_web::Result<web::Json<PostResponse>> {
    //println!("POST_MSG: {state:?}");    

    // Cell
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    
    // we lock and have access to Vec messages
    let mut ms = state
        //.messages //VEC
        .hash_map // HASH
        .lock() // get access to data inside Mutex + blocks until another thread
        .unwrap(); // -> MutexGuard<Vec<String>> // will panic on Err !!!

    // /CLEAR do not reset counter, yet.
    let message_id = MSG_ID_COUNTER.fetch_add(1,              
                                              //Ordering::SeqCst,
                                              MSG_ID_ORD,
    );          
    
    //println!("BEFORE: {ms:?}");    
    // and we push are new MSG to Vec
    //ms.push(msg.message.clone()); // clone as Vec owns each element
    /* VEC
    ms.push(
        //msg.message.clone()
        Message {
            body: msg.message.clone(),
            id: message_id,
        }
    ); // clone as Vec owns each element
    */

    // HASH
    ms.insert(
        message_id,
        msg.message.clone(),
    );
    
    //println!("AFTER: {ms:?}");
    
    Ok(web::Json(
        PostResponse {
            server_id: state.server_id, // here is our messages: Vec
            request_count: request_count,
            //message: msg.message.clone(), // because it is shared
            //id: message_id,
            message: Message {
                body: msg.message.clone(),
                id: message_id,
            },
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
pub async fn clear(state: web::Data<AppState>) -> actix_web::Result<web::Json<IndexResponse>> {
    //println!("CLEAR");
    
    let request_count = state.request_count.get() + 1; // we still count
    state.request_count.set(request_count);

    let mut ms = state
        // VEC
        //.messages
        // HASH
        .hash_map
        .lock()
        .unwrap(); // niet goed !!! make it safe 
    
    // VEC
    //ms.clear(); // messages are flushed
    // HASH
    ms.clear();
    
    // actualy this is nearly the same as after start with no messages
    // but few server_id and counter count

    Ok(web::Json(
        IndexResponse {
            server_id: state.server_id,
            request_count: request_count,
            // VEC
            //messages: vec![], // no messages for json
            // HASH
            hash_map: HashMap::new(), // no need to create new as we have old
            //hash_map: ms.clone(), // ok but still expenssive?
        }
    ))
}

/// SEARCH via hash
/// 
/// path as String
/// i did not make it work for usize because do no fing way to verify valid usize?
///
pub async fn search(state: web::Data<AppState>,
                //idx: web::Path<usize>) -> actix_web::Result<web::Json<SearchResponse>> {
                    idx: web::Path<String>) -> actix_web::Result<web::Json<SearchResponse>> {

    //println!("IDX: {idx:?}");
    
    // deconstruct to inner value
    let to_parse_idx = idx.into_inner();

    let path = format!("/search/{}", // take this from req
                       to_parse_idx,
    );

    // let's try parse
    let parsed_idx = match to_parse_idx.parse::<usize>() {
        Ok(i) => {
            Some(i)
        },
        Err(why) => {
            eprintln!("foookin INDEX: {to_parse_idx}\nREASON >>> {why}");

            None
        },
    };

    //println!("PARSED_IDX: {parsed_idx:?}");
    
    // we still add to this thread counter
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    // we lock msg vec
    let ms = state
        .hash_map
        .lock()
        .unwrap();

    //println!("MS: {ms:?}");

    //let result = match ms.get(&to_parse_idx.clone()) {
    let result = match parsed_idx {
        Some(i) =>  
            match ms.get(&i) {
                Some(msg) => Some(
                    Message {
                        id: i,
                        body: msg.to_string(),
                    }
                ),
                None => None,
            },
        None => None,
    };
    
    /*
    //let result = match ms.get(&to_parse_idx.clone()) {
    let result = match ms.get(&parsed_idx.clone()) {
        Some(msg) => Some(
            Message {
                id: to_parse_idx.clone(),
                body: msg.to_string(),
            }
        ),
        None => None,
    };
    */

    //println!("RESULT: {result:?}");
    
    Ok(
        web::Json(
            // let's build struct for json
            SearchResponse {
                server_id: state.server_id,
                request_count:request_count,
                result: result,
                path: path,
            }
        )
    )
}


/// DELETE via id -> return all msg hash without deleted one
/// 
/// path as String
///
pub async fn delete(state: web::Data<AppState>,
                    idx: web::Path<String>) -> Result<web::Json<IndexResponse>> {

    println!("IDX: {idx:?}");
    
    // deconstruct to inner value
    let to_parse_idx = idx.into_inner();

    /* DO NOT USE HERE
    let path = format!("/delete/{}", // take this from req
                       to_parse_idx,
    );
    */

    // let's try parse
    let parsed_idx = match to_parse_idx.parse::<usize>() {
        Ok(i) => {
            Some(i)
        },
        Err(why) => {
            eprintln!("foookin INDEX: {to_parse_idx}\nREASON >>> {why}");

            None
        },
    };

    println!("PARSED_IDX: {parsed_idx:?}");
    
    // we still add to this thread counter
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    // we lock msg vec, but now as MUT because we delete
    // we did not do MUT for push ?
    let mut msg = state
        .hash_map
        .lock()
        .unwrap();

    println!("MSG before DEL: {msg:?}");

    // try to make it let shorter !!!
    let result = match parsed_idx {
        Some(i) =>  
            // DELETE
            match msg.remove(&i) {
                Some(msg) => {
                    println!("DELETED: {msg}");

                    // later this will be another Json Response
                    Some(format!("{}: {}",
                                 i,
                                 msg,
                    ))
                },
                None => {
                    // later this will be another Json Response
                    println!("NOT FOUND SO: {parsed_idx:?} stay");

                    None
                },
            },
        None => {
            println!("DELETE key {to_parse_idx:?} not valid Type");
            None
        },
    };
    
    println!("RESULT: {result:?}");
    
    Ok(
        web::Json(
            IndexResponse {                          
                server_id: state.server_id,          
                request_count: request_count,        
                hash_map: msg.clone(),
            }                                        
        )
    )
}


/// LAST via Hash key: id
///
/// beware: not correct if last was deleted as we take it MSG_ID_COUNTER
/// and not via iter Hash_map
///
/// do we want/need such a method?
///
#[get("/last")]
pub async fn last(state: web::Data<AppState>) -> actix_web::Result<web::Json<LastResponse>> {

    // we still add to this thread counter
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);

    // we lock msg vec
    let ms = state
        .hash_map
        .lock()
        .unwrap();

    //let last_id = &MSG_ID_COUNTER.load(Ordering::SeqCst) - 1;
    let last_id = &MSG_ID_COUNTER.load(MSG_ID_ORD) - 1;

    //println!("LAST: {:?}", last_id);

    let result = match ms.get(&last_id) {
        Some(msg) =>
            Some(
                // our last msg
                Message {
                    id: last_id,
                    body: msg.to_string(),
                }
            ),
        None => None,
    };

    //println!("RESULT: {result:?}");
    
    Ok(
        web::Json(
            // let's build struct for json
            LastResponse {
                server_id: state.server_id,
                request_count:request_count,
                result: result,
            }
        )
    )
}
