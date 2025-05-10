use console::style;
use facet::Facet;
use facet_args::from_slice;
use std::env;
use std::process::exit;
use unjust_core::{ensure_cache_dir, find_justfile, is_first_use};
use unjust_list::handle_list_command;
use which::which;

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
            handle_use_command(command_args);
        }
        "sync" => {
            handle_sync_command(command_args);
        }
        "init" => {
            handle_init_command(command_args);
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

fn handle_use_command(args: &[&str]) {
    // Parse arguments
    let use_args = match from_slice::<UseArgs>(args) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{} {}", style("Error parsing arguments:").red().bold(), e);
            exit(1);
        }
    };

    // Get the repo name
    let repo = match use_args.repo {
        Some(ref repo) => repo,
        None => {
            eprintln!("{} Repository not specified", style("Error:").red().bold());
            exit(1);
        }
    };

    // Find the Justfile
    match find_justfile(repo, use_args.separate_upstream_justfile) {
        Ok(Some(justfile)) => {
            // Check if just is installed
            if which("just").is_err() {
                eprintln!(
                    "{} 'just' command not found. Please install just: https://github.com/casey/just",
                    style("Error:").red().bold()
                );
                exit(1);
            }

            println!(
                "{} Using Justfile from: {}",
                style("Success:").green().bold(),
                justfile.path.display()
            );

            // In a real implementation, we would actually execute just here
            println!(
                "Would execute just with Justfile: {}",
                justfile.path.display()
            );
        }
        Ok(None) => {
            eprintln!(
                "{} Justfile not found for repo: {}",
                style("Error:").red().bold(),
                repo
            );
            eprintln!(
                "Run '{} sync {}' to sync from remote",
                style("unjust").green(),
                style(repo).green()
            );
            exit(1);
        }
        Err(e) => {
            eprintln!("{} {}", style("Error:").red().bold(), e);
            exit(1);
        }
    }
}

fn handle_sync_command(args: &[&str]) {
    // Parse arguments
    let sync_args = match from_slice::<SyncArgs>(args) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{} {}", style("Error parsing arguments:").red().bold(), e);
            exit(1);
        }
    };

    println!(
        "{} Sync command not fully implemented yet",
        style("Note:").yellow().bold()
    );

    if let Some(ref repo) = sync_args.repo {
        println!("Would sync repo: {}", repo);
    } else {
        println!("Would sync all repos");
    }
}

fn handle_init_command(args: &[&str]) {
    // Parse arguments
    let init_args = match from_slice::<InitArgs>(args) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{} {}", style("Error parsing arguments:").red().bold(), e);
            exit(1);
        }
    };

    println!(
        "{} Init command not fully implemented yet",
        style("Note:").yellow().bold()
    );

    let name = init_args.name.unwrap();
    println!("Would initialize new Justfile with name: {}", name);

    if let Some(ref template) = init_args.template {
        println!("Would use template: {}", template);
    } else {
        println!("Would create a basic Justfile");
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

/// Arguments for the "use" command
#[derive(Facet, Debug)]
struct UseArgs<'a> {
    /// Repo identifier (username/repo)
    #[facet(positional)]
    pub repo: Option<&'a str>,

    /// Store upstream and current repo Justfiles separately
    #[facet(named)]
    pub separate_upstream_justfile: bool,

    /// Force refresh from remote
    #[facet(named, short = 'f')]
    pub force: bool,
}

/// Arguments for the "sync" command
#[derive(Facet, Debug)]
struct SyncArgs<'a> {
    /// Repo to sync (if specific)
    #[facet(positional)]
    pub repo: Option<&'a str>,

    /// Force push to remote repo
    #[facet(named)]
    pub force_push: bool,
}

/// Arguments for the "init" command
#[derive(Facet, Debug)]
struct InitArgs {
    /// Name for the new Justfile
    #[facet(positional, default)]
    pub name: Option<String>,

    /// Template to use (existing Justfile name)
    #[facet(named, short = 't')]
    pub template: Option<String>,
}

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
