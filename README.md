# WRESTIC

:star: Star me up!

![wrestic](misc/wrestic.png)

Wrestic is a backup tool built in Rust that provides a wrapper around Restic, a popular backup program. With Wrestic, you can easily configure and run backups of your files and directories, and take advantage of Restic's powerful features such as deduplication, encryption, and compression. Whether you need to back up your personal files or your organization's data, Wrestic can help you automate the process and ensure your data is safe and secure.

> ⚠️ currently only works with Backblaze B2

## TABLE OF CONTENTS[![](https://raw.githubusercontent.com/aregtech/areg-sdk/master/docs/img/pin.svg)](#table-of-contents)
- [WRESTIC](#wrestic)
  - [TABLE OF CONTENTS](#table-of-contents)
  - [INSTALLATION](#installation)
    - [DOWNLOAD BINARY](#download-binary)
    - [BUILD FROM SOURCE](#build-from-source)
  - [CONFIGURATION](#configuration)
  - [USAGE](#usage)
  - [COMPLETIONS](#completions)



## INSTALLATION

### DOWNLOAD BINARY

```sh
curl -sL $(curl -s https://api.github.com/repos/alvaro17f/wrestic/releases/latest | grep browser_download_url | cut -d '"' -f 4) | sudo tar zxf - -C /usr/bin --overwrite
```

### BUILD FROM SOURCE
Requirements:
- [git](https://git-scm.com/)
- [rust](https://rust-lang.org/)

```sh
git clone https://github.com/alvaro17f/wrestic.git
cd wrestic
cargo build --release
sudo cp target/release/wrestic /usr/bin
```

## CONFIGURATION

Copy `wrestic.toml` to `/home/$USER/.config/wrestic/wrestic.toml` and modify the content for your needs.

## USAGE

Simply run `sudo wrestic`.

```sh
$ wrestic help

Restic wrapper built in Rust

Usage: wrestic [OPTIONS] [COMMAND]

Commands:
  backup     Make a backup of all your repositories
  restore    Restore a snapshot
  snapshots  List snapshots
  forget     Delete a snapshot
  init       Initialize all of your repositories
  check      Check repository health
  repair     Fix any issue
  cache      Clean cache
  update     Update Wrestic
  help       Print this message or the help of the given subcommand(s)

Options:
      --generate <GENERATOR>  [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help                  Print help
  -V, --version               Print version

```

## COMPLETIONS

To get `<TAB>` completions run `sudo wrestic --generate <your shell>`