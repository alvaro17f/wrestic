use cmd_lib::run_cmd;
use color_print::cprintln;

use crate::utils::utils::{clear, pause, read_input};

pub fn cache() {
    clear();
    cprintln!("<g>CACHE");
    println!();
    cprintln!("<y>Do you want to clean cache? (Y/n): ");
    if read_input(true) {
        if run_cmd!(
            restic cache --cleanup
        )
        .is_err()
        {
            cprintln!("<r>Failed to clean cache");
        }
        pause();
    }
}
