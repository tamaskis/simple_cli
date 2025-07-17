/// Macro to print an error message to standard error and immediately exit with status code 1.
///
/// This macro accepts formatting arguments similarly to `println!` and `eprintln!`.
///
/// # Arguments
///
/// * `$($arg:tt)*` - A format string followed by optional arguments, just like `eprintln!`. These
///   are used to construct the error message that will be printed to `stderr`.
///
/// # Example
///
/// ```ignore
/// exit_code_1!("An error occurred: {}", "something went wrong");
/// ```
///
/// This will print:
///
/// ```text
/// An error occurred: something went wrong
/// ```
///
/// And then terminate the program with exit code 1.
///
/// # Note
///
/// This macro will not return; it terminates the process immediately.
#[macro_export]
macro_rules! exit_code_1 {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        std::process::exit(1);
    }};
}

/// Runs a system command and exits with code 1 if an error occurs.
///
/// This macro calls [`run_command`] with the provided command and an optional directory. If the
/// command fails, it uses the [`exit_code_1!`] macro to print the error message and exit
/// immediately.
///
/// # Arguments
///
/// * `$cmd` - A slice of string slices representing the command and its arguments. The first
///   element should be the command name, followed by its arguments.
/// * `$dir` - Path to the directory that the command specified by `cmd` should be run in. If not
///   provided, the command runs in the current working directory.
///
/// # Example
///
/// ```ignore
/// use simple_cli::run_command;
///
/// // Run command in current directory.
/// run_command!(&["git", "status"]);
///
/// // Run command in a specific directory.
/// run_command!(&["git", "status"], Path::new("/path/to/repo"));
/// ```
#[macro_export]
macro_rules! run_command {
    // With directory argument.
    ($cmd:expr, $dir:expr) => {{
        if let Err(e) = $crate::utils::cli_utils::run_command($cmd, Some($dir)) {
            $crate::exit_code_1!("{}", e);
        }
    }};
    // Without directory argument.
    ($cmd:expr) => {{
        if let Err(e) = $crate::utils::cli_utils::run_command($cmd, None) {
            $crate::exit_code_1!("{}", e);
        }
    }};
}
