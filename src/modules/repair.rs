use cmd_lib::run_cmd;
use color_print::cprintln;

use crate::utils::utils::{clear, pause, read_input};

pub fn repair(bucket: &str, repository: &str, noconfirm: bool) {
    clear();
    cprintln!("<g>REPAIR");
    println!();

    if !noconfirm {
        cprintln!("<y>Do you want to repair your repository? (Y/n): ");
        if !read_input(true) {
            return;
        }
    }

    if read_input(true) {
        if run_cmd!(
            restic -r b2:$bucket:$repository unlock;
            restic -r b2:$bucket:$repository rebuild-index;
            restic -r b2:$bucket:$repository prune;
            restic -r b2:$bucket:$repository check;
        )
        .is_err()
        {
            cprintln!("<r>Failed to repair");
        }

        if !noconfirm {
            pause();
        }
    }
}
