use core::num;
use std::{collections::HashMap, fmt::format, path::Path, fs::File, io::Write};

use serde_json::to_string;

pub struct User {
    client_addr: String,
    name: String,
    address_book: HashMap<String, String>,
}

impl User {
    pub fn new(name: String, client_addr: String) -> Self {
        Self {
            name,
            client_addr,
            address_book: HashMap::new(),
        }
    }

    pub fn new_from_file(client_addr: &String) -> Self {
        todo!()
    }

    pub fn name(&self) -> String {
        (&self.name).to_string()
    }

    pub fn add_contact(&mut self, name: &str, telephone: &str) {
        self.address_book
            .insert(String::from(name), String::from(telephone));
    }

    pub fn search_contact(&self, name: &str) -> String {
        if let Some((name, number)) = self.address_book.get_key_value(name) {
            format!("{}: {}\r\n", name, number)
        } else {
            format!("No number for desired name\r\n")
        }
    }

    pub fn remove_contact(&mut self, name: &str) -> String{
        match self.address_book.remove(name) {
            Some(_) => format!("Number removed for: {}\r\n", name),
            None => format!("No number found for name\r\n")
        }
    }

    pub fn save_to_file(&self) {
        let header = format!("{}\n",&self.name);
        let data = to_string(&self.address_book).unwrap();

        let file_name = format!("{}.txt",&self.client_addr);

        let mut file = File::create(file_name).expect("failed to create user file...");

        file.write(header.as_bytes()).unwrap();
        file.write(data.as_bytes()).unwrap();
    }
}
