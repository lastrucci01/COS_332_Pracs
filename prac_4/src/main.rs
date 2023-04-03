mod file;
mod handlers;
mod user;
mod headers;
mod methods;

use std::{net::TcpListener, thread};

use crate::handlers::handle_client;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:6969").unwrap();
    let address = listener.local_addr().expect("Failed to get address");
    println!("Server listening on http://{}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
