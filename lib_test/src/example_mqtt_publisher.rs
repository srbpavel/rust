use crate::toml_mqtt_config_struct::{TomlConfig};

use mqtt_publisher::{Broker,
                     MsgData,

                     //BrokerTrait,
};

pub fn sample(config: TomlConfig) {

    //let lord = &config.broker["lord"];
    
    // BUILD broker
    let broker_l = Broker {
        machine: &config.broker["lord"].machine,
        client_id: &config.broker["lord"].client_id,
        interval: config.broker["lord"].interval,

        username: &config.broker["lord"].username,
        password: &config.broker["lord"].password,

        debug: config.broker["lord"].debug,
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
        println!("\nBROKER: {broker_l:?}\nDATA: {:?}", data_batch);
    }

    // SEND L
    match broker_l.send_msg_to_topic(&data_batch) {
        Ok(_) => {},
        Err(transmit_error) => {
            eprintln!("\nERROR send_msg_to_topic: {transmit_error:?}");
        },
    }

    /* METYNKA */
    let broker_m = Broker {
        machine: &config.broker["metynka"].machine,
        client_id: &config.broker["metynka"].client_id,
        interval: config.broker["metynka"].interval,

        username: &config.broker["metynka"].username,
        password: &config.broker["metynka"].password,

        debug: config.broker["metynka"].debug,
    };

    // DEBUG input
    if broker_m.debug {
        println!("BROKER: {broker_m:?}\nDATA: {:?}", data_batch);
    }
    
    // SEND M
    match broker_m.send_msg_to_topic(&data_batch) {
        Ok(_) => {},
        Err(transmit_error) => {
            eprintln!("\nERROR send_msg_to_topic: {transmit_error:?}");
        },
    }
}
