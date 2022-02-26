mod cli;
mod read;

use clap::Parser;
use cli::{Cli,Command};
use read::{display_info, cat};

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Info(cmd) => {
            
            display_info(&cmd.file)?;
            Ok(())
        }      
        Command::Cat(cmd) => {
            cat(&cmd.file)?;
            Ok(())
        }
    }
}


