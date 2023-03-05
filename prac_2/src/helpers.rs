use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
};

use crate::{error_enum::ErrorChecksEnum, io_enum::IOEnum, user_struct::User};

pub fn handle_incoming_client(mut stream: TcpStream) {
    let client_addr = stream.peer_addr().unwrap().to_string();
    println!("Client {} connected!", &client_addr);

    let mut buffer = [0; 512];

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
        let name = match stream.read(&mut buffer) {
            Ok(size) => String::from_utf8_lossy(&buffer[0..size]).trim().to_string(),
            Err(_) => panic!(),
        };

        let user = User::new(name);

        let msg = IOEnum::Greeting {
            name: (user.name()),
        }
        .output();
        stream.write_all(msg.as_bytes()).unwrap();

        user
    };

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(size) => {
                let message = String::from_utf8_lossy(&buffer[..size]).trim().to_owned();
                println!(
                    "Received message from client {}: {}",
                    stream.peer_addr().unwrap(),
                    message
                );

                let commands = tokenise(&message);

                for c in commands {
                    println!("{}", c);
                }

                let response = format!("You said: {}\r\n", message);
                stream.write(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}

fn tokenise(message: &str) -> Vec<&str> {
    let mut command: Vec<&str> = Vec::new();
    for word in message.split_whitespace() {
        command.push(word);
    }
    command
}

fn is_saved(client_addr: &String) -> bool {
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
