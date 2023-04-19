use std::{
    collections::{BTreeMap, HashMap},
    sync::Mutex,
};

use chrono::{DateTime, Utc};
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

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn appointments(&self) -> BTreeMap<String, (String, String)> {
        let appointments = self.appointments.clone();
        let mut sorted_map: BTreeMap<String, (i64, String)> = BTreeMap::new();
        for key in appointments.keys() {
            let value = appointments.get(key).unwrap();
            sorted_map.insert(key.to_string(), (value.0, (*value.1).to_string()));
        }
        sorted_map
            .iter()
            .map(|(id, (date, name))| {
                let datetime = DateTime::<Utc>::from_utc(
                    chrono::NaiveDateTime::from_timestamp_opt(*date, 0).unwrap(),
                    Utc,
                );

                let formatted_date = datetime.format("%d-%m-%y").to_string();

                (id.to_owned(), (formatted_date, name.to_owned()))
            })
            .collect::<BTreeMap<String, (String, String)>>()
    }

    pub fn add_appointment(&mut self, id: Uuid, (date, person): (i64, String)) {
        self.appointments.insert(id.to_string(), (date, person));
    }

    pub fn remove_appointment(&mut self, id: &str) {
        self.appointments.remove(id);
    }

    pub fn search_appointments(&mut self, search_term: &str) -> BTreeMap<String, (String, String)> {
        let map = self.appointments();
        map.iter()
            .filter(|(_, (_, name))| name.contains(search_term))
            .map(|v| (v.0.to_owned(), v.1.to_owned()))
            .collect::<BTreeMap<String, (String, String)>>()
    }

    pub fn write_to_redis(&self, redis_mutex: &mut Mutex<Client>) {
        let mut conn = redis_mutex.get_mut().unwrap().get_connection().unwrap();

        let _: String = conn
            .set(&self.username, to_string(&self.appointments).unwrap())
            .unwrap();
    }
}
