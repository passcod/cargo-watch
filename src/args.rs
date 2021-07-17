use clap::{App, AppSettings, Arg, ArgMatches, ErrorKind, SubCommand};
use std::{env, process};

pub fn parse() -> ArgMatches<'static> {
    let footnote = "You can use the `-- command` style instead, note you'll need to use full commands, it won't prefix `cargo` for you.\n\nBy default, your entire project is watched, except for the target/ and .git/ folders, and your .ignore and .gitignore files are used to filter paths.".to_owned();

    #[cfg(windows)] let footnote = format!("{}\n\nOn Windows, patterns given to -i have forward slashes (/) automatically converted to backward ones (\\) to ease command portability.", footnote);

    let mut app = App::new(env!("CARGO_PKG_NAME"))
        .bin_name("cargo")
        .version(env!("CARGO_PKG_VERSION"))
        .help_message("")
        .version_message("")
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::DontCollapseArgsInUsage)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::StrictUtf8)
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("watch")
                .author(env!("CARGO_PKG_HOMEPAGE"))
                .about(env!("CARGO_PKG_DESCRIPTION"))
                .usage("cargo watch [FLAGS] [OPTIONS]")
                .help_message("Display this message")
                .version_message("Display version information")
                .arg(
                    Arg::with_name("once")
                        .long("testing-only--once")
                        .hidden(true),
                )
                .arg(
                    Arg::with_name("clear")
                        .short("c")
                        .long("clear")
                        .help("Clear the screen before each run"),
                )
                .arg(
                    Arg::with_name("log:debug")
                        .long("debug")
                        .help("Show debug output"),
                )
                .arg(
                    Arg::with_name("log:info")
                        .long("why")
                        .help("Show paths that changed"),
                )
                .arg(
                    Arg::with_name("ignore-nothing")
                        .long("ignore-nothing")
                        .help("Ignore nothing, not even target/ and .git/"),
                )
                .arg(
                    Arg::with_name("no-gitignore")
                        .long("no-gitignore")
                        .help("Don’t use .gitignore files"),
                )
                .arg(
                    Arg::with_name("no-ignore")
                        .long("no-ignore")
                        .help("Don’t use .ignore files"),
                )
                .arg(
                    Arg::with_name("no-restart")
                        .long("no-restart")
                        .help("Don’t restart command while it’s still running"),
                )
                .arg(
                    Arg::with_name("packages:all")
                        .long("all")
                        .conflicts_with("packages:one")
                        .hidden(true)
                        .help("Reserved for workspace support"),
                )
                .arg(
                    Arg::with_name("poll")
                        .long("poll")
                        .help("Force use of polling for file changes"),
                )
                .arg(
                    Arg::with_name("postpone")
                        .long("postpone")
                        .help("Postpone first run until a file changes"),
                )
                .arg(
                    Arg::with_name("watch-when-idle")
                        .long("watch-when-idle")
                        .help("Ignore events emitted while the commands run. Will become default behaviour in 8.0."),
                )
                .arg(
                    Arg::with_name("features")
                        .long("features")
                        .takes_value(true)
                        .help("List of features passed to cargo invocations"),
                )
                .arg(
                    Arg::with_name("log:quiet")
                        .short("q")
                        .long("quiet")
                        .help("Suppress output from cargo-watch itself"),
                )
                .arg(
                    Arg::with_name("cmd:cargo")
                        .short("x")
                        .long("exec")
                        .takes_value(true)
                        .value_name("cmd")
                        .multiple(true)
                        .empty_values(false)
                        .min_values(1)
                        .number_of_values(1)
                        .help("Cargo command(s) to execute on changes [default: check]"),
                )
                .arg(
                    Arg::with_name("cmd:shell")
                        .short("s")
                        .long("shell")
                        .takes_value(true)
                        .value_name("cmd")
                        .multiple(true)
                        .empty_values(false)
                        .min_values(1)
                        .number_of_values(1)
                        .help("Shell command(s) to execute on changes"),
                )
                .arg(
                    Arg::with_name("delay")
                        .short("d")
                        .long("delay")
                        .takes_value(true)
                        .empty_values(false)
                        .default_value("0.5")
                        .help("File updates debounce delay in seconds"),
                )
                .arg(
                    Arg::with_name("ignore")
                        .short("i")
                        .long("ignore")
                        .takes_value(true)
                        .value_name("pattern")
                        .multiple(true)
                        .empty_values(false)
                        .min_values(1)
                        .number_of_values(1)
                        .help("Ignore a glob/gitignore-style pattern"),
                )
                .arg(
                    Arg::with_name("packages:one")
                        .short("p")
                        .long("package")
                        .takes_value(true)
                        .value_name("spec")
                        .multiple(true)
                        .empty_values(false)
                        .min_values(1)
                        .hidden(true)
                        .help("Reserved for workspace support"),
                )
                .arg(
                    Arg::with_name("watch")
                        .short("w")
                        .long("watch")
                        .takes_value(true)
                        .multiple(true)
                        .empty_values(false)
                        .min_values(1)
                        .number_of_values(1)
                        .default_value(".")
                        .help("Watch specific file(s) or folder(s)"),
                )
                .arg(
                    Arg::with_name("use-shell")
                        .long("use-shell")
                        .takes_value(true)
                        .help(if cfg!(windows) {
                            "Use a different shell. Try --use-shell=powershell, which will become the default in 8.0."
                        } else {
                            "Use a different shell. E.g. --use-shell=bash"
                        }),
                )
                .arg(
                    Arg::with_name("workdir")
                        .short("C")
                        .long("workdir")
                        .takes_value(true)
                        .help("Change working directory before running command [default: crate root]"),
                )
                .arg(
                    Arg::with_name("cmd:trail")
                        .raw(true)
                        .help("Full command to run. -x and -s will be ignored!"),
                )
                .after_help(footnote.as_str()),
        );

    // Allow invocation of cargo-watch with both `cargo-watch watch ARGS`
    // (as invoked by cargo) and `cargo-watch ARGS`.
    let mut args: Vec<String> = env::args().collect();
    args.insert(1, "watch".into());

    let matches = match app.get_matches_from_safe_borrow(args) {
        Ok(matches) => matches,
        Err(err) => {
            match err.kind {
                ErrorKind::HelpDisplayed => {
                    println!("{}", err);
                    process::exit(0);
                }

                ErrorKind::VersionDisplayed => {
                    // Unlike HelpDisplayed, VersionDisplayed emits the output
                    // by itself (clap-rs/clap#1390). It also does so without a
                    // trailing newline, so we print one ourselves.
                    println!();
                    process::exit(0);
                }

                _ => app.get_matches(),
            }
        }
    };

    matches.subcommand.unwrap().matches
}
