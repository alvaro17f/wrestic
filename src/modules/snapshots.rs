use std::time::Duration;

use crate::{
    modules::selector::selector,
    utils::tools::{clear, pause},
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};
use indicatif::ProgressBar;
use std::process::Command;

pub fn snapshots(bucket: &str, repository: &str, noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<g>SNAPSHOTS");
    println!();
    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Do you want to list your snapshots? (Y/n): "))
            .default(true)
            .interact()?
    {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_message("Loading snapshots...");

        let out = Command::new("restic")
            .arg("-r")
            .arg(format!("b2:{}:{}", &bucket, &repository))
            .arg("--verbose")
            .arg("--verbose")
            .arg("snapshots")
            .output()?;

        pb.finish_and_clear();

        println!("{}", String::from_utf8(out.stdout)?);

        if !noconfirm {
            pause()?;
            selector()?;
        }
    } else {
        selector()?;
    }
    Ok(())
}
