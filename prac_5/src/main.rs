pub mod bind;
pub mod delete;
pub mod search;

use rustyline::{DefaultEditor, Editor};
use std::collections::HashMap;
use std::io::{stdin, stdout, BufRead, BufReader, Read, Write};
use std::net::TcpStream;

use crate::bind::BindRequest;
use crate::delete::DeleteRequest;
use crate::search::SearchRequest;

fn main() {
    let mut stream = TcpStream::connect("34.243.143.185:389").unwrap();
    let mut rl = DefaultEditor::new().unwrap();

    let bind = BindRequest::new("cn=admin,dc=example,dc=com", "Rich0725");
    stream.write(&bind.bind(0)).unwrap();
    let mut buf = [0; 4096];
    match stream.read(&mut buf) {
        Ok(read) => read,
        Err(e) => panic!("{}", e),
    };

    println!("Bind: {}Success!{}", "\x1b[32m", "\x1b[0m");
    print_intro();

    let search = SearchRequest::new("ou=planes,dc=example,dc=com");
    let delete = DeleteRequest::new("ou=planes,dc=example,dc=com");

    let mut msg_id = 1;
    loop {
        let opp = rl.readline("").unwrap();
        match opp.as_str() {
            "Q" => {
                stream.write(&bind.unbind(msg_id)).unwrap();
                println!("Unbind: {}Success!{}", "\x1b[32m", "\x1b[0m");
            }
            "S" => {
                let name = rl
                    .readline(&format!(
                        "{}Search{} for an {}aeroplane{}: ",
                        "\x1b[32m", "\x1b[0m", "\x1b[1m", "\x1b[0m"
                    ))
                    .unwrap();

                msg_id += 1;
                stream.write(&search.search(msg_id, name.clone())).unwrap();
                let mut buf = [0; 4096];
                let resp = match stream.read(&mut buf) {
                    Ok(read) => read,
                    Err(e) => panic!("{}", e),
                };

                let results = SearchRequest::<'_>::decode(buf[..resp].to_vec());

                if results.is_empty() {
                    println!(
                        "{}No asset found{} with name: {}{}{}\n",
                        "\x1b[31m", "\x1b[0m", "\x1b[1m", name, "\x1b[0m"
                    );
                } else {
                    println!(
                        "{}Asset found{} with name: {}{}{}",
                        "\x1b[32m",
                        "\x1b[0m",
                        "\x1b[1m",
                        results.get("name").unwrap(),
                        "\x1b[0m"
                    );
                    println!(
                        "The speed of {}{}{} is: {}{}{}\n",
                        "\x1b[1m",
                        results.get("name").unwrap(),
                        "\x1b[0m",
                        "\x1b[1m",
                        results.get("speed").unwrap(),
                        "\x1b[0m"
                    );
                }
            }
            "D" => {
                let name = rl
                    .readline(&format!(
                        "{}Delete{} an {}aeroplane{}: ",
                        "\x1b[33m", "\x1b[0m", "\x1b[1m", "\x1b[0m"
                    ))
                    .unwrap();

                stream.write(&delete.delete(msg_id, name)).unwrap();

                let mut buf = [0; 4096];
                let resp = match stream.read(&mut buf) {
                    Ok(read) => read,
                    Err(e) => panic!("{}", e),
                };
                if let Some(()) = delete.decode(buf[..resp].to_vec()) {
                    println!("{}Successful!{}", "\x1b[32m", "\x1b[0m\n");
                } else {
                    println!(
                        "{}Unsuccessful{}... No object found!\n",
                        "\x1b[31m", "\x1b[0m"
                    );
                }
            }
            _ => println!("Unrecognised command... Try again!"),
        };
    }
}

fn print_intro() {
    println!();
    println!(
        "{}Welcome to {}Richard & Thabos LDAP asset server{}",
        "\x1b[1m", "\x1b[25m", "\x1b[0m"
    );

    println!("Type {}{}S{} to search.", "\x1b[1m", "\x1b[32m", "\x1b[0m");
    println!("Type {}{}D{} to delete.", "\x1b[1m", "\x1b[33m", "\x1b[0m");
    println!("Type {}{}Q{} to quit.", "\x1b[1m", "\x1b[31m", "\x1b[0m");
    println!();
}
