// EASY_CONFIG
mod example_easy_config;
mod mqtt_toml_config_struct;

// MQTT_PUBLISHER
mod example_mqtt_publisher;


fn main() {
    // EXAMPLE >>> EASY_CONFIG
    let config = example_easy_config::sample();

    match &*config.service_type {

        // EXAMPLE >>> MQTT_PUBLISH
        "pub" => {    
            
            example_mqtt_publisher::sample_publish(config);
        },
        
        // EXAMPLE >>> MQTT_SUBSCRIBE
        "sub" => {
            example_mqtt_publisher::sample_subscribe(config);
        },
        
        other => {
            eprint!("INVALID mqtt type: {other:?}");
        }
    }
}
