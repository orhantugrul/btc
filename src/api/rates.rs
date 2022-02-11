use crate::model::rate::Rate;
use std::collections::HashMap;

const URL: &str = "https://blockchain.info/ticker";

pub type Rates = HashMap<String, Rate>;

pub async fn get_rates() -> Result<Rates, Box<dyn std::error::Error>> {
    let response = reqwest::get(URL).await?.json().await?;
    let rates: Rates = serde_json::from_value(response)?;

    Ok(rates)
}

pub async fn get_rate_by_pair(
    pair: &str
) -> Result<Rate, Box<dyn std::error::Error>> {
    let rate = get_rates()
        .await?
        .get(pair)
        .map_or_else(Rate::empty, |rate| rate.clone());

    Ok(rate)
}