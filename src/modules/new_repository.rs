use crate::{
    modules::selector::selector,
    utils::tools::{clear, pause},
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn new_repository(bucket: &str, noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<g>NEW REPOSITORY");
    println!();
    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!(
                "<y>Do you want to create a new repository? (Y/n): "
            ))
            .default(true)
            .interact()?
    {
        clear()?;
        // ask user to input new repository name and store it in new_repository variable
        let mut new_repository = String::new();
        cprintln!("<y>Enter new repository name: ");
        std::io::stdin().read_line(&mut new_repository)?;

        if run_cmd!(
        restic -r b2:$bucket:$new_repository init;
        )
        .is_err()
        {
            cprintln!("<r>Failed to create a new repository");
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
