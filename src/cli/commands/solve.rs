use std::process::{Command, Stdio};
use adventofcode_2023::days::Day;

pub fn handle(day: Day) {
    let cmd_args = vec!["run".to_string(), "--bin".to_string(), day.to_string()];
    let mut cmd = Command::new("cargo")
        .args(cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}