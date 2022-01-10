use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn connect() {
    let listener = TcpListener::bind("127.0.0.1:5141");
    if let Ok(listener_ok) = listener {
        for stream in listener_ok.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream);
        }
    } else {
        println!("Error bind!")
    }
}
