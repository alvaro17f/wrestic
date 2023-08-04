use crate::utils::utils::{clear, pause, read_input};
use cmd_lib::run_cmd;
use color_print::cprintln;

pub fn backup(
    user: &str,
    bucket: &str,
    repository: &str,
    keep_last: &str,
    backup_folder: &str,
    noconfirm: bool,
) {
    clear();
    cprintln!("<g>BACKUP");
    println!();
    if !noconfirm {
        cprintln!("<y>Do you want to perform a backup? (Y/n): ");
        if !read_input(true) {
            return;
        }
    }
    if run_cmd!(
        restic -r b2:$bucket:$repository --verbose --verbose backup /home/$user$backup_folder
        restic -r b2:$bucket:$repository --verbose --verbose forget --keep-last $keep_last
    )
    .is_err()
    {
        cprintln!("<r>Failed to backup");
    }
    if !noconfirm {
        pause();
    }
}
