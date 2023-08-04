use cmd_lib::run_cmd;
use color_print::cprintln;

use crate::utils::utils::{clear, pause, read_input};

pub fn check(bucket: &str, repository: &str) {
    clear();
    cprintln!("<g>CHECK");
    println!();
    cprintln!("<y>Do you want to check if your repo is working fine? (Y/n): ");
    if read_input(true) {
        if run_cmd!(
            restic -r b2:$bucket:$repository check;
        )
        .is_err()
        {
            cprintln!("<r>Failed to check");
        }
        pause();
    }
}
