//! `tmats` — view, extract, and check TMATS (IRIG 106 Chapter 9).
//!
//! This is the scaffold of the command-line tool. Its commands (`show`,
//! `extract`, `checksum`, `verify`, `stamp`, and later `validate` and `diff`)
//! are specified in `docs/USE-CASES.md` (UC-17) and designed in
//! `docs/CLI.md`; none is implemented yet, because the project is in its
//! documentation-first design phase.

use std::process::ExitCode;

/// Exit status for a command-line usage error (conventional value 2).
const EXIT_USAGE: u8 = 2;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("-V" | "--version") => {
            println!("{}", version_line());
            ExitCode::SUCCESS
        }
        Some("-h" | "--help") => {
            print!("{}", help_text());
            ExitCode::SUCCESS
        }
        _ => {
            eprint!("{}", help_text());
            ExitCode::from(EXIT_USAGE)
        }
    }
}

/// `tmats X.Y.Z (irig106-tmats X.Y.Z)`: both versions, so a report always
/// says which library produced the output.
fn version_line() -> String {
    format!(
        "tmats {} (irig106-tmats {})",
        env!("CARGO_PKG_VERSION"),
        irig106_tmats::VERSION
    )
}

fn help_text() -> String {
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
