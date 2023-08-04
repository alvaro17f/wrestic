use cmd_lib::run_cmd;
use color_print::cprintln;

use crate::utils::utils::{clear, pause, read_input};

pub fn snapshots(bucket: &str, repository: &str) {
    clear();
    cprintln!("<g>BACKUP");
    println!();
    cprintln!("<y>Do you want to list your snapshots? (Y/n): ");
    if read_input(true) {
        if run_cmd!(
            restic -r b2:$bucket:$repository snapshots
        )
        .is_err()
        {
            cprintln!("<r>Failed to list snapshots");
        }

        pause();
    }
}
