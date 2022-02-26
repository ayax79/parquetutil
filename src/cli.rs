use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
/// Provides tools for working with parquet
pub struct Cli {

    #[clap(subcommand)]
    pub command: Command
}


#[derive(Subcommand, Debug)]
pub enum Command {
    Info(InfoCommand),
    Cat(CatCommand)
}

/// Outputs the contents of a parcarquet file
#[derive(Debug, Args)]
pub struct InfoCommand {
    /// Parquet file to show the contents of
    #[clap(short = 'f', long = "file")]
    pub file: String
}

#[derive(Debug, Args)]
pub struct CatCommand {
    #[clap(short = 'f', long = "file")]
    pub file: String
}