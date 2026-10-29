//! HTTP Provider implementation of BlockchainProvider trait
//! Demonstrates Open/Closed: new provider types can be added without changing existing code

use async_trait::async_trait;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{
    transaction::eip2718::TypedTransaction, Address, BlockId, BlockNumber, H256,
    TransactionReceipt, TransactionRequest, U256,
};
use std::sync::Arc;

use crate::core::{BlockchainProvider, FrameworkError, Result};

/// HTTP-based Ethereum provider
/// SRP: Only responsible for HTTP RPC communication
pub struct HttpProvider {
    inner: Arc<Provider<Http>>,
}

impl HttpProvider {
    /// Create a new HTTP provider from RPC URL
    pub fn new(rpc_url: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)
            .map_err(|e| FrameworkError::Provider(format!("Failed to create HTTP provider: {}", e)))?;
        Ok(Self {
            inner: Arc::new(provider),
        })
    }

    /// Get the underlying ethers provider (for advanced use)
    pub fn inner(&self) -> Arc<Provider<Http>> {
        Arc::clone(&self.inner)
    }
}

#[async_trait]
impl BlockchainProvider for HttpProvider {
    async fn get_block_number(&self) -> Result<U256> {
        self.inner
            .get_block_number()
            .await
            .map(|n| U256::from(n.as_u64()))
            .map_err(|e| FrameworkError::Provider(e.to_string()))
    }

    async fn get_balance(&self, address: Address, block: Option<BlockNumber>) -> Result<U256> {
        let block_id: Option<BlockId> = block.map(|b| b.into());
        self.inner
            .get_balance(address, block_id)
            .await
            .map_err(|e| FrameworkError::Provider(e.to_string()))
    }

    async fn get_chain_id(&self) -> Result<U256> {
        self.inner
            .get_chainid()
            .await
            .map_err(|e| FrameworkError::Provider(e.to_string()))
    }

    async fn estimate_gas(&self, tx: &TransactionRequest) -> Result<U256> {
        let typed: TypedTransaction = tx.clone().into();
        self.inner
            .estimate_gas(&typed, None)
            .await
            .map_err(|e| FrameworkError::Provider(e.to_string()))
    }

    async fn get_transaction_receipt(&self, tx_hash: H256) -> Result<Option<TransactionReceipt>> {
        self.inner
            .get_transaction_receipt(tx_hash)
            .await
            .map_err(|e| FrameworkError::Provider(e.to_string()))
    }
}
