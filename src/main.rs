mod api;
mod model;

use api::rates::{get_rate_by_pair, get_rates};
use model::{args::Args, args::Parser};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.pair {
        Some(pair) => println!("{}", get_rate_by_pair(&pair).await?),
        None => get_rates()
            .await?
            .values()
            .for_each(|rate| println!("{}", rate)),
    };

    Ok(())
}
