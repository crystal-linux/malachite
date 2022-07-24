pub enum AppExitCode {
    RunAsRoot = 1,
    BuildInWorkspace = 2,
    PkgNotFound = 3,
    InvalidMode = 4,
    DirNotEmpty = 5,
    ConfigNotFound = 6,
    NoPkgs = 7,
    ConfigParseError = 8,
    InvalidRepo = 9,
    NotInit = 10,
    NotClean = 11,
}
