use crate::{
    modules::selector::selector,
    utils::tools::{clear, pause},
};
use anyhow::Result;
use color_print::cprintln;
use std::process::Command;

pub fn update(noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<g>UPDATER");
    println!();
    let output = Command::new("sh")
        .arg("-c")
        .arg(r#"curl -sL $(curl -s https://api.github.com/repos/alvaro17f/wrestic/releases/latest | grep browser_download_url | cut -d '"' -f 4) | sudo tar zxf - -C /usr/bin --overwrite"#)
        .output()?;

    if output.status.success() {
        cprintln!("<g>Wrestic updated successfully!");
    } else {
        cprintln!("<r>Command failed with status: <k>{}", output.status);
    }
    pause()?;

    if !noconfirm {
        selector()?;
    }
    Ok(())
}
