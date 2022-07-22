pub enum AppExitCode {
    RunAsRoot = 1,
    BuildInWorkspace = 2,
    PkgNotFound = 3,
    InvalidMode = 4,
    DirNotEmpty = 5,
    RepoNotFound = 6,
    ConfigNotFound = 7,
    NoPkgs = 8,
    ConfigParseError = 9,
}
