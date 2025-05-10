use console::style;
use facet::Facet;
use facet_args::from_slice;
use unjust_core::list_justfiles;

/// Arguments for the "list" command
#[derive(Facet, Debug)]
pub struct ListArgs {
    /// Show full paths
    #[facet(named, short = 'p')]
    pub paths: bool,
}

/// Handle the "list" command
///
/// Returns the exit code (0 for success, 1 for error)
pub fn handle_list_command(args: &[&str]) -> i32 {
    // Parse arguments
    let list_args = match from_slice::<ListArgs>(args) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{} {}", style("Error parsing arguments:").red().bold(), e);
            return 1;
        }
    };

    match list_justfiles() {
        Ok(justfiles) => {
            if justfiles.is_empty() {
                println!(
                    "No Justfiles found. Use '{}' to add one.",
                    style("unjust sync <repo>").green()
                );
                return 0;
            }

            println!("{} Available Justfiles:", style("Success:").green().bold());
            for (i, justfile) in justfiles.iter().enumerate() {
                if list_args.paths {
                    println!(
                        "{}. {} ({})",
                        i + 1,
                        style(&justfile.repo_name).green(),
                        justfile.path.display()
                    );
                } else {
                    println!("{}. {}", i + 1, style(&justfile.repo_name).green());
                }
            }
            0
        }
        Err(e) => {
            eprintln!("{} {}", style("Error:").red().bold(), e);
            1
        }
    }
}
