use paho_mqtt::{
    self as mqtt,

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
        mqtt::ConnectOptionsBuilder::new()
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
        mqtt::CreateOptionsBuilder::new()
            .server_uri(String::from(self.machine)) // protocol://host:port
            .client_id(String::from(self.client_id))
            .finalize()
    }

    /// initial client connection
    fn connect(&self) -> Result<Client, Error> {

        // USER 
        let options = self.connect_options();
        
        // BROKER
        match mqtt::Client::new(self.create_options()) {
            
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
    fn try_reconnect(&self,
                     client: &Client) -> bool {

        let delay = 5000;
        let cycles = 10;
        
        println!("connection lost: waiting to retry connection");

        for _ in 0..=cycles {
            std::thread::sleep(
                std::time::Duration::from_millis(delay));

            if client.reconnect().is_ok() {
                println!("successfully reconnected");

                return true;
            }
        }

        println!("unable to reconnect");

        false
    }

    /// sub to all topics
    fn subscribe_topics(&self,
                        client: &Client,
                        topics: &[&str],
                        qos: &[i32]) {
        
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
                     //qos: &[i32]) -> Result<ServerResponse, Error> {
                     qos: &[i32]) {
        
        let client_result = self.connect();
        
        match client_result {
            // CONNECTED + SUB need's to be mutable
            Ok(mut client) => {

                let rx = &client.start_consuming();

                // ERROR: not enough QOS args -> PahoDescr(-9, "Bad QoS")
                // &["semici", "vcely"],
                // &[1],
                /*
                if let Err(why) = &client.subscribe_many(topics,
                                                       qos,
                ) {
                    println!("ERROR subscribes topics: {topics:?}\nREASON >>> {:?}", why);

                    std::process::exit(1);
                };
                */
                self.subscribe_topics(&client,
                                      topics,
                                      qos,
                );
                
                println!("TOPICS: {topics:?} -> waiting for RX...");

                for msg in rx.iter() {

                    // MSG
                    if let Some(msg) = msg {

                        parse_msg(msg)

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
                // FUTURE USE
                /*
                    .iter()
                    .inspect(|m| {
                        if self.debug {
                            println!("msg_result: {:?}", m);
                        }
                    })
                    .collect::<Vec<_>>();
                */
                
                // CLOSE session -> Result
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

                          now = chrono::Local::now(),
                          body = self.body,
                          user = broker.username,
        );    
        
        if broker.debug {
            println!("TRANSMIT:\n topic: {:?}\n msg: {:?}",
                     self.topic,
                     msg,
            );
        }
        
        mqtt::Message::new(
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

    // FUTURE USE - parse topic 
    match msg.topic() {

        "vcely" => {

            println!("\nINCOMMING <vcely> :\n topic: {}\n payload[str]: {:?}",
                     msg.topic(),
                     msg.payload_str(), // as string
            );
        },
        
        "semici" => {
            println!("\nINCOMMING <semici> :\n topic: {}\n payload[raw]: {:?}",
                     msg.topic(),
                     msg.payload(), // RAW 
            );
        },
        
        _ => {
            println!("\nINCOMMING <...> :\n topic: {}\n msg: {:?}",
                     msg.topic(),
                     msg, // full msg
            );
        },
    }
}
