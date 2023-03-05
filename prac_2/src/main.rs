mod helpers;
mod user_struct;

use std::net::TcpListener;

use std::{fs, thread};

use crate::helpers::handle_incoming_client;

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
