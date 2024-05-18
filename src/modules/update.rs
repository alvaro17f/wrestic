use crate::macros::errors::error;
use crate::utils::{
    get_current_shell::get_current_shell,
    root_checker::root_checker,
    tools::{clear, pause},
};
use anyhow::{Context, Result};
use cmd_lib::run_cmd;
use color_print::cprintln;
use flate2::read::GzDecoder;
use indicatif::ProgressBar;
use std::{
    env::current_exe,
    fs::{remove_file, File},
    io::{BufReader, ErrorKind},
    os::unix::process::CommandExt,
    path::Path,
    process::Command,
    time::Duration,
};
use tar::Archive;

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
        .context(error!("Failed fetching the latest version of wrestic."))?;
    let version_string = String::from_utf8_lossy(&output.stdout);
    let version = version_string.trim().to_string();
    if version.is_empty() {
        Err(error!(
            "Failed fetching the latest version of wrestic. Try again later."
        ))?;
    }
    Ok(version)
}

fn extract_wrestic(file_path: &str, extract_path: &str) -> Result<()> {
    let file = File::open(file_path)?;
    let gz = GzDecoder::new(file);
    let tar = BufReader::new(gz);
    let mut archive = Archive::new(tar);
    let extract_path_parent = Path::new(extract_path).parent().unwrap().to_owned();

    match archive.unpack(&extract_path_parent) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                Command::new("sudo")
                    .arg("-E")
                    .arg("tar")
                    .arg("-xvf")
                    .arg(file_path)
                    .arg("-C")
                    .arg(extract_path_parent.to_str().unwrap())
                    .exec();
                Ok(())
            } else {
                Err(e.into())
            }
        }
    }
}

fn download_latest_version(shell: &str, url: &str, tmp_path: &str) -> Result<()> {
    let command = format!(
        r#"curl -sL $(curl -s "{url}" | grep browser_download_url | cut -d '"' -f 4) -o {tmp_path}"#
    );

    let command_result = run_cmd!(
        $shell -c $command 2>/dev/null;
    );

    if let Err(e) = command_result {
        if format!("{}", e).contains("Permission denied") {
            root_checker()?;

            if run_cmd!(
                sudo -E $shell -c $command 2>/dev/null;
            )
            .is_err()
            {
                Err(error!("Failed downloading the latest version of wrestic"))?;
            }
        } else {
            Err(error!("Failed downloading the latest version of wrestic"))?;
        }
    }

    Ok(())
}

fn remove_file_with_permission_check(file_path: &str) -> Result<()> {
    if let Err(e) = remove_file(file_path) {
        if e.kind() == ErrorKind::PermissionDenied {
            let output = Command::new("sudo")
                .arg("-E")
                .arg("rm")
                .arg(file_path)
                .output()
                .expect("Failed to execute command");

            if !output.status.success() {
                return Err(error!("Failed removing file"));
            }
        } else {
            return Err(error!("Failed removing file"));
        }
    }

    Ok(())
}

pub fn update() -> Result<()> {
    #[cfg(not(feature = "no-self-update"))]
    const SELF_UPDATE: bool = true;
    #[cfg(feature = "no-self-update")]
    const SELF_UPDATE: bool = false;

    if SELF_UPDATE {
        clear()?;
        cprintln!("<c,u,s>UPDATER");
        println!();

        let current_executable = &current_exe()?;
        let bin_path = current_executable.to_str().unwrap();
        let tmp_path = "/tmp/wrestic.tar.gz";
        let url = "https://api.github.com/repos/alvaro17f/wrestic/releases/latest";

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

            download_latest_version(&shell, url, tmp_path)?;

            pb.finish_and_clear();

            remove_file_with_permission_check(bin_path)?;

            pb.finish_and_clear();

            if extract_wrestic(tmp_path, bin_path).is_err() {
                Err(error!("Failed extracting the latest version of wrestic"))?;
            }

            pb.finish_and_clear();

            cprintln!(
                "<g,u>Wrestic has been updated to version <k>{}<g,u>!",
                get_latest_version(url)?
            );
        }

        Ok(())
    } else {
        Err(error!("Self-update is disabled for this build. You should probably use your system package manager to update"))
    }
}
