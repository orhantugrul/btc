pub use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "btc")]
#[clap(author = "Orhan Tugrul <orhan.tugrul.61@gmail.com>")]
#[clap(version = "1.0")]
#[clap(author, version, about)]
pub struct Args {
    /// Specific currency pair target
    #[clap(short, long)]
    pub pair: Option<String>,

    /// Watch data feed recursively
    #[clap(short, long)]
    pub watch: bool,
}
