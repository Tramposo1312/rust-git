use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init {},
    Add {
        #[arg(required = true)]
        files: Vec<String>,
    },
    Status {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init {}) => {
            println!("Initialising repository...");
            //...
        }
        Some(Commands::Add { files }) => {
            println!("Adding files to staging area:");
            for file in files {
                println!(" {}", file);
            }
            //...
        }
        Some(Commands::Status {}) => {
            println!("SHowing repository status...")
        }
        None => {
            println!("No command provided. Use --help for available commands")
        }
    }
}
