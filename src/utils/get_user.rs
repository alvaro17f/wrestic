#![allow(dead_code)]
use crate::macros::errors::error;
use crate::utils::tools::select;
use anyhow::Result;
use std::fs;

pub fn get_user() -> Result<String> {
    let mut users = Vec::new();

    for entry in fs::read_dir("/home")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let config_path = path.join(".config/wrestic/wrestic.toml");
            if config_path.exists() {
                if let Some(user) = path.file_name().and_then(|os_str| os_str.to_str()) {
                    users.push(user.to_string());
                }
            }
        }
    }

    match users.len() {
        0 => Err(error!(
            "No users found. Please create a config file at ~/.config/wrestic/wrestic.toml for a user."
        )),
        1 => Ok(users[0].to_string()),
        _ => {
            let selection = select("Who are you?", users.clone());
            Ok(users[selection].to_string())
        }
    }
}
