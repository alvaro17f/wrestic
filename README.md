# WRESTIC

:star: Star me up!

Wrestic is a backup tool built in Rust that provides a wrapper around Restic, a popular backup program. With Wrestic, you can easily configure and run backups of your files and directories, and take advantage of Restic's powerful features such as deduplication, encryption, and compression. Whether you need to back up your personal files or your organization's data, Wrestic can help you automate the process and ensure your data is safe and secure.


## Table of contents[![](https://raw.githubusercontent.com/aregtech/areg-sdk/master/docs/img/pin.svg)](#table-of-contents)
- [WRESTIC](#wrestic)
  - [Table of contents](#table-of-contents)
  - [INSTALLATION](#installation)
    - [BUILD FROM SOURCE](#build-from-source)
    - [DOWNLOAD BINARY](#download-binary)
  - [CONFIGURATION](#configuration)
  - [USAGE](#usage)



## INSTALLATION

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
### DOWNLOAD BINARY

```sh
curl -sL $(curl -s https://api.github.com/repos/alvaro17f/wrestic/releases/latest | grep browser_download_url | cut -d '"' -f 4) | sudo tar zxf - -C /usr/bin --overwrite
```

## CONFIGURATION

Simply copy `.env.example` to `/home/$USER/.config/wrestic/.env` and modify the content for your needs.

## USAGE

```sh
$ wrestic help

Restic wrapper in Rust

Usage: wrestic [COMMAND]

Commands:
  backup     Make a backup
  restore    Restore a snapshot
  snapshots  List all snapshots
  check      Check repository health
  repair     Fix any issue
  cache      Clean cache
  new        Create a new repository
  forget     Delete snapshots
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```
