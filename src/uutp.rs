use std::io::{Read, Write};
use std::net::TcpStream;

use crate::models::error::Error;
use crate::models::http_method::HttpMethod;
use crate::models::url::Url;
use crate::utils::http_request::HttpRequest;

pub struct Uutp {}

impl Uutp {
    pub fn get(url: &str) -> Result<String, Error> {
        let url = Url::parse(url).unwrap();

        let request = HttpRequest {
            method: HttpMethod::GET,
            host: url.host.clone(),
            path: url.path.clone(),
            conn: "close".to_string(),
        }
        .build();
        println!("{}", request);
        let mut stream = TcpStream::connect(format!("{}:80", url.host)).unwrap();
        stream.write_all(request.as_bytes()).unwrap();

        let mut buffer = [0; 2048];
        stream.read(&mut buffer).unwrap();
        let request_str = std::str::from_utf8(&buffer).unwrap();

        let lines: Vec<String> = request_str.lines().map(|line| line.to_string()).collect();

        let mut collect = false;
        let mut body = String::from("");
        for line in &lines {
            if collect {
                body.push_str(line);
            }
            if line.is_empty() {
                collect = true;
            }
        }
        body = body.trim_matches(char::from(0)).to_string();
        stream.shutdown(std::net::Shutdown::Both).unwrap();
        Ok(body)
    }
}
