use crate::{
    modules::selector::selector,
    utils::{
        get_current_shell::get_current_shell,
        macros::error,
        tools::{clear, pause},
    },
};
use anyhow::{Context, Result};
use cmd_lib::run_cmd;
use color_print::cprintln;
use indicatif::ProgressBar;
use std::{env, process::Command, time::Duration};
use which::which;

fn get_current_version() -> Result<String> {
    let version = env!("CARGO_PKG_VERSION").to_string();
    Ok(version)
}
fn get_latest_version(url: &str) -> Result<String> {
    let shell = get_current_shell()?;
    let output = Command::new(shell)
        .arg("-c")
        .arg(format!(
            r#"curl -s "{url}" | grep tag_name | grep -Eo '[0-9.]+'"#
        ))
        .output()
        .context(error!("Failed fetching the latest version of Wrestic."))?;
    let version_string = String::from_utf8_lossy(&output.stdout);
    let version = version_string.trim().to_string();
    if version.is_empty() {
        Err(error!(
            "Failed fetching the latest version of Wrestic. Try again later."
        ))?;
    }
    Ok(version)
}

pub fn update(noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<c,u,s>UPDATER");
    println!();

    let url = "https://api.github.com/repos/alvaro17f/wrestic/releases/latest";
    let command = format!(
        r#"curl -sL $(curl -s "{url}" | grep browser_download_url | cut -d '"' -f 4) -o /tmp/wrestic.tar.gz"#
    );

    if get_current_version()? >= get_latest_version(url)? {
        cprintln!("<g,u>Wrestic is already up to date!\n");
        pause()?
    } else {
        cprintln!(
            "<y>Wrestic is outdated!\n<r>current: <k>{}\n<g>latest: <k>{}\n",
            get_current_version()?,
            get_latest_version(url)?
        );

        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_message("Updating wrestic...");

        let shell = get_current_shell()?;

        if run_cmd!(
            $shell -c $command;
        )
        .is_err()
        {
            pb.finish_and_clear();
            Err(error!("Failed downloading the latest version of Wrestic."))?;
        }

        which("tar").context(error!("Package: tar - is not installed"))?;
        if run_cmd!(
            tar -zxf /tmp/wrestic.tar.gz -C /usr/bin --overwrite;
            rm -rf /tmp/wrestic.tar.gz;
        )
        .is_err()
        {
            pb.finish_and_clear();
            Err(error!("Failed extracting wrestic into /usr/bin."))?;
        } else {
            pb.finish_and_clear();
            cprintln!("<g,u>Wrestic was successfully updated\n");
        }

        pause()?;
    }

    if !noconfirm {
        selector()?;
    }
    Ok(())
}
