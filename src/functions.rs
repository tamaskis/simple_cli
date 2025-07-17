use std::path::Path;
use std::process::Command;

/// Runs a system command.
///
/// # Arguments
///
/// * `cmd` - A slice of string slices representing the command and its arguments. The first element
///   should be the command name, followed by its arguments.
/// * `dir` - Path to the directory that the command specified by `cmd` should be run in. If `None`,
///   the command runs in the current working directory.
///
/// # Returns
///
/// A result where:
///
/// * `Ok` contains `()` if the command executes successfully (exit status zero).
/// * `Err` contains an error message if the command fails to run or exits with a non-zero status.
///
/// # Example
///
/// ```ignore
/// use simple_cli::run_command;
/// use std::path::Path;
///
/// // Run `git status` inside the given repository directory.
/// run_command(&["git", "status"], Some(Path::new("/path/to/repo"))).unwrap();
///
/// // Run `ls -la` in the current directory.
/// run_command(&["ls", "-la"], None).unwrap();
/// ```
pub fn run_command(cmd: &[&str], dir: Option<&Path>) -> Result<(), String> {
    // Create the Command object with the first element as the command name.
    let mut command = Command::new(cmd[0]);

    // Add remaining elements as command arguments.
    command.args(&cmd[1..]);

    // Set the current directory if provided.
    if let Some(dir_path) = dir {
        command.current_dir(dir_path);
    }

    // Execute the command and capture the exit status.
    let status = command
        .status()
        .map_err(|e| format!("Failed to run command {cmd:?}: {e}"))?;

    // Check if the command exited successfully.
    if status.success() {
        Ok(())
    } else {
        Err(format!("Command {cmd:?} failed with status: {status:?}"))
    }
}
