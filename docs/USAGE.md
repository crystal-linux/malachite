# Detailed Usage
Work it harder, make it better! 

### Global Flags

| Flag              | Description                                                                                                                            |
|-------------------|----------------------------------------------------------------------------------------------------------------------------------------|
| `--verbose`, `-v` | Prints lots of debug information to `stderr`. If something doesn't go right, sending us the output with this enabled will help greatly |
| `--exclude`, `-x` | Excludes the supplied package from the current operation. Can be used multiple times.                                                  |

### Basic Commands

| Action                                                                                  | Command                                   | Extra Flags                                                                                                      |
|-----------------------------------------------------------------------------------------|-------------------------------------------|------------------------------------------------------------------------------------------------------------------|
| Build a package/packages.                                                               | `mlc build <package>` [all if left empty] | `--no-regen`: Doesn't regenerate repository after build                                                          |
| Generate pacman repository                                                              | `mlc repo-gen`                            |                                                                                                                  |
| Update local repos/PKGBUILDs                                                            | `mlc pull/update` [all if left empty]     | `--no-regen`: If `mode.repository.build_on_update` is `true`, Do not regenerate repository after package rebuild |
| Create and/or open config file                                                          | `mlc conf`                                |                                                                                                                  |
| Initialises repo/workspace based on config in mlc.toml                                  | `mlc clone/init`                          |                                                                                                                  |
| Displays an info panel/overview of the current repo                                     | `mlc info/status`                         |                                                                                                                  |
| Resets Malachite repository by deleting all directories, omitting `mlc.toml` and `.git` | `mlc clean/reset`                         | `--force`: Remove dirty directories (unstaged, untracked, etc)                                                   |

### Exit Codes

| AppExitCode (named Enum) | Exit code (i32) | Error Description                                                                                      |
|--------------------------|-----------------|--------------------------------------------------------------------------------------------------------|
| `RunAsRoot`              | `1`             | Malachite was run as root. This is highly discouraged. So highly, in fact, that it will refuse to run. |
| `PkgsNotFound`           | `2`             | No packages were specified/found for the desired operation                                             |
| `DirNotEmpty`            | `3`             | The creation of a Malachite repository was attempted in a non-empty directory                          |
| `ConfigParseError`       | `4`             | The config file could not be parsed                                                                    |
| `RepoParseError`         | `5`             | The repository info could not be parsed                                                                |
| `RepoNotClean`           | `6`             | The git repository is not clean and cannot be removed without `--force`                                |
