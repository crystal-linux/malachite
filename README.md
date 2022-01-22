<p align="center">
  <a href="https://git.getcryst.al/crystal/ame/">
    <img src="https://git.getcryst.al/crystal/branding/raw/branch/main/logos/crystal-logo-minimal.png" alt="Logo" width="150" height="150">
  </a>
</p>
<h2 align="center">Malachite</h2>
<p align="center">
    <a href="https://discord.gg/yp4xpZeAgW"><img alt="Discord" src="https://img.shields.io/discord/825473796227858482?color=blue&label=Discord&logo=Discord&logoColor=white"?link=https://discord.gg/yp4xpZeAgW&link=https://discord.gg/yp4xpZeAgW></a>
</p>

<p align="center">Malachite is a simple yet useful workspace and local repository management tool, made for packagers of Arch Linux based distributions.</p>

## Basic usage

| Action                                                 | Command               |
|--------------------------------------------------------|-----------------------|
| Build a package                                        | mlc build \<package\> |
| Generate local repository                              | mlc repo-gen          |
| Update local repos/PKGBUILDs                           | mlc pull/update       |
| Create and/or open config file                         | mlc conf              |
| Initialises repo/workspace based on config in mlc.toml | mlc init              |
| Prunes old, duplicate packages from repository         | mlc prune \<days\>    |

## Exit codes overview

| Exit Code (i32) | Reason                                                   |
|-----------------|----------------------------------------------------------|
| 1               | Running ame as UID 0 / root                              |
| 2               | Failed adding package to database                        |
| 3               | Failed initialising database                             |
| 4               | Error creating cache and/or database paths               |
| 5               | Could not find one or more required package dependencies |
| 6               | User cancelled package installation                      |
| 7               | Pacman error when installing package                     |

## How to build:

Tested on latest Cargo (1.60.0-nightly)

<br>

#### Debug/development builds

- `cargo build`

#### Optimised/release builds

- `cargo build --release`

<!--

echo "MLC_UWU=true" >> ~/.zshrc
echo "MLC_UWU=true" >> ~/.bashrc
set -Ux MLC_UWU true

:)

-->