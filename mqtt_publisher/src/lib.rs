use paho_mqtt::{
    ConnectOptionsBuilder,
    CreateOptionsBuilder,
    Client,
    Error,
    Message,

    create_options::{
        CreateOptions,
    },
    connect_options::{
        ConnectOptions,
    },
};

use chrono::Local;


/// settings for broker
#[derive(Debug)]
pub struct Broker<'b> {
    pub machine: &'b str,
    pub client_id: &'b str,
    pub interval: u64,

    pub username: &'b str,
    pub password: &'b str,

    pub debug: bool,
}


impl Broker<'_> {
    /// user credentials + interval
    fn connect_options(&self) -> ConnectOptions {
        ConnectOptionsBuilder::new()
            .keep_alive_interval(
                std::time::Duration::from_secs(self.interval)
            )
            .clean_session(true)
            .user_name(self.username)
            .password(self.password)
            .finalize()
    }

    /// broker_machine + client_id
    fn create_options(&self) -> CreateOptions {
        CreateOptionsBuilder::new()
            .server_uri(String::from(self.machine)) // protocol://host:port
            .client_id(String::from(self.client_id))
            .finalize()
    }

    /// initial client connection
    fn connect(&self) -> Result<Client, Error> {

        // USER 
        let options = self.connect_options();
        
        // BROKER
        match Client::new(self.create_options()) {
            
            // CLIENT options valid
            Ok(client) => {

                match client.connect(options) {
                    
                    // CLIENT connected
                    Ok(response) => {
                        if self.debug {
                            println!("\nRESPONSE: {response:?}\nUSER: {}",
                                     &self.username,
                            );
                        }
                            
                        Ok(client)
                    },
                    
                    /*
                    ERROR user/pass -> PahoDescr(5, "CONNACK return code")
                    ERROR broker PORT -> PahoDescr(-1, "TCP/TLS connect failure")
                    ERROR broker HOST/IP -> Paho(-1)
                     */
                    Err(response_error) => {
                        eprintln!("\nSERVER RESPONSE: ERROR: Unable to connect\nREASON >>> {response_error:?}");
                        
                        Err(response_error)
                    }
                }
                
            },
            
            /*
            ERROR broker wrong protocol -> Paho(-14)
             */
            Err(client_error) => {
                eprintln!("\nCLIENT build: Error\nREASON >>> {client_error:?}");
                
                Err(client_error)
            },
        }
    }

    /// reconnect
    ///
    /// if in systemctl service it will restart after error as per config
    ///
    /// otherwise change here to infinite wait
    fn try_reconnect(&self,
                     client: &Client) -> bool {

        let delay = 5; // SEC
        let cycles = 10;
        
        println!("connection lost: waiting to retry connection: {cycles}x times with {delay}s delay <- {now:?}",
                 now = Local::now(),
        );

        for _ in 0..cycles {
            std::thread::sleep(
                std::time::Duration::from_millis(delay * 1000));

            if client.reconnect().is_ok() {
                println!("successfully reconnected <- {now:?}",
                         now = Local::now(),
                );

                return true;
            }
        }

        println!("unable to reconnect <- {now:?}",
                 now = Local::now(),
        );

        false
    }

    /// sub to all topics
    fn subscribe_topics(&self,
                        client: &Client,
                        topics: &[&str],
                        qos: &[i32]) {
        /*
        ERROR: not enough QOS args -> PahoDescr(-9, "Bad QoS")
        &["semici", "vcely"],
        &[1],
        */
        if let Err(why) = &client.subscribe_many(topics,
                                                 qos,
        ) {
            println!("EXIT subscribes topics: {topics:?}\nREASON >>> {:?}", why);
            
            std::process::exit(1);
        };
    }

    /// SUB call
    pub fn subscribe(&self,
                     topics: &[&str],
                     qos: &[i32]) {
        
        let client_result = self.connect();
        
        match client_result {
            // CONNECTED + SUB need's to be mutable
            Ok(mut client) => {

                let listener = &client.start_consuming();

                self.subscribe_topics(&client,
                                      topics,
                                      qos,
                );
                
                println!("TOPICS: {topics:?} QOS: {qos:?} -> waiting for incomming data... <- {now:?}",
                         now = Local::now(),
                );

                // LISTENER
                for msg_in in listener.iter() {

                    if let Some(msg_to_parse) = msg_in {
                        // MSG PARSER via topics
                        parse_msg(msg_to_parse)

                    // ALIVE 
                    } else if !client.is_connected() {

                        println!("not connected...");

                        // RECONNECT
                        if self.try_reconnect(&client) {
                            println!("repeat subscribe topics...");

                            self.subscribe_topics(&client,
                                                  topics,
                                                  qos,
                            );
                            
                        } else { break; }
                    }
                }
            },

            Err(_) => {
                std::process::exit(1)
            },
        };
    }

    /// broker connection + transmit all messages
    pub fn send_msg_to_topic(&self,
                             data: &Vec<MsgData>) -> Result<(), Error> {

        let c = self.connect();
        
        match c {
            // CONNECTED + PUB not mutable
            Ok(client) => {
                
                if self.debug {
                    println!("\nUSER: {}",
                             &self.username,
                    );
                }
                
                // TRANSIMT msg
                let _msg_results = publish_all_msg(&client,
                                                   &self,
                                                   &data,
                );
                // FUTURE USE -> Err MSG alarma
                /*
                    .iter()
                    .inspect(|m| {
                        if self.debug {
                            println!("msg_result: {:?}", m);
                        }
                    })
                    .collect::<Vec<_>>();
                */
                
                // CLOSE session
                client.disconnect(None)
            },
            
            Err(why) => {
                Err(why)
            },
        }
    }
}


