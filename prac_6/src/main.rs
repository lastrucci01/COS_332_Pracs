pub mod friend;

use std::{
    error::Error,
    io::{Read, Write},
    net::TcpStream,
};

use chrono::{Datelike, Duration, Local, NaiveDate};
use friend::{friends_from_file, Friend};

// Define a struct to hold a friend's information

fn main() -> Result<(), Box<dyn Error>> {
    let friends = friends_from_file("dates.txt");

    let upcoming_birthdays = upcoming_birthdays(friends);

    // Compose the email message
    let email_body = compose_email_body(upcoming_birthdays);
    let message = format!(
            "From: sender@332.birthday.com\r\nTo: lastrucci63@gmail.com\r\nSubject: Upcoming Birthdays\r\n\r\n{}",
            email_body
        );

    // Send the email using SMTP
    let smtp_server = "127.0.0.1";
    let smtp_port = 25;
    let mut stream = TcpStream::connect(format!("{}:{}", smtp_server, smtp_port))?;
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let helo_command = "EHLO localhost\r\n";
    stream.write_all(helo_command.as_bytes())?;
    stream.read(&mut buffer)?;

    let mail_from_command = "MAIL FROM:<birthday@332.mail.server>\r\n";
    stream.write_all(mail_from_command.as_bytes())?;
    stream.read(&mut buffer)?;

    let rcpt_to_command = "RCPT TO:<lastrucci63@gmail.com>\r\n";
    stream.write_all(rcpt_to_command.as_bytes())?;
    stream.read(&mut buffer)?;

    let data_command = "DATA\r\n";
    stream.write_all(data_command.as_bytes())?;
    stream.read(&mut buffer)?;

    stream.write_all(message.as_bytes())?;
    let end_message = "\r\n.\r\n";
    stream.write_all(end_message.as_bytes())?;
    stream.read(&mut buffer)?;

    let quit_command = "QUIT\r\n";
    stream.write_all(quit_command.as_bytes())?;

    Ok(())
}

pub fn upcoming_birthdays(friends: Vec<Friend>) -> Vec<Friend> {
    let filter_vec = friends
        .into_iter()
        .filter(|friend| {
            let day = friend.day();
            let month = friend.month();
            let current_year = Local::now().year();

            let birthday = NaiveDate::from_ymd_opt(current_year, month, day);

            let today = Local::now().date_naive();

            let six_days_from_today = today.checked_add_signed(Duration::days(7)).unwrap();

            if birthday >= Some(today) && birthday <= Some(six_days_from_today) {
                true
            } else {
                false
            }
        })
        .collect::<Vec<Friend>>();

    filter_vec
}

pub fn compose_email_body(friends: Vec<Friend>) -> String {
    if friends.is_empty() {
        format!("No upcoming friends birthday...")
    } else {
        let mut body = String::new();

        body.push_str("Upcoming Birthdays:\n");

        for f in friends {
            let part = format!("{} - {}/{}\n", f.name(), f.day(), f.month());
            body.push_str(&part);
        }

        body.push_str("Remeber to wish them!\n");

        body
    }
}
