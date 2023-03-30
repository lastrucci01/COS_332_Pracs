mod api;
mod formatting;

use chrono::{DateTime, Datelike, Timelike};
use regex::Regex;
use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::{env, thread};
use tera::{Context, Tera};

use crate::api::fetch_city;

static CITIES: [&str; 5] = ["joburg", "london", "new_york", "shanghai", "moscow"];

fn handle_client(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    let mut stream = stream;

    loop {
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                // Attempt to decode the incoming data as UTF-8
                match std::str::from_utf8(&buffer[0..n]) {
                    Ok(s) => println!("Received data: {}", s),
                    Err(e) => println!("Failed to decode incoming data as UTF-8: {}", e),
                }
            }
            Ok(_) => {
                // Connection was closed
                break;
            }
            Err(e) => {
                println!("Error reading from socket: {}", e);
                break;
            }
        }
    }
    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();/
    // let request = String::from_utf8_lossy(&buffer[..]);

    // let response = handle_request(request);

    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
}

fn get_contents(path: &str) -> String {
    let mut exe_path = env::current_exe().unwrap();
    for _ in 0..3 {
        exe_path = exe_path.parent().unwrap().to_path_buf();
    }
    let filepath = format!("{}/{}", exe_path.to_str().unwrap(), path);
    let mut file = File::open(&filepath).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn handle_request(request: Cow<str>) -> String {
    println!("HANDLING REQ");
    let mut parts = request.trim().split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap().trim_start_matches('/');
    println!("{} - {}", method, path);

    match (method, path) {
        ("GET", "") => {
            let jozi = build_info("joburg");

            let mut context = Context::new();
            context.insert("current_time", &jozi.time);
            context.insert("current_date", &jozi.date);
            context.insert("week_day", &jozi.day);
            context.insert("timezone", &jozi.timezone);

            let response_body = render_html("index.html", context);
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
                response_body.len(),
                response_body
            )
        }
        ("GET", "style.css") => {
            let response_body = get_contents("static/style.css");
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/css\r\nContent-Length: {}\r\n\r\n{}",
                response_body.len(),
                response_body
            )
        }
        ("GET", path) => {
            if CITIES.contains(&path) {
                let jozi = build_info("joburg");

                let mut context = Context::new();
                context.insert("current_time", &jozi.time);
                context.insert("current_date", &jozi.date);
                context.insert("week_day", &jozi.day);
                context.insert("timezone", &jozi.timezone);

                let other = build_info(path);
                println!("{:?}", other);
                context.insert("other_time", &other.time);
                context.insert("other_date", &other.date);
                context.insert("other_day", &other.day);
                context.insert("other_timezone", &other.timezone);

                let response_body = render_html("other.html", context);
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                    response_body.len(),
                    response_body
                )
            } else {
                let response_body = get_contents("static/error_code/404.html");
                format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                response_body.len(),
                response_body
            )
            }
        }
        _ => {
            let response_body = get_contents("static/error_code/404.html");
            format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                response_body.len(),
                response_body
            )
        }
    }
}

#[derive(Debug)]
struct Info {
    pub time: String,
    pub date: String,
    pub day: String,
    pub timezone: String,
}

fn build_info(city: &str) -> Info {
    let resp = fetch_city(city).unwrap();
    let datetime = resp.datetime;
    let (date, time, offset) = parse_datetime(datetime).unwrap();

    let timezone = resp.timezone.to_string() + " " + &offset;
    let day = week_day(resp.day_of_week).to_string();

    Info {
        time,
        date,
        day,
        timezone,
    }
}

fn week_day(day: u64) -> &'static str {
    match day {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => panic!("Invalid day of week"),
    }
}

fn parse_datetime(dt_str: String) -> Option<(String, String, String)> {
    let dt = DateTime::parse_from_rfc3339(&dt_str).ok()?;

    let date = format!("{:04}-{:02}-{:02}", dt.year(), dt.month(), dt.day());
    let time = format!("{:02}:{:02}:{:02}", dt.hour(), dt.minute(), dt.second(),);

    let re = Regex::new(r"([+-]\d{2}):(\d{2})$").unwrap();
    let caps = re.captures(&dt_str)?;
    let tz_offset = format!(
        "{}{}",
        caps.get(1).unwrap().as_str(),
        caps.get(2).unwrap().as_str()
    );

    Some((date, time, tz_offset))
}

fn render_html(file_name: &str, context: Context) -> String {
    let mut exe_path = env::current_exe().unwrap();
    for _ in 0..3 {
        exe_path = exe_path.parent().unwrap().to_path_buf();
    }

    let path = exe_path.to_str().unwrap().to_string() + "/static/pages/*.html";

    let tera = Tera::new(&path);

    tera.unwrap().render(&file_name, &context).unwrap()
}

fn main() {
    let listener = TcpListener::bind("34.249.78.103:6969").unwrap();
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
