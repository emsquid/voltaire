use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Options {
    /// Text to analyze
    pub text: String,

    /// Show more informations
    #[arg(short, long)]
    pub verbose: bool,
}
