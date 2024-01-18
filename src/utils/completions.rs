#![allow(clippy::single_char_pattern)]
use crate::utils::get_user::get_user;
use crate::utils::macros::error;
use anyhow::Result;
use clap_complete::{generate, Generator};
use std::io::ErrorKind;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

fn create_completions_file(file_path: &str, output: &[u8]) -> Result<()> {
    let path = Path::new(file_path);

    if !path.exists() {
        if let Some(parent) = path.parent() {
            match fs::create_dir_all(parent) {
                Ok(_) => {}
                Err(e) if e.kind() == ErrorKind::PermissionDenied => {
                    let status = Command::new("sudo")
                        .arg("mkdir")
                        .arg("-p")
                        .arg(parent.to_str().unwrap())
                        .status()?;

                    if !status.success() {
                        return Err(error!(
                            "Failed to create directory. Please run this program with 'sudo'."
                        ));
                    }
                }
                Err(_) => return Err(error!("Failed to create directory.")),
            }
        }

        match File::create(path) {
            Ok(_) => {}
            Err(e) if e.kind() == ErrorKind::PermissionDenied => {
                let status = Command::new("sudo")
                    .arg("touch")
                    .arg(path.to_str().unwrap())
                    .status()?;

                if !status.success() {
                    return Err(error!(
                        "Failed to create file. Please run this program with 'sudo'."
                    ));
                }
            }
            Err(_) => return Err(error!("Failed to create file.")),
        }
    }

    match fs::write(path, output) {
        Ok(_) => {}
        Err(e) if e.kind() == ErrorKind::PermissionDenied => {
            let mut child = Command::new("sudo")
                .arg("sh")
                .arg("-c")
                .arg(format!(
                    "echo '{}' > {}",
                    String::from_utf8_lossy(output).replace("'", "'\\''"),
                    path.to_str().unwrap()
                ))
                .stdin(Stdio::piped())
                .spawn()?;

            child.stdin.as_mut().unwrap().write_all(output)?;

            let status = child.wait()?;

            if !status.success() {
                return Err(error!(
                    "Failed to write to file. Please run this program with 'sudo'."
                ));
            }
        }
        Err(_) => return Err(error!("Failed to write to file.")),
    }

    Ok(())
}

pub fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

pub fn set_completions<G: Generator + std::fmt::Debug + Copy>(
    gen: G,
    cmd: &mut clap::Command,
) -> Result<()> {
    let mut output = Vec::new();

    generate(gen, cmd, cmd.get_name().to_string(), &mut output);

    let shell = format!("{:#?}", gen);
    let user = get_user;

    let file_path = if shell.to_lowercase().contains("bash") {
        format!(
            "/home/{}/.local/share/bash-completion/completions/wrestic",
            user()?
        )
    } else if shell.to_lowercase().contains("zsh") {
        "/usr/local/share/zsh/site-functions/_wrestic".to_string()
    } else if shell.to_lowercase().contains("fish") {
        format!("/home/{}/.config/fish/completions/wrestic.fish", user()?)
    } else {
        panic!("{:?}", "Shell not supported")
    };

    create_completions_file(&file_path, &output)
}
