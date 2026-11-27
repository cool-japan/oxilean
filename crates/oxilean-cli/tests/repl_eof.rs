//! Regression test: the REPL must exit at end of input instead of spinning.

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[test]
fn repl_exits_cleanly_at_eof() {
    let home = std::env::temp_dir().join(format!("oxilean_repl_eof_test_{}", std::process::id()));
    std::fs::create_dir_all(&home).expect("create temp home dir");

    let mut child = Command::new(env!("CARGO_BIN_EXE_oxilean"))
        .env("HOME", &home)
        .env("USERPROFILE", &home)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to spawn oxilean binary");

    let deadline = Instant::now() + Duration::from_secs(20);
    let status = loop {
        match child.try_wait().expect("failed to poll child") {
            Some(status) => break Some(status),
            None if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
            None => std::thread::sleep(Duration::from_millis(50)),
        }
    };
    let _ = std::fs::remove_dir_all(&home);

    let status = status.expect("REPL did not exit within 20s after stdin reached EOF");
    assert!(status.success(), "REPL should exit 0 at EOF, got {status}");
}
