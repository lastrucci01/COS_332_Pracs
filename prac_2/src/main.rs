mod io_enum;
mod user_struct;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use std::{fs, thread};

use crate::io_enum::IOEnum;
use crate::user_struct::User;

static PORT_NO: &str = "6969";

fn main() {
    let listener = TcpListener::bind(String::from(":::") + PORT_NO).unwrap();
    println!("Server running on port {}...", PORT_NO);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_incoming_client(stream));
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

        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        match stream.read(&mut buffer) {
            Ok(size) => {
                let name = String::from_utf8_lossy(&buffer[..size]).trim().to_owned();
                let user = User::new(name);
                let msg = IOEnum::Greeting {
                    name: (user.name()),
                }
                .output();
                stream.write_all(msg.as_bytes()).unwrap();
                user
            }
            Err(e) => {
                println!("Error - {}", e);
                panic!()
            }
        }
    };

    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                let message = String::from_utf8_lossy(&buffer[..size]).trim().to_owned();
                println!("Received message: {}", message);
                stream.write_all(&buffer[..size]).unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}

pub fn is_saved(client_addr: &String) -> bool {
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
