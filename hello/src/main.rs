use std::fs::File;
use std::{
    io,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let mut file = File::open("src/hello.html").unwrap();
    let mut file2 = File::open("src/goodbye.html").unwrap();
    let response = "HTTP/1.1 200 OK\r\n";
    stream.write_all(response.as_bytes()).unwrap();
    stream
        .write_all("Content-Type: text/html\r\n".as_bytes())
        .unwrap();
    io::copy(&mut file, &mut stream).unwrap();
    io::copy(&mut file2, &mut stream).unwrap();
}
