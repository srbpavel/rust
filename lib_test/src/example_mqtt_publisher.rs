use crate::mqtt_toml_config_struct::{TomlConfig};

use mqtt_publisher::{Broker,
                     MsgData,

                     //BrokerTrait,
};

pub fn sample(config: TomlConfig) {

    // TOPICS
    let topics_batch = &config.topics.values
        .iter()
        /*
        .inspect(|t| {
            println!("\nTOPIC: {:?}", t);
        })
        */
        .filter(|t| t.status)
        .map(|t| MsgData {
            topic: &t.name,
            body: &t.body,
            qos: t.qos,
        },)
        .collect::<Vec<_>>();

    // DEBUG TOPICS input
    if config.topics.debug {
        println!("\nTOPICS: {topics_batch:#?}");
    }
        
    /* LORD */
    // BUILD
    let b = &config.broker["lord"];
    
    let broker_l = Broker {
        machine: &b.machine,
        client_id: &b.client_id,
        interval: b.interval,

        username: &b.username,
        password: &b.password,

        debug: b.debug,
    };

    // SEND
    match broker_l.send_msg_to_topic(&topics_batch) {
        Ok(_) => {},
        Err(transmit_error) => {
            eprintln!("\nERROR send_msg_to_topic: {transmit_error:?}");
        },
    }

    /* METYNKA */
    // BUILD
    let b = &config.broker["metynka"];
    
    let broker_m = Broker {
        machine: &b.machine,
        client_id: &b.client_id,
        interval: b.interval,

        username: &b.username,
        password: &b.password,

        debug: b.debug,
    };
    
    // SEND
    match broker_m.send_msg_to_topic(&topics_batch) {
        Ok(_) => {},
        Err(transmit_error) => {
            eprintln!("\nERROR send_msg_to_topic: {transmit_error:?}");
        },
    }
}
