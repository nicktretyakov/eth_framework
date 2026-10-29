//! WebSocket Provider for real-time subscriptions
//! Demonstrates extensibility (Open/Closed Principle)

use async_trait::async_trait;
use ethers::providers::{Middleware, Provider, Ws};
use ethers::types::{
    transaction::eip2718::TypedTransaction, Address, BlockId, BlockNumber, H256,
    TransactionReceipt, TransactionRequest, U256,
};
use std::sync::Arc;

use crate::core::{BlockchainProvider, FrameworkError, Result};

/// WebSocket-based Ethereum provider
/// Useful for event streaming and real-time monitoring
pub struct WsProvider {
    inner: Arc<Provider<Ws>>,
}

impl WsProvider {
    /// Connect to a WebSocket endpoint
    pub async fn connect(ws_url: &str) -> Result<Self> {
        let provider = Provider::<Ws>::connect(ws_url)
            .await
            .map_err(|e| FrameworkError::Provider(format!("WS connection failed: {}", e)))?;
        Ok(Self {
            inner: Arc::new(provider),
        })
    }

    pub fn inner(&self) -> Arc<Provider<Ws>> {
        Arc::clone(&self.inner)
    }
}

#[async_trait]
impl BlockchainProvider for WsProvider {
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
