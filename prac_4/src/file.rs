use std::{env, fs::File, io::Read};

use tera::{Context, Tera};

pub fn get_contents(path: &str) -> String {
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

pub fn render_html(file_name: &str, context: Context) -> String {
    let mut exe_path = env::current_exe().unwrap();
    for _ in 0..3 {
        exe_path = exe_path.parent().unwrap().to_path_buf();
    }

    let path = exe_path.to_str().unwrap().to_string() + "/static/pages/*.html";

    let tera = Tera::new(&path);
    tera.unwrap().render(&file_name, &context).unwrap()
}
