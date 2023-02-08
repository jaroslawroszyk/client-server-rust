use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn handle_client(mut stream: std::net::TcpStream) {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            return;
        }
        let msg = String::from_utf8_lossy(&buf[0..bytes_read]);
        println!("Recived msg: {}", msg);
        let response = format!("response: {}",msg);
        let _ = stream.write(response.as_bytes());
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on 127.0.0.1:8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted connection from {:?}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
