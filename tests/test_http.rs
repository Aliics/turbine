extern crate turbine;
use turbine::http::tcp;
use std::{net::TcpStream, thread};

// to be honest, there isn't a fantastic way of testing this
// Although, I do have good faith in the Rust developers that
// their lib will do as documented. So this is more or less,
// just a check of see if this all binds and listens.
#[test]
fn sanity_check() {
    let tcp = tcp::start_server("localhost:8080".to_string());

    thread::spawn(move || tcp::listen(tcp));

    TcpStream::connect("localhost:8080").unwrap();
}
