/*
// EASY_CONFIG
mod example_easy_config;
mod mqtt_toml_config_struct;

// MQTT_PUBLISHER
mod example_mqtt_client;
*/

// INFLUXDB_CLIENT
mod example_influxdb_client;
    

fn main() {
    let err_msg = example_influxdb_client::start();

    println!("\nMAIN: {err_msg:?}");

    /*
    // EXAMPLE >>> EASY_CONFIG
    let config = example_easy_config::sample();

    match &*config.service_type {

        // EXAMPLE >>> MQTT_PUBLISH
        "pub" => {    
            
            example_mqtt_client::sample_publish(config);
        },
        
        // EXAMPLE >>> MQTT_SUBSCRIBE
        "sub" => {
            example_mqtt_client::sample_subscribe(config);
        },
        
        other => {
            eprint!("INVALID mqtt type: {other:?}");
        }
    }
    */
}
