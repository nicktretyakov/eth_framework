//! Local wallet signer implementation
//! SRP: Responsible only for key management and signing

use async_trait::async_trait;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, TransactionRequest};
use ethers::types::transaction::eip2718::TypedTransaction;
use std::str::FromStr;

use crate::core::{FrameworkError, Result, TransactionSigner};

/// Local private-key based signer
pub struct LocalSigner {
    wallet: LocalWallet,
}

impl LocalSigner {
    /// Create from private key hex string (with or without 0x prefix)
    pub fn from_private_key(private_key: &str, chain_id: u64) -> Result<Self> {
        let wallet = LocalWallet::from_str(private_key)
            .map_err(|e| FrameworkError::Signer(format!("Invalid private key: {}", e)))?
            .with_chain_id(chain_id);
        Ok(Self { wallet })
    }

    pub fn wallet(&self) -> &LocalWallet {
        &self.wallet
    }
}

#[async_trait]
impl TransactionSigner for LocalSigner {
    fn address(&self) -> Address {
        self.wallet.address()
    }

    async fn sign_transaction(&self, tx: TransactionRequest) -> Result<ethers::types::Transaction> {
        let typed_tx: TypedTransaction = tx.into();
        let signature = self
            .wallet
            .sign_transaction(&typed_tx)
            .await
            .map_err(|e| FrameworkError::Signer(e.to_string()))?;

        // Note: Full RLP reconstruction is complex; for production use
        // the middleware's send_transaction which handles signing internally.
        // This is a simplified demonstration of the trait.
        let signed = ethers::types::Transaction {
            hash: Default::default(),
            nonce: typed_tx.nonce().copied().unwrap_or_default(),
            block_hash: None,
            block_number: None,
            transaction_index: None,
            from: self.wallet.address(),
            to: typed_tx.to().cloned().and_then(|t| match t {
                ethers::types::NameOrAddress::Address(a) => Some(a),
                _ => None,
            }),
            value: typed_tx.value().copied().unwrap_or_default(),
            gas_price: typed_tx.gas_price(),
            gas: typed_tx.gas().copied().unwrap_or_default(),
            input: typed_tx.data().cloned().unwrap_or_default(),
            v: signature.v.into(),
            r: signature.r,
            s: signature.s,
            transaction_type: None,
            access_list: None,
            max_priority_fee_per_gas: None,
            max_fee_per_gas: None,
            chain_id: Some(self.wallet.chain_id().into()),
            other: Default::default(),
        };
        Ok(signed)
    }
}
