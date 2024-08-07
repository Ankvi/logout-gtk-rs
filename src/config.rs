use std::fs::read_to_string;

use homedir::my_home;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Operation {
    pub exec: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ButtonConfig {
    pub icon: String,
    pub text: String,
    pub operation: Operation,
}

pub fn load_button_config() -> Vec<ButtonConfig> {
    let config_file_path = my_home()
        .unwrap()
        .expect("Could not find home directory for user")
        .as_path()
        .join(".config/logout-gtk/config.json");

    let config = match config_file_path.exists() {
        true => read_to_string(config_file_path).expect("Could not read config file"),
        false => String::from_utf8_lossy(include_bytes!("config/default.json")).to_string()
    };

    serde_json::from_str(&config)
        .expect("Could not parse config into a button configuration. Check syntax")
}
