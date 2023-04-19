use std::{collections::HashMap, sync::Mutex};

use chrono::NaiveDate;
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
            let appointments = user.appointments();
            context.insert("appointments", &appointments);
            context
        };

        let response_body = render_html("appointments.html", context);
        let mut headers = get_basic_headers(&response_body, "html");

        headers.insert("Set-cookie".to_owned(), format!("name={}", user.username()));
        headers.insert("Location".to_owned(), "/".to_owned());

        format_response(response_body, headers, "200")
    } else {
        home()
    }
}

pub fn signup_get() -> String {
    let response_body = get_contents("static/pages/signup.html");
    let headers = get_basic_headers(&response_body, "html");

    format_response(response_body, headers, "200")
}

pub fn signup_post(username: &str, redis_mutex: &mut Mutex<Client>) -> String {
    let user = User::new_user(username, redis_mutex);

    if let Some(user) = user {
        let mut context = Context::new();
        context.insert("username", &user.username());

        let response_body = render_html("appointments.html", context);
        let mut headers = get_basic_headers(&response_body, "html");

        headers.insert("Set-cookie".to_owned(), format!("name={}", user.username()));
        headers.insert("Location".to_owned(), "/".to_owned());

        format_response(response_body, headers, "200")
    } else {
        home()
    }
}

pub fn appointments(user: User) -> String {
    let context = if user.appointments().is_empty() {
        let mut context = Context::new();
        context.insert("username", &user.username());
        context
    } else {
        let mut context = Context::new();
        context.insert("username", &user.username());

        context.insert("username", &user.username());
        let appointments = user.appointments();
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

    user.add_appointment(id, (unix_time, person.to_string()));
    user.write_to_redis(redis_mutex);
    appointments(user)
}

pub fn search_appointments(
    username: &str,
    redis_mutex: &mut Mutex<Client>,
    search_term: &str,
) -> String {
    let mut user = User::read_user(username, redis_mutex).unwrap();
    let mut context = Context::new();
    context.insert("username", &user.username());

    context.insert("username", &user.username());
    let appointments = user.search_appointments(search_term);
    context.insert("appointments", &appointments);

    let response_body = render_html("appointments.html", context);
    let headers = get_basic_headers(&response_body, "html");

    format_response(response_body, headers, "200")
}

pub fn delete_appointment(
    username: &str,
    redis_mutex: &mut Mutex<Client>,
    body: HashMap<String, String>,
) -> String {
    let mut user = User::read_user(username, redis_mutex).unwrap();

    let id = body.get("id").unwrap();
    user.remove_appointment(id);
    user.write_to_redis(redis_mutex);
    appointments(user)
}

pub fn logout() -> String {
    let response_body = get_contents("static/pages/login.html");
    let mut headers = get_basic_headers(&response_body, "html");

    headers.insert(
        "Set-cookie".to_owned(),
        format!("name=; Max-Age=0; expires=Thu, 01 Jan 1970 00:00:00 GMT\r\n"),
    );

    format_response(response_body, headers, "200")
}

pub fn four0four() -> String {
    let response_body = get_contents("static/error_code/404.html");
    let headers = get_basic_headers(&response_body, "html");

    format_response(response_body, headers, "404")
}
