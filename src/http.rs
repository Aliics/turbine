#[allow(dead_code)]
pub mod tcp {
    use std::{
        io::Write,
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
                thread::spawn(|| handle_stream(&mut stream.unwrap()));
            }
        })
    }

    fn handle_stream(stream: &mut TcpStream) {
        stream.flush().unwrap();
    }

    fn parse_status_line(request: String) -> (String, String, String) {
        let status_line: &str = request.split('\n').collect::<Vec<&str>>()[0];
        let pieces: Vec<&str> = status_line.split(' ').collect();
        (pieces[0].to_string(), pieces[1].to_string(), pieces[2].to_string())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn parse_request_status_line() {
            let request = fs::read_to_string("tests/example_request").unwrap();
            let (method, path, version) = parse_status_line(request);

            assert_eq!(method, "GET");
            assert_eq!(path, "/index.html");
            assert_eq!(version, "HTTP/1.1");
        }
    }
}
