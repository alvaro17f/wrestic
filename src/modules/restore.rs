use cmd_lib::run_cmd;
use color_print::cprintln;

use crate::utils::tools::{clear, pause, read_input};

pub fn restore(user: &str, bucket: &str, repository: &str, restore_folder: &str) {
    clear();
    cprintln!("<g>BACKUP");
    println!();
    cprintln!("<y>Do you want to perform a backup? (Y/n): ");
    if read_input(true) {
        if run_cmd!(
            restic -r b2:$bucket:$repository restore latest --target /home/$user$restore_folder
        )
        .is_err()
        {
            cprintln!("<r>Failed to restore");
        }
        pause();
    }
}
