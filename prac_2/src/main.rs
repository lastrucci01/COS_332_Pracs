use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    println!("Server running on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Client connected!");

                let welcome_message = "Welcome to the Rust telnet server!\r\n";
                stream.write(welcome_message.as_bytes()).unwrap();

                loop {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(size) => {
                            let message = String::from_utf8_lossy(&buffer[..size]);
                            println!("Received message: {}", message);

                            let response = format!("You said: {}\r\n", message.trim());
                            stream.write(response.as_bytes()).unwrap();
                        }
                        Err(e) => {
                            println!("Failed to read from socket: {}", e);
                            break;
                        }
                    }
                }

                println!("Client disconnected!");
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}
