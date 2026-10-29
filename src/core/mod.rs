//! Core abstractions following SOLID principles.
//! 
//! - Single Responsibility: Each trait has one clear purpose
//! - Open/Closed: Extend via new implementations without modifying existing code
//! - Liskov Substitution: Implementations are interchangeable via traits
//! - Interface Segregation: Small, focused traits
//! - Dependency Inversion: Depend on abstractions (traits), not concrete types

pub mod error;
pub mod traits;

pub use error::*;
pub use traits::*;
