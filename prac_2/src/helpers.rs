use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
};

use regex::Regex;

use crate::user_struct::User;

pub fn handle_incoming_client(mut stream: TcpStream) {
    let client_addr = stream.peer_addr().unwrap().to_string();
    println!("Client {} connected!", &client_addr);

    let mut buffer = [0; 512];

    let mut user = if is_saved(&client_addr) {
        let user = User::new_from_file(&client_addr);
        let msg = format!(
            "Welcome to your address book, {}!\nEnter `help` to find out available commands :)\r\n",
            user.name()
        );
        stream.write_all(msg.as_bytes()).unwrap();

        user
    } else {
        let msg = format!("Aha, a new user!\nMight I have a name to call you? -> ");
        stream.write_all(msg.as_bytes()).unwrap();
        let name = match stream.read(&mut buffer) {
            Ok(size) => String::from_utf8_lossy(&buffer[0..size]).trim().to_string(),
            Err(_) => panic!(),
        };

        let user = User::new(name, client_addr);

        let msg = format!(
            "Welcome to your address book, {}!\nEnter `help` to find out available commands :)\r\n",
            user.name()
        );
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

                if commands.len() > 0 {
                    let response: String = match commands[0] {
                        "add" => {
                            if commands.len() != 3 {
                                format!("Two arguments (name, number) required for `add`\r\n")
                            } else if is_not_name(commands[1]) {
                                format!("Incorrect format for `name` arg \r\n")
                            } else if is_not_number(commands[2]) {
                                format!("Incorrect format for `number` arg\r\n")
                            } else {
                                user.add_contact(commands[1], commands[2]);
                                format!(
                                    "{} successfully added to your telephone book!\r\n",
                                    commands[1]
                                )
                            }
                        }
                        "search" => {
                            if commands.len() != 2 {
                                format!("One argument (name) required for `search`\r\n")
                            } else if is_not_name(commands[1]) {
                                format!("Incorrect format for `name` arg \r\n")
                            } else {
                                user.search_contact(commands[1])
                            }
                        }
                        "remove" => {
                            if commands.len() != 2 {
                                format!("One argument (name) required for `search`\r\n")
                            } else if is_not_name(commands[1]) {
                                format!("Incorrect format for `name` arg \r\n")
                            } else {
                                user.remove_contact(commands[1])
                            }
                        }
                        "help" => {
                            format!(
                                r#"available commands:\r\nadd <name> <number> -> add a contact to your telephone book\r\nsearch <name> -> search for a contact in your telephone\r\nremove <name> -> remove a contact from your telephone book\r\nhelp -> this command!\r\nquit -> exit the address book application\r\n"#
                            )
                        }
                        "quit" => {
                            user.save_to_file();
                            let response = format!("Thanks for using the telephone book!\r\nSee you soon!\r\n");
                            stream.write(response.as_bytes()).unwrap();
                            break
                        }
                        _ => {
                            format!("unrecognised command: {}\r\n", message)
                        }
                    };

                    stream.write(response.as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}

fn is_not_name(name: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z]+$").unwrap();
    !re.is_match(name)
}

fn is_not_number(number: &str) -> bool {
    let re = Regex::new(r"^[0-9]+$").unwrap();
    !re.is_match(number)
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
