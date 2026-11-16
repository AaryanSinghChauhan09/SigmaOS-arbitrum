//! SigmaOS-arbitrum: Arbitrum L2 blockchain model.
//!
//! Real implementation: chains, transactions, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chain {
    pub id: String,
    pub name: String,
    pub rpc_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub id: String,
    pub chain_id: String,
    pub from: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArbitrumError {
    EmptyChainName,
    DuplicateChain(String),
}

impl fmt::Display for ArbitrumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyChainName => write!(f, "chain name must not be empty"),
            Self::DuplicateChain(id) => write!(f, "duplicate chain {:?}", id),
        }
    }
}

impl std::error::Error for ArbitrumError {}

pub struct Arbitrum {
    pub name: String,
    chains: BTreeMap<String, Chain>,
    transactions: BTreeMap<String, Transaction>,
}

impl Arbitrum {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), chains: BTreeMap::new(), transactions: BTreeMap::new() }
    }

    pub fn add_chain(&mut self, c: Chain) -> Result<(), ArbitrumError> {
        if c.name.trim().is_empty() { return Err(ArbitrumError::EmptyChainName); }
        let id = c.id.clone();
        if self.chains.contains_key(&id) {
            return Err(ArbitrumError::DuplicateChain(id.clone()));
        }
        self.chains.insert(id, c);
        Ok(())
    }

    pub fn chain_count(&self) -> usize { self.chains.len() }

    pub fn get_chain(&self, id: &str) -> Option<&Chain> { self.chains.get(id) }

    pub fn add_transaction(&mut self, t: Transaction) {
        self.transactions.insert(t.id.clone(), t);
    }

    pub fn get_transaction(&self, id: &str) -> Option<&Transaction> { self.transactions.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = Arbitrum::new("Arbitrum");
        assert_eq!(a.name, "Arbitrum");
        assert_eq!(a.chain_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = Arbitrum::new("Arbitrum");
        let chain = Chain { id: "c1".into(), name: "Arbitrum One".into(), rpc_url: "https://arb1.arbitrum.io/rpc".into() };
        a.add_chain(chain).unwrap();
        assert_eq!(a.chain_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = Arbitrum::new("Arbitrum");
        let chain = Chain { id: "x".into(), name: "".into(), rpc_url: "x".into() };
        assert!(a.add_chain(chain).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = Arbitrum::new("Arbitrum");
        let chain = Chain { id: "x".into(), name: "X".into(), rpc_url: "x".into() };
        a.add_chain(chain).unwrap();
        let chain2 = Chain { id: "x".into(), name: "X".into(), rpc_url: "x".into() };
        assert!(a.add_chain(chain2).is_err());
    }
}
