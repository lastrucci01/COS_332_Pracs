use std::{
    collections::{BTreeMap, HashMap},
    sync::Mutex,
};

use chrono::{DateTime, NaiveDate, Utc};
use redis::Client;
use tera::Context;
use uuid::Uuid;

use crate::{
    file::{get_contents, render_html},
    headers::{format_response, get_basic_headers},
    user::User,
};

pub fn content(value: &str) -> String {
    let content = match value {
        "css" => ("css", get_contents("static/style.css")),
        "js" => ("js", get_contents("static/script.js")),
        _ => panic!("Unsupported content type: {}", value),
    };
    let response_body = content.1;
    let headers = get_basic_headers(&response_body, content.0);

    format_response(response_body, headers, "200")
}

pub fn home() -> String {
    let response_body = get_contents("static/pages/login.html");
    let headers = get_basic_headers(&response_body, "html");

    format_response(response_body, headers, "200")
}

pub fn login(username: &str, redis_mutex: &mut Mutex<Client>) -> String {
    let user = User::read_user(username, redis_mutex);

    if let Some(user) = user {
        let context = if user.appointments().is_empty() {
            let mut context = Context::new();
            context.insert("username", &user.username());
            context
        } else {
            let mut context = Context::new();
            context.insert("username", &user.username());

            let appointments_hash = user.appointments();
            let appointments_unix: HashMap<i64, String> =
                appointments_hash
                    .values()
                    .fold(HashMap::new(), |mut acc, (date, name)| {
                        acc.insert(*date, name.to_string());
                        acc
                    });
            let mut sorted_map: BTreeMap<i64, String> = BTreeMap::new();
            for key in appointments_unix.keys() {
                let value = appointments_unix.get(key).unwrap();
                sorted_map.insert(*key, value.to_string());
            }
            let appointments = sorted_map
                .iter()
                .map(|(date, name)| {
                    let datetime = DateTime::<Utc>::from_utc(
                        chrono::NaiveDateTime::from_timestamp_opt(*date, 0).unwrap(),
                        Utc,
                    );

                    let formatted_date = datetime.format("%d-%m-%y").to_string();

                    (formatted_date, name.to_string())
                })
                .collect::<BTreeMap<String, String>>();

            context.insert("appointments", &appointments);
            context
        };

        let response_body = render_html("appointments.html", context);
        let mut headers = get_basic_headers(&response_body, "html");

        headers.insert("Set-cookie".to_owned(), format!("name={}", user.username()));
        headers.insert("Location".to_owned(), "/appointments".to_owned());

        format_response(response_body, headers, "200")
    } else {
        home()
    }
}

pub fn signup(username: &str, redis_mutex: &mut Mutex<Client>) -> String {
    let user = User::new_user(username, redis_mutex);

    if let Some(user) = user {
        let mut context = Context::new();
        context.insert("username", &user.username());

        let response_body = render_html("appointments.html", context);
        let mut headers = get_basic_headers(&response_body, "html");

        headers.insert("Set-cookie".to_owned(), format!("name={}", user.username()));
        headers.insert("Location".to_owned(), "/appointments".to_owned());

        format_response(response_body, headers, "200")
    } else {
        home()
    }
}

pub fn appointments(username: &str, redis_mutex: &mut Mutex<Client>) -> String {
    let user = User::read_user(username, redis_mutex).unwrap();

    let context = if user.appointments().is_empty() {
        let mut context = Context::new();
        context.insert("username", &user.username());
        context
    } else {
        let mut context = Context::new();
        context.insert("username", &user.username());

        let appointments_hash = user.appointments();
        let appointments_unix: HashMap<i64, String> =
            appointments_hash
                .values()
                .fold(HashMap::new(), |mut acc, (date, name)| {
                    acc.insert(*date, name.to_string());
                    acc
                });
        let mut sorted_map: BTreeMap<i64, String> = BTreeMap::new();
        for key in appointments_unix.keys() {
            let value = appointments_unix.get(key).unwrap();
            sorted_map.insert(*key, value.to_string());
        }
        let appointments = sorted_map
            .iter()
            .map(|(date, name)| {
                let datetime = DateTime::<Utc>::from_utc(
                    chrono::NaiveDateTime::from_timestamp_opt(*date, 0).unwrap(),
                    Utc,
                );

                let formatted_date = datetime.format("%d-%m-%y").to_string();

                (formatted_date, name.to_string())
            })
            .collect::<BTreeMap<String, String>>();

        context.insert("appointments", &appointments);
        context
    };

    let response_body = render_html("appointments.html", context);
    let headers = get_basic_headers(&response_body, "html");

    format_response(response_body, headers, "200")
}

pub fn add_appointment(
    username: &str,
    redis_mutex: &mut Mutex<Client>,
    body: HashMap<String, String>,
) -> String {
    let mut user = User::read_user(username, redis_mutex).unwrap();

    let id = Uuid::new_v4();
    let date_str = body.get("date").unwrap();
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    let unix_time = date.and_hms_opt(0, 0, 0).unwrap().timestamp();

    let person = body.get("name").unwrap();

    user.add_appointment(redis_mutex, id, (unix_time, person.to_string()));

    appointments(username, redis_mutex)
}

pub fn logout() -> String {
    let response_body = get_contents("static/pages/login.html");
    let mut headers = get_basic_headers(&response_body, "html");

    headers.insert(
        "Set-cookie".to_owned(),
        format!("name=; expires=Thu, 01 Jan 1970 00:00:00 GMT"),
    );
    headers.insert("Location".to_owned(), "/appointments".to_owned());

    format_response(response_body, headers, "200")
}

pub fn four0four() -> String {
    let response_body = get_contents("static/error_code/404.html");
    let headers = get_basic_headers(&response_body, "html");

    format_response(response_body, headers, "404")
}
