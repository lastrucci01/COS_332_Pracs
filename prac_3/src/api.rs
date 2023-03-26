use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::HashMap;

lazy_static! {
    static ref TIMEZONES: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert(
            "joburg",
            "http://worldtimeapi.org/api/timezone/Africa/Johannesburg",
        );
        map.insert(
            "new_york",
            "http://worldtimeapi.org/api/timezone/America/New_York",
        );
        map.insert(
            "london",
            "http://worldtimeapi.org/api/timezone/Europe/London",
        );
        map.insert(
            "shanghai",
            "http://worldtimeapi.org/api/timezone/Asia/Shanghai",
        );
        map.insert(
            "moscow",
            "http://worldtimeapi.org/api/timezone/Europe/Moscow",
        );
        map
    };
}

pub struct TimeZone {
    pub datetime: String,
    pub timezone: String,
    pub day_of_week: u64,
}

pub fn fetch_city(city: &str) -> Option<TimeZone> {
    match TIMEZONES.contains_key(city) {
        true => {
            let url = TIMEZONES.get(city).unwrap().to_owned();
            let response = reqwest::blocking::get(url).unwrap().text().unwrap();
            let json_value: Value = serde_json::from_str(&response).unwrap();
            Some(TimeZone {
                datetime: json_value["datetime"].as_str().unwrap().to_owned(),
                timezone: json_value["timezone"].as_str().unwrap().to_owned(),
                day_of_week: json_value["day_of_week"].as_u64().unwrap(),
            })
        }
        false => None,
    }
}
