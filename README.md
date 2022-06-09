<p align="center">
  <a href="https://git.getcryst.al/crystal/ame/">
    <img src="https://via.placeholder.com/15/A900FF/000000?text=+" alt="Logo" width="150" height="150">
  </a>
</p>
<h2 align="center">Malachite</h2>
<p align="center">
    <a href="https://discord.gg/yp4xpZeAgW"><img alt="Discord" src="https://img.shields.io/discord/825473796227858482?color=blue&label=Discord&logo=Discord&logoColor=white"?link=https://discord.gg/yp4xpZeAgW&link=https://discord.gg/yp4xpZeAgW></a>
</p>

<p align="center">Malachite is a simple yet useful workspace and local repository management tool, made for packagers of Arch Linux based distributions.</p>

## Basic usage

| Action                                                 | Command      |
|--------------------------------------------------------|--------------|
| Build a package                                        | mlc build \<package\> |
| Generate local repository                              | mlc repo-gen |
| Update local repos/PKGBUILDs                           | mlc pull/update |
| Create and/or open config file                         | mlc conf     |
| Initialises repo/workspace based on config in mlc.toml | mlc init     |
| Prunes old duplicate packages from repository          | mlc prune    |

### Pacman Repository Creation

- `mlc config` to create the config (and also populate it)
- `mlc init` to build repository base from config file
- `mlc build <package>/--all` to either build individual packages or all packages in mlc.toml
- `mlc repo-gen` to generate functional pacman repository at \<name\>/\<name\>.db from built packages

## How to build:

Tested on latest Cargo (1.60.0-nightly)

<br>

#### Debug/development builds

- `cargo build`

#### Optimised/release builds

- `cargo build --release`
