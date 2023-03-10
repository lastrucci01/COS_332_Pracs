use std::{collections::HashMap, fs::File, io::{Write, BufReader, BufRead}};



pub struct User {
    name: String,
    address_book: HashMap<String, String>,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            name,
            address_book: HashMap::new(),
        }
    }

    pub fn new_from_file(name: &String) -> Self {
        let path = format!("{}.txt", name);
        let user_file = File::open(path).unwrap();
        let reader = BufReader::new(user_file);
        let mut temp_book = HashMap::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<_> = line.split(",").collect();
            temp_book.insert(String::from(parts[0]), String::from(parts[1]));
        }

        Self { name: String::from(name), address_book: temp_book }
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
        let file_name = format!("{}.txt",&self.name);

        let mut file = File::create(file_name).expect("failed to create user file...");

        if self.address_book.is_empty() {
            return
        }

        let mut data = String::from("");

        for (name, tel) in &self.address_book {
             let part = format!("{},{}\n", name, tel);
             data.push_str(&part);
        }


        file.write(data.as_bytes()).unwrap();
    }
}
