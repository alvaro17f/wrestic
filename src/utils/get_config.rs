use crate::utils::macros::error;
use anyhow::{Context, Result};
use config::Config;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Settings {
    pub user: String,
    pub backend: String,
    pub name: String,
    pub repository: String,
    pub restic_password: String,
    pub backup_folder: String,
    pub restore_folder: String,
    pub keep_last: String,
    pub env: Option<HashMap<String, String>>,
}

pub fn get_config() -> Result<Vec<Settings>> {
    fn find_config_file() -> Option<PathBuf> {
        let home_dir = PathBuf::from("/home/");
        let mut path = PathBuf::new();
        for entry in fs::read_dir(home_dir).ok()? {
            let entry = entry.ok()?;
            let mut env_path = entry.path();
            env_path.push(".config/wrestic/wrestic.toml");
            if env_path.exists() {
                path = env_path;
                break;
            }
        }
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }
    let config = Config::builder()
        .add_source(config::File::with_name(
            find_config_file()
                .context(error!("Failed to find config file"))?
                .to_str()
                .context(error!("Failed to convert config path to string"))?,
        ))
        .build()?;

    let user = find_config_file()
        .unwrap()
        .iter()
        .nth(2)
        .and_then(|f| f.to_str())
        .unwrap_or_default()
        .to_string();

    let settings_table = config.get_table("settings")?;

    let mut settings: Vec<Settings> = Vec::new();

    for (key, value) in &settings_table {
        let deserialized_value = value.clone().try_deserialize::<serde_json::Value>()?;

        let settings_struct = Settings {
            user: user.clone(),
            name: key.to_string().replace('\"', ""),

            backend: deserialized_value
                .get("BACKEND")
                .context(error!(format!(
                    "Failed to get the value of BACKEND for {key}"
                )))?
                .to_string()
                .replace('\"', ""),
            repository: deserialized_value
                .get("REPOSITORY")
                .context(error!(format!(
                    "Failed to get the value of REPOSITORY for {key}"
                )))?
                .to_string()
                .replace('\"', ""),
            restic_password: deserialized_value
                .get("RESTIC_PASSWORD")
                .context(error!(format!(
                    "Failed to get the value of RESTIC_PASSWORD for {key}"
                )))?
                .to_string()
                .replace('\"', ""),
            backup_folder: deserialized_value
                .get("BACKUP_FOLDER")
                .context(error!(format!(
                    "Failed to get the value of BACKUP_FOLER for {key}"
                )))?
                .to_string()
                .replace('\"', ""),
            restore_folder: deserialized_value
                .get("RESTORE_FOLDER")
                .context(error!(format!(
                    "Failed to get the value of RESTORE_FOLDER for {key}"
                )))?
                .to_string()
                .replace('\"', ""),
            keep_last: deserialized_value
                .get("KEEP_LAST")
                .context(error!(format!(
                    "Failed to get the value of KEEP_LAST for {key}"
                )))?
                .to_string()
                .replace('\"', ""),
            env: deserialized_value.get("env").iter().next().map(|x| {
                x.as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string().replace('\"', "")))
                    .collect()
            }),
        };

        settings.push(settings_struct);
    }
    settings.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(settings)
}
