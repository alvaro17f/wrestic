use anyhow::{Context, Result};
use config::Config;

use crate::macros::anyhow::error;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Settings {
    pub user: String,
    pub name: String,
    pub bucket: String,
    pub repository: String,
    pub restic_password: String,
    pub account_id: String,
    pub account_key: String,
    pub backup_folder: String,
    pub restore_folder: String,
    pub keep_last: String,
}

pub fn get_config() -> Result<Vec<Settings>> {
    let config = Config::builder()
        .add_source(config::File::with_name(
            "/home/alvaro17f/.config/wrestic/wrestic.toml",
        ))
        .build()?;

    let user_table = config.get_table("user")?;

    let user = user_table
        .clone()
        .get_key_value("USER")
        .unwrap()
        .to_owned()
        .1
        .to_string();

    let settings_table = config.get_table("settings")?;

    let mut settings: Vec<Settings> = Vec::new();

    for (key, value) in &settings_table {
        let deserialized_value = value.clone().try_deserialize::<serde_json::Value>()?;

        let settings_struct = Settings {
            user: user.clone(),
            name: key.to_string().replace('\"', ""),

            bucket: deserialized_value
                .get("BUCKET")
                .context(error!("Failed to get the value of BUCKET for {key}"))?
                .to_string()
                .replace('\"', ""),
            repository: deserialized_value
                .get("REPOSITORY")
                .context(error!("Failed to get the value of REPOSITORY for {key}"))?
                .to_string()
                .replace('\"', ""),
            restic_password: deserialized_value
                .get("RESTIC_PASSWORD")
                .context(error!(
                    "Failed to get the value of RESTIC_PASSWORD for {key}"
                ))?
                .to_string()
                .replace('\"', ""),
            account_id: deserialized_value
                .get("B2_ACCOUNT_ID")
                .context(error!("Failed to get the value of B2_ACCOUNT_ID for {key}"))?
                .to_string()
                .replace('\"', ""),
            account_key: deserialized_value
                .get("B2_ACCOUNT_KEY")
                .context(error!(
                    "Failed to get the value of B2_ACCOUNT_KEY for {key}"
                ))?
                .to_string()
                .replace('\"', ""),
            backup_folder: deserialized_value
                .get("BACKUP_FOLDER")
                .context(error!("Failed to get the value of BACKUP_FOLER for {key}"))?
                .to_string()
                .replace('\"', ""),
            restore_folder: deserialized_value
                .get("RESTORE_FOLDER")
                .context(error!(
                    "Failed to get the value of RESTORE_FOLDER for {key}"
                ))?
                .to_string()
                .replace('\"', ""),
            keep_last: deserialized_value
                .get("KEEP_LAST")
                .context(error!("Failed to get the value of KEEP_LAST for {key}"))?
                .to_string()
                .replace('\"', ""),
        };

        settings.push(settings_struct);
    }

    Ok(settings)
}
