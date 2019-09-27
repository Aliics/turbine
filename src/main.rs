pub mod conf;
pub mod http;

use conf::Config;
use http::tcp;
use std::fs;

fn main() {
    let config = get_config();
    let listener = tcp::start_server(config.clone());

    tcp::listen(listener, config.clone()).join().unwrap();
}

fn get_config() -> Config {
    let config_str = fs::read_to_string("config.toml").unwrap();
    Config::from(config_str)
}
