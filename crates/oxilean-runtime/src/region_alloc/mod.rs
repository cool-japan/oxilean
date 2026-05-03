//! Region-based memory allocator for bulk-freeable proof-checking data.
//!
//! Provides a pool of contiguous `Region`s that can be allocated from
//! bump-pointer style and freed in bulk, making them ideal for temporary
//! data generated during proof checking passes.

pub mod functions;
pub mod types;

pub use functions::align_up;
pub use types::{AllocStats, Region, RegionAllocator, RegionConfig, RegionHandle, RegionId};
