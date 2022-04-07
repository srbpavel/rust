/// CONTENT
use crate::handler::AppState;
use actix_web::{
    web::{self, BufMut, Bytes, BytesMut},
    Error, HttpResponse, Responder, Result,
};
//use log::debug;
use futures_util::{Stream, StreamExt};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::mpsc::{channel, Receiver, Sender};

const PATH_DELIMITER: char = '/';
const PATH_LIST_SUFFIX: &'static str = "/*";

pub type ContentKey = String;
pub type BinaryValue = Binary;

/// K for map
#[derive(Debug, Clone)]
pub struct Content {
    id: String,
}

/// V for map
#[derive(Debug, Clone)]
pub struct Binary {
    data: BytesMut,
    completed: bool,
    clients: Vec<(Sender<Bytes>, bool)>,
}

impl Binary {
    pub fn new() -> Self {
        Self {
            data: web::BytesMut::new(),
            completed: false,
            clients: Vec::new(),
        }
    }
}

/// channel for continuos chunk streaming
///
pub struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_recv(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// put_content PAYLOAD
///
pub async fn put_content_p(
    mut payload: web::Payload,
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let AppState { binary_map } = &*state.into_inner();

    let content_id = path.into_inner();

    let new_content = Content {
        id: remove_suffix(&content_id, PATH_DELIMITER).to_string(),
    };

    let mut buf = Binary::new();
    let mut actual_clients = Vec::new();

    while let Some(chunk) = payload.next().await {
        let data = chunk?;
        let just_last_chunk = data.clone();

        buf.data.put(data);

        if let Some(record) = binary_map.get(&new_content.id.clone()) {
            actual_clients = record.clients.clone();
        };

        let all_clients: Vec<_> = actual_clients
            .iter_mut()
            .map(|(client, initial_start)| {
                let for_client = if *initial_start {
                    *initial_start = false;

                    buf.data.clone().freeze()
                } else {
                    just_last_chunk.clone()
                };

                async {
                    let result = client.clone().try_send(for_client);

                    if let Ok(()) = result {
                        Some((client.clone(), initial_start))
                    } else {
                        None
                    }
                }
            })
            .collect();

        let mut alive_clients = Vec::new();
        let results = futures::future::join_all(all_clients).await;

        results.iter().for_each(|r| {
            if let Some((c, i)) = r {
                alive_clients.push((c.clone(), **i));
            }
        });

        binary_map.insert(
            new_content.id.clone(),
            Binary {
                data: buf.data.clone(),
                completed: false,
                clients: alive_clients.clone(),
            },
        );
    }

    binary_map.insert(
        new_content.id,
        Binary {
            data: buf.data,
            completed: true,     // all data are uploaded
            clients: Vec::new(), // so channels for clients are not needed
        },
    );

    Ok(HttpResponse::Ok().body("Status::UploadOk"))
}

/// get_content
///
pub async fn get_content(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let mut content_id = path.into_inner();

    content_id = remove_suffix(&content_id, PATH_DELIMITER).to_string();

    // limit hardcoded here
    let (tx, rx) = channel(100);

    match state.binary_map.get_mut(&content_id) {
        Some(mut r) => {
            if r.completed {
                return HttpResponse::Ok().body(r.data.clone());
            }

            // insert new client TX channel + first_time flag
            r.clients.push((tx, true));
        }
        None => return HttpResponse::Ok().body("Status::IdNotFound"),
    };

    HttpResponse::Ok().streaming(Client(rx))
}

/// delete_content
///
pub async fn delete_content(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let mut content_id = path.into_inner();

    content_id = remove_suffix(&content_id, PATH_DELIMITER).to_string();

    let result = match &mut state.binary_map.remove(&content_id) {
        Some(_) => "Status::DeleteOk",
        None => "Status::DeleteBinaryError",
    };

    HttpResponse::Ok().body(result)
}

/// list_content
///
/// no limit or padding
///
pub async fn list_content(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let mut content_id = path.into_inner();

    /*
    content_id = match content_id.strip_suffix(PATH_DELIMITER) {
        Some(c) => String::from(c),
        None => content_id,
    };
    */

    content_id = remove_suffix(&content_id, PATH_DELIMITER).to_string();
    
    let search_pattern = match content_id.strip_suffix(PATH_LIST_SUFFIX) {
        Some(p) => p,
        None => return HttpResponse::Ok().body("notValidSearchPattern"),
    };

    let result = format!(
        "\nLIST[{}]:\n PATTERN: {} <-> {}\n ALL: {:?}\n FILTER: {:?}",
        state.binary_map.len(),
        content_id,
        search_pattern,
        state
            .binary_map
            .clone()
            .iter()
            .map(|d| d.key().clone())
            .collect::<Vec<_>>(),
        state
            .binary_map
            .clone()
            .iter()
            /*
            .filter(|d| d.key().starts_with(&search_pattern))
            .map(|d| d.key().clone())
            */
            .filter_map(|d| if d.key().starts_with(&search_pattern) {
                Some(d.key().clone())
            } else {
                None
            })
            .collect::<Vec<_>>(),
    );

    HttpResponse::Ok().body(result)
}

/// remove trailing char
///
fn remove_suffix<'a>(text: &'a str, pattern: char) -> &'a str {
    match &text.strip_suffix(pattern) {
        Some(t) => t,
        None => text,
    }
}
