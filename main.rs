use std::net::{TcpListener, TcpStream};
use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "7878".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Failed to bind port");

    println!("Listening on port {}", port);
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");
        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).expect("Failed to read from stream");
    
    let request_vec: Vec<&str> = request_line.split_whitespace().collect();

    if request_vec.len() < 2 {
        // Not a valid request
        return;
    }

    let method = request_vec[0];
    let path = request_vec[1];

    match method {
        "GET" => match path {
            "/health" => send_response(stream, 200, "OK".to_string()),
            _ => {
                if path.starts_with("/static/") {
                    let path = format!(".{}", path); // Convert to relative path
                    serve_static_file(stream, &path);
                } else {
                    send_response(stream, 404, "Not Found".to_string());
                }               
            }
        },
        _ => send_response(stream, 405, "Method Not Allowed".to_string()),
    }
}

fn send_response(mut stream: TcpStream, status_code: u16, body: String) {
    let status_line = match status_code {
        200 => "HTTP/1.1 200 OK\r\n",
        404 => "HTTP/1.1 404 Not Found\r\n",
        405 => "HTTP/1.1 405 Method Not Allowed\r\n",
        _ => "HTTP/1.1 500 Internal Server Error\r\n",
    };

    let response = format!(
        "{}Content-Length: {}\r\n\r\n{}",
        status_line,
        body.len(),
        body
    );

    stream.write_all(response.as_bytes()).expect("Failed to write to stream");
    stream.flush().expect("Failed to flush");
}

fn serve_static_file(mut stream: TcpStream, path: &str) {
    match File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            send_response(stream, 200, contents);
        },
        Err(_) => send_response(stream, 404, "Not Found".to_string()),
    }
}