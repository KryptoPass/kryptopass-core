//! Vault-specific access control for the independent private-search service.

mod access;

pub use access::{VaultSearchSession, attach, is_unlocked, set_locked};
