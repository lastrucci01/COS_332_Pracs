use std::{collections::HashMap, net::TcpStream, fs, io::{Write, Read}};

use crate::{user_struct::User, io_enum::IOEnum};

pub struct ClientHandlerStruct {
    clients: HashMap<String, (User, TcpStream)>,
}


impl ClientHandlerStruct {
    pub fn new() -> Self {
        Self {
            clients:HashMap::new(),
        }
    }

    pub fn handle_incoming_client(&mut self, mut stream: TcpStream) {

        let client_addr = stream.peer_addr().unwrap().to_string();
        println!("Client {} connected!", &client_addr);
        
        let user = if self.is_saved(&client_addr) {
            let user = User::new_from_file(&client_addr);
            let msg = IOEnum::Greeting { name: (user.name()) }.output();
            stream.write_all(msg.as_bytes()).unwrap();
            user
        } else {
            let msg = IOEnum::NewUser.output();
            stream.write_all(msg.as_bytes()).unwrap();
            let mut name = String::new(); 
            stream.read_to_string(&mut name).unwrap();
            User::new(name)
        };  
        
        self.add_client(client_addr, user, stream);

        loop {

        }
    }


 
    pub fn is_saved(&mut self, client_addr: &String) -> bool {
        let mut base_path = std::env::current_exe().unwrap();
        base_path.push("users");
        let user_paths = fs::read_dir(base_path).unwrap();

        for path in user_paths {
            if let Ok(dir) = path {
                let file_name = dir.file_name().to_str().unwrap().to_string();

                println!("{:?}", file_name);
                return file_name == format!("{}.txt",client_addr);
            }
        }
        false
    }

    pub fn add_client(&mut self, client_addr: String, user: User, stream: TcpStream ) {
        self.clients.insert(client_addr, (user, stream));
    }

    pub fn save_quit(&self, client_addr: String) {

    }

    pub fn search(&self, client_addr: String) {

    }
    pub fn delete_user(&self, client_addr: String) {

    }
}