use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Alignment file in FASTA format
    #[arg(short, long, default_value = "tests/test_files_in/listeria0.aln")]
    pub alignment: String,

    /// Write the likelihood of the tree and alignment, do not optimise
    #[arg(long, default_value_t = false)]
    pub no_optimise: bool,
}

/// Function to parse command line args into [`Args`] struct
pub fn cli_args() -> Args {
    Args::parse()
}
