//! Unified Ethereum service combining provider + signer
//! Facade pattern + DIP

use async_trait::async_trait;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::LocalWallet;
use ethers::types::{
    Address, BlockNumber, H256, TransactionReceipt, TransactionRequest, U256,
};
use std::sync::Arc;

use crate::core::{
    BlockchainProvider, FrameworkError, Result, TransactionSender, TransactionSigner,
};
use crate::providers::HttpProvider;
use crate::signers::LocalSigner;

/// High-level service that combines provider and optional signer
/// This is the main entry point for most applications
pub struct EthService {
    provider: Arc<HttpProvider>,
    signer: Option<Arc<LocalSigner>>,
    client: Option<Arc<SignerMiddleware<Provider<Http>, LocalWallet>>>,
}

impl EthService {
    /// Create a read-only service (no signer)
    pub fn read_only(rpc_url: &str) -> Result<Self> {
        let provider = Arc::new(HttpProvider::new(rpc_url)?);
        Ok(Self {
            provider,
            signer: None,
            client: None,
        })
    }

    /// Create a full service with signing capability
    pub fn with_signer(rpc_url: &str, private_key: &str, chain_id: u64) -> Result<Self> {
        let provider = Arc::new(HttpProvider::new(rpc_url)?);
        let signer = Arc::new(LocalSigner::from_private_key(private_key, chain_id)?);

        let client = SignerMiddleware::new(
            provider.inner().as_ref().clone(),
            signer.wallet().clone(),
        );

        Ok(Self {
            provider,
            signer: Some(signer),
            client: Some(Arc::new(client)),
        })
    }

    /// Get the address of the configured signer
    pub fn address(&self) -> Option<Address> {
        self.signer.as_ref().map(|s| s.address())
    }
}

#[async_trait]
impl BlockchainProvider for EthService {
    async fn get_block_number(&self) -> Result<U256> {
        self.provider.get_block_number().await
    }

    async fn get_balance(&self, address: Address, block: Option<BlockNumber>) -> Result<U256> {
        self.provider.get_balance(address, block).await
    }

    async fn get_chain_id(&self) -> Result<U256> {
        self.provider.get_chain_id().await
    }

    async fn estimate_gas(&self, tx: &TransactionRequest) -> Result<U256> {
        self.provider.estimate_gas(tx).await
    }

    async fn get_transaction_receipt(&self, tx_hash: H256) -> Result<Option<TransactionReceipt>> {
        self.provider.get_transaction_receipt(tx_hash).await
    }
}

#[async_trait]
impl TransactionSender for EthService {
    async fn send_transaction(&self, tx: TransactionRequest) -> Result<TransactionReceipt> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| FrameworkError::Signer("No signer configured".into()))?;

        let pending = client
            .send_transaction(tx, None)
            .await
            .map_err(|e| FrameworkError::Transaction(e.to_string()))?;

        pending
            .await
            .map_err(|e| FrameworkError::Transaction(e.to_string()))?
            .ok_or_else(|| FrameworkError::Transaction("Transaction dropped".into()))
    }

    async fn send_transaction_async(&self, tx: TransactionRequest) -> Result<H256> {
        let client = self
            .client
            .as_ref()
            .ok_or_else(|| FrameworkError::Signer("No signer configured".into()))?;

        let pending = client
            .send_transaction(tx, None)
            .await
            .map_err(|e| FrameworkError::Transaction(e.to_string()))?;

        Ok(*pending)
    }
}
