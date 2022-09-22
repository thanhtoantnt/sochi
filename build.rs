use std::process::Command;

fn main() {
    // // Always run `cargo fmt` for every build command
    // Command::new("cargo")
    //     .args(&["+stable", "fmt"])
    //     .output()
    //     .expect("Rustfmt should be installed");

    // Get Git revision information
    let git_rev_output = Command::new("git")
        .args(&["log", "--pretty=format:%h", "-n", "1"])
        .output()
        .unwrap();
    let git_rev = String::from_utf8(git_rev_output.stdout).unwrap();
    let git_time_output = Command::new("git")
        .args(&["show", "-s", "--format=%cs", git_rev.as_str()])
        .output()
        .unwrap();
    let git_time = String::from_utf8(git_time_output.stdout).unwrap();
    let git_version =
        "Git:".to_owned() + git_rev.as_str() + ":" + git_time.as_str();
    println!("cargo:rustc-env=GIT_VERSION={}", git_version);
}
