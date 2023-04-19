use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
    sync::Mutex,
};

use redis::Client;
use url::form_urlencoded;

use crate::{
    headers::get_username,
    methods::{
        add_appointment, content, delete_appointment, four0four, home, login, logout,
        search_appointments, signup_get, signup_post,
    },
};

pub fn handle_client(mut stream: TcpStream) {
    let redis_client = Client::open("redis://127.0.0.1/").unwrap();
    let mut redis_mutex = Mutex::new(redis_client);

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8((&buffer[..]).to_vec()).unwrap();

    let response = handle_request(request, &mut redis_mutex);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_request(request: String, redis_mutex: &mut Mutex<Client>) -> String {
    let (header, body) = request.trim().split_once("\r\n\r\n").unwrap();

    let mut parts = header.trim().split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    println!("{} - {}\n ------------ \n{}", method, path, body.clone());

    let cookie_header = header.lines().find(|line| line.starts_with("Cookie:"));
    dbg!(cookie_header);
    let username = if let Some(cookies) = cookie_header {
        let cookie_str = cookies.trim_start_matches("Cookie: ").trim();
        get_username(cookie_str)
    } else {
        None
    };
    match (method, path) {
        ("GET", "/") => {
            if let Some(username) = username {
                login(&username, redis_mutex)
            } else {
                home()
            }
        }
        ("GET", "/signup") => signup_get(),
        ("GET", "/style.css") => content("css"),
        ("GET", "/script.js") => content("js"),
        ("POST", "/signuo") => {
            let form: HashMap<String, String> = parse_body(body.trim_end_matches(char::from(0)));
            signup_post(form.get("signup").unwrap(), redis_mutex)
        }
        ("POST", "/") => {
            let form: HashMap<String, String> = parse_body(body.trim_end_matches(char::from(0)));

            let req_type = form.get("type").unwrap();

            match req_type.as_str() {
                "login" => login(form.get("login").unwrap(), redis_mutex),
                "add_appointment" => {
                    if let Some(username) = username {
                        add_appointment(&username, redis_mutex, form)
                    } else {
                        panic!("Should not have gotten here")
                    }
                }
                "search" => {
                    if let Some(username) = username {
                        search_appointments(&username, redis_mutex, form.get("search").unwrap())
                    } else {
                        panic!("Should not have gotten here")
                    }
                }
                "delete" => {
                    if let Some(username) = username {
                        delete_appointment(&username, redis_mutex, form)
                    } else {
                        panic!("Should not have gotten here")
                    }
                }
                _ => panic!("failed matching POST request type"),
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
