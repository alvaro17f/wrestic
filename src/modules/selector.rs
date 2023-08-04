use crate::{
    modules::{
        backup::backup, cache::cache, check::check, new_repository::new_repository, repair::repair,
        restore::restore, snapshots::snapshots,
    },
    utils::{get_env::dotenv, utils::clear},
};
use anyhow::Result;
use color_print::cprintln;
use std::{io::stdin, process::exit};

pub fn selector() -> Result<()> {
    clear();
    let env = dotenv()?;
    println!();
    cprintln!("<g>RESTIC");
    println!();
    println!("1. Backup");
    println!("2. Restore");
    println!("3. Snapshots");
    println!("4. Check");
    println!("5. Repair");
    println!("6. Cache");
    println!("7. New Repository");
    println!("0. Exit");
    println!();
    cprintln!("<y>Enter your choice: ");
    loop {
        let prompt: &mut String = &mut String::new();
        stdin().read_line(prompt).expect("failed to read line");
        match prompt.as_str() {
            "1\n" => {
                backup(
                    &env.user,
                    &env.bucket,
                    &env.repository,
                    &env.keep_last,
                    &env.backup_folder,
                );
                selector()?
            }
            "2\n" => {
                restore(&env.user, &env.bucket, &env.repository, &env.restore_folder);
                selector()?
            }
            "3\n" => {
                snapshots(&env.bucket, &env.repository);
                selector()?
            }
            "4\n" => {
                check(&env.bucket, &env.repository);
                selector()?
            }
            "5\n" => {
                repair(&env.bucket, &env.repository);
                selector()?
            }
            "6\n" => {
                cache();
                selector()?
            }
            "7\n" => {
                new_repository(&env.bucket);
                selector()?
            }
            "0\n" => exit(0),
            _ => {
                clear();
                println!();
                cprintln!("<g>RESTIC");
                println!();
                println!("1. Backup");
                println!("2. Restore");
                println!("3. Snapshots");
                println!("4. Check");
                println!("5. Repair");
                println!("6. Cache");
                println!("7. New Repository");
                println!("0. Exit");
                println!();
                cprintln!("<y>Enter your choice: ");
            }
        }
    }
}

// docker_down(&user)?;
// docker_up(&user)?;
