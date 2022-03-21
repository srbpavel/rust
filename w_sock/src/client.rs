//! Simple websocket client.

use std::{io,
          thread,
};

use actix_web::web::Bytes;
use awc::ws;
use futures_util::{
    SinkExt as _,
    StreamExt as _,
};
use tokio::{
    select,
    sync::mpsc,
};
use tokio_stream::wrappers::UnboundedReceiverStream;


#[actix_web::main]
async fn main() {
    std::env::set_var(
        "RUST_LOG",
        "websocket_client=debug,actix_web=debug,actix_server=info",
    );

    env_logger::init();
    
    /*
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("info")
    );
    */

    log::info!("starting echo WebSocket client");

    // channel
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

    let mut cmd_rx = UnboundedReceiverStream::new(cmd_rx);

    // run blocking terminal input reader on separate thread
    // https://doc.rust-lang.org/std/thread/
    // JoinHandle { .. }
    let input_thread = thread::spawn(move || loop {
        let mut cmd = String::with_capacity(32);

        // out keyboard input
        if io::stdin()
            .read_line(&mut cmd)
            .is_err() {
            log::error!("error reading line");
            return;
        }

        println!("  TX: {:?}",
                 cmd,
        );

        // TRANSMIT CHANNEL 
        cmd_tx
            .send(cmd)
            .unwrap();
    });

    //via awc client connect so we have response + web_socket
    let (res, mut ws) = awc::Client::new()
        // https://docs.rs/awc/3.0.0/awc/struct.Client.html#method.ws
        // init web socket connection
        // return web socket connection builder
        .ws("ws://127.0.0.1:8080/ws")
        // https://docs.rs/awc/3.0.0/awc/ws/struct.WebsocketsRequest.html#method.connect
        // return Result<
        // (ClientResponse, Framed<BoxedSocket, Codec>),
        // WsClientError>
        .connect()
        .await
        .unwrap();

    log::debug!("response: {res:?}");
    log::info!("connected; server will echo messages sent");

    let mut loop_counter = 0;
    
    // infinite 
    loop {
        loop_counter += 1;

        // https://tokio.rs/tokio/tutorial/select
        // awaits on channels
        select! {
            // let's read socket
            // https://docs.rs/actix-codec/0.5.0/actix_codec/struct.Framed.html#method.next_item
            //
            // actix_codec::framed::Framed<Box<dyn awc::client::connection::ConnectionIo>, Codec> 
            //
            // understand why here .next() but in doc .next_item()
            // probably lookin at wrong place in doc
            //
            Some(msg) = ws.next() => {
                match msg {
                    Ok(ws::Frame::Text(txt)) => {
                        println!("    receive Text[{}] -> {:?}\n",
                                 &loop_counter,
                                 txt,
                        );

                        // log echoed messages from server
                        log::info!("  Server: {:?}", txt)
                    }

                    //Ok(ws::Frame::Ping(_)) => {
                    Ok(ws::Frame::Ping(data)) => {
                        println!("  receive Ping[{}] -> {:?}",
                                 &loop_counter,
                                 data,
                        );

                        // /* // simulate PONG timeout
                        // respond to ping
                        ws.send(
                            ws::Message::Pong(
                                //Bytes::new()
                                Bytes::from(
                                    [
                                        &data,
                                        format!(" / {}",
                                                chrono::Utc::now(),
                                        )
                                            .as_bytes(),
                                    ].concat()
                                )
                            )
                        )
                            .await
                            .unwrap();
                        // */
                    }

                    _ => {}
                }
            }


            // RECEIVE
            Some(cmd) = cmd_rx.next() => {
                if cmd.is_empty() {
                    continue;
                }

                println!("   RX[{}]: {:?}",
                         &loop_counter,
                         cmd,
                );

                // send text msg
                ws.send(
                    ws::Message::Text(
                        cmd.into()
                    )
                )
                    .await
                    .unwrap();
            }

            else => break
        }
    }

    // join method returns thread::Result
    // understand this better
    input_thread
        .join()
        .unwrap();
}
