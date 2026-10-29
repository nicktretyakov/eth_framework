//! High-level services composing lower-level components
//! Demonstrates Dependency Inversion and Facade pattern

mod eth_service;
mod concurrent_monitor;

pub use eth_service::*;
pub use concurrent_monitor::*;
