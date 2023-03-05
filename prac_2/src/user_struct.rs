use std::collections::HashMap;

pub struct User {
    name: String,
    address_book: HashMap<String, String>
}

impl User {
    pub fn new(name: String) -> Self {
        Self { name, address_book: HashMap::new() }
    }

    pub fn new_from_file(client_addr: &String) -> Self {
        todo!()
    }

    pub fn name(&self) -> String { (&self.name).to_string() }

    pub fn add_contact(&self, name: &str, telephone: &str) {

    }

    pub fn search_contact(&self, name: String) {

    }

    pub fn remove_contact(&self, name: String) {

    }

    pub fn save_to_file(&self) {

    }
    
}

