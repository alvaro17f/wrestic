use crate::{
    modules::selector::selector,
    utils::tools::{clear, pause},
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn new_repository(bucket: &str, name: Option<&str>, noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<g>NEW REPOSITORY");
    println!();
    let name = match name {
        Some(name) => name.to_string(),
        None => {
            let mut name = String::new();
            cprintln!("<y>Enter new repository name: ");
            std::io::stdin().read_line(&mut name)?;
            name
        }
    };

    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!(
                "<y>Do you want to create a new repository? (Y/n): "
            ))
            .default(true)
            .interact()?
    {
        clear()?;

        if run_cmd!(
            restic -r b2:$bucket:$name init;
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
