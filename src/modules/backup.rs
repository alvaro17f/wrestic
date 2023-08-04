use cmd_lib::run_cmd;
use color_print::cprintln;

use crate::utils::utils::{pause, read_input, clear};

pub fn backup(user: &str, bucket: &str, repository: &str, keep_last: &str, backup_folder: &str) {
    clear();
    cprintln!("<g>BACKUP");
    println!();
    cprintln!("<y>Do you want to perform a backup? (Y/n): ");
    if read_input(true) {
        if run_cmd!(
            restic -r b2:$bucket:$repository --verbose --verbose backup /home/$user$backup_folder
            restic -r b2:$bucket:$repository --verbose --verbose forget --keep-last $keep_last
        )
        .is_err()
        {
            cprintln!("<r>Failed to backup");
        }
        pause();
    }
}
