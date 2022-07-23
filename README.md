<p align="center">
  <a href="https://github.com/crystal-linux/Malachite">
    <img src="https://getcryst.al/site/assets/other/logo.png" alt="Logo" width="150" height="150">
  </a>
</p>

<h2 align="center">Malachite</h2>

<p align="center">
    <a href="https://github.com/crystal-linux/.github/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-GPL--3.0-blue.svg" alt="License">
    <a href="https://github/crystal-linux/malachite"><img alt="GitHub isses" src="https://img.shields.io/github/issues-raw/crystal-linux/malachite"></a>
    <a href="https://github/crystal-linux/malachite"><img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr-raw/crystal-linux/malachite"></a><br>
    <a href="https://discord.gg/hYJgu8K5aA"><img alt="Discord" src="https://img.shields.io/discord/825473796227858482?color=blue&label=Discord&logo=Discord&logoColor=white"> </a>
   <a href="https://github.com/ihatethefrench"> <img src="https://img.shields.io/badge/Maintainer-@ihatethefrench-brightgreen" alt="The maintainer of this repository" href="https://github.com/ihatethefrench"></a><br>
    <a href="https://fosstodon.org/@crystal_linux"><img alt="Mastodon Follow" src="https://img.shields.io/mastodon/follow/108618426259408142?domain=https%3A%2F%2Ffosstodon.org">
    <a href="https://twitter.com/crystal_linux"><img alt="Twitter Follow" src="https://img.shields.io/twitter/follow/crystal_linux"></a>
</p>



<p align="center">Malachite is a simple yet useful workspace and local repository management tool, made for packagers of Arch Linux based distributions.</p>

### Basic Usage Guide

| Action                                                 | Command                                   |
|--------------------------------------------------------|-------------------------------------------|
| Build a package                                        | mlc build \<package\> [all if left empty] |
| Generate local repository                              | mlc repo-gen                              |
| Update local repos/PKGBUILDs                           | mlc pull/update [all if left empty]       |
| Create and/or open config file                         | mlc conf                                  |
| Initialises repo/workspace based on config in mlc.toml | mlc clone/init                            |

### Pacman Repository Creation

- `mlc config` to create the config (and also populate it)
- `mlc init` to build repository base from config file
- `mlc build <package>` to either build individual packages, or don't specify package names to build all packages in mlc.toml
  - `build` typically automatically updates the repository unless `--no-regen` is passed, if so: 
- `mlc repo-gen` to generate functional pacman repository at \<name\>/\<name\>.db from built packages


##  \>> [Detailed Usage Guide](docs/GETTING_STARTED.md) <<


## How to build:

Tested on latest Cargo (1.60.0-nightly)

### Debug/development builds

- `cargo build`

### Optimised/release builds

- `cargo build --release`
