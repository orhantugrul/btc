use serde::{Deserialize, Serialize};
use std::fmt::{self};

#[derive(Serialize, Deserialize, Clone)]
pub struct Rate {
    #[serde(rename(deserialize = "15m"))]
    pub d15m: f64,
    pub last: f64,
    pub buy: f64,
    pub sell: f64,
    pub symbol: String,
}

impl Rate {
    pub fn empty() -> Rate {
        Rate {
            d15m: 0.0,
            last: 0.0,
            buy: 0.0,
            sell: 0.0,
            symbol: String::new(),
        }
    }
}

impl fmt::Display for Rate {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "15m: {}, last: {}, buy: {}, sell: {}, symbol: \"{}\"",
            self.d15m, self.last, self.buy, self.sell, self.symbol
        )
    }
}
