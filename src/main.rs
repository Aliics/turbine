use std::{
    format, fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

const SERVER_ADDR: &str = "localhost:8080";

fn main() {
    let tcp = TcpListener::bind(SERVER_ADDR).unwrap();

    for incoming in tcp.incoming() {
        thread::spawn(|| handle_request(incoming.unwrap()));
    }
}

fn handle_request(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();

    let response_data = parse_status_line(&mut buf);

    stream.write(response_data.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_status_line(buf: &mut [u8]) -> String {
    let buf_str = String::from_utf8_lossy(buf);
    let buf_split: Vec<&str> = buf_str.split(' ').collect();
    let page_path = format!("pages/{}", buf_split[1]);

    match fs::read_to_string(page_path) {
        Ok(html) => format_http_response("http/ok-html", html),
        Err(_) => format_http_response("http/not-found", String::new()),
    }
}

fn format_http_response(status_file: &str, html: String) -> String {
    let http_resp = fs::read_to_string(status_file).unwrap();
    format!("{}\r\n{}", http_resp, html)
}
