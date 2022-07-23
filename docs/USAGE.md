# Detailed Usage
Work it harder, make it better! 

### Global Flags

| Flag              | Description                                                                                                                            |
|-------------------|----------------------------------------------------------------------------------------------------------------------------------------|
| `--verbose`, `-v` | Prints lots of debug information to `stderr`. If something doesn't go right, sending us the output with this enabled will help greatly |
| `--exclude`, `-x` | Excludes the supplied package from the current operation. Can be used multiple times.                                                  |

### Basic Commands

| Action                                                 | Command                                   | Extra Flags                                                                                                      |
|--------------------------------------------------------|-------------------------------------------|------------------------------------------------------------------------------------------------------------------|
| Build a package/packages.                              | `mlc build <package>` [all if left empty] | `--no-regen`: Doesn't regenerate repository after build                                                          |
| Generate pacman repository                             | `mlc repo-gen`                            |                                                                                                                  |
| Update local repos/PKGBUILDs                           | `mlc pull/update` [all if left empty]     | `--no-regen`: If `mode.repository.build_on_update` is `true`, Do not regenerate repository after package rebuild |
| Create and/or open config file                         | `mlc conf`                                |                                                                                                                  |
| Initialises repo/workspace based on config in mlc.toml | `mlc clone/init`                          |                                                                                                                  |

### Exit Codes

| AppExitCode (named Enum) | Exit code (i32) | Error Description                                                                                      |
|--------------------------|-----------------|--------------------------------------------------------------------------------------------------------|
| `RunAsRoot`              | `1`             | Malachite was ran as root. This is highly discouraged. So highly, in fact, that it will refuse to run. |
| `BuildInWorkspace`       | `2`             | Malachite was ran in Workspace mode, but a Repository-mode-specific operation was supplied             |
| `PkgNotFound`            | `3`             | A build was attempted for a package that does not exist                                                |
| `InvalidMode`            | `4`             | Malachite was launched in a mode other than `workspace` or `repository`                                |
| `DirNotEmpty`            | `5`             | The creation of a Malachite repository was attempted in a non-empty directory                          |
| `ConfigNotFound`         | `6`             | The default config file (`./mlc.toml`) was not found                                                   |
| `NoPkgs`                 | `7`             | Somehow, no packages were supplied to/found in the relevant operation                                  |
| `ConfigParseError`       | `8`             | The config file could not be parsed                                                                    |
| `InvalidRepo`            | `9`             | The generated repository somehow contains no packages                                                  |




