mod macros;
mod modules;
mod utils;

use anyhow::{Ok, Result};
use modules::selector::selector;
use utils::root_checker::is_root;

fn main() -> Result<()> {
    is_root()?;
    selector()?;

    Ok(())
}
