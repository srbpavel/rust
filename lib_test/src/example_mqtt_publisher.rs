// local crate
use mqtt_publisher::{Broker,
                     MsgData,
                     
                     send_msg_to_topic,
};

pub fn sample() {

    // BUILD broker
    let broker_l = Broker {
        machine: "tcp://jozefina:1883",
        client_id: "SPONGEBOB_RUST_L",
        interval: 20,

        username: "lord",
        password: "lord",

        //debug: true,
        debug: false,
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
