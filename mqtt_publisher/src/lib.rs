use paho_mqtt::{
    self as mqtt,

    Client,
    Error,
    Message,
    //ServerResponse,

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
    /// reconnect 
    fn try_reconnect(&self,
                     client: &Client) -> bool
    {
        println!("Connection lost. Waiting to retry connection");
        for _ in 0..12 {
            std::thread::sleep(std::time::Duration::from_millis(5000));
            if client.reconnect().is_ok() {
                println!("Successfully reconnected");
                return true;
            }
        }
        println!("Unable to reconnect after several attempts.");
        false
    }

    /// initial connection
    fn connect(&self) -> Result<Client, Error> {

        // USER 
        let options = self.connect_options();
        
        // BROKER
        match mqtt::Client::new(self.create_options()) {
            
            // CLIENT options valid
            //Ok(mut client) => {
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
                        //Ok(&mut client)
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
    
    pub fn subscribe_topics(&self,
                            topics: &[&str],
                            //qos: &[i32]) -> Result<ServerResponse, Error> {
                            qos: &[i32]) {
        
        let c = self.connect();
        
        match c {
            // SUB need's to be mutable
            Ok(mut client) => {

                let rx = &client.start_consuming();
                
                if let Err(e) = &client.subscribe_many(topics,
                                                       qos,
                ) {
                    println!("Error subscribes topics: {:?}", e);
                    std::process::exit(1);
                };
                
                println!("Processing requests...");
                for msg in rx.iter() {
                    if let Some(msg) = msg {
                        println!("{}", msg);
                    } else if !client.is_connected() {

                        println!("not connected...");
                        
                        if self.try_reconnect(&client) {
                            println!("Resubscribe topics...");
                            
                            if let Err(e) = &client.subscribe_many(topics,
                                                                   qos,
                            ) {
                                
                                println!("Error subscribes topics: {:?}", e);
                                std::process::exit(1);
                            };
                            
                        } else { break; }
                    }
                }
            },

            Err(_) => {
                std::process::exit(1)
            },
        };
    }
    
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

    /// broker connection + transmit all messages
    pub fn send_msg_to_topic(&self,
                             data: &Vec<MsgData>) -> Result<(), Error> {

        let c = self.connect();
        
        match c {
            // PUB not mutable
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
                
                // CLOSE session
                /*
                if let Err(r) = client.disconnect(None) {
                    return Err(r)
                };
                */

                client.disconnect(None)
            },
            
            Err(why) => {
                //std::process::exit(1)
                Err(why)
            },
        }//;
    }
    
    /*
    pub fn send_msg_to_topic(&self,
                             data: &Vec<MsgData>) -> Result<ServerResponse, Error> {
        
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
                        
                        // CLOSE session
                        if let Err(r) = client.disconnect(None) {
                            return Err(r)
                        };
                        
                        Ok(response)
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
    */
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
