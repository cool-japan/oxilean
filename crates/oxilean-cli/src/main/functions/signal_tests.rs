//! A test of the SIGINT handler `signal_util::install_ctrlc_handler`
//! installs: the signal sets the interrupted flag instead of ending the
//! process.

use super::signal_util;
use std::time::{Duration, Instant};

#[cfg(unix)]
#[test]
fn sigint_sets_the_interrupted_flag() {
    signal_util::clear_interrupt();
    signal_util::install_ctrlc_handler();
    let status = std::process::Command::new("kill")
        .args(["-INT", &std::process::id().to_string()])
        .status()
        .expect("the kill command runs");
    assert!(status.success());
    let deadline = Instant::now() + Duration::from_secs(10);
    while !signal_util::was_interrupted() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(signal_util::was_interrupted());
    signal_util::clear_interrupt();
}
