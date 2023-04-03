use std::collections::HashMap;


pub fn format_response(
    response_body: String,
    headers: HashMap<String, String>,
    code: &str,
) -> String {
    let mut header_string = String::new();
    header_string.push_str(&format!("HTTP/1.1 {}", code));
    let head_len = headers.len();
    for (i, h) in headers.into_iter().enumerate() {
        header_string.push_str(&build_header(h));
        if i != head_len - 1 {
            header_string.push_str("\r\n")
        }
    }


    format!("{}\r\n\r\n{}", header_string, response_body)
}

fn build_header(header: (String, String)) -> String {
    let k = header.0;
    let v = header.1;

    match k.as_str() {
        "Set-cookie" => {
            format!("{}: {}", k, v)
        }
        "Location" => {
            format!("{}: {}", k, v)
        }
        "Content-Type" => {
            format!("{}: {}", k, v)
        }
        "Content-Length" => {
            format!("{}: {}", k, v)
        }
        h => panic!("header not found - {}", h),
    }
}

pub fn get_basic_headers(
    response_body: &str,
    content_type: &str,
) -> HashMap<String, String> {
    let mut headers: HashMap<String, String> = HashMap::new();

    headers.insert(
        String::from("Content-Length"),
        response_body.len().to_string(),
    );
    headers.insert(
        String::from("Content-Type"),
        match content_type {
            "html" => String::from("text/html"),
            "css" => String::from("text/css"),
            "js" => String::from("application/javascript"),
            _ => panic!("Content type failed"),
        },
    );
    headers
}
