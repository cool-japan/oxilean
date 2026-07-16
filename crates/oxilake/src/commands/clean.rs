//! `oxilake clean` — remove cached build artifacts.

use crate::cache::CacheManager;
use anyhow::Result;

/// Remove cached build artifacts.
///
/// If `package_name` is provided, only that package's cache entries are removed.
/// Otherwise the entire cache is cleared.
pub fn run(package_name: Option<&str>) -> Result<()> {
    let cache = CacheManager::new().map_err(|e| anyhow::anyhow!("{}", e))?;

    match package_name {
        Some(name) => {
            cache
                .clear_package(name)
                .map_err(|e| anyhow::anyhow!("{}", e))?;
            println!("Cleared cache for package '{name}'.");
        }
        None => {
            cache.clear_all().map_err(|e| anyhow::anyhow!("{}", e))?;
            println!("Cache cleared.");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cache::CacheManager;
    use std::env;
    use std::fs;

    #[test]
    fn test_clean_clears_specific_package() {
        let cache_dir = env::temp_dir().join("oxilake_clean_test_specific_001");
        fs::remove_dir_all(&cache_dir).ok();
        let cache = CacheManager::with_dir(cache_dir.clone()).expect("create cache");

        cache
            .store_artifact("mypkg", 1, b"data", "out.leanc")
            .expect("store");
        cache
            .store_artifact("otherpkg", 2, b"other", "out.leanc")
            .expect("store other");

        assert!(cache.has_artifact("mypkg", 1));
        assert!(cache.has_artifact("otherpkg", 2));

        cache.clear_package("mypkg").expect("clear mypkg");

        assert!(!cache.has_artifact("mypkg", 1), "mypkg should be cleared");
        assert!(
            cache.has_artifact("otherpkg", 2),
            "otherpkg should be untouched"
        );

        fs::remove_dir_all(&cache_dir).ok();
    }

    #[test]
    fn test_clean_clears_all() {
        let cache_dir = env::temp_dir().join("oxilake_clean_test_all_002");
        fs::remove_dir_all(&cache_dir).ok();
        let cache = CacheManager::with_dir(cache_dir.clone()).expect("create cache");

        cache
            .store_artifact("pkg_a", 10, b"aaa", "a.leanc")
            .expect("store a");
        cache
            .store_artifact("pkg_b", 20, b"bbb", "b.leanc")
            .expect("store b");

        assert_eq!(cache.artifact_count().expect("count"), 2);

        cache.clear_all().expect("clear all");

        assert_eq!(
            cache.artifact_count().expect("count after clear"),
            0,
            "all artifacts should be cleared"
        );

        fs::remove_dir_all(&cache_dir).ok();
    }
}
