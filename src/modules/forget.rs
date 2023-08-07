use crate::{
    modules::selector::selector,
    utils::tools::{clear, pause},
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn forget(
    bucket: &str,
    repository: &str,
    delete_snapshots: &[String],
    noconfirm: bool,
) -> Result<()> {
    clear()?;
    let delete_snapshots = delete_snapshots.join(" ");
    cprintln!("<g>DELETE");
    println!();
    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Do you want to delete snapshots? (Y/n): "))
            .default(true)
            .interact()?
    {
        if run_cmd!(
            restic -r b2:$bucket:$repository --verbose --verbose forget $delete_snapshots;
        )
        .is_err()
        {
            cprintln!("<r>Failed to check");
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
