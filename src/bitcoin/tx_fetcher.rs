use std::collections::HashMap;
use std::io::Cursor;
use reqwest::blocking::get;
use crate::bitcoin::tx::tx::{Network, Tx};

pub struct TxFetcher {
    cache: HashMap<String, Tx>,
}

impl TxFetcher {

    pub fn new() -> TxFetcher {
        Self {
            cache: HashMap::new()
        }
    }

    pub fn get_url(network: Network) -> String {
        match network {
            Network::MAINNET => String::from("https://mempool.space/api"),
            Network::TESTNET => String::from("https://mempool.space/testnet4/api"),
        }
    }

    pub fn fetch(&mut self, tx_id: &str, network: Network) -> Result<&Tx, Box<dyn std::error::Error>> {

        if !self.cache.contains_key(tx_id) {
            let url = format!("{}/tx/{}/hex",Self::get_url(network), tx_id);
            let body = get(url)?.text()?;
            let tx_bytes = hex::decode(body.trim())?;
            let mut stream = Cursor::new(tx_bytes);
            let tx = Tx::parse(&mut stream)?;

            self.cache.insert(tx_id.to_string(), tx);
        }

        Ok(self.cache.get(tx_id).unwrap())
    }
}

#[cfg(test)]
mod tx_fetcher_tests {
    use crate::bitcoin::tx_fetcher::TxFetcher;

    #[test]
    fn fetch_valid() -> Result<(), Box<dyn std::error::Error>> {
        let mut fetcher = TxFetcher::new();
        let tx = fetcher.fetch("9e067aedc661fca148e13953df75f8ca6eada9ce3b3d8d68631769ac60999156", crate::bitcoin::tx::tx::Network::MAINNET)?;

        assert_eq!(tx.tx_ins.len(), 1);
        assert_eq!(tx.tx_outs.len(), 5);
        assert_eq!(tx.id, "9e067aedc661fca148e13953df75f8ca6eada9ce3b3d8d68631769ac60999156");

        Ok(())
    }
}