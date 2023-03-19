use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
};

use regex::Regex;

use crate::user_struct::User;

pub fn handle_incoming_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    let msg = format!(
        "{}Welcome to the telephone book!{}\nType your username to log in or simply press enter to create an account :)\r\n{}",
        "\x1B[1m\x1B[4m", // bold and underlined
        "\x1B[0m", // reset
        "\x1B[32m", // green
    );
    stream.write_all(msg.as_bytes()).unwrap();
    let input = match stream.read(&mut buffer) {
        Ok(size) => String::from_utf8_lossy(&buffer[0..size]).trim().to_string(),
        Err(_) => panic!(),
    };

    let mut user = if !input.is_empty() {
        let user = User::new_from_file(&input);
        let msg = format!(
            "{}{}Welcome back, {}!{}\n{}Enter `help` to find out available commands :){}\r\n",
            "\x1B[0m", // reset
            "\x1B[1m", // bold
            user.name(),
            "\x1B[0m",  // reset
            "\x1B[32m", // green
            "\x1B[0m",  // reset
        );
        stream.write_all(msg.as_bytes()).unwrap();

        user
    } else {
        let msg = format!(
            "{}{}Aha, a new user!{}\nMight I have a name to call you? -> ",
            "\x1B[0m",  // reset
            "\x1B[33m", // yellow
            "\x1B[0m",  // reset
        );
        stream.write_all(msg.as_bytes()).unwrap();
        let username = loop {
            let possible_username = match stream.read(&mut buffer) {
                Ok(size) => String::from_utf8_lossy(&buffer[0..size]).trim().to_string(),
                Err(_) => panic!(),
            };
            if !search_users(&possible_username) {
                break possible_username;
            } else {
                let msg = format!("That username is taken, perhaps another one? ->",);
                stream.write_all(msg.as_bytes()).unwrap();
            }
        };

        let user = User::new(username);

        let msg = format!(
                "{}{}Welcome to your telephone book, {}!{}\n{}Enter `help` to find out available commands :){}\r\n",
                "\x1B[0m", // reset
                "\x1B[1m", // bold
                user.name(),
                "\x1B[0m",  // reset
                "\x1B[32m", // green
                "\x1B[0m",  // reset
            );
        stream.write_all(msg.as_bytes()).unwrap();

        user
    };

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(size) => {
                let message = String::from_utf8_lossy(&buffer[..size]).trim().to_owned();
                let commands = tokenise(&message);

                if commands.len() > 0 {
                    let response: String = match commands[0] {
                        "add" => {
                            if commands.len() != 3 {
                                format!("Two args (name, number) required for `add`\r\n")
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
                        "update" => {
                            if commands.len() != 3 {
                                format!("Two arguments (name, number) required for `update`\r\n")
                            } else if is_not_name(commands[1]) {
                                format!("Incorrect format for `name` arg \r\n")
                            } else {
                                user.update_contact(commands[1], commands[2])
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
                                format!("Incorrect format for `name` arg\r\n",)
                            } else {
                                user.remove_contact(commands[1])
                            }
                        }
                        "list" => user.list(),
                        "help" => {
                            format!(
                                "{}available commands:{}\r\n{}add <name> <number>{} -> add a contact to your telephone book\r\n{}update <name> <number>{} -> update a contact in your telephone book\r\n{}search <name>{} -> search for a contact in your telephone\r\n{}remove <name>{} -> remove a contact from your telephone book\r\n{}list{} -> list telephone book!\r\n{}help{} -> this command!\r\n{}quit{} -> exit the address book application\r\n",
                                "\x1B[1m", // bold
                                "\x1B[0m", // reset

                                "\x1B[32m", // green
                                "\x1B[0m", // reset

                                "\x1B[32m", // green
                                "\x1B[0m", // reset

                                "\x1B[33m", // yellow
                                "\x1B[0m", // reset

                                "\x1B[34m", // blue
                                "\x1B[0m", // reset

                                "\x1B[35m", // magenta
                                "\x1B[0m", // reset

                                "\x1B[36m", // cyan
                                "\x1B[0m", // reset

                                "\x1B[31m", // red
                                "\x1B[0m", // reset
                            )
                        }
                        "quit" => {
                            user.save_to_file();
                            let response = format!(
                                "Thanks for using the telephone book!\r\nSee you soon!\r\n"
                            );
                            stream.write(response.as_bytes()).unwrap();
                            break;
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

fn search_users(name: &str) -> bool {
    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.ok().unwrap();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        if file_name_str.contains(name) {
            return true;
        }
    }
    false
}
