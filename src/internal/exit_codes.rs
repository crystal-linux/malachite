pub enum AppExitCode {
    #[cfg(target_os = "linux")]
    RunAsRoot = 1,
    PkgsNotFound = 2,
    DirNotEmpty = 3,
    ConfigParseError = 4,
    RepoParseError = 5,
    RepoNotClean = 6,
}
