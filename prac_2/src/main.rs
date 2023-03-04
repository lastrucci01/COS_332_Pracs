mod io_enum;
mod user_struct;


use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{thread, fs};

use crate::io_enum::IOEnum;
use crate::user_struct::User;

static PORT_NO: &str = "6969";

fn main() {
    let listener = TcpListener::bind(String::from(":::") + PORT_NO).unwrap();
    println!("Server running on port {}...", PORT_NO);


    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {                    
                   handle_incoming_client(stream)
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

pub fn handle_incoming_client(mut stream: TcpStream) {
    let client_addr = stream.peer_addr().unwrap().to_string();
    println!("Client {} connected!", &client_addr);

    let user = if is_saved(&client_addr) {
        let user = User::new_from_file(&client_addr);
        let msg = IOEnum::Greeting {
            name: (user.name()),
        }
        .output();
        stream.write_all(msg.as_bytes()).unwrap();
        user
    } else {
        let msg = IOEnum::NewUser.output();
        stream.write_all(msg.as_bytes()).unwrap();
        let mut name = String::new();
        stream.read_to_string(&mut name).unwrap();
        let user = User::new_from_file(&client_addr);
        let msg = IOEnum::Greeting {
            name: (user.name()),
        }
        .output();
        stream.write_all(msg.as_bytes()).unwrap();
        user
    };

    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received message: {}", message);
                stream.write_all(&buffer[..n]).unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}

pub fn is_saved( client_addr: &String) -> bool {
    let mut base_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    base_path.push("users");
    let user_paths = fs::read_dir(base_path.to_str().clone().unwrap()).unwrap();

    for path in user_paths {
        if let Ok(dir) = path {
            let file_name = dir.file_name().to_str().unwrap().to_string();

            println!("{:?}", file_name);
            return file_name == format!("{}.txt", client_addr);
        }
    }
    false
}