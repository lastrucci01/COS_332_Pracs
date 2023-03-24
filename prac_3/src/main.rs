use reqwest::Error;
use serde_json::Value;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Debug)]
struct TimeResponse {
    abbreviation: String,
    client_ip: String,
    datetime: String,
    day_of_week: u8,
    day_of_year: u16,
    dst: bool,
    dst_from: Option<String>,
    dst_offset: i32,
    dst_until: Option<String>,
    raw_offset: i32,
    timezone: String,
    unixtime: u64,
    utc_datetime: String,
    utc_offset: String,
    week_number: u8,
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = reqwest::blocking::get("http://worldtimeapi.org/api/ip").unwrap();
    let time_response: Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    println!("Current time: {:?}", time_response);
    let response = format!(
        "HTTP/1.1 404 NOT FOUND\r\n\r\nCurrent time in seconds since Unix epoch: {:?}",
        time_response
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();

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
