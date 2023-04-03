use std::{collections::HashMap, sync::Mutex};

use redis::{Client, Commands};
use serde::Serialize;
use serde_json::{from_str, to_string};
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    username: String,
    appointments: HashMap<String, (i64, String)>,
}

impl User {
    pub fn read_user(username: &str, redis_mutex: &mut Mutex<Client>) -> Option<Self> {
        let mut conn = redis_mutex.get_mut().unwrap().get_connection().unwrap();

        let appointment_res: Result<String, redis::RedisError> = conn.get(username);
        if let Ok(appointments_str) = appointment_res {
            Some(Self {
                username: username.to_string(),
                appointments: from_str(&appointments_str).unwrap(),
            })
        } else {
            None
        }
    }

    pub fn new_user(username: &str, redis_mutex: &mut Mutex<Client>) -> Option<Self> {
        let mut conn = redis_mutex.get_mut().unwrap().get_connection().unwrap();
        let appointment_res: Result<String, redis::RedisError> = conn.get(username);
        if let Err(_) = appointment_res {
            let appointments: HashMap<String, (i64, String)> = HashMap::new();
            let appointments_str = to_string(&appointments).unwrap();
            let _: String = conn
                .set(username.as_bytes(), appointments_str.as_bytes())
                .unwrap();

            Some(Self {
                username: username.to_string(),
                appointments: appointments,
            })
        } else {
            None
        }
    }

    // pub fn user(&self) -> User {
    //     User {
    //         username: self.username,
    //         appointments: self.appointments,
    //     }
    // }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }

    pub fn appointments(&self) -> HashMap<String, (i64, String)> {
        self.appointments.clone()
    }

    pub fn add_appointment(
        &mut self,
        redis_mutex: &mut Mutex<Client>,
        id: Uuid,
        (date, person): (i64, String),
    ) {
        self.appointments.insert(id.to_string(), (date, person));
        let mut conn = redis_mutex.get_mut().unwrap().get_connection().unwrap();
        let _: String = conn
            .set(&self.username, to_string(&self.appointments).unwrap())
            .unwrap();
    }

    pub fn remove_appointment(&mut self) {}
    pub fn update_appoint(&mut self) {}
    pub fn list_appointments(&mut self) {}
}
