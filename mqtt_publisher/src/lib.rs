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

/*
pub trait BrokerTrait {
    fn send_msg_to_topic(&self,
                         data: &Vec<MsgData>) -> Result<ServerResponse, Error>;

    fn create_options(&self) -> CreateOptions;

    fn connect_options(&self) -> ConnectOptions;

}

impl BrokerTrait for Broker<'_> {
*/
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

    /// broker connection + transmit all messages
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
                        publish_all_msg(&client,
                                        &self,
                                        &data,
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
/// do not forget MQTT TOPIC read|write USER permission is set in BROKER config
#[derive(Debug)]
pub struct MsgData<'d> {
    pub topic: &'d str,
    pub body: &'d str,
    pub qos: i32,
}


/*
pub trait MsgDataTrait {
    fn build_msg(&self,
                 broker: &Broker) -> Message;

}

impl MsgDataTrait for MsgData<'_> {
*/
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
                   data: &Vec<MsgData>) {

    // PUB MSG to TOPIC
    let pub_info = data
        .into_iter()
        .inspect(|d| {
            if broker.debug {
                println!("\nSINGLE_DATA: {d:?}");
            }
        })
        .map(|single_data| {

            let msg = single_data.build_msg(broker);

            /* // DEBUG payload
            if debug {
                println!("MSG: {msg:?}");
            }
            */
            
            client.publish(msg)
        })
        .collect::<Vec<_>>();
    
    if broker.debug {
        println!("\nPUB_INFO: {pub_info:?}");
    }
}
