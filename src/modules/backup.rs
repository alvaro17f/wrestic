use crate::{
    modules::repair::repair,
    utils::utils::{clear, pause, read_input},
};
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
        restic -r b2:$bucket:$repository --verbose --verbose backup /home/$user$backup_folder;
    )
    .is_err()
    {
        cprintln!("<r>Failed to backup");
    }
    if run_cmd!(
        restic -r b2:$bucket:$repository --verbose --verbose forget --keep-last $keep_last;
    )
    .is_err()
    {
        cprintln!("<r>Failed to forget keeping last {keep_last} snapshots, let's try to repair: ");
        repair(bucket, repository, true);

        if run_cmd!(
            restic -r b2:$bucket:$repository --verbose --verbose forget --keep-last $keep_last;
        )
        .is_err()
        {
            cprintln!(
                "<r>Houston, we have a problem! Failed to forget keeping last {keep_last} snapshots AGAIN."
            );
        }
    }

    if !noconfirm {
        pause();
    }
}
