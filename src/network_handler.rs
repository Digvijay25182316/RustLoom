use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::collections::HashMap;
use std::time::Instant;
use crate::router::handle_route;

#[derive(Debug, Clone)]
pub enum RequestType {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    UNKNOWN,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub method: RequestType,
    pub path: String,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: u16,
    pub content_type: String,
    pub body: String,
}

impl Response {
    pub fn new(status_code: u16, content_type: &str, body: &str) -> Self {
        Response {
            status_code,
            content_type: content_type.to_string(),
            body: body.to_string(),
        }
    }

    pub fn to_http(&self) -> String {
        format!(
            "HTTP/1.1 {} OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.content_type,
            self.body.len(),
            self.body
        )
    }
}

pub fn network_handler(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let start_total = Instant::now();

    let mut buffer = [0; 4096];
    let bytes_read = stream.read(&mut buffer)?;
    let start_parse = Instant::now();

    let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request = parse_request(&request_str);
    let parse_duration = start_parse.elapsed();

    let start_route = Instant::now();
    let response = handle_route(request);
    let route_duration = start_route.elapsed();

    let start_write = Instant::now();
    stream.write_all(response.to_http().as_bytes())?;
    stream.flush()?;
    let write_duration = start_write.elapsed();

    let total_duration = start_total.elapsed();
    println!(
        "⏱ Timing (parse: {:?}, route: {:?}, write: {:?}, total: {:?})",
        parse_duration, route_duration, write_duration, total_duration
    );

    Ok(())
}


fn parse_request(raw: &str) -> Request {
    let mut lines = raw.lines();
    let request_line = lines.next().unwrap_or("");
    let mut parts = request_line.split_whitespace();

    let method_str = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/").to_string();

    let method = match method_str {
        "GET" => RequestType::GET,
        "POST" => RequestType::POST,
        "PUT" => RequestType::PUT,
        "PATCH" => RequestType::PATCH,
        "DELETE" => RequestType::DELETE,
        "OPTIONS" => RequestType::OPTIONS,
        _ => RequestType::UNKNOWN,
    };

    let mut headers = HashMap::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(": ") {
            headers.insert(k.to_string(), v.to_string());
        }
    }

    let body = lines.collect::<Vec<&str>>().join("\n");

    Request {
        method,
        path,
        body,
    }
}
