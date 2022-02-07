// EASY_CONFIG
mod example_easy_config;
mod mqtt_toml_config_struct;

// MQTT_PUBLISHER
mod example_mqtt_publisher;


fn main() {
    // EXAMPLE >>> EASY_CONFIG
    let config = example_easy_config::sample();

    // EXAMPLE >>> MQTT_PUBLISH
    example_mqtt_publisher::sample(config);
}
