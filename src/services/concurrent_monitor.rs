//! Concurrent block/transaction monitor demonstrating multithreading with Tokio
//! Shows safe concurrent access via Arc + channels

use ethers::providers::Middleware;
use ethers::types::{Address, Transaction};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::core::{FrameworkError, Result};
use crate::providers::HttpProvider;

/// Concurrent monitor that watches for transactions involving specific addresses
/// Demonstrates multithreading: spawns background tasks that communicate via channels
pub struct ConcurrentMonitor {
    provider: Arc<HttpProvider>,
    target_addresses: Vec<Address>,
}

impl ConcurrentMonitor {
    pub fn new(provider: Arc<HttpProvider>, targets: Vec<Address>) -> Self {
        Self {
            provider,
            target_addresses: targets,
        }
    }

    /// Start monitoring in a background task.
    /// Returns a receiver for matching transactions and a JoinHandle.
    pub fn start(
        self,
        buffer_size: usize,
    ) -> (mpsc::Receiver<Transaction>, JoinHandle<Result<()>>) {
        let (tx, rx) = mpsc::channel(buffer_size);

        let handle = tokio::spawn(async move {
            self.run(tx).await
        });

        (rx, handle)
    }

    async fn run(self, tx: mpsc::Sender<Transaction>) -> Result<()> {
        let provider = self.provider.inner();
        let targets: Vec<_> = self
            .target_addresses
            .iter()
            .map(|a| format!("{:?}", a).to_lowercase())
            .collect();

        // Polling-based monitor (for HTTP; WS would use subscriptions)
        let mut last_block = provider
            .get_block_number()
            .await
            .map_err(|e| FrameworkError::Provider(e.to_string()))?;

        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

            let current = provider
                .get_block_number()
                .await
                .map_err(|e| FrameworkError::Provider(e.to_string()))?;

            if current > last_block {
                for block_num in (last_block.as_u64() + 1)..=current.as_u64() {
                    if let Ok(Some(block)) = provider
                        .get_block_with_txs(ethers::types::BlockNumber::Number(block_num.into()))
                        .await
                    {
                        for tx_in_block in block.transactions {
                            let from_match = targets
                                .iter()
                                .any(|t| format!("{:?}", tx_in_block.from).to_lowercase() == *t);
                            let to_match = tx_in_block
                                .to
                                .map(|to| {
                                    targets
                                        .iter()
                                        .any(|t| format!("{:?}", to).to_lowercase() == *t)
                                })
                                .unwrap_or(false);

                            if from_match || to_match {
                                if tx.send(tx_in_block).await.is_err() {
                                    // Receiver dropped
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
                last_block = current;
            }
        }
    }
}

/// Utility: run multiple concurrent read operations safely
pub async fn concurrent_balance_checks(
    provider: Arc<dyn crate::core::BlockchainProvider>,
    addresses: Vec<Address>,
) -> Result<Vec<(Address, ethers::types::U256)>> {
    let mut handles = Vec::new();

    for addr in addresses {
        let provider = Arc::clone(&provider);
        let handle = tokio::spawn(async move {
            let balance = provider.get_balance(addr, None).await?;
            Ok::<_, FrameworkError>((addr, balance))
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(Ok(pair)) => results.push(pair),
            Ok(Err(e)) => return Err(e),
            Err(e) => return Err(FrameworkError::Other(eyre::eyre!("Join error: {}", e))),
        }
    }

    Ok(results)
}
