use crate::{
    modules::{repair::repair, selector::selector},
    utils::tools::{clear, pause},
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use regex::Regex;
use std::process::Command;

pub fn forget(
    bucket: &str,
    repository: &str,
    delete_snapshots: Option<&[String]>,
    noconfirm: bool,
) -> Result<()> {
    clear()?;
    cprintln!("<g>DELETE");
    println!();
    let delete_snapshots = match delete_snapshots {
        Some(snapshots) => snapshots.join(" "),
        None => {
            let restic = Command::new("restic")
                .arg("-r")
                .arg(format!("b2:{}:{}", bucket, repository))
                .arg("--verbose")
                .arg("--verbose")
                .arg("snapshots")
                .output()?;
            let restic = String::from_utf8(restic.stdout)?;

            let selections = Regex::new(r"(\w+)\s+(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})")?
                .captures_iter(&restic)
                .map(|cap| format!("[{}] - {}", &cap[1], &cap[2]))
                .collect::<Vec<String>>();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(cformat!("<g>Snapshots:"))
                .default(0)
                .max_length(10)
                .items(&selections[..])
                .interact()?;

            let selection = Regex::new(r"(\w+)\s+(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})")?
                .captures_iter(&restic)
                .map(|cap| format!("{}", &cap[1]))
                .collect::<Vec<String>>()[selection]
                .clone();

            selection.to_string()
        }
    };

    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Do you want to delete snapshots? (Y/n): "))
            .default(true)
            .interact()?
    {
        if run_cmd!(

            restic -r b2:$bucket:$repository forget $delete_snapshots;
        )
        .is_err()
        {
            cprintln!("<r>Failed to delete snapshots! Let's try to repair:");
            repair(bucket, repository, true)?;
            if run_cmd!(
                restic -r b2:$bucket:$repository forget $delete_snapshots;
            )
            .is_err()
            {
                cprintln!("<r>Houston, we have a problem! Failed to delete the snapshot AGAIN.");
            }
        }
        if !noconfirm {
            pause()?;
            selector()?;
        }
    } else {
        selector()?;
    }
    Ok(())
}
