//! Justfile management for unjust

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Information about a Justfile
#[derive(Debug, Clone)]
pub struct Justfile {
    /// Repository it belongs to
    pub repo_name: String,

    /// Path to the Justfile
    pub path: PathBuf,
}

impl Justfile {
    /// Create a new Justfile
    pub fn new(repo_name: String, path: PathBuf) -> Self {
        Self { repo_name, path }
    }

    /// Check if the Justfile exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Read the content of the Justfile
    pub fn read_content(&self) -> io::Result<String> {
        fs::read_to_string(&self.path)
    }

    /// Get the display name for this Justfile
    pub fn display_name(&self) -> String {
        self.repo_name.clone()
    }
}

/// Look for Justfiles in a directory
#[allow(dead_code)]
pub fn find_justfiles_in_dir(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut result = Vec::new();

    if dir.exists() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.file_name().and_then(|n| n.to_str()) == Some("Justfile") {
                result.push(path);
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // use unjust_core::{Justfile, find_justfile, list_justfiles};

    #[test]
    fn test_justfile_new() {
        let repo_name = "test/repo".to_string();
        let path = PathBuf::from("/path/to/justfile");

        let justfile = Justfile::new(repo_name.clone(), path.clone());

        assert_eq!(justfile.repo_name, repo_name);
        assert_eq!(justfile.path, path);
    }

    #[test]
    fn test_justfile_display_name() {
        let repo_name = "user/project".to_string();
        let path = PathBuf::from("/tmp/justfile");

        let justfile = Justfile::new(repo_name.clone(), path);

        assert_eq!(justfile.display_name(), repo_name);
    }

    fn setup_test_cache_with_justfiles() -> io::Result<TempDir> {
        let temp_dir = TempDir::new()?;
        let cache_dir = temp_dir.path();

        // Create a few test repositories
        let repos = ["repo1", "repo2", "repo3"];

        for repo in repos.iter() {
            let repo_dir = cache_dir.join(repo);
            fs::create_dir_all(&repo_dir)?;

            // Create a Justfile in each repo
            let justfile_path = repo_dir.join("Justfile");
            fs::write(justfile_path, "# Test Justfile")?;
        }

        Ok(temp_dir)
    }

    #[test]
    #[ignore]
    fn test_find_justfile() -> io::Result<()> {
        let temp_dir = setup_test_cache_with_justfiles()?;
        let _cache_path = temp_dir.path();

        // Note: We'd need to patch ensure_cache_dir to use our temp directory
        // For this example, we're just showing the test structure

        // Test finding an existing justfile
        // let result = find_justfile("repo1", false);
        // assert!(result.is_ok());
        // assert!(result.unwrap().is_some());

        // Test finding a non-existent justfile
        // let result = find_justfile("non-existent", false);
        // assert!(result.is_ok());
        // assert!(result.unwrap().is_none());

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_list_justfiles() -> io::Result<()> {
        let _temp_dir = setup_test_cache_with_justfiles()?;

        // Again, we'd need to patch get_cache_dir
        // let result = list_justfiles();
        // assert!(result.is_ok());

        // let justfiles = result.unwrap();
        // assert_eq!(justfiles.len(), 3);

        // Check that all expected repos are in the list
        // let repo_names: Vec<String> = justfiles.iter().map(|j| j.repo_name.clone()).collect();
        // assert!(repo_names.contains(&"repo1".to_string()));
        // assert!(repo_names.contains(&"repo2".to_string()));
        // assert!(repo_names.contains(&"repo3".to_string()));

        Ok(())
    }
}
