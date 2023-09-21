use clap::Command;
use clap_complete::{generate, Generator};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};
pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

pub fn set_completions<G: Generator + std::fmt::Debug + Copy>(gen: G, cmd: &mut Command) {
    let mut output = Vec::new();

    generate(gen, cmd, cmd.get_name().to_string(), &mut output);

    let shell = format!("{:#?}", gen);

    let file_path = if shell.to_lowercase().contains("zsh") {
        "/usr/local/share/zsh/site-functions/_wrestic"
    } else if shell.to_lowercase().contains("bash") {
        "/etc/bash_completion.d/wrestic"
    } else {
        panic!("{:?}", "Shell not supported")
    };

    let path = Path::new(file_path);

    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        File::create(path).unwrap();
    }

    if let Ok(mut file) = File::create(path) {
        file.write_all(&output).unwrap();
    } else {
        io::stdout().write_all(&output).unwrap();
    }
}
