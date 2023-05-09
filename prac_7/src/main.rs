use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // Gmail POP server address and port
    let server = "pop.gmail.com";
    let port = 995;

    // Create a TLS connector and connect over SSL/TLS
    let mut builder = TlsConnector::builder();
    builder.min_protocol_version(Some(native_tls::Protocol::Tlsv12));
    let connector = builder.build().unwrap();

    // Connect to the server
    let tcp_stream = TcpStream::connect((server, port)).unwrap();
    let mut tls_stream = connector.connect(server, tcp_stream).unwrap();

    // Authenticate with the server using your Gmail credentials
    let username = "lastrucci63@gmail.com";
    let password = "wpuzbjyjvwhuimav";

    let mut buf = [0; 4096];
    let r = tls_stream.read(&mut buf).unwrap();
    println!("{:?}", String::from_utf8(buf[..r].to_vec()));

    tls_stream
        .write(format!("USER {}\r\n", username).as_bytes())
        .unwrap();
    tls_stream.flush().unwrap();

    let mut buf = [0; 4096];
    let r = tls_stream.read(&mut buf).unwrap();
    println!("{:?}", String::from_utf8(buf[..r].to_vec()));

    tls_stream
        .write(format!("PASS {}\r\n", password).as_bytes())
        .unwrap();
    tls_stream.flush().unwrap();

    let mut buf = [0; 4096];
    let r = tls_stream.read(&mut buf).unwrap();
    println!("{:?}", String::from_utf8(buf[..r].to_vec()));

    // List all emails in the inbo
    tls_stream.write(format!("LIST\r\n").as_bytes()).unwrap();
    tls_stream.flush().unwrap();

    let mut buf = [0; 16384];
    let r = tls_stream.read(&mut buf).unwrap();
    println!("R {:?}", r);
    let list = String::from_utf8(buf[..r].to_vec()).unwrap();
    let parts: Vec<&str> = list.split("\r\n").collect();
    for p in parts {
        let message_id = p.split_whitespace().collect::<Vec<_>>()[0];
        tls_stream
            .write(format!("RETR {}\r\n", message_id).as_bytes())
            .unwrap();
        tls_stream.flush().unwrap();

        let mut readbuf = [0; 16384];
        let r = tls_stream.read(&mut readbuf).unwrap();

        println!("strr {}", String::from_utf8(readbuf[..r].to_vec()).unwrap());
        println!("r {}", r);
    }
}

// let num = p.split_whitespace().collect::<Vec<_>>()[1];
//         tls_stream
//             .write(format!("RETR {}\r\n", num).as_bytes())
//             .unwrap();
//         tls_stream.flush().unwrap();
//         let mut readbuf = [0; 4096];
//         let read = tls_stream.read(&mut readbuf).unwrap();
//         let retr = String::from_utf8(buf[..read].to_vec()).unwrap();
//         let parts: Vec<&str> = list.split("\r\n").collect();
//         for p in parts {
//             if p.starts_with("Subject:") {
//                 let subject = String::from_utf8(buf[9..].to_vec()).unwrap();
//                 println!("Subject: {}", subject);
//             }
//         }
//         let mut readbuf = [0; 4096];
//         let read = tls_stream.read(&mut readbuf).unwrap();
//     }
//     let mut readbuf = [0; 4096];
//     let read = tls_stream.read(&mut readbuf).unwrap();
