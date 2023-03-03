use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // initialize connection
    stream.write_all(b"Welcome to the Telnet server!\n").unwrap();
    let mut buffer = [0; 512];

    // loop to receive and process commands
    loop {
        // read incoming data
        let bytes_read = stream.read(&mut buffer).unwrap();
        let command = String::from_utf8_lossy(&buffer[..bytes_read]);

        // process the command
        if command.trim() == "quit" {
            stream.write_all(b"Goodbye!\n").unwrap();
            break;
        } else {
            // echo the command back to the client
            stream.write_all(command.as_bytes()).unwrap();
            stream.write_all(b"\n").unwrap();
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // loop to handle incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // handle the connection in a new thread
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    Ok(())
}
