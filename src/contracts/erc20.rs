//! Type-safe ERC20 contract bindings example
//! Uses abigen for compile-time safety

use ethers::prelude::*;
use std::sync::Arc;

// Generate type-safe bindings for ERC20
abigen!(
    IERC20,
    r#"[
        function name() external view returns (string)
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
        function totalSupply() external view returns (uint256)
        function balanceOf(address account) external view returns (uint256)
        function transfer(address recipient, uint256 amount) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
        function approve(address spender, uint256 amount) external returns (bool)
        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool)
        event Transfer(address indexed from, address indexed to, uint256 value)
        event Approval(address indexed owner, address indexed spender, uint256 value)
    ]"#
);

/// High-level ERC20 service with convenience methods
/// SRP: Only handles ERC20-specific logic
pub struct Erc20Service<M: Middleware> {
    contract: IERC20<M>,
}

impl<M: Middleware + 'static> Erc20Service<M> {
    pub fn new(address: Address, client: Arc<M>) -> Self {
        Self {
            contract: IERC20::new(address, client),
        }
    }

    pub async fn name(&self) -> Result<String, ContractError<M>> {
        self.contract.name().call().await
    }

    pub async fn symbol(&self) -> Result<String, ContractError<M>> {
        self.contract.symbol().call().await
    }

    pub async fn decimals(&self) -> Result<u8, ContractError<M>> {
        self.contract.decimals().call().await
    }

    pub async fn total_supply(&self) -> Result<U256, ContractError<M>> {
        self.contract.total_supply().call().await
    }

    pub async fn balance_of(&self, account: Address) -> Result<U256, ContractError<M>> {
        self.contract.balance_of(account).call().await
    }

    /// Send transfer and wait for receipt
    pub async fn transfer(
        &self,
        to: Address,
        amount: U256,
    ) -> Result<Option<TransactionReceipt>, ContractError<M>> {
        let call = self.contract.transfer(to, amount);
        let pending = call.send().await?;
        Ok(pending.await?)
    }

    /// Approve spender
    pub async fn approve(
        &self,
        spender: Address,
        amount: U256,
    ) -> Result<Option<TransactionReceipt>, ContractError<M>> {
        let call = self.contract.approve(spender, amount);
        let pending = call.send().await?;
        Ok(pending.await?)
    }
}
