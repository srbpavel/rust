use std::time::{Duration,
                Instant,
};

use actix::prelude::*;
use actix_web_actors::ws;

// this starts after connected
/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

impl MyWebSocket {
    pub fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self,
          ctx: &mut <Self as Actor>::Context) {

        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            let ping_msg = format!("{}",
                                   chrono::Utc::now(),
            );
            
            println!("  Ping send: {ping_msg}");
            //ctx.ping(b"");
            ctx.ping(ping_msg.as_bytes());
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self,
               ctx: &mut Self::Context) {

        println!("let's start hearth_beat");
        
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self,
              msg: Result<ws::Message, ws::ProtocolError>,
              ctx: &mut Self::Context) {

        // process websocket messages
        //println!("WS: {:#?}", msg);

        match msg {
            Ok(ws::Message::Ping(msg)) => {
                println!("   Ping recieve: {:?}",
                         msg,
                );

                self.hb = Instant::now();

                ctx.pong(&msg);
            },
            
            //Ok(ws::Message::Pong(_)) => {
            Ok(ws::Message::Pong(msg)) => {
                println!("   Pong receive: {:?}",
                         msg,
                );

                self.hb = Instant::now();
            },
            
            Ok(ws::Message::Text(text)) => {
                println!("\n   Text receive: {:?}\n",
                         text,
                );

                ctx.text(text)
            },
            
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),

            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            
            _ => {
                println!("    Other receive --> we stop");
                
                ctx.stop()
            },
        }
    }
}
