use super::PreparedImport;
use kryptopass_import::{EntryType, EntryWarningKind, ImportError};

/// Counts per entry type, labelled for display; QML picks the icon from `kind`.
pub fn summary_json(plan: &PreparedImport) -> String {
    let source = &plan.source;
    let types: Vec<_> = source
        .entries_by_type
        .iter()
        .map(|(kind, count)| {
            let (kind, label) = entry_type_display(*kind);
            serde_json::json!({ "kind": kind, "label": crate::i18n::tr(label), "count": count })
        })
        .collect();
    serde_json::json!({
        "types": types,
        "folders": plan.batch.folders.len(),
        "items": plan.batch.items.len(),
        "trashed": source.trashed_entries,
        "skipped": source.skipped_entries,
    })
    .to_string()
}

fn entry_type_display(kind: EntryType) -> (&'static str, &'static str) {
    match kind {
        EntryType::Login => ("login", crate::i18n::tr_noop("Logins")),
        EntryType::Note => ("note", crate::i18n::tr_noop("Notes")),
        EntryType::Alias => ("login", crate::i18n::tr_noop("Aliases (saved as logins)")),
        EntryType::Card => ("card", crate::i18n::tr_noop("Payment cards")),
        EntryType::Identity => ("identity", crate::i18n::tr_noop("Identities")),
        EntryType::Custom => ("custom", crate::i18n::tr_noop("Custom secrets")),
        EntryType::Wifi => ("wifi", crate::i18n::tr_noop("Wi-Fi networks")),
        EntryType::SshKey => ("ssh-key", crate::i18n::tr_noop("SSH keys")),
        EntryType::Totp => ("totp", crate::i18n::tr_noop("TOTP authenticators")),
        EntryType::License => ("license", crate::i18n::tr_noop("Licenses")),
    }
}

/// Conversion notes; `loss` marks data that will not reach the vault.
pub fn warnings_json(plan: &PreparedImport) -> String {
    let mut notes = Vec::new();
    let mut push = |count: u32, text: &str, loss: bool| {
        if count > 0 {
            notes.push(serde_json::json!({
                "text": crate::i18n::trf(text, &[count.to_string()]),
                "loss": loss,
            }));
        }
    };
    for (kind, text, loss) in [
        (
            EntryWarningKind::UnsupportedEntryType,
            crate::i18n::tr_noop("%1 items of an unsupported type will not be imported."),
            true,
        ),
        (
            EntryWarningKind::AttachmentsNotImported,
            crate::i18n::tr_noop("%1 items have attachments that will not be imported."),
            true,
        ),
        (
            EntryWarningKind::UnreadableSshKey,
            crate::i18n::tr_noop("%1 unsupported SSH keys will not be imported."),
            true,
        ),
        (
            EntryWarningKind::PasskeysAsReferences,
            crate::i18n::tr_noop(
                "%1 passkeys will be saved only as a reference; they cannot be used to sign in.",
            ),
            false,
        ),
        (
            EntryWarningKind::ConflictingPublicKey,
            crate::i18n::tr_noop(
                "%1 public keys will be saved as a reference because they do not match their private key.",
            ),
            false,
        ),
        (
            EntryWarningKind::AutofillRulesAsReferences,
            crate::i18n::tr_noop("%1 autofill rules will be saved as a reference."),
            false,
        ),
        (
            EntryWarningKind::SourceFieldsPreserved,
            crate::i18n::tr_noop("%1 items keep data from the source as reference fields."),
            false,
        ),
    ] {
        let count = plan
            .warnings
            .iter()
            .filter(|warning| warning.kind == kind)
            .count();
        push(u32::try_from(count).unwrap_or(u32::MAX), text, loss);
    }
    let adaptations = &plan.adaptations;
    push(
        adaptations.aliases_as_logins,
        crate::i18n::tr_noop("%1 aliases will be saved as logins with their email."),
        false,
    );
    push(
        adaptations.folder_descriptions,
        crate::i18n::tr_noop("%1 folder descriptions will be saved as notes."),
        false,
    );
    push(
        adaptations.values_as_custom,
        crate::i18n::tr_noop("%1 values do not fit their field and will be saved as added fields."),
        false,
    );
    push(
        adaptations.entries_as_custom,
        crate::i18n::tr_noop(
            "%1 items do not fit their template and will be saved as custom items.",
        ),
        false,
    );
    push(
        adaptations.rejected,
        crate::i18n::tr_noop("%1 items do not fit any template and will not be imported."),
        true,
    );
    serde_json::Value::Array(notes).to_string()
}

pub fn review_error(error: &anyhow::Error) -> &'static str {
    match error.downcast_ref::<ImportError>() {
        Some(ImportError::Decryption) => crate::i18n::tr_noop(
            "The export could not be decrypted. Check the credentials in the decryption service and try again.",
        ),
        Some(ImportError::DecryptionRequired) => crate::i18n::tr_noop(
            "This integration needs a decryption service to read the chosen file.",
        ),
        Some(ImportError::Limit) => crate::i18n::tr_noop(
            "The file or the adapted data exceed the import limits. Export fewer items and try again.",
        ),
        Some(ImportError::Content) => crate::i18n::tr_noop(
            "The export contains data that the current vault cannot represent. No item was imported. Keep the original and check its types and sizes.",
        ),
        Some(_) => crate::i18n::tr_noop(
            "The file is not compatible with the selected integration. Check the manager and the export format.",
        ),
        None => crate::i18n::tr_noop(
            "The file could not be read. Check its location and permissions and try again.",
        ),
    }
}
