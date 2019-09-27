#[allow(dead_code)]
pub mod tcp {
    use std::{
        io::Write,
        net::{TcpListener, TcpStream},
        thread,
        thread::JoinHandle,
    };
    use crate::conf::Config;

    pub fn start_server(config: Config) -> TcpListener {
        TcpListener::bind(config.server.address).unwrap()
    }

    pub fn listen(tcp: TcpListener, config: Config) -> JoinHandle<()> {
        thread::spawn(move || {
            for stream in tcp.incoming() {
                let cloned_config = config.clone();
                thread::spawn(move || handle_stream(&mut stream.unwrap(), cloned_config));
            }
        })
    }

    fn handle_stream(stream: &mut TcpStream, config: Config) {
        stream.flush().unwrap();
    }

    fn parse_status_line(request: String) -> (String, String, String) {
        let status_line: &str = request.split('\n').collect::<Vec<&str>>()[0];
        let pieces: Vec<&str> = status_line.split(' ').collect();
        (
            pieces[0].to_string(),
            pieces[1].to_string(),
            pieces[2].to_string(),
        )
    }

    fn get_path_file_ext(path: String) -> String {
        let dropped_params = match path.find("?") {
            Some(i) => path.split_at(i).0.to_string(),
            None => path,
        };
        let split_path: Vec<&str> = dropped_params.split(".").collect();
        let last_index = split_path.len() - 1;
        if last_index > 0 {
            split_path[last_index].to_string()
        } else {
            String::new()
        }
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

        #[test]
        fn getting_path_file_ext() {
            let path = "index.html".to_string();

            let actual_ext = get_path_file_ext(path);

            assert_eq!(actual_ext, "html");
        }

        #[test]
        fn getting_complex_path_file_ext() {
            let path = "main.css?foo=bar".to_string();

            let actual_ext = get_path_file_ext(path);

            assert_eq!(actual_ext, "css");
        }

        #[test]
        fn getting_blank_file_ext() {
            let path = "home".to_string();

            let actual_ext = get_path_file_ext(path);

            assert_eq!(actual_ext, String::new());
        }
    }
}
