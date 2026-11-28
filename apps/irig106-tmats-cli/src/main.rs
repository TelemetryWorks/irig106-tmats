//! Thin executable adapter for the reusable scaffold.

use std::process::ExitCode;

fn main() -> ExitCode {
    irig106_tmats_cli::run(
        std::env::args().skip(1),
        &mut std::io::stdout(),
        &mut std::io::stderr(),
    )
    .unwrap_or_else(|_| ExitCode::from(4))
}
