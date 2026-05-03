//! # File Watcher Integration
//!
//! A pure-Rust, polling-based file watcher for incremental builds.
//!
//! No external crates are required: change detection is performed by
//! comparing [`std::fs::Metadata`] snapshots (mtime + size) between
//! successive calls to [`FileWatcher::poll`].
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use std::path::Path;
//! use oxilean_build::file_watcher::{FileWatcher, WatcherConfig};
//!
//! let config = WatcherConfig {
//!     poll_interval_ms: 200,
//!     recursive: true,
//!     extensions: vec!["lean".to_string(), "rs".to_string()],
//! };
//! let mut watcher = FileWatcher::new(config);
//! watcher.watch(Path::new("src")).expect("failed to watch");
//!
//! loop {
//!     let events = watcher.poll();
//!     for ev in events {
//!         println!("{:?} — {:?}", ev.kind, ev.path);
//!     }
//!     // Sleep for poll_interval_ms before the next poll (caller's
//!     // responsibility; the watcher itself is synchronous).
//!     std::thread::sleep(std::time::Duration::from_millis(200));
//!     break; // just for the doctest to terminate
//! }
//! ```

pub mod functions;
pub mod types;

#[cfg(test)]
mod tests;

pub use functions::*;
pub use types::*;
