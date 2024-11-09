use std::process::{Command, Output};

/// Run a command and return the output
pub fn run_command(command: &str) -> std::io::Result<Output> {
    let command_parts = command.split_whitespace().collect::<Vec<&str>>();
    let executable = command_parts[0];
    let args = &command_parts[1..];

    let mut command = Command::new(executable);
    command.args(args);

    command.output()
}
