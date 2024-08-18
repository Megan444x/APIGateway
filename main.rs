use std::net::TcpListener;
use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "7878".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Failed to bind port");

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).expect("Failed to read from stream");

    let response = if request_line.contains("/health") {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            "OK".len(),
            "OK"
        )
    } else {
        format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
            "Not Found".len(),
            "Not Found"
        )
    };

    stream.write_all(response.as_bytes()).expect("Failed to write to stream");
    stream.flush().expect("Failed to flush");
}