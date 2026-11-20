//! Adapters for operating-system cryptography, browser integration, and UI resources.

pub mod approvals;
pub mod entry_icons;
pub mod openpgp;

pub use openpgp::GnuPgOpenPgp;

/// Upper bound for the encrypted form of one supported plaintext payload.
pub use kryptopass_core::util::MAX_CIPHERTEXT_BYTES;

pub mod browser;
