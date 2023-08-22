use crate::{
    modules::selector::selector,
    utils::tools::{clear, pause},
};
use anyhow::Result;
use color_print::cprintln;
use indicatif::ProgressBar;
use std::{process::Command, time::Duration};

pub fn update(noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<c,u,s>UPDATER");
    println!();

    if get_installed_version()? >= get_latest_version()? {
        cprintln!("<g>Wrestic is already up to date!");
        pause()?
    } else {
        cprintln!(
            "<y>Wrestic is outdated!\n<r>current: <k>{}<g>latest: <k>{}\n",
            get_installed_version()?,
            get_latest_version()?
        );

        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_message("Updating wrestic...");

        let output = Command::new("sh")
        .arg("-c")
        .arg(r#"curl -sL $(curl -s https://api.github.com/repos/alvaro17f/wrestic/releases/latest | grep browser_download_url | cut -d '"' -f 4) | sudo tar zxf - -C /usr/bin --overwrite"#)
        .output()?;

        pb.finish_and_clear();

        if output.status.success() {
            cprintln!("<g,u>Wrestic updated successfully!\n");
        } else {
            cprintln!("<r>Command failed with status: <k>{}", output.status);
            cprintln!("<k>{}", String::from_utf8_lossy(&output.stderr));
        }
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
fn get_latest_version() -> std::io::Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(r#"curl -s https://api.github.com/repos/alvaro17f/wrestic/releases/latest | grep tag_name | grep -Eo '[0-9.]+'"#)
        .output()?;
    let version_string = String::from_utf8_lossy(&output.stdout);
    let version = version_string.trim().to_string();
    Ok(version)
}
