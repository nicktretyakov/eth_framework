//! Utility helpers

use ethers::types::{Address, U256};
use std::str::FromStr;

use crate::core::{FrameworkError, Result};

/// Parse an Ethereum address from string
pub fn parse_address(s: &str) -> Result<Address> {
    Address::from_str(s).map_err(|e| FrameworkError::Config(format!("Invalid address: {}", e)))
}

/// Convert wei to ether (as f64, for display only)
pub fn wei_to_eth(wei: U256) -> f64 {
    let wei_f = wei.as_u128() as f64;
    wei_f / 1e18
}

/// Convert ether to wei
pub fn eth_to_wei(eth: f64) -> U256 {
    U256::from((eth * 1e18) as u128)
}
