use clap::Parser;

#[derive(Parser, Clone)]
#[command(author, version, about)]
pub struct Options {
    /// Text to analyze
    pub text: String,

    /// Maximum number of corrections
    #[arg(short, long, default_value_t = 3, value_parser = clap::value_parser!(u8).range(1..))]
    pub number: u8,
    /// show more informations
    #[arg(short, long)]
    pub verbose: bool,
}
