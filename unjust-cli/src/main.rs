use console::style;
use std::env;
use std::process::exit;
use unjust_core::{ensure_cache_dir, is_first_use};
use unjust_init::handle_init_command;
use unjust_list::handle_list_command;
use unjust_sync::handle_sync_command;
use unjust_use::handle_use_command;

fn main() {
    // Get command line arguments, skipping the program name
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage(&args[0]);
        exit(1);
    }

    // Convert args to string slices for facet_args
    let args_slice: Vec<&str> = args.iter().skip(1).map(|s| s.as_str()).collect();

    // First-time setup
    if is_first_use() {
        match ensure_cache_dir() {
            Ok(_) => println!("{}", style("Initialized unjust cache directory.").green()),
            Err(e) => {
                eprintln!("{} {}", style("Error:").red().bold(), e);
                exit(1);
            }
        }
    }

    // Parse the command
    let command = args_slice[0];
    let command_args = &args_slice[1..];

    match command {
        "use" => {
            let exit_code = handle_use_command(command_args);
            exit(exit_code);
        }
        "sync" => {
            let exit_code = handle_sync_command(command_args);
            exit(exit_code);
        }
        "init" => {
            let exit_code = handle_init_command(command_args);
            exit(exit_code);
        }
        "list" => {
            let exit_code = handle_list_command(command_args);
            exit(exit_code);
        }
        _ => {
            eprintln!(
                "{} Unknown command: {}",
                style("Error:").red().bold(),
                command
            );
            print_usage(&args[0]);
            exit(1);
        }
    }
}

fn print_usage(program: &str) {
    eprintln!(
        "{} {}",
        style("Usage:").yellow().bold(),
        style(format!("{} [command] [options]", program)).bold()
    );
    eprintln!("\n{}:", style("Commands").yellow().bold());
    eprintln!(
        "  {} [--separate-upstream-justfile] [--force|-f] <repo>",
        style("use").green()
    );
    eprintln!("      Use a Justfile from remote storage");
    eprintln!("  {} [--force-push] [repo]", style("sync").green());
    eprintln!("      Sync Justfiles with remote storage");
    eprintln!("  {} [-t template] [name]", style("init").green());
    eprintln!("      Initialize a new Justfile for the current repo");
    eprintln!("  {} [--paths|-p]", style("list").green());
    eprintln!("      List available Justfiles");
}

// Command argument structs using facet

#[cfg(test)]
mod tests {
    // use super::*;
    use assert_cmd::Command;
    use predicates::prelude::*;
    use tempfile::TempDir;

    #[test]
    #[ignore]
    fn test_cli_no_args() {
        let mut cmd = Command::cargo_bin("unjust").unwrap();

        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Usage:"));
    }

    #[test]
    #[ignore]
    fn test_cli_unknown_command() {
        let mut cmd = Command::cargo_bin("unjust").unwrap();

        cmd.arg("unknown")
            .assert()
            .failure()
            .stderr(predicate::str::contains("Error:"))
            .stderr(predicate::str::contains("Unknown command"));
    }

    #[test]
    #[ignore]
    fn test_cli_list_command() {
        // Set up a test cache directory with some justfiles
        let _temp_dir = TempDir::new().unwrap();
        // Again, we'd need to patch the cache directory location

        let mut cmd = Command::cargo_bin("unjust").unwrap();

        cmd.arg("list").assert().success();
        // In a real test, we'd verify the output contains our test repos
    }

    #[test]
    #[ignore]
    fn test_cli_use_command() {
        let mut cmd = Command::cargo_bin("unjust").unwrap();

        // Test without a repo argument (should fail)
        cmd.arg("use")
            .assert()
            .failure()
            .stderr(predicate::str::contains("Repository not specified"));

        // Test with a non-existent repo
        cmd.arg("use")
            .arg("nonexistent/repo")
            .assert()
            .failure()
            .stderr(predicate::str::contains("not found"));
    }

    #[test]
    #[ignore]
    fn test_cli_init_command() {
        let mut cmd = Command::cargo_bin("unjust").unwrap();

        // Test basic init
        cmd.arg("init")
            .assert()
            .success()
            .stdout(predicate::str::contains("Would initialize new Justfile"));

        // Test init with name
        cmd.arg("init")
            .arg("custom")
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "Would initialize new Justfile with name: custom",
            ));
    }

    // These are integration tests that test multiple components together

    // unjust-core/tests/integration/mod.rs
    // This file would contain integration tests that span multiple modules

    // Example integration test:
    // #[test]
    // #[ignore]
    // fn test_full_workflow() {
    //     // 1. Initialize a cache directory
    //     // 2. Create a repository
    //     // 3. Add a Justfile
    //     // 4. List Justfiles
    //     // 5. Use the Justfile
    //     // Each step would verify the expected state
    // }

    // You might also want property-based tests for more complex behavior
    // Example with test-case for different argument patterns:

    // #[test_case("repo", false, false ; "basic repo")]
    // #[test_case("user/repo", true, false ; "namespaced repo without force")]
    // #[test_case("user/repo", true, true ; "namespaced repo with force")]
    // fn test_use_command_variations(repo: &str, separate: bool, force: bool) {
    //     // Test the use command with different parameter combinations
    // }
}
