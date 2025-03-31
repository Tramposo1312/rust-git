use clap::{Parser, Subcommand};
mod commands;

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

    Commit {
        #[arg(short, long)]
        message: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init {}) => {
            commands::init::execute()?;
        }
        Some(Commands::Add { files }) => {
            commands::add::execute(files)?;
        }
        Some(Commands::Status {}) => {
            commands::status::execute()?;
        }
        Some(Commands::Commit { message }) => {
            commands::commit::execute(message)?;
        }
        None => {
            println!("No command provided. Use --help for available commands")
        }
    }

    Ok(())
}
