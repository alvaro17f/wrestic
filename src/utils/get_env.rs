use crate::macros::anyhow::{error, uerror};
use anyhow::{Context, Ok, Result};
use std::{env, fs};

pub struct Env {
    pub backup_folder: String,
    pub bucket: String,
    pub keep_last: String,
    pub repository: String,
    pub restore_folder: String,
    pub user: String,
}

pub fn dotenv() -> Result<Env> {
    let read_dotenv = fs::read_to_string(".env").context(error!("Failed to read .env file"))?;

    let dotenv: Vec<(&str, &str)> = read_dotenv
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.splitn(2, '=');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap().trim_end_matches('\n');
            (key, value)
        })
        .collect();

    for (key, value) in &dotenv {
        env::set_var(key, value);
    }

    let user = env::var("USER").context(uerror!("please, add $USER to .env file"))?;
    if user.contains("root") {
        Err(uerror!("please, add $USER to .env file"))?;
    }
    let bucket = env::var("BUCKET").context(uerror!("please, add $BUCKET to .env file"))?;
    let repository =
        env::var("REPOSITORY").context(uerror!("please, add $REPOSITORY to .env file"))?;
    let keep_last =
        env::var("KEEP_LAST").context(uerror!("please, add $KEEP_LAST to .env file"))?;
    let backup_folder =
        env::var("BACKUP_FOLDER").context(uerror!("please, add $BACKUP_FOLDER to .env file"))?;
    let restore_folder =
        env::var("RESTORE_FOLDER").context(uerror!("please, add $RESTORE_FOLDER to .env file"))?;

    let env = Env {
        backup_folder,
        bucket,
        keep_last,
        repository,
        restore_folder,
        user,
    };

    Ok(env)
}
