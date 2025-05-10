// unjust-cli/tests/cli_tests.rs
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use temp_env::with_var;
use tempfile::TempDir;

// Helper function to set up a test environment
fn setup_test_env() -> TempDir {
    TempDir::new().unwrap()
}

// Helper to create a test repository
fn create_test_repo(
    cache_dir: &Path,
    repo_name: &str,
    justfile_content: &str,
) -> std::io::Result<()> {
    let repo_dir = cache_dir.join(repo_name);
    fs::create_dir_all(&repo_dir)?;

    let justfile_path = repo_dir.join("Justfile");
    fs::write(justfile_path, justfile_content)?;

    Ok(())
}

#[test]
fn test_cli_no_args() {
    let mut cmd = Command::cargo_bin("unjust").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_unknown_command() {
    let mut cmd = Command::cargo_bin("unjust").unwrap();

    cmd.arg("unknown")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error:"))
        .stderr(predicate::str::contains("Unknown command"));
}

#[test]
fn test_cli_list_command() -> std::io::Result<()> {
    // Set up a test cache directory with some justfiles
    let temp_dir = setup_test_env();
    let cache_dir = temp_dir.path();

    // Create test repositories
    create_test_repo(cache_dir, "repo1", "# Test repo 1")?;
    create_test_repo(cache_dir, "user/repo2", "# Test repo 2")?;

    with_var(
        "UNJUST_CACHE_DIR",
        Some(temp_dir.path().to_str().unwrap()),
        || {
            let mut cmd = Command::cargo_bin("unjust").unwrap();

            // Test basic list
            cmd.arg("list")
                .assert()
                .success()
                .stdout(predicate::str::contains("repo1"))
                .stdout(predicate::str::contains("user/repo2"));

            // Test list with paths flag
            let mut cmd = Command::cargo_bin("unjust").unwrap();
            cmd.arg("list")
                .arg("--paths")
                .assert()
                .success()
                .stdout(predicate::str::contains("repo1"))
                .stdout(predicate::str::contains("user/repo2"))
                .stdout(predicate::str::contains(temp_dir.path().to_str().unwrap()));
        },
    );

    Ok(())
}

#[test]
fn test_cli_use_command() -> std::io::Result<()> {
    // Set up a test environment
    let temp_dir = setup_test_env();
    let cache_dir = temp_dir.path();

    // Create a test repository
    create_test_repo(cache_dir, "test/repo", "# Test justfile")?;

    with_var(
        "UNJUST_CACHE_DIR",
        Some(cache_dir.to_str().unwrap()),
        || {
            // Test with a valid repo argument
            let mut cmd = Command::cargo_bin("unjust").unwrap();
            cmd.arg("use")
                .arg("test/repo")
                .assert()
                .success()
                .stdout(predicate::str::contains("Using Justfile from"));

            // Test without a repo argument (should fail)
            let mut cmd = Command::cargo_bin("unjust").unwrap();
            cmd.arg("use")
                .assert()
                .failure()
                .stderr(predicate::str::contains("Repository not specified"));

            // Test with a non-existent repo
            let mut cmd = Command::cargo_bin("unjust").unwrap();
            cmd.arg("use")
                .arg("nonexistent/repo")
                .assert()
                .failure()
                .stderr(predicate::str::contains("not found"));

            // Test with --separate-upstream-justfile flag
            let mut cmd = Command::cargo_bin("unjust").unwrap();
            cmd.arg("use")
                .arg("--separate-upstream-justfile")
                .arg("test/repo")
                .assert()
                .success();

            // Test with --force flag
            let mut cmd = Command::cargo_bin("unjust").unwrap();
            cmd.arg("use")
                .arg("--force")
                .arg("test/repo")
                .assert()
                .success();
        },
    );

    Ok(())
}

#[test]
fn test_cli_init_command() {
    // Test basic init
    let mut cmd = Command::cargo_bin("unjust").unwrap();
    cmd.arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Would initialize new Justfile"));

    // Test init with name
    let mut cmd = Command::cargo_bin("unjust").unwrap();
    cmd.arg("init")
        .arg("custom")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Would initialize new Justfile with name: custom",
        ));

    // Test init with template
    let mut cmd = Command::cargo_bin("unjust").unwrap();
    cmd.arg("init")
        .arg("-t")
        .arg("template")
        .assert()
        .success()
        .stdout(predicate::str::contains("Would use template: template"));
}

#[test]
fn test_cli_sync_command() {
    // Set up a test environment
    let temp_dir = setup_test_env();
    let cache_dir = temp_dir.path();

    // Use with_var to temporarily set the environment variable for this test
    with_var(
        "UNJUST_CACHE_DIR",
        Some(cache_dir.to_str().unwrap()),
        || {
            // Test sync with no arguments
            let mut cmd = Command::cargo_bin("unjust").unwrap();
            cmd.arg("sync")
                .assert()
                .success()
                .stdout(predicate::str::contains("Would sync all repos"));

            // Test sync with repo argument
            let mut cmd = Command::cargo_bin("unjust").unwrap();
            cmd.arg("sync")
                .arg("test/repo")
                .assert()
                .success()
                .stdout(predicate::str::contains("Would sync repo: test/repo"));

            // Test sync with --force-push flag
            let mut cmd = Command::cargo_bin("unjust").unwrap();
            cmd.arg("sync").arg("--force-push").assert().success();
        },
    );
}

// This test simulates a complete workflow using the CLI
#[test]
fn test_end_to_end_workflow() -> std::io::Result<()> {
    use std::process::Command as ProcessCommand;

    // Setup
    let temp_dir = setup_test_env();
    let cache_dir = temp_dir.path();
    let work_dir = TempDir::new()?;

    // Create a git repository for testing
    let repo_path = work_dir.path().join("test-repo");
    fs::create_dir_all(&repo_path)?;

    // Initialize git repository (skip if git is not available)
    if ProcessCommand::new("git")
        .args(["--version"])
        .status()
        .is_err()
    {
        println!("Skipping git-dependent test because git command failed");
        return Ok(());
    }

    // Use with_var to temporarily set the environment variable for this test
    with_var(
        "UNJUST_CACHE_DIR",
        Some(cache_dir.to_str().unwrap()),
        || {
            ProcessCommand::new("git")
                .args(["init"])
                .current_dir(&repo_path)
                .status()?;

            // Set git config for test
            ProcessCommand::new("git")
                .args(["config", "user.name", "Test User"])
                .current_dir(&repo_path)
                .status()?;

            ProcessCommand::new("git")
                .args(["config", "user.email", "test@example.com"])
                .current_dir(&repo_path)
                .status()?;

            // Now use our CLI to initialize a Justfile
            let mut cmd = Command::cargo_bin("unjust").unwrap();

            // Set the current directory to our test repo
            cmd.current_dir(&repo_path);

            // Initialize a new Justfile
            cmd.arg("init").assert().success();

            // Create a test repo in the cache (simulating a sync)
            create_test_repo(cache_dir, "test/repo", "# Test justfile for workflow")?;

            // Use the Justfile
            let mut cmd = Command::cargo_bin("unjust").unwrap();
            cmd.current_dir(&repo_path)
                .arg("use")
                .arg("test/repo")
                .assert()
                .success();

            // List available Justfiles
            let mut cmd = Command::cargo_bin("unjust").unwrap();
            cmd.current_dir(&repo_path)
                .arg("list")
                .assert()
                .success()
                .stdout(predicate::str::contains("test/repo"));

            Ok(())
        },
    )
}