/// msg_data
///
/// do not forget !!!
/// TOPIC read|write USER permission is set in BROKER [mosquitto/...] config
#[derive(Debug)]
pub struct MsgData<'d> {
    pub topic: &'d str,
    pub body: &'d str,
    pub qos: i32,
}


impl MsgData<'_> {
    /// construct message with topic 
    fn build_msg(&self,
                 broker: &Broker) -> Message {

        // JSON sample payload -> FUTURE USE
        // fix -> template formater or via hash map
        let msg = format!("{{\"data\": \"{body}\", \
                           \"datetime\": \"{now}\", \
                           \"user\": \"{user}\" \
                           }}",

                          now = Local::now(),
                          body = self.body,
                          user = broker.username,
        );    
        
        if broker.debug {
            println!("TRANSMIT:\n topic: {:?}\n msg: {:?}",
                     self.topic,
                     msg,
            );
        }
        
        Message::new(
            self.topic,
            msg,
            self.qos,
        )
    }
}


/// transmit all messages
fn publish_all_msg(client: &Client,
                   broker: &Broker,
                   data: &Vec<MsgData>) -> Vec<Result<(), Error>> {

    // PUB MSG to TOPIC
    data
        .into_iter()
        .inspect(|d| {
            if broker.debug {
                println!("\nSINGLE_DATA: {d:?}");
            }
        })
        .map(|single_data| {

            let msg = single_data.build_msg(broker);
            
            client.publish(msg)
        })
        .collect::<Vec<_>>()
}


/// parse incommint msg
fn parse_msg(msg: Message) {

    let now = Local::now();
    
    // FUTURE USE - parse topic 
    match msg.topic() {

        "vcely" => {

            println!("\nINCOMMING <vcely> {now}\n topic: {}\n payload[str]: {:?}",
                     msg.topic(),
                     msg.payload_str(), // as string
            );
        },
        
        "semici" => {
            println!("\nINCOMMING <semici> {now}\n topic: {}\n payload[raw]: {:?}",
                     msg.topic(),
                     msg.payload(), // RAW 
            );
        },
        
        _ => {
            println!("\nINCOMMING <...> {now}\n topic: {}\n msg: {:?}",
                     msg.topic(),
                     msg, // full msg
            );
        },
    }
}
