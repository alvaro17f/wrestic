use cmd_lib::run_cmd;
use color_print::cprintln;

use crate::utils::utils::{clear, pause, read_input};

pub fn new_repository(bucket: &str) {
    clear();
    cprintln!("<g>NEW REPOSITORY");
    println!();
    cprintln!("<y>Do you want to create a new repository? (Y/n): ");
    if read_input(true) {
        clear();
        // ask user to input new repository name and store it in new_repository variable
        let mut new_repository = String::new();
        cprintln!("<y>Enter new repository name: ");
        let _ = std::io::stdin().read_line(&mut new_repository);

        if run_cmd!(
        restic -r b2:$bucket:$new_repository init;
        )
        .is_err()
        {
            cprintln!("<r>Failed to create a new repository");
        }
        pause();
    }
}
