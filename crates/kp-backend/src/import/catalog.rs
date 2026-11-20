//! Public importer metadata projected for QML and the native file chooser.

use anyhow::Result;
use kryptopass_import::{EncryptionSupport, EntryType, FeatureSupport, ImporterRegistry};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ImporterDescription {
    id: &'static str,
    name: &'static str,
    #[serde(skip_serializing_if = "String::is_empty")]
    icon_source: String,
    formats: Vec<&'static str>,
    file_filters: Vec<String>,
    entry_types: Vec<String>,
    capabilities: Vec<Capability>,
    decryption_available: bool,
    requires_decryptor: bool,
}

#[derive(Serialize)]
struct Capability {
    label: String,
    support: &'static str,
}

pub fn json() -> Result<String> {
    json_for_registry(&super::registry()?)
}

fn json_for_registry(registry: &ImporterRegistry) -> Result<String> {
    let descriptions: Vec<_> = registry
        .importers()
        .map(|registered| {
            let info = registered.info;
            let features = info.features;
            let patterns = info
                .formats
                .iter()
                .map(|format| format!("*.{}", format.extension))
                .collect::<Vec<_>>()
                .join(" ");
            let mut capabilities: Vec<_> = [
                (crate::i18n::tr_noop("Folders"), features.folders),
                (
                    crate::i18n::tr_noop("Folder descriptions"),
                    features.folder_descriptions,
                ),
                (crate::i18n::tr_noop("Favorites"), features.favorites),
                (crate::i18n::tr_noop("Trash"), features.trash),
                (crate::i18n::tr_noop("Original dates"), features.timestamps),
                (
                    crate::i18n::tr_noop("Additional fields"),
                    features.additional_fields,
                ),
            ]
            .into_iter()
            .map(|(label, supported)| Capability {
                label: crate::i18n::tr(label),
                support: if supported {
                    "supported"
                } else {
                    "unsupported"
                },
            })
            .collect();
            capabilities.extend(
                [
                    ("Passkeys", features.passkeys),
                    (
                        crate::i18n::tr_noop("Autofill rules"),
                        features.autofill_rules,
                    ),
                    (crate::i18n::tr_noop("Attachments"), features.attachments),
                ]
                .into_iter()
                .map(|(label, support)| Capability {
                    label: crate::i18n::tr(label),
                    support: match support {
                        FeatureSupport::Supported => "supported",
                        FeatureSupport::ReferenceOnly => "reference",
                        FeatureSupport::Unsupported => "unsupported",
                    },
                }),
            );
            capabilities.push(Capability {
                label: crate::i18n::tr("Encrypted files"),
                support: if registered.decryption_available {
                    "supported"
                } else {
                    "unsupported"
                },
            });
            let icon_source = registry
                .get(info.id)
                .and_then(|importer| importer.website())
                .map(|host| {
                    crate::infrastructure::entry_icons::source_for(Some(
                        &kryptopass_core::EntryIcon::Favicon { host: host.into() },
                    ))
                })
                .unwrap_or_default();
            ImporterDescription {
                id: info.id,
                name: info.name,
                icon_source,
                formats: info.formats.iter().map(|format| format.extension).collect(),
                file_filters: vec![
                    format!("{} ({patterns})", info.name),
                    crate::i18n::tr("All files (*)"),
                ],
                entry_types: features
                    .entry_types
                    .iter()
                    .map(|kind| crate::i18n::tr(entry_type_label(*kind)))
                    .collect(),
                capabilities,
                decryption_available: registered.decryption_available,
                requires_decryptor: features.encrypted_exports
                    == EncryptionSupport::RequiresDecryptor,
            }
        })
        .collect();
    Ok(serde_json::to_string(&descriptions)?)
}

pub fn entry_type_label(kind: EntryType) -> &'static str {
    match kind {
        EntryType::Login => "Logins",
        EntryType::Note => crate::i18n::tr_noop("Notes"),
        EntryType::Alias => crate::i18n::tr_noop("Alias"),
        EntryType::Card => crate::i18n::tr_noop("Cards"),
        EntryType::Identity => crate::i18n::tr_noop("Identities"),
        EntryType::Custom => crate::i18n::tr_noop("Custom items"),
        EntryType::Wifi => crate::i18n::tr_noop("Wi-Fi networks"),
        EntryType::SshKey => crate::i18n::tr_noop("SSH keys"),
        EntryType::Totp => crate::i18n::tr_noop("TOTP authenticators"),
        EntryType::License => crate::i18n::tr_noop("Licenses"),
    }
}

#[cfg(test)]
#[path = "../../tests/unit/infrastructure/import_catalog.rs"]
mod tests;
