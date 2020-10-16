use std::process::{Command, Output};

const GIT: &str = "git describe --always --dirty=-modified";

fn git_version() {
    let version =
        String::from_utf8(run_command(GIT).stdout).expect("Git output is not valid UTF-8");

    println!("cargo:rustc-env=GIT_VERSION={}", version);
}

fn run_command(cmd: &str) -> Output {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", cmd]).output()
    } else {
        Command::new("sh").args(&["-c", cmd]).output()
    }
    .expect("Unable to run command")
}

fn main() {
    if cfg!(debug_assertions) {
        println!("cargo:rustc-env=GIT_VERSION=DEBUG");
    } else {
        git_version();
    }
}
