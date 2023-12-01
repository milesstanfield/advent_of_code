use super::output::check_and_format_output;
use std::process::{Command, Stdio};

pub fn cat_file(file: &str) -> String {
    let output = Command::new("cat")
        .arg(file)
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    check_and_format_output(output)
}
