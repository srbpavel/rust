use paho_mqtt::{
    self as mqtt,

    Client,
    Error,
    Message,
    ServerResponse,

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

pub trait BrokerTrait {
    fn send_msg_to_topic(&self,
                         data: &Vec<MsgData>) -> Result<ServerResponse, Error>;

    fn create_options(&self) -> CreateOptions;

    fn connect_options(&self) -> ConnectOptions;

}

impl BrokerTrait for Broker<'_> {

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
            .server_uri(String::from(self.machine))
            .client_id(String::from(self.client_id))
            .finalize()
    }
    
    fn send_msg_to_topic(&self,
                         data: &Vec<MsgData>) -> Result<ServerResponse, Error> {
        
        // USER 
        //let options = connect_options(&self);
        let options = self.connect_options();
        
        // BROKER
        //match mqtt::Client::new(create_options(&self)) {
        match mqtt::Client::new(self.create_options()) {
            
            // CLIENT options valid
            Ok(client) => {
                
                match client.connect(options) {
                    
                    // CLIENT connected
                    Ok(response) => {
                        
                        if self.debug {
                            println!("RESPONSE: {response:?}\nUSER: {}",
                                     &self.username,
                            );
                        }
                        
                        // TRANSIMT msg
                        publish_msg(&client,
                                    &data,
                                    self.debug,
                        );
                        
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
}


/// msg_data
///
/// do not forget that MQTT TOPIC read|write is valid only for some user's
#[derive(Debug)]
pub struct MsgData<'d> {
    pub topic: &'d str,
    pub body: &'d str,
    pub qos: i32,
}

pub trait MsgDataTrait {
    fn build_msg(&self,
                 debug: bool) -> Message;

}

impl MsgDataTrait for MsgData<'_> {
    /// construct message with topic 
    fn build_msg(&self,
                 debug: bool) -> Message {
        
        let msg = format!("{{\"data\": \"{body}\" \"datetime\": \"{now}\"}}",
                          now = chrono::Local::now(),
                          body = self.body,
        );    
        
        if debug {
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

/*
/// broker_machine + client_id
fn create_options(broker: &Broker) -> CreateOptions {
    mqtt::CreateOptionsBuilder::new()
        .server_uri(String::from(broker.machine))
        .client_id(String::from(broker.client_id))
        .finalize()
}


/// user credentials + interval
fn connect_options(broker: &Broker) -> ConnectOptions {
    mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(
            std::time::Duration::from_secs(broker.interval)
        )
        .clean_session(true)
        .user_name(broker.username)
        .password(broker.password)
        .finalize()
}
*/

/*
/// construct message with topic 
fn build_msg(data: &MsgData,
             debug: bool) -> Message {

    let msg = format!("{{\"data\": \"{body}\" \"datetime\": \"{now}\"}}",
                      now = chrono::Local::now(),
                      body = data.body,
    );    

    if debug {
        println!("TRANSMIT:\n topic: {:?}\n msg: {:?}",
                 data.topic,
                 msg,
        );
    }
    
    mqtt::Message::new(
        data.topic,
        msg,
        data.qos,
    )
}
*/

/// transmit all messages
fn publish_msg(client: &Client,
               data: &Vec<MsgData>,
               debug: bool) {

    // PUB MSG to TOPIC
    let pub_info = data
        .into_iter()
        .inspect(|d| {
            if debug {
                println!("SINGLE_DATA: {d:?}");
            }
        })
        .map(|single_data| {

            let msg = single_data.build_msg(debug);
            /*
            let msg = build_msg(single_data,
                                debug,
            );
            */
            
            client.publish(msg)
        })
        /*
        .filter(|f| match f {
            // /* INVALID
            Ok(_) => false,
            Err(_) => true,
            // */
        })
        */
        .collect::<Vec<_>>();
    
    if debug {
        println!("PUB_INFO: {pub_info:?}");
    }
}


/*
/// broker connection + transmit all messages
pub fn send_msg_to_topic(broker: &Broker,
                         data: &Vec<MsgData>) -> Result<ServerResponse, Error> {
    
    // USER 
    let options = connect_options(&broker);
    
    // BROKER
    match mqtt::Client::new(create_options(&broker)) {

        // CLIENT options valid
        Ok(client) => {

            match client.connect(options) {

                // CLIENT connected
                Ok(response) => {

                    if broker.debug {
                        println!("RESPONSE: {response:?}\nUSER: {}",
                                 broker.username,
                        );
                    }

                    // TRANSIMT msg
                    publish_msg(&client,
                                &data,
                                broker.debug,
                    );
                    
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
