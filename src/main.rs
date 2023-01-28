use clap::Parser;

mod options;
mod result;
mod voltaire;

#[tokio::main]
async fn main() {
    let options = options::Options::parse();

    match voltaire::Voltaire::from(options.text).await {
        Ok(voltaire) => voltaire.print(options.verbose),
        Err(err) => eprintln!("{err}"),
    };
}
