use crate::{
    utils::macros::error,
    utils::{
        get_current_shell::get_current_shell,
        tools::{clear, pause},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::process::exit;
use which::which;

pub fn restic_checker() -> Result<()> {
    let url = "https://api.github.com/repos/restic/restic/releases/latest";
    let command = format!(
        r#"curl -sL $(curl -s "{url}" | grep browser_download_url | grep linux_amd64 | cut -d '"' -f 4) -o /tmp/restic.bz2 && bunzip2 /tmp/restic.bz2 && chmod +x /tmp/restic && mv -f /tmp/restic /usr/bin/restic"#
    );
    match which("restic") {
        Ok(_) => Ok(()),
        Err(_) => {
            clear()?;
            cprintln!("<c,u,s>RESTIC");
            println!();
            cprintln!("<r>Restic not found\n");
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(cformat!("<y>Would you like to install Restic? (Y/n):"))
                .default(true)
                .interact()?
            {
                let shell = get_current_shell()?;
                if run_cmd!(
                    $shell -c $command;
                )
                .is_err()
                {
                    Err(error!("Failed to install Restic"))?
                } else {
                    cprintln!("<g,u>Restic installed successfully!");
                    pause()?;
                    Ok(())
                }
            } else {
                exit(0);
            }
        }
    }
}
