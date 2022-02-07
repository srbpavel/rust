use crate::toml_mqtt_config_struct::{TomlConfig};

use mqtt_publisher::{Broker,
                     MsgData,
                     
                     send_msg_to_topic,
};


pub fn sample(config: TomlConfig) {

    // BUILD broker
    let broker_l = Broker {
        machine: &config.broker.machine,
        client_id: &config.broker.client_id,
        interval: config.broker.interval,

        username: &config.broker.username,
        password: &config.broker.password,

        debug: config.broker.debug,
    };

    // BUILD some topics with messages
    let data_batch = vec![
        MsgData {
            topic: "semici",
            body: "foOoKuMe is KiNg na SEMIKOVI",
            qos: 0,
        },

        MsgData {
            topic: "vcely",
            body: "da Ma is QuuEn na TRUBCOVI",
            qos: 0,
        },
    ];

    /* LORD */
    // DEBUG input
    if broker_l.debug {
        println!("BROKER: {broker_l:?}\nDATA: {:?}", data_batch);
    }

    // SEND L
    match send_msg_to_topic(&broker_l, &data_batch) {
        Ok(_) => {},
        Err(transmit_error) => {
            eprintln!("\nERROR send_msg_to_topic: {transmit_error:?}");
        },
    }
}
