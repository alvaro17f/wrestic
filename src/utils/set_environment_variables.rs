use crate::utils::get_config::Settings;
use anyhow::Result;
use std::env;

pub fn set_environment_variables(setting: &Settings) -> Result<()> {
    env::set_var("USER", &setting.user);
    env::set_var("RESTIC_PASSWORD", &setting.restic_password);
    for env in &setting.env {
        for (key, value) in env {
            env::set_var(key, value);
        }
    }
    Ok(())
}
