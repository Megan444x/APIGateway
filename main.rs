use std::net::{TcpListener, TcpStream};
use std::env;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::fs::File;
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
    if let Ok(request_line) = read_request_line(&stream) {
        let request_vec: Vec<&str> = request_line.split_whitespace().collect();
        process_request(stream, &request_vec);
    }
}

fn read_request_line(stream: &TcpStream) -> io::Result<String> {
    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    Ok(request_line)
}

fn process_request(stream: TcpStream, request_vec: &[&str]) {
    if request_vec.len() < 2 {
        // Not a valid request
        return;
    }

    let (method, path) = (request_vec[0], request_vec[1]);
    route_request(method, path, stream);
}

fn route_request(method: &str, path: &str, stream: TcpStream) {
    match method {
        "GET" => get_request_handler(path, stream),
        _ => method_not_allowed(stream),
    }
}

fn get_request_handler(path: &str, stream: TcpStream) {
    match path {
        "/health" => send_response(stream, 200, "OK"),
        _ => {
            if path.startsWith("/static/") {
                let path = format!(".{}", path); // Convert to relative path
                serve_static_file(stream, &path);
            } else {
                not_found(stream);
            }               
        }
    }
}

fn not_found(stream: TcpStream) {
    send_response(stream, 404, "Not Found");
}

fn method_not_allowed(stream: TcpStream) {
    send_response(stream, 405, "Method Not Allowed");
}

fn send_response(mut stream: TcpStream, status_code: u16, body: &str) {
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

fn serve_static_file(stream: TcpStream, path: &str) {
    let content = match File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                Some(contents)
            } else {
                None
            }
        },
        Err(_) => None,
    };
    
    match content {
        Some(contents) => send_response(stream, 200, &contents),
        None => not_found(stream),
    }
}