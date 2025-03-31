use clap::{Parser, Subcommand};
mod commands;
mod utils;

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

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Some(Commands::Init {}) => commands::init::execute(),
        Some(Commands::Add { files }) => commands::add::execute(files),
        Some(Commands::Status {}) => commands::status::execute(),
        Some(Commands::Commit { message }) => commands::commit::execute(message),
        None => {
            println!("No command provided. Try --help for available commands.");
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);

        if e.to_string().contains("UTF-8") {
            eprintln!("This may be caused by non-text files or encoding issues in the repository.");
            eprintln!("Try using the fixed version of this tool.");
        }

        std::process::exit(1);
    }
}
