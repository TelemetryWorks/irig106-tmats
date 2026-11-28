//! Guards the lockstep release rule (docs/RELEASING.md): the CLI and the
//! library always carry the same version, and the CLI pins the library to
//! exactly that version.

use std::process::Command;

/// The CLI crate and the library report the same version.
///
/// Requirements: L1-REL-001
#[test]
fn cli_and_library_versions_match() {
    assert_eq!(env!("CARGO_PKG_VERSION"), irig106_tmats::VERSION);
}

/// The CLI's dependency on the library is an exact `=X.Y.Z` pin at the
/// workspace version, so a mismatched pair can never be published.
///
/// Requirements: L1-REL-001
#[test]
fn library_dependency_is_pinned_exactly() {
    let manifest = include_str!("../../../Cargo.toml");
    let expected = format!("version = \"={}\"", env!("CARGO_PKG_VERSION"));
    let line = manifest
        .lines()
        .find(|l| l.trim_start().starts_with("irig106-tmats ="))
        .expect("workspace irig106-tmats dependency line");
    assert!(
        line.contains(&expected),
        "dependency line `{line}` must contain `{expected}`"
    );
    assert!(line.contains(r#"path = "crates/irig106-tmats""#));
    let app_manifest = include_str!("../Cargo.toml");
    assert!(app_manifest.contains("irig106-tmats.workspace = true"));
}

/// `tmats --version` prints both versions and succeeds.
///
/// Requirements: L1-REL-001
#[test]
fn version_flag_reports_both_versions() {
    let out = Command::new(env!("CARGO_BIN_EXE_tmats"))
        .arg("--version")
        .output()
        .expect("run tmats");
    assert!(out.status.success());
    let text = String::from_utf8(out.stdout).expect("utf-8");
    let version = env!("CARGO_PKG_VERSION");
    assert_eq!(
        text.trim(),
        format!("tmats {version} (irig106-tmats {version})")
    );
}

/// An unknown invocation is a usage error: help on stderr, exit status 2.
#[test]
fn unknown_invocation_is_a_usage_error() {
    let out = Command::new(env!("CARGO_BIN_EXE_tmats"))
        .arg("frobnicate")
        .output()
        .expect("run tmats");
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("Usage: tmats"));
}

/// Only the help/version/usage scaffold is exercised, not TMATS commands.
#[test]
fn runner_can_be_called_repeatedly_without_exiting_the_host() {
    use std::process::ExitCode;

    for flag in ["--version", "--help", "unimplemented"] {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let class = irig106_tmats_cli::run([flag.to_owned()], &mut stdout, &mut stderr)
            .expect("write scaffold output");
        if flag == "unimplemented" {
            assert_eq!(class, ExitCode::from(2));
            assert!(stdout.is_empty());
            assert_eq!(stderr, irig106_tmats_cli::help_text().as_bytes());
        } else {
            assert_eq!(class, ExitCode::SUCCESS);
            assert!(stderr.is_empty());
            let expected = if flag == "--version" {
                format!("{}\n", irig106_tmats_cli::version_line())
            } else {
                irig106_tmats_cli::help_text()
            };
            assert_eq!(stdout, expected.as_bytes());
        }
    }
}
