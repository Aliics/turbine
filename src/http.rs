mod response {
    pub const HTTP_OK: &str = "HTTP/1.1 200 OK";
    pub const HTTP_NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n\r\nNothing here! Oh noes!";
}

#[allow(dead_code)]
pub mod tcp {
    use crate::conf::Config;
    use crate::http::response::{HTTP_NOT_FOUND, HTTP_OK};
    use std::{
        fs,
        io::{Read, Write},
        net::{TcpListener, TcpStream},
        thread,
        thread::JoinHandle,
    };

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
        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap();

        let request_str = String::from_utf8_lossy(&mut buf).to_string();
        let status_line = parse_status_line(request_str);

        let mut directory_clone = config.web.directory.clone();
        directory_clone.push_str(status_line.1.as_str());

        let file_ext = get_path_file_ext(status_line.1);
        let ct_header = format!("Content-Type: text/{}", file_ext);

        let response = match fs::read_to_string(directory_clone) {
            Ok(data) => format!("{}\r\n{}\r\n\r\n{}", HTTP_OK, ct_header, data),
            Err(_) => HTTP_NOT_FOUND.to_string(),
        };

        stream.write(response.as_bytes()).unwrap();
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
