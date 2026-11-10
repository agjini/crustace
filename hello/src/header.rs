use std::fmt::{Display, Formatter};
use std::io::Write;
use std::net::TcpStream;

pub enum Status {
    Ok,
    NotFound,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Ok => {
                write!(f, "200 OK")
            }
            Status::NotFound => {
                write!(f, "404 NOT FOUND")
            }
        }
    }
}

pub struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

pub fn write_status(stream: &mut TcpStream, status: Status) {
    let status = format!("HTTP/1.1 {status}\r\n");
    stream.write_all(status.as_bytes()).unwrap();
}

pub fn write_headers(stream: &mut TcpStream, headers: Vec<Header>) {
    for header in headers {
        stream.write_all(header.name.as_bytes()).unwrap();
        stream.write_all(": ".as_bytes()).unwrap();
        stream.write_all(header.value.as_bytes()).unwrap();
        stream.write_all(b"\r\n").unwrap();
    }
    stream.write_all(b"\r\n").unwrap();
}
