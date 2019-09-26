use std::{
    format, fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let tcp = TcpListener::bind("localhost:8080").unwrap();

    for incoming in tcp.incoming() {
        thread::spawn(|| say_hello(incoming.unwrap()));
    }
}

fn say_hello(mut stream: TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();

    let buf_str = String::from_utf8_lossy(&mut buf);
    let buf_split: Vec<&str> = buf_str.split(' ').collect();
    let page_path = format!("pages/{}", buf_split[1]);

    let response_data = match fs::read_to_string(page_path) {
        Ok(html) => {
            let http_resp = fs::read_to_string("http/ok-html").unwrap();
            format!("{}\r\n{}", http_resp, html)
        }
        Err(..) => fs::read_to_string("http/not-found").unwrap(),
    };

    stream.write(response_data.as_bytes()).unwrap();
    stream.flush().unwrap();
}
