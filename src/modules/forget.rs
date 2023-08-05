use cmd_lib::run_cmd;
use color_print::cprintln;

use crate::utils::tools::{clear, pause, read_input};

pub fn forget(
    bucket: &str,
    repository: &str,
    delete_snapshots: &Vec<String>,
    noconfirm: bool,
) {
    clear();
    let delete_snapshots = delete_snapshots.join(" ");
    cprintln!("<g>DELETE");
    println!();
    if !noconfirm {
        cprintln!("<y>Do you want to delete snapshots? (Y/n): ");
        if !read_input(true) {
            return;
        }
    }
    if run_cmd!(
        restic -r b2:$bucket:$repository --verbose --verbose forget $delete_snapshots;
    )
    .is_err()
    {
        cprintln!("<r>Failed to check");
    }
    if !noconfirm {
        pause();
    }
}
