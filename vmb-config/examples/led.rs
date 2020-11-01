use vmb_config::parse;
use std::env;

use std::convert::TryFrom;
use std::collections::HashMap;

const DEVICE: &str = "led";

#[derive(Debug)]
struct LedConfig {
    address: Option<u16>,
    path: Option<String>,
    filename: Option<String>
}

#[derive(Debug)]
enum Error {
    Address,
}

impl TryFrom<HashMap<String, String>> for LedConfig {
    type Error = Error;

    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        let address = match value.get("address") {
            Some(address) => Some(address.parse::<u16>().map_err(|_| Error::Address)?),
            None => None
        };

        Ok(LedConfig {
            address,
            path: value.get("path").map(|s| s.clone()),
            filename: value.get("filename").map(|s| s.clone())
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let config = parse::<LedConfig, _>(filename.to_string(), DEVICE.to_string());
    println!("{:?}", config);
}
