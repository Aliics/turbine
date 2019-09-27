extern crate turbine;
use turbine::http::tcp;
use turbine::conf::Config;
use std::{net::TcpStream, thread, fs};

// to be honest, there isn't a fantastic way of testing this
// Although, I do have good faith in the Rust developers that
// their lib will do as documented. So this is more or less,
// just a check of see if this all binds and listens.
#[test]
fn sanity_check() {
    let ser_config = fs::read_to_string("tests/example_conf.toml").unwrap();
    let de_config = Config::from(ser_config);
    let tcp = tcp::start_server(de_config.clone());

    thread::spawn(move || tcp::listen(tcp, de_config.clone()));

    TcpStream::connect("localhost:8080").unwrap();
}
