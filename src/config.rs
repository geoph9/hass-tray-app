use toml::{Value, de::Error};
use serde::Deserialize;
use std::io::prelude::*;
use std::fs::File;

#[derive(Deserialize)]
pub struct Config {
    pub weather_settings: WeatherSettings,
    pub toggleables: Toggleables,
    pub user_settings: UserSettings
}

#[derive(Deserialize)]
pub struct WeatherSettings {
    pub show_weather_stats: bool,
    pub refresh_stats: bool,  // if true, the weather stats will be updated ever $refresh_rate seconds
    pub refresh_rate: u8,  // in seconds (refresh_stats needs to be true)
    pub show_humidity: bool,  // True if you have a humidity sensor
    pub temp_entity_id: String,
    pub humidity_entity_id: String, // Needed only if $show_humidity is true
}

#[derive(Deserialize)]
pub struct Toggleables {
    pub entity_ids: Vec<String>  // e.g. ['switch.kitchen_lights', 'switch.livingroom_tv_relay']
}

#[derive(Deserialize)]
pub struct UserSettings {
    pub hass_access_token: String,
    pub hass_url: String,
}


pub fn read_config(path: &str) -> Config {
    let mut file = match File::open(path) {
        Ok(f) => f
        , Err(e) => panic!("Error occurred opening file: {} - Err: {}", path, e)
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    let config: Config = toml::from_str(&*contents).unwrap();

    config
}