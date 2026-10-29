//! Core traits implementing Interface Segregation and Dependency Inversion

use async_trait::async_trait;
use ethers::types::{Address, H256, U256, TransactionRequest, TransactionReceipt, BlockNumber};
use std::sync::Arc;

use super::error::Result;

/// Trait for blockchain providers (read-only operations)
/// ISP: Focused only on reading chain state
#[async_trait]
pub trait BlockchainProvider: Send + Sync {
    /// Get the current block number
    async fn get_block_number(&self) -> Result<U256>;

    /// Get balance of an address
    async fn get_balance(&self, address: Address, block: Option<BlockNumber>) -> Result<U256>;

    /// Get chain ID
    async fn get_chain_id(&self) -> Result<U256>;

    /// Estimate gas for a transaction
    async fn estimate_gas(&self, tx: &TransactionRequest) -> Result<U256>;

    /// Get transaction receipt
    async fn get_transaction_receipt(&self, tx_hash: H256) -> Result<Option<TransactionReceipt>>;
}

/// Trait for transaction signers
/// ISP: Focused only on signing
#[async_trait]
pub trait TransactionSigner: Send + Sync {
    /// Get the signer's address
    fn address(&self) -> Address;

    /// Sign a transaction request
    async fn sign_transaction(&self, tx: TransactionRequest) -> Result<ethers::types::Transaction>;
}

/// Trait for sending transactions
/// ISP: Focused on broadcasting
#[async_trait]
pub trait TransactionSender: Send + Sync {
    /// Send a signed transaction and wait for receipt
    async fn send_transaction(
        &self,
        tx: TransactionRequest,
    ) -> Result<TransactionReceipt>;

    /// Send and return pending tx hash immediately
    async fn send_transaction_async(
        &self,
        tx: TransactionRequest,
    ) -> Result<H256>;
}

/// Trait for contract interaction
/// ISP: Focused on contract calls
#[async_trait]
pub trait ContractInteractor: Send + Sync {
    /// Call a read-only (view/pure) function
    async fn call(&self, data: Vec<u8>) -> Result<Vec<u8>>;

    /// Execute a state-changing function (requires signer)
    async fn execute(&self, data: Vec<u8>, value: Option<U256>) -> Result<TransactionReceipt>;
}

/// High-level service combining provider + signer (facade)
/// DIP: Depends on abstractions
#[async_trait]
pub trait EthereumService: BlockchainProvider + TransactionSender + Send + Sync {
    fn provider(&self) -> Arc<dyn BlockchainProvider>;
    fn signer(&self) -> Option<Arc<dyn TransactionSigner>>;
}
