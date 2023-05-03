pub mod bind;
pub mod delete;
pub mod search;

use std::collections::HashMap;
use std::io::{stdin, stdout, Read, Write};
use std::net::TcpStream;

use crate::bind::BindRequest;
use crate::search::SearchRequest;

fn main() {
    let mut stream = TcpStream::connect("34.243.143.185:389").unwrap();

    let bind = BindRequest::new("cn=admin,dc=example,dc=com", "Rich0725");

    stream.write(&bind.bind(0)).unwrap();

    let mut buf = [0; 4096];
    let response = match stream.read(&mut buf) {
        Ok(read) => read,
        Err(e) => panic!("{}", e),
    };

    println!(
        "{}Welcome to {}Richard & Thabos LDAP asset server{}",
        "\x1b[1m", "\x1b[25m", "\x1b[0m"
    );
    println!("Type {}{}S{} to search.", "\x1b[1m", "\x1b[32m", "\x1b[0m");
    println!("Type {}{}D{} to delete.", "\x1b[1m", "\x1b[33m", "\x1b[0m");
    println!("Type {}{}Q{} to quit.", "\x1b[1m", "\x1b[31m", "\x1b[0m");
    println!();

    let search = SearchRequest::new("ou=planes,dc=example,dc=com");
    let mut flags = (false, false);

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let message_id = 3;
        let base_dn = "ou=planes,dc=example,dc=com";

        match flags {
            (true, _) => {
                println!("SEARCH -> {}", input);
                let search_req = search.search(2, input.to_owned());
                // println!("SEARCH \n {:x?}", search_req);
                let out = req(&mut stream, search_req);
                // println!("{:x?}", out);
                decode(out);
                // println!("HI");
                flags.0 = false;
            }
            (_, true) => {
                println!("DELETE -> {}", input);
                // place delete logic here
                flags.1 = false;
            }
            (false, false) => {}
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

        stdout().flush().unwrap();
    }
}

fn req(stream: &mut TcpStream, to_send: Vec<u8>) -> Vec<u8> {
    println!("{:x?}", to_send.clone());
    stream.write(&to_send).unwrap();
    let mut buf = [0; 4096];
    stream.flush().unwrap();
    let response = match stream.read(&mut buf) {
        Ok(read) => read,
        Err(e) => panic!("{}", e),
    };

    buf[..response].to_vec()
}

fn decode(bytes: Vec<u8>) {
    let mut index_of_64: usize;
    let mut count: usize = 0;

    while count < bytes.len() {
        let byte_val = bytes[count];
        if byte_val == 0x64 {
            index_of_64 = count;
            let entry_len = bytes[index_of_64 + 1];
            let mut curr_pos = index_of_64 + 2;

            while curr_pos < (index_of_64 + entry_len as usize) {
                if bytes[curr_pos] == 0x04 {
                    let attr_len = bytes[curr_pos + 1];

                    let from = curr_pos + 2;
                    let to = from + attr_len as usize;
                    println!("{:?}", String::from_utf8(bytes[from..to].to_vec()));
                    curr_pos = to.clone();
                    continue;
                }
                curr_pos += 1
            }
            count = curr_pos;
        } else {
            count += 1;
        }
    }
}
