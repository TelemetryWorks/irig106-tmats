//! Reusable help/version scaffold; no TMATS command behavior is implemented.

use std::io::{self, Write};
use std::process::ExitCode;

/// Run the existing scaffold using caller-owned arguments and output sinks.
///
/// Does not acquire input or exit the host process. This is not the final
/// TMATS command API; only the existing help/version/usage paths are present.
pub fn run(
    args: impl IntoIterator<Item = String>,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
) -> io::Result<ExitCode> {
    match args.into_iter().next().as_deref() {
        Some("-V" | "--version") => {
            writeln!(stdout, "{}", version_line())?;
            Ok(ExitCode::SUCCESS)
        }
        Some("-h" | "--help") => {
            write!(stdout, "{}", help_text())?;
            Ok(ExitCode::SUCCESS)
        }
        _ => {
            write!(stderr, "{}", help_text())?;
            Ok(ExitCode::from(2))
        }
    }
}

/// `tmats X.Y.Z (irig106-tmats X.Y.Z)`: both versions, so a report always
/// says which library produced the output.
pub fn version_line() -> String {
    format!(
        "tmats {} (irig106-tmats {})",
        env!("CARGO_PKG_VERSION"),
        irig106_tmats::VERSION
    )
}

/// Scaffold help; TMATS commands remain unimplemented.
pub fn help_text() -> String {
    format!(
        "{}\n\
         View, extract, and check IRIG 106 Chapter 9 TMATS.\n\
         \n\
         Usage: tmats [-h | --help] [-V | --version]\n\
         \n\
         Commands are not implemented yet. The planned set (show, extract,\n\
         checksum, verify, stamp, validate, diff) is described in\n\
         https://github.com/TelemetryWorks/irig106-tmats/blob/main/docs/CLI.md\n",
        version_line()
    )
}
