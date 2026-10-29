//! Generic contract interactor
//! Demonstrates Dependency Inversion: depends on traits, not concrete providers/signers

use async_trait::async_trait;
use ethers::types::{Address, Bytes, TransactionRequest, TransactionReceipt, U256};
use ethers::providers::{Middleware, Provider, Http};
use ethers::middleware::SignerMiddleware;
use ethers::signers::LocalWallet;
use std::sync::Arc;

use crate::core::{ContractInteractor, FrameworkError, Result};
use crate::providers::HttpProvider;
use crate::signers::LocalSigner;

/// Generic contract wrapper that can call any contract via ABI-encoded data
pub struct GenericContract {
    address: Address,
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl GenericContract {
    /// Create a new contract interactor
    pub fn new(
        address: Address,
        provider: &HttpProvider,
        signer: &LocalSigner,
    ) -> Result<Self> {
        let client = SignerMiddleware::new(
            provider.inner().as_ref().clone(),
            signer.wallet().clone(),
        );
        Ok(Self {
            address,
            client: Arc::new(client),
        })
    }

    pub fn address(&self) -> Address {
        self.address
    }
}

#[async_trait]
impl ContractInteractor for GenericContract {
    async fn call(&self, data: Vec<u8>) -> Result<Vec<u8>> {
        let tx = TransactionRequest::new()
            .to(self.address)
            .data(Bytes::from(data));

        let result = self
            .client
            .call(&tx.into(), None)
            .await
            .map_err(|e| FrameworkError::Contract(e.to_string()))?;

        Ok(result.to_vec())
    }

    async fn execute(&self, data: Vec<u8>, value: Option<U256>) -> Result<TransactionReceipt> {
        let mut tx = TransactionRequest::new()
            .to(self.address)
            .data(Bytes::from(data));

        if let Some(v) = value {
            tx = tx.value(v);
        }

        let pending = self
            .client
            .send_transaction(tx, None)
            .await
            .map_err(|e| FrameworkError::Transaction(e.to_string()))?;

        let receipt = pending
            .await
            .map_err(|e| FrameworkError::Transaction(e.to_string()))?
            .ok_or_else(|| FrameworkError::Transaction("Transaction dropped from mempool".into()))?;

        Ok(receipt)
    }
}
