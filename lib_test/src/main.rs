// EASY_CONFIG
mod example_easy_config;

/* // MQTT_PUBLISHER
mod mqtt_toml_config_struct;
mod example_mqtt_client;
*/

// INFLUXDB_CLIENT
mod example_influxdb_client;
mod influxdb_toml_config_struct;

fn main() {
    // EXAMPLE >>> EASY_CONFIG
    let config = example_easy_config::sample();
    //println!("\nCONFIG: {config:#?}");

    // INFLUXDB_CLIENT
    let influxdb_status_msg = example_influxdb_client::start(config);

    println!("\nLIB_TEST: {influxdb_status_msg:?}");

    
    /* MQTT
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
