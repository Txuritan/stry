pub mod build {
    use std::process::{Command, Output};

    const GIT: &str = "git describe --always --dirty=-modified";

    pub fn git_version() {
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
}
