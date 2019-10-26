//! Watch files in a Cargo project and compile it when they change
#![forbid(unsafe_code)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(non_ascii_literal))]
#![cfg_attr(feature = "cargo-clippy", allow(cast_sign_loss))]
#![cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation))]

#[macro_use]
extern crate clap;
extern crate watchexec;

use clap::{ArgMatches, Error, ErrorKind};
use std::{
    env::set_current_dir,
    path::{PathBuf, MAIN_SEPARATOR},
};
use watchexec::cli::Args;

pub mod args;
pub mod cargo;
pub mod watch;

pub fn change_dir() {
    cargo::root()
        .and_then(|p| set_current_dir(p).ok())
        .unwrap_or_else(|| {
            Error::with_description("Not a Cargo project, aborting.", ErrorKind::Io).exit();
        });
}

pub fn get_commands(debug: bool, matches: &ArgMatches) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();

    // Cargo commands are in front of the rest
    if matches.is_present("cmd:cargo") {
        for cargo in values_t!(matches, "cmd:cargo", String).unwrap_or_else(|e| e.exit()) {
            let mut cmd: String = "cargo ".into();
            cmd.push_str(&cargo);
            commands.push(cmd);
        }
    }

    // Shell/raw commands go last
    if matches.is_present("cmd:shell") {
        for shell in values_t!(matches, "cmd:shell", String).unwrap_or_else(|e| e.exit()) {
            commands.push(shell);
        }
    }

    // Default to `cargo check`
    if commands.is_empty() {
        commands.push("cargo check".into());
    }

    if debug {
        println!(">>> Commands: {:?}", commands);
    }

    commands
}

pub fn get_ignores(debug: bool, matches: &ArgMatches) -> (bool, Vec<String>) {
    let mut opts = Vec::new();

    if matches.is_present("ignore-nothing") {
        if debug {
            println!(">>> Ignoring nothing");
        }

        return (true, Vec::new());
    }

    let novcs = matches.is_present("no-gitignore");
    if debug {
        println!(">>> Load Git/VCS ignores: {:?}", !novcs);
    }

    // Mac
    opts.push(format!("*{}.DS_Store", MAIN_SEPARATOR));

    // Vim
    opts.push("*.sw?".into());
    opts.push("*.sw?x".into());

    // Emacs
    opts.push("#*#".into());
    opts.push(".#*".into());
    
    // Kate
    opts.push(".*.kate-swp".into());

    // VCS
    opts.push(format!("*{s}.hg{s}**", s = MAIN_SEPARATOR));
    opts.push(format!("*{s}.git{s}**", s = MAIN_SEPARATOR));
    opts.push(format!("*{s}.svn{s}**", s = MAIN_SEPARATOR));

    // SQLite
    opts.push("*.db".into());
    opts.push("*.db-*".into());
    opts.push(format!("*{s}*.db-journal{s}**", s = MAIN_SEPARATOR));

    // Rust
    opts.push(format!("*{s}target{s}**", s = MAIN_SEPARATOR));

    if debug {
        println!(">>> Default ignores: {:?}", opts);
    }

    if matches.is_present("ignore") {
        for ignore in values_t!(matches, "ignore", String).unwrap_or_else(|e| e.exit()) {
            #[cfg(windows)]
            let ignore = ignore.replace("/", &MAIN_SEPARATOR.to_string());
            opts.push(ignore);
        }
    }

    if debug {
        println!(">>> All ignores: {:?}", opts);
    }

    (novcs, opts)
}

pub fn get_debounce(debug: bool, matches: &ArgMatches) -> u32 {
    if matches.is_present("delay") {
        let debounce = value_t!(matches, "delay", f32).unwrap_or_else(|e| e.exit());
        if debug {
            println!(">>> File updates debounce: {} seconds", debounce);
        }
        (debounce * 1000.0) as u32
    } else {
        500
    }
}

pub fn get_watches(debug: bool, matches: &ArgMatches) -> Vec<PathBuf> {
    let mut opts = Vec::new();
    if matches.is_present("watch") {
        for watch in values_t!(matches, "watch", String).unwrap_or_else(|e| e.exit()) {
            opts.push(watch.into());
        }
    }

    if debug {
        println!(">>> Watches: {:?}", opts);
    }

    opts
}

pub fn get_watchexec_args(debug: bool, matches: &ArgMatches) -> Args {
    let (novcs, ignores) = get_ignores(debug, &matches);
    let debounce = get_debounce(debug, &matches);

    let arglist = Args {
        filters: vec![],
        no_shell: false,
        once: matches.is_present("once"),
        signal: None,
        restart: !matches.is_present("no-restart"),

        poll: matches.is_present("poll"),
        poll_interval: debounce,
        debounce: u64::from(debounce),

        ignores,
        no_vcs_ignore: novcs,
        no_ignore: false,

        clear_screen: matches.is_present("clear"),
        debug,
        run_initially: !matches.is_present("postpone"),

        cmd: get_commands(debug, &matches),
        paths: get_watches(debug, &matches),
    };

    if debug {
        println!(">>> Watchexec arguments: {:?}", arglist);
    }

    arglist
}
