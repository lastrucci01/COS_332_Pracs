use std::{env, fs::read_to_string};

#[derive(Debug)]
pub struct Friend {
    name: String,
    date: (u32, u32),
}

impl Friend {
    pub fn new(line: &str) -> Self {
        let mut date_name = line.split_whitespace();
        let date = date_name.next().unwrap();
        let day_month = date.split('/').collect::<Vec<&str>>();

        let mut day_month = day_month.into_iter();
        let day = day_month.next().unwrap().parse::<u32>().unwrap();
        let month = day_month.next().unwrap().parse::<u32>().unwrap();

        let name = date_name.next().unwrap();

        Self {
            name: name.to_string(),
            date: (day, month),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }
    pub fn month(&self) -> u32 {
        self.date.1
    }
    pub fn day(&self) -> u32 {
        self.date.0
    }
}

pub fn friends_from_file(filename: &str) -> Vec<Friend> {
    let mut friends: Vec<Friend> = Vec::new();

    let binding = env::current_exe().unwrap();
    let path = binding
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut path_buf = path.to_path_buf();
    path_buf.push("src");
    path_buf.push(filename);

    let file_contents = read_to_string(path_buf).expect("Could not read file");

    let mut it = file_contents.split('\n');
    while let Some(line) = it.next()  {
        friends.push(Friend::new(line));
    }

    friends
}
