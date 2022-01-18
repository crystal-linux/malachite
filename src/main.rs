use clap::{App, AppSettings, Arg, ArgSettings, SubCommand};

fn main() {
    fn build_app() -> App<'static, 'static> {
        let app = App::new("Malachite")
            .version(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .multiple(true)
                    .set(ArgSettings::Global)
                    .help("Sets the level of verbosity")
            )
            .arg(
                Arg::with_name("exclude")
                    .short("e")
                    .long("exclude")
                    .multiple(true)
                    .set(ArgSettings::Global)
                    .help("Excludes packages from given operation")
            )
            .arg(
                Arg::with_name("all")
                    .long("all")
                    .set(ArgSettings::Global)
                    .help("Operates on every possible package")
            )
            .subcommand(
                SubCommand::with_name("build")
                    .about("Builds the given packages")
                    .arg(
                        Arg::with_name("package(s)")
                            .help("The packages to operate on")
                            .required(true)
                            .multiple(true)
                            .index(1),
                    )
            )
            .subcommand(
                SubCommand::with_name("prune")
                    .about("Prunes duplicate packages older than X days from the repository")
                    .arg(
                        Arg::with_name("days")
                            .help("How old a duplicate package needs to be (in days) to be pruned")
                            .required(true)
                            .index(1)
                    )
            )
            .settings(&[
                AppSettings::GlobalVersion,
                AppSettings::VersionlessSubcommands,
                AppSettings::ArgRequiredElseHelp,
                AppSettings::InferSubcommands
            ]);
        app
    }

    let matches = build_app().get_matches();

}
