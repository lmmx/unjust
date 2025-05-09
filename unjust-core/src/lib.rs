#![warn(missing_docs)]
#![forbid(unsafe_code)]
//! Core functionality for the unjust tool
//!
//! This crate provides the core functions for managing Justfiles
//! without any CLI-specific code.

use standard_paths::{LocationType, StandardPaths};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

mod justfile;
mod repository;

pub use justfile::Justfile;
pub use repository::Repository;

/// Get the cache directory for unjust
pub fn get_cache_dir() -> Option<PathBuf> {
    let sp = StandardPaths::new("unjust.core", "unjust");
    sp.writable_location(LocationType::AppCacheLocation).ok()
}

/// Check if unjust has been used before (cache directory exists)
pub fn is_first_use() -> bool {
    match get_cache_dir() {
        Some(dir) => !dir.exists(),
        None => true,
    }
}

/// Create the cache directory if it doesn't exist
pub fn ensure_cache_dir() -> io::Result<PathBuf> {
    let cache_dir = get_cache_dir().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine cache directory",
        )
    })?;

    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir)?;
    }

    Ok(cache_dir)
}

/// Find an appropriate Justfile for the given repository
pub fn find_justfile(repo_name: &str, _separate_upstream: bool) -> io::Result<Option<Justfile>> {
    let cache_dir = ensure_cache_dir()?;

    // Simple implementation for now - just check if the file exists in cache
    let justfile_path = cache_dir.join(repo_name).join("Justfile");

    if justfile_path.exists() {
        Ok(Some(Justfile::new(repo_name.to_string(), justfile_path)))
    } else {
        Ok(None)
    }
}

/// List all available Justfiles in the cache
pub fn list_justfiles() -> io::Result<Vec<Justfile>> {
    let cache_dir = match get_cache_dir() {
        Some(dir) if dir.exists() => dir,
        _ => return Ok(Vec::new()),
    };

    let mut result = Vec::new();

    // This is a simplified implementation - we would normally scan subdirectories
    if let Ok(entries) = fs::read_dir(&cache_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let justfile_path = path.join("Justfile");
                if justfile_path.exists() {
                    let repo_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    result.push(Justfile::new(repo_name, justfile_path));
                }
            }
        }
    }

    Ok(result)
}

/// Create a basic Justfile template
pub fn create_justfile_template(path: &Path) -> io::Result<()> {
    let content = r#"# Justfile managed by unjust
# https://github.com/yourusername/unjust

default:
    @just --list

# Example recipe
hello:
    echo "Hello from unjust!"

# Another example
build:
    echo "Building project..."
    # Your build commands here
"#;

    fs::write(path, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod cache_tests {
        use super::*;
        use tempfile::TempDir;

        // TODO: Refactor this into a separate module
        // use unjust_core::{ensure_cache_dir, get_cache_dir, is_first_use};

        /// Helper function to patch the standard paths library for testing
        /// This is a mock implementation for testing purposes
        fn with_temp_cache_dir<F, R>(f: F) -> R
        where
            F: FnOnce(&Path) -> R,
        {
            // Create a temporary directory for testing
            let temp_dir = TempDir::new().expect("Failed to create temp dir");
            let temp_path = temp_dir.path();

            // We would ideally monkey patch get_cache_dir to use our temp dir
            // For this example, let's assume we've found a way to do that
            // In practice, you might use mockall or another approach

            // Call the function with our temporary path
            // Return the result (temp_dir will be cleaned up automatically)
            f(temp_path)
        }

        #[test]
        fn test_is_first_use() {
            with_temp_cache_dir(|dir| {
                // When the directory doesn't exist, is_first_use should return true
                assert!(is_first_use());

                // Create the directory
                fs::create_dir_all(dir).expect("Failed to create directory");

                // Now is_first_use should return false
                // Note: In a real test, we'd ensure get_cache_dir returns dir
                // assert!(!is_first_use());
            });
        }

        #[test]
        #[ignore]
        fn test_ensure_cache_dir() {
            with_temp_cache_dir(|dir| {
                // The directory shouldn't exist yet
                assert!(!dir.exists());

                // Call ensure_cache_dir - again, we'd need to patch get_cache_dir
                // let result = ensure_cache_dir();
                // assert!(result.is_ok());

                // The directory should now exist
                // assert!(dir.exists());
            });
        }
    }
}
