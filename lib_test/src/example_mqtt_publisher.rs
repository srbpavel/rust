// dummy settings and data for broker/topic/msg
use crate::mqtt_toml_config_struct::{TomlConfig};

use mqtt_publisher::{Broker,
                     MsgData,
};


#[allow(dead_code)]
pub fn sample_subscribe(config: TomlConfig) {

    // TOPICS
    let mut topics_batch = config.topics.values
        .iter()
        .filter(|t| t.status)
        .map(|t| {

            MsgData {
                topic: &t.name,
                body: &t.body,
                qos: t.qos,
            }
        })
        .collect::<Vec<_>>();

    // + one more tester
    /*
    topics_batch.push(
        MsgData {
            topic: "hrebecek",
            body: "ja mam peerko",
            qos: 1,
        }
    );
    */

    // BROKER
    let b = &config.broker["lord"];
    
    let broker = Broker {
        machine: &b.machine,
        
        client_id: &b.client_id,

        interval: b.interval,
        
        username: &b.username,
        password: &b.password,
        
        debug: b.debug,

        lifetime: b.sub_lifetime,
        reconnect_delay: b.sub_reconnect_delay,
    };

    /*
    println!("topics: {:?}\nqos: {:?}",
             topics_list,
             qos_list,
    );
    */

    // SUBSCRIBE
    /*
    broker.subscribe(&topics_list,
                     &qos_list,
    );
    */
    broker.subscribe(&topics_batch);
        
}


#[allow(dead_code)]
pub fn sample_publish(config: TomlConfig) {

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

    // ALL BROKERS
    let _ =
        vec![&config.broker["lord"],
             &config.broker["metynka"],
        ]
        .into_iter()
        /* // FUTURE USE
        .inspect(|b| {
            println!("\nCLIENT via config: {}",
                     &format!("{}_rust_{}",
                              config.host,
                              //config.name,
                              &b.username,
                     )
                     .to_uppercase(),
            );
        })
        */
        .map(|b| {

            let broker = Broker {
                machine: &b.machine,

                //client_id: &b.client_id,
                client_id: &format!("{}_rust__{}",
                                    config.host,
                                    &b.username,
                )
                    .to_uppercase(),
                
                interval: b.interval,
                
                username: &b.username,
                password: &b.password,
                
                debug: b.debug,

                lifetime: b.sub_lifetime,
                reconnect_delay: b.sub_reconnect_delay,
            };

            match broker.send_msg_to_topic(&topics_batch) {
                Ok(_) => {},
                Err(transmit_error) => {
                    eprintln!("\nERROR send_msg_to_topic: {transmit_error:?}");
                },
            }
        })
        .collect::<Vec<_>>();
}
