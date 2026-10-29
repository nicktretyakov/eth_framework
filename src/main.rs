//! Ethereum Smart Contract Framework in Rust
//! 
//! Demonstrates:
//! - ethers-rs for EVM interaction
//! - OOP-style design via traits & structs
//! - SOLID principles
//! - Multithreading / concurrency with Tokio
//!
//! Architecture:
//!   core/       → Traits (abstractions) + Error types   [DIP, ISP]
//!   providers/  → HTTP / WS providers                   [SRP, OCP]
//!   signers/    → Local wallet signer                  [SRP]
//!   contracts/  → Generic + ERC20 bindings             [SRP]
//!   services/   → High-level facades + concurrent monitor
//!   utils/      → Helpers

mod core;
mod providers;
mod signers;
mod contracts;
mod services;
mod utils;

use std::sync::Arc;
use log::{info, warn};

use crate::core::BlockchainProvider;
use crate::providers::HttpProvider;
use crate::services::{EthService, concurrent_balance_checks};
use crate::utils::{parse_address, wei_to_eth};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("=== Ethereum Smart Contract Framework (Rust + ethers-rs) ===\n");
    println!("SOLID + OOP + Multithreading demonstration\n");

    // ------------------------------------------------------------------
    // 1. Configuration (in production use env vars / config files)
    // ------------------------------------------------------------------
    // Public RPC endpoints (no API key required for demo)
    let rpc_url = "https://eth.llamarpc.com"; // Ethereum mainnet public RPC
    // Alternative testnets: https://rpc.sepolia.org , https://rpc.ankr.com/eth_goerli

    // WETH contract on mainnet (for read-only demo)
    let weth_address = parse_address("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")?;

    // Example addresses for concurrent balance checks
    let demo_addresses = vec![
        parse_address("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")?, // WETH
        parse_address("0xdAC17F958D2ee523a2206206994597C13D831ec7")?, // USDT
        parse_address("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")?, // USDC
    ];

    // ------------------------------------------------------------------
    // 2. Read-only service (no private key needed)
    // ------------------------------------------------------------------
    info!("Creating read-only EthService...");
    let service = EthService::read_only(rpc_url)?;

    // Basic chain info
    let chain_id = service.get_chain_id().await?;
    let block_number = service.get_block_number().await?;
    info!("Connected to chain_id = {}, current block = {}", chain_id, block_number);

    // ------------------------------------------------------------------
    // 3. Concurrent balance checks (multithreading demo)
    // ------------------------------------------------------------------
    info!("Running concurrent balance checks for {} addresses...", demo_addresses.len());

    let provider: Arc<dyn BlockchainProvider> = Arc::new(HttpProvider::new(rpc_url)?);
    let balances = concurrent_balance_checks(provider, demo_addresses.clone()).await?;

    for (addr, bal) in &balances {
        println!(
            "  Address {:?}: {} ETH ({} wei)",
            addr,
            wei_to_eth(*bal),
            bal
        );
    }

    // ------------------------------------------------------------------
    // 4. Type-safe ERC20 interaction (WETH)
    // ------------------------------------------------------------------
    info!("Interacting with WETH contract via type-safe bindings...");

    // For abigen contracts we need a Middleware. Use the underlying provider.
    let http_provider = HttpProvider::new(rpc_url)?;
    let client = Arc::new(http_provider.inner().as_ref().clone());

    use crate::contracts::Erc20Service;
    let weth = Erc20Service::new(weth_address, client);

    match weth.name().await {
        Ok(name) => println!("  Token name   : {}", name),
        Err(e) => warn!("  Could not fetch name: {}", e),
    }
    match weth.symbol().await {
        Ok(symbol) => println!("  Token symbol : {}", symbol),
        Err(e) => warn!("  Could not fetch symbol: {}", e),
    }
    match weth.decimals().await {
        Ok(decimals) => println!("  Decimals     : {}", decimals),
        Err(e) => warn!("  Could not fetch decimals: {}", e),
    }
    match weth.total_supply().await {
        Ok(supply) => println!("  Total supply : {} (raw)", supply),
        Err(e) => warn!("  Could not fetch totalSupply: {}", e),
    }

    // ------------------------------------------------------------------
    // 5. Example of sending a transaction (commented – requires private key)
    // ------------------------------------------------------------------
    /*
    // WARNING: Never hardcode private keys in production code!
    // Use environment variables or secure key stores.
    let private_key = std::env::var("PRIVATE_KEY")
        .expect("Set PRIVATE_KEY env var");
    let chain_id = 1u64; // mainnet

    let full_service = EthService::with_signer(rpc_url, &private_key, chain_id)?;
    let from = full_service.address().unwrap();
    info!("Signer address: {:?}", from);

    // Example: send 0.001 ETH to another address
    let to = parse_address("0x...")?;
    let tx = TransactionRequest::new()
        .to(to)
        .value(U256::from(1_000_000_000_000_000u64)); // 0.001 ETH

    let receipt = full_service.send_transaction(tx).await?;
    info!("Transaction mined: {:?}", receipt.transaction_hash);
    info!("  Block number : {:?}", receipt.block_number);
    info!("  Gas used     : {:?}", receipt.gas_used);
    */

    // ------------------------------------------------------------------
    // 6. Architecture summary
    // ------------------------------------------------------------------
    println!("\n=== Framework Architecture (SOLID) ===");
    println!("• Single Responsibility : each module has one clear job");
    println!("• Open/Closed           : add new providers/signers via traits");
    println!("• Liskov Substitution   : any BlockchainProvider can be swapped");
    println!("• Interface Segregation : small focused traits (Provider, Signer, Sender)");
    println!("• Dependency Inversion  : high-level services depend on traits, not concretes");
    println!("\n• Multithreading        : Tokio tasks + Arc + channels for concurrent RPC calls");
    println!("• ethers-rs             : type-safe ABI bindings (abigen), middleware, providers");

    println!("\nDemo completed successfully.");
    Ok(())
}
