//! Repository management for unjust

use std::io;
use std::path::{Path, PathBuf};

/// Information about a repository
#[derive(Debug, Clone)]
pub struct Repository {
    /// Repository name (username/repo)
    pub name: String,

    /// Path to the repository in the cache
    pub path: PathBuf,

    /// Upstream repository name (if any)
    pub upstream: Option<String>,
}

impl Repository {
    /// Create a new Repository
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            upstream: None,
        }
    }

    /// Create a new Repository with upstream info
    pub fn with_upstream(name: String, path: PathBuf, upstream: String) -> Self {
        Self {
            name,
            path,
            upstream: Some(upstream),
        }
    }

    /// Check if the repository exists in the cache
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Get the path to the Justfile in this repository
    pub fn justfile_path(&self) -> PathBuf {
        self.path.join("Justfile")
    }

    /// Check if this repository has a Justfile
    pub fn has_justfile(&self) -> bool {
        self.justfile_path().exists()
    }
}

/// Detect the repository for the current directory
#[allow(dead_code)]
pub fn detect_current_repo() -> io::Result<Option<Repository>> {
    // This is a simplified implementation - in a real implementation,
    // we would use git commands to detect the current repo

    // For now, just return None to indicate we couldn't detect a repo
    Ok(None)
}

/// Parse a repository name into a Repository object
#[allow(dead_code)]
pub fn parse_repo_name(name: &str, cache_dir: &Path) -> Repository {
    // Simple implementation - just create a Repository
    // In a real implementation, we would parse username/repo format
    // and handle GitHub URLs

    let path = cache_dir.join(name);
    Repository::new(name.to_string(), path)
}

#[cfg(test)]
mod tests {
    use super::*;
    // use tempfile::TempDir;

    #[test]
    fn test_repository_new() {
        let name = "user/repo".to_string();
        let path = PathBuf::from("/path/to/repo");

        let repo = Repository::new(name.clone(), path.clone());

        assert_eq!(repo.name, name);
        assert_eq!(repo.path, path);
        assert_eq!(repo.upstream, None);
    }

    #[test]
    fn test_repository_with_upstream() {
        let name = "user/repo".to_string();
        let path = PathBuf::from("/path/to/repo");
        let upstream = "upstream/repo".to_string();

        let repo = Repository::with_upstream(name.clone(), path.clone(), upstream.clone());

        assert_eq!(repo.name, name);
        assert_eq!(repo.path, path);
        assert_eq!(repo.upstream, Some(upstream));
    }

    #[test]
    fn test_repository_justfile_path() {
        let name = "user/repo".to_string();
        let path = PathBuf::from("/path/to/repo");

        let repo = Repository::new(name, path.clone());

        assert_eq!(repo.justfile_path(), path.join("Justfile"));
    }
}
