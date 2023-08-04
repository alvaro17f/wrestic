use crate::macros::anyhow::uerror;
use anyhow::{Context, Ok, Result};
use cmd_lib::run_cmd;
use std::fs;

pub fn docker_down(user: &str) -> Result<()> {
    if run_cmd!(
        docker-compose -f cd /home/$user/Workspace/mailcow-dockerized/docker-compose.yml down;
    )
    .is_err()
    {
        Err(uerror!("Failed to stop mailcow container"))?;
    }
    for dir in fs::read_dir(format!("/home/{user}/Workspace/docker"))
        .context(uerror!("failed to read dirs in the docker folder"))?
    {
        let dir = dir?.path();
        if run_cmd!(
            cd $dir;
            docker-compose down;
        )
        .is_err()
        {
            Err(uerror!("Failed to stop docker containers"))?;
        }
    }
    Ok(())
}

pub fn docker_up(user: &str) -> Result<()> {
    if run_cmd!(
        docker-compose -f cd /home/$user/Workspace/mailcow-dockerized/docker-compose.yml up -d;
    )
    .is_err()
    {
        Err(uerror!("Failed to start mailcow container"))?;
    }
    for dir in fs::read_dir(format!("/home/{user}/Workspace/docker"))
        .context(uerror!("failed to read dirs in the docker folder"))?
    {
        let dir = dir?.path();
        if run_cmd!(
            cd $dir;
            docker-compose up -d;
        )
        .is_err()
        {
            Err(uerror!("Failed to start docker containers"))?;
        }
    }
    Ok(())
}
