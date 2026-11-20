//! The desktop backend without UI: profile paths, vault access, OpenPGP,
//! keyring commands, backups, imports and private search.
//!
//! It was moved out of kp-desktop as is. The UI still installs two hooks at
//! start-up, translation ([`i18n::set_translator`]) and icon normalization
//! ([`infrastructure::entry_icons::set_icon_normalizer`]), until the backend
//! returns typed results and does that work itself.

pub mod app_info;
pub mod backup;
pub mod i18n;
pub mod import;
pub mod infrastructure;
pub mod keyring;
pub mod process_security;
pub mod search;
pub mod storage;
pub mod transport_diagnostics;
pub mod vault_access;
