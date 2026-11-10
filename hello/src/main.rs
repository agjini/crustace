use crate::header::{Header, Status, write_headers, write_status};
use std::fs::File;
use std::{
    io,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

mod header;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let Some(header) = http_request.get(0) else {
        return;
    };

    if header == "GET / HTTP/1.1" {
        write_response(stream, "hello.html", Status::Ok);
    } else {
        write_response(stream, "404.html", Status::NotFound);
    }
}

fn write_response(mut stream: TcpStream, file: &str, status: Status) {
    let mut file = File::open(format!("src/{file}")).unwrap();

    write_status(&mut stream, status);

    let headers = vec![
        Header::new("Content-Type", "text/html"),
        Header::new(
            "Content-Length",
            &file.metadata().unwrap().len().to_string(),
        ),
    ];
    write_headers(&mut stream, headers);

    io::copy(&mut file, &mut stream).unwrap();
}
