pub enum AppExitCode {
    RunAsRoot = 1,
    BuildInWorkspace = 2,
    PkgNotFound = 3,
    InvalidMode = 4,
    DirNotEmpty = 5,
    DirNotGit = 6,
    ConfigNotFound = 7,
}