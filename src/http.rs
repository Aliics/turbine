pub mod tcp {
    use std::{
        net::{TcpListener, TcpStream},
        thread,
        thread::JoinHandle,
    };

    pub fn start_server(address: String) -> TcpListener {
        TcpListener::bind(address).unwrap()
    }

    pub fn listen(tcp: TcpListener) -> JoinHandle<()> {
        thread::spawn(move || {
            for stream in tcp.incoming() {
                thread::spawn(|| handle_stream(stream.unwrap()));
            }
        })
    }

    fn handle_stream(stream: TcpStream) {}
}
