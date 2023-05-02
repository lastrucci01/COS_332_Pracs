pub mod bind;
pub mod search;

use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

fn main() {
    // let mut stream = TcpStream::connect("54.78.173.174:389").unwrap();

    // let req = BindRequest::new("cn=admin,dc=example,dc=com", "Rich0725");

    // stream.write(&req.bind(0)).unwrap();

    // let mut buf = [0; 4096];

    // let response = match stream.read(&mut buf) {
    //     Ok(read) => read,
    //     Err(e) => panic!("{}", e),
    // };

    // dbg!(response);
    println!(
        "{}Welcome to {}Richard & Thabos LDAP asset server{}",
        "\x1b[1m", "\x1b[25m", "\x1b[0m"
    );

    println!("Type {}{}S{} to search.", "\x1b[1m", "\x1b[32m", "\x1b[0m");
    println!("Type {}{}D{} to delete.", "\x1b[1m", "\x1b[33m", "\x1b[0m");
    println!("Type {}{}Q{} to quit.", "\x1b[1m", "\x1b[31m", "\x1b[0m");
    println!();

    let mut flags = (false, false);

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match flags {
            (true, _) => {
                println!("SEARCH -> {}", input);
                // place search logic here
                flags.0 = false;
            }
            (_, true) => {
                println!("DELETE -> {}", input);
                // place delete logic here
                flags.1 = false;
            }
            (false, false) => {}
            _ => panic!(),
        }

        if input.trim() == "Q" {
            break;
        } else if input.trim() == "S" {
            print!(
                "{}Search{} for an {}aeroplane{}: ",
                "\x1b[32m", "\x1b[0m", "\x1b[1m", "\x1b[0m"
            );
            flags.0 = true;
        } else if input.trim() == "D" {
            print!(
                "{}Delete{} an {}aeroplane{}: ",
                "\x1b[33m", "\x1b[0m", "\x1b[1m", "\x1b[0m"
            );
            flags.1 = true;
        }
        let base_dn = "ou=planes,dc=example,dc=com";

        let message_id = 3;

        // stream.write(&search_request).unwrap();

        // let response = match stream.read(&mut buf) {
        //     Ok(read) => read,
        //     Err(e) => panic!("{}", e),
        // };

        // let res = buf[..response].to_vec();
        // dbg!(res);

        stdout().flush().unwrap();
    }
}

fn decode(bytes: Vec<u8>) -> HashMap<String, String> {
    todo!()
}
