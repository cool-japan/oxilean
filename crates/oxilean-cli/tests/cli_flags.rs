//! Integration tests for the informational flags of the `oxilean` binary.

use std::process::Command;

fn run(arg: &str) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_oxilean"))
        .arg(arg)
        .output()
        .expect("failed to run oxilean binary");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
    )
}

#[test]
fn help_flags_exit_zero_with_usage() {
    for arg in ["help", "--help", "-h"] {
        let (ok, stdout) = run(arg);
        assert!(ok, "{arg} should exit 0");
        assert_eq!(
            stdout.lines().next(),
            Some("OxiLean - Pure Rust Interactive Theorem Prover"),
            "{arg}"
        );
    }
}

#[test]
fn version_flags_exit_zero_with_version() {
    let expected = format!("OxiLean version {}", env!("CARGO_PKG_VERSION"));
    for arg in ["version", "--version", "-V"] {
        let (ok, stdout) = run(arg);
        assert!(ok, "{arg} should exit 0");
        assert_eq!(stdout.lines().next(), Some(expected.as_str()), "{arg}");
        assert!(!stdout.contains("SLOC"), "{arg}");
    }
}

#[test]
fn missing_file_still_fails_with_exit_one() {
    let missing = std::env::temp_dir().join("oxilean_nonexistent_cli_flags_test.lean");
    let out = Command::new(env!("CARGO_BIN_EXE_oxilean"))
        .arg(&missing)
        .output()
        .expect("failed to run oxilean binary");
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("Failed to read file"));
}
