use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["describe", "--always", "--dirty=-modified"])
        .output()?;
    let git_hash = String::from_utf8(output.stdout)?;

    println!("cargo:rustc-env=GIT_VERSION={}", git_hash);

    Ok(())
}
