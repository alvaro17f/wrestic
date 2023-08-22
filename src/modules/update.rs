use crate::{
    macros::anyhow::error,
    modules::selector::selector,
    utils::tools::{clear, pause},
};
use anyhow::{Context, Result};
use cmd_lib::run_cmd;
use color_print::cprintln;
use indicatif::ProgressBar;
use std::{process::Command, time::Duration};

pub fn update(noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<c,u,s>UPDATER");
    println!();

    let url = "https://api.github.com/repos/alvaro17f/wrestic/releases/latest";
    let command = format!(
        r#"curl -sL $(curl -s "{url}" | grep browser_download_url | cut -d '"' -f 4) | sudo tar zxf - -C /usr/bin --overwrite"#
    );

    if get_installed_version()? >= get_latest_version(&url)? {
        cprintln!("<g,u>Wrestic is already up to date!\n");
        pause()?
    } else {
        cprintln!(
            "<y>Wrestic is outdated!\n<r>current: <k>{}<g>latest: <k>{}\n",
            get_installed_version()?,
            get_latest_version(&url)?
        );

        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_message("Updating wrestic...");

        run_cmd!(
            sh -c $command;
        )
        .context(error!("failed fetching the latest version from wrestic."))?;

        pb.finish_and_clear();

        cprintln!("<g,u>Wrestic was successfully updated\n");

        pause()?;
    }

    if !noconfirm {
        selector()?;
    }
    Ok(())
}

fn get_installed_version() -> std::io::Result<String> {
    let output = Command::new("wrestic").arg("--version").output()?;
    let version_string = String::from_utf8_lossy(&output.stdout);
    let version = version_string.trim_start_matches("wrestic ").to_string();
    Ok(version)
}
fn get_latest_version(url: &str) -> std::io::Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            r#"curl -s "{url}" | grep tag_name | grep -Eo '[0-9.]+'"#
        ))
        .output()?;
    let version_string = String::from_utf8_lossy(&output.stdout);
    let version = version_string.trim().to_string();
    Ok(version)
}
