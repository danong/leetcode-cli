mod leetcode;

use clap::{Parser, Subcommand};
use std::process::ExitCode;

/// Simple CLI to interface with Leetcode
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Download a problem locally
    Download { id: u32 },
    /// Submit a solution to Leetcode's servers
    Submit { id: u32 },
    /// List available problems
    List,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Download { id }) => match leetcode::download(id) {
            Ok(question) => println!("{}", question.content),
            Err(error) => {
                eprintln!("Download failed: {error}");
                return ExitCode::FAILURE;
            }
        },
        Some(Commands::Submit { id }) => {
            println!("Submitting {id}");
        }
        Some(Commands::List) => match leetcode::get_all_questions() {
            Ok(map) => {
                let mut first_ten: Vec<_> = map.iter().collect();
                first_ten.sort_by_key(|&(k, _)| k);
                println!("First 10 problems:");
                println!("{:#?}", &first_ten[..10.min(first_ten.len())]);
            }
            Err(error) => {
                eprintln!("Download failed: {error}");
                return ExitCode::FAILURE;
            }
        },
        None => {}
    }

    ExitCode::SUCCESS
}
