use assert_cmd::prelude::*;
use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
    process::{Command, Stdio},
    thread::sleep,
    time::{Duration, Instant},
};
use wait_timeout::ChildExt;

fn touch(n: u8) -> io::Result<()> {
    let path: PathBuf = format!("./tests/touchdata/{}.txt", n).into();
    let mut file = OpenOptions::new().create(true).write(true).open(path)?;

    writeln!(&mut file, "{:?}", Instant::now())?;
    Ok(())
}

fn std_to_string<T: io::Read>(handle: &mut Option<T>) -> String {
    if let Some(ref mut handle) = handle {
        let mut buf = String::with_capacity(1024);
        handle.read_to_string(&mut buf).unwrap();
        buf
    } else {
        unreachable!()
    }
}

// fsevents has trouble
#[cfg(not(target_os = "macos"))]
#[test]
fn without_poll() {
    let mut main = Command::cargo_bin("cargo-watch")
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "--testing-only--once",
            "--no-gitignore",
            "-w",
            "./tests/touchdata/",
            "-s",
            "echo it runs",
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_secs(2));
    touch(0).unwrap();

    if main
        .wait_timeout(Duration::from_secs(30))
        .unwrap()
        .is_none()
    {
        main.kill().unwrap();
    }

    main.wait_with_output().unwrap().assert().success();
}

#[test]
fn with_poll() {
    let mut main = Command::cargo_bin("cargo-watch")
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "--testing-only--once",
            "--no-gitignore",
            "--poll",
            "-w",
            "./tests/touchdata/",
            "-s",
            "echo it runs",
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_secs(2));
    touch(1).unwrap();

    if main
        .wait_timeout(Duration::from_secs(30))
        .unwrap()
        .is_none()
    {
        main.kill().unwrap();
    }

    main.wait_with_output().unwrap().assert().success();
}

#[test]
#[cfg(not(windows))] // annoyingly, there’s some kind of encoding or extra bytes getting added here, needs debugging
fn with_announce() {
    let mut main = Command::cargo_bin("cargo-watch")
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "--testing-only--once",
            "--no-gitignore",
            "--poll",
            "-w",
            "./tests/touchdata/",
            "-s",
            "echo with announce",
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_secs(2));
    touch(2).unwrap();

    if main
        .wait_timeout(Duration::from_secs(30))
        .unwrap()
        .is_none()
    {
        main.kill().unwrap();
    }

    insta::assert_snapshot!("with_announce.stderr", std_to_string(&mut main.stderr));
    insta::assert_snapshot!("with_announce.stdout", std_to_string(&mut main.stdout));
}

#[test]
fn without_announce() {
    let mut main = Command::cargo_bin("cargo-watch")
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "--testing-only--once",
            "--no-gitignore",
            "--quiet",
            "--poll",
            "-w",
            "./tests/touchdata/",
            "-s",
            "echo without announce",
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_secs(2));
    touch(3).unwrap();

    if main
        .wait_timeout(Duration::from_secs(30))
        .unwrap()
        .is_none()
    {
        main.kill().unwrap();
    }

    insta::assert_snapshot!("without_announce.stderr", std_to_string(&mut main.stderr));
    insta::assert_snapshot!("without_announce.stdout", std_to_string(&mut main.stdout));
}

#[cfg(unix)]
#[test]
fn with_error() {
    let mut main = Command::cargo_bin("cargo-watch")
        .unwrap()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&[
            "--testing-only--once",
            "--no-gitignore",
            "--poll",
            "-w",
            "./tests/touchdata/",
            "-s",
            "echo with error",
            "-s",
            "false",
        ])
        .spawn()
        .unwrap();

    sleep(Duration::from_secs(2));
    touch(4).unwrap();

    if main
        .wait_timeout(Duration::from_secs(30))
        .unwrap()
        .is_none()
    {
        main.kill().unwrap();
    }

    insta::assert_snapshot!("with_error.stderr", std_to_string(&mut main.stderr));
    insta::assert_snapshot!("with_error.stdout", std_to_string(&mut main.stdout));
}
