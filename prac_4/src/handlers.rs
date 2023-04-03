use std::{
    borrow::Cow,
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
    sync::Mutex,
};

use redis::{Client};
use url::form_urlencoded;

use crate::{
    methods::{add_appointment, content, four0four, home, login, logout, signup},
};

pub fn handle_client(mut stream: TcpStream) {
    let redis_client = Client::open("redis://127.0.0.1/").unwrap();
    let mut redis_mutex = Mutex::new(redis_client);

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);

    let response = handle_request(request, &mut redis_mutex);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_request(request: Cow<str>, redis_mutex: &mut Mutex<Client>) -> String {
    let (header, body) = request.trim().split_once("\r\n\r\n").unwrap();

    let mut parts = header.trim().split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    println!("{} - {}\n ------------ \n{}", method, path, body.clone());

    let cookie_header = header.lines().find(|line| line.starts_with("Cookie:"));

    let username = if let Some(cookie_header) = cookie_header {
        Some(cookie_header.trim_start_matches("Cookie:").trim())
    } else {
        None
    };

    match (method, path) {
        ("GET", "/") => {
            if let Some(username) = username {
                let form: HashMap<String, String> =
                    parse_body(username.trim_end_matches(char::from(0)));
                login(form.get("name").unwrap(), redis_mutex)
            } else {
                home()
            }
        }
        ("GET", "/style.css") => content("css"),
        ("GET", "/script.js") => content("js"),
        ("POST", "/login") => {
            let form: HashMap<String, String> = parse_body(body.trim_end_matches(char::from(0)));
            login(form.get("login").unwrap(), redis_mutex)
        }
        ("POST", "/signup") => {
            let form: HashMap<String, String> = parse_body(body.trim_end_matches(char::from(0)));
            signup(form.get("signup").unwrap(), redis_mutex)
        }
        ("POST", "/add_appointment") => {
            let form: HashMap<String, String> = parse_body(body.trim_end_matches(char::from(0)));
            if let Some(username) =
                parse_body(username.unwrap().trim_end_matches(char::from(0))).get("name")
            {
                add_appointment(username, redis_mutex, form)
            } else {
                todo!()
            }
        }
        ("GET", "/logout?") => logout(),

        _ => four0four(),
    }
}



fn parse_body(body: &str) -> HashMap<String, String> {
    form_urlencoded::parse(body.as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>()
}
