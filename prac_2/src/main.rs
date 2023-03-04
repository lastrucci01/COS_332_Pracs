mod client_handler_struct;
mod io_enum;
mod user_struct;


use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::client_handler_struct::ClientHandlerStruct;

fn handle_client(client_addr: String) {
    println!("Client {} connected!", client_addr);

    // let mut buffer = [0; 1024];
    // loop {
    //     match stream.read(&mut buffer) {
    //         Ok(size) => {
    //             let message = String::from_utf8_lossy(&buffer[..size]).trim().to_owned();
    //             println!("Received message from client {}: {}", client, message);

    //             if message == "quit" {
    //                 println!("Client {} disconnected.", client);
    //                 clients.remove(&client);
    //                 break;
    //             }

    //             let response = format!("You said: {}\r\n", message);
    //             stream.write(response.as_bytes()).unwrap();
    //         }
    //         Err(e) => {
    //             println!("Failed to read from socket: {}", e);
    //             clients.remove(&client);
    //             break;
    //         }
    //     }
    //}
}

static PORT_NO: &str = "6969";

fn main() {
    let listener = TcpListener::bind(String::from(":::") + PORT_NO).unwrap();
    println!("Server running on port {}...", PORT_NO);

    let mut data = Arc::new(Mutex::new(ClientHandlerStruct::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let shared_data = Arc::clone(&data);
                thread::spawn(move || {                    
                   shared_data.lock().unwrap().handle_incoming_client(stream)
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}
