use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Where to find file
    #[arg(short, long)]
    pub directory: String,
    /// File name will be found
    #[arg(short, long)]
    pub filename: String,
}
