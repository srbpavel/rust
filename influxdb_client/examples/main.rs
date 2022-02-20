///
///
///
extern crate influxdb_client; // LIB

// via [dependencies]
//extern crate template_formater;

// via [dev-dependencies]
extern crate easy_config;
extern crate csv;
extern crate chrono;
extern crate serde;

// EASY_CONFIG          
mod example_easy_config;
mod influxdb_toml_config_struct;

// EXAMPLE
mod read_write_verify;


fn main() {
    /*
    println!("EXAMPLE: influxdb_client\n +{}",
             influxdb_client::flux_query::DEFAULT_COUNT,
    );
    */

    // SAMPLA DATA >>> EASY_CONFIG
    let config = example_easy_config::sample_config();
    //println!("\nCONFIG: {config:#?}");    

    
    // INFLUXDB_CLIENT
    let influxdb_status_msg = read_write_verify::start(config);
    
    if influxdb_status_msg.is_err() {
        println!("LIB_TEST: {influxdb_status_msg:?}");
    }
}
