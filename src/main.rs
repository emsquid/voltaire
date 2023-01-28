use clap::Parser;

mod options;
mod result;
mod voltaire;

#[tokio::main]
async fn main() {
    let options = options::Options::parse();

    match voltaire::Voltaire::from(&options).await {
        Ok(voltaire) => voltaire.print(),
        Err(err) => eprintln!("{err}"),
    };
}
