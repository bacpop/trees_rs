
use clap::{ArgGroup, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File to mutate (fasta with single contig)
    #[arg(short, long, default_value = "listeria0.aln")]
    pub alignment: String,
}

/// Function to parse command line args into [`Args`] struct
pub fn cli_args() -> Args {
    Args::parse()
}
