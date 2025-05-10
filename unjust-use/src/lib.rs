use console::style;
use facet::Facet;
use unjust_core::find_justfile;
use which::which;

/// Arguments for the "use" command
#[derive(Facet, Debug)]
pub struct UseArgs<'a> {
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

/// Handle the "use" command
///
/// Returns the exit code (0 for success, 1 for error)
pub fn handle_use_command(args: &[&str]) -> i32 {
    // Parse arguments
    let use_args = match facet_args::from_slice::<UseArgs>(args) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{} {}", style("Error parsing arguments:").red().bold(), e);
            return 1;
        }
    };

    // Get the repo name
    let repo = match use_args.repo {
        Some(repo) => repo,
        None => {
            eprintln!("{} Repository not specified", style("Error:").red().bold());
            return 1;
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
                return 1;
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
            0
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
            1
        }
        Err(e) => {
            eprintln!("{} {}", style("Error:").red().bold(), e);
            1
        }
    }
}
