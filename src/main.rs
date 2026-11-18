use clap::{Parser, Subcommand};

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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Download { id }) => {
            println!("Downloading {id}");
        }
        Some(Commands::Submit { id }) => {
            println!("Submitting {id}");
        }
        Some(Commands::List {}) => {
            println!("Available IDs:");
            for i in 0..10 {
                println!("ID: {i}");
            }
        }
        None => {}
    }
}
