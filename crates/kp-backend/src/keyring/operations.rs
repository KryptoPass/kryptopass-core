//! Runs one validated request against the keyring and words its outcome.

use kryptopass_keyring::{
    DeleteScope, ExpiryTarget, Fetched, Fingerprint, ImportReport, Keyring, Published, SubkeyUsage,
};

use super::{
    errors,
    request::{self, InvalidRequest, OperationOutcome, Request},
};

/// Parses one request and runs it on the user's keyring; every passphrase
/// goes through the system pinentry agent.
pub fn execute_operation(request_json: &str) -> Result<OperationOutcome, String> {
    let request = request::parse_request(request_json).map_err(|error| match error {
        InvalidRequest::TooLarge => {
            crate::i18n::tr("The OpenPGP request exceeds the allowed size.")
        }
        InvalidRequest::Malformed(detail) => {
            crate::i18n::trf("The OpenPGP request is not valid: %1", &[detail])
        }
        InvalidRequest::Value(input) => errors::invalid_input(input),
    })?;
    run(&Keyring::system(), request).map_err(|error| errors::describe(&error))
}

fn run(keyring: &Keyring, request: Request) -> kryptopass_keyring::Result<OperationOutcome> {
    Ok(match request {
        Request::Create { user_id, expiry } => {
            let fingerprint = keyring.create_key(&user_id, expiry)?;
            keyring.mark_as_own(&fingerprint)?;
            selected(
                crate::i18n::trf(
                    "OpenPGP key created with an encryption subkey: %1.",
                    &[fingerprint.to_string()],
                ),
                &fingerprint,
            )
        }
        Request::Import { path } => {
            let report = keyring.import_file(&path)?;
            OperationOutcome {
                message: import_message(&report),
                public_key: None,
                selected_fingerprint: report.first_key.as_ref().map(ToString::to_string),
            }
        }
        Request::ExportPublic { fingerprint, path } => {
            let published = keyring.export_public_to(&fingerprint, &path)?;
            selected(
                crate::i18n::trf(
                    "Public key exported to %1.%2",
                    &[
                        published.path.display().to_string(),
                        durability_warning(&published),
                    ],
                ),
                &fingerprint,
            )
        }
        Request::CopyPublic { fingerprint } => {
            let armored = keyring.public_key_armored(&fingerprint)?;
            OperationOutcome {
                public_key: Some(armored),
                ..selected(
                    crate::i18n::trf("Public key ready to copy: %1.", &[fingerprint.to_string()]),
                    &fingerprint,
                )
            }
        }
        Request::BackupSecret { fingerprint, path } => {
            let published = keyring.backup_secret_to(&fingerprint, &path)?;
            selected(
                crate::i18n::trf(
                    "Private copy saved with 0600 permissions in %1.%2",
                    &[
                        published.path.display().to_string(),
                        durability_warning(&published),
                    ],
                ),
                &fingerprint,
            )
        }
        Request::ChangePassphrase { fingerprint } => {
            keyring.change_passphrase(&fingerprint)?;
            selected(
                crate::i18n::trf("Passphrase updated for %1.", &[fingerprint.to_string()]),
                &fingerprint,
            )
        }
        Request::SetExpiry {
            fingerprint,
            target,
            expiry,
        } => {
            keyring.set_expiry(&fingerprint, &target, expiry)?;
            let source = match target {
                ExpiryTarget::Primary => crate::i18n::tr_noop("Primary key expiry updated for %1."),
                ExpiryTarget::Subkeys(_) => {
                    crate::i18n::tr_noop("Expiry of the selected subkeys updated for %1.")
                }
            };
            selected(
                crate::i18n::trf(source, &[fingerprint.to_string()]),
                &fingerprint,
            )
        }
        Request::AddUserId {
            fingerprint,
            user_id,
        } => {
            keyring.add_user_id(&fingerprint, &user_id)?;
            selected(
                crate::i18n::trf("Identity added to %1.", &[fingerprint.to_string()]),
                &fingerprint,
            )
        }
        Request::SetPrimaryUserId {
            fingerprint,
            user_id,
        } => {
            keyring.set_primary_user_id(&fingerprint, &user_id)?;
            selected(
                crate::i18n::trf(
                    "Primary identity updated for %1.",
                    &[fingerprint.to_string()],
                ),
                &fingerprint,
            )
        }
        Request::RevokeUserId {
            fingerprint,
            user_id,
        } => {
            keyring.revoke_user_id(&fingerprint, &user_id)?;
            selected(
                crate::i18n::trf("Identity revoked in %1.", &[fingerprint.to_string()]),
                &fingerprint,
            )
        }
        Request::AddSubkey {
            fingerprint,
            usage,
            expiry,
        } => {
            keyring.add_subkey(&fingerprint, usage, expiry)?;
            selected(
                crate::i18n::trf(
                    "Subkey of %1 created for %2.",
                    &[usage_name(usage), fingerprint.to_string()],
                ),
                &fingerprint,
            )
        }
        Request::RevokeSubkey {
            fingerprint,
            subkey,
        } => {
            keyring.revoke_subkey(&fingerprint, &subkey)?;
            selected(
                crate::i18n::trf("Subkey %1 revoked.", &[subkey.to_string()]),
                &fingerprint,
            )
        }
        Request::WriteRevocation { fingerprint, path } => {
            let published = keyring.write_revocation_certificate(&fingerprint, &path)?;
            selected(
                crate::i18n::trf(
                    "Revocation certificate prepared in %1.%2",
                    &[
                        published.path.display().to_string(),
                        durability_warning(&published),
                    ],
                ),
                &fingerprint,
            )
        }
        Request::RevokeKey { fingerprint } => {
            keyring.revoke_key(&fingerprint)?;
            selected(
                crate::i18n::trf("Key %1 revoked locally.", &[fingerprint.to_string()]),
                &fingerprint,
            )
        }
        Request::Certify {
            target,
            signer,
            user_ids,
            kind,
        } => {
            let certified = keyring.certify(&target, &signer, &user_ids, kind)?;
            selected(
                crate::i18n::trf(
                    "%1 identity(ies) certified in %2.",
                    &[certified.to_string(), target.to_string()],
                ),
                &target,
            )
        }
        Request::Fetch {
            fingerprint,
            keyserver,
        } => {
            let source = match keyring.fetch(&fingerprint, keyserver.as_ref())? {
                Fetched::Imported => {
                    crate::i18n::tr_noop("Key %1 imported from the configured external source.")
                }
                Fetched::Refreshed => {
                    crate::i18n::tr_noop("Key %1 updated from the configured external source.")
                }
            };
            selected(
                crate::i18n::trf(source, &[fingerprint.to_string()]),
                &fingerprint,
            )
        }
        Request::MarkAsOwn { fingerprint } => {
            tracing::info!(
                key = %short(&fingerprint),
                "Keyring: user asked to mark a key as own (ultimate ownertrust)"
            );
            match keyring.mark_as_own(&fingerprint) {
                Ok(()) => {
                    tracing::info!(key = %short(&fingerprint), "Keyring: key marked as own");
                }
                Err(error) => {
                    tracing::warn!(
                        key = %short(&fingerprint),
                        error = %error,
                        "Keyring: could not mark the key as own"
                    );
                    return Err(error);
                }
            }
            selected(
                crate::i18n::trf(
                    "Key %1 marked as yours: GnuPG can now encrypt with it.",
                    &[fingerprint.to_string()],
                ),
                &fingerprint,
            )
        }
        Request::Delete { fingerprint, scope } => {
            keyring.delete(&fingerprint, scope)?;
            let source = match scope {
                DeleteScope::PublicOnly => crate::i18n::tr_noop("Public part deleted for %1."),
                DeleteScope::SecretOnly => crate::i18n::tr_noop(
                    "Private material deleted; the public part is kept for %1.",
                ),
                DeleteScope::Everything => {
                    crate::i18n::tr_noop("Public key and private material deleted for %1.")
                }
            };
            selected(
                crate::i18n::trf(source, &[fingerprint.to_string()]),
                &fingerprint,
            )
        }
    })
}

/// Last 16 hex digits of a fingerprint: enough to tell keys apart in a log.
fn short(fingerprint: &Fingerprint) -> String {
    let text = fingerprint.as_str();
    text[text.len().saturating_sub(16)..].to_owned()
}

fn selected(message: String, fingerprint: &Fingerprint) -> OperationOutcome {
    OperationOutcome {
        message,
        public_key: None,
        selected_fingerprint: Some(fingerprint.to_string()),
    }
}

fn usage_name(usage: SubkeyUsage) -> String {
    crate::i18n::tr(match usage {
        SubkeyUsage::Encrypt => "cifrado",
        SubkeyUsage::Sign => "firma",
        SubkeyUsage::Authenticate => crate::i18n::tr_noop("authentication"),
    })
}

fn durability_warning(published: &Published) -> String {
    if published.durable {
        String::new()
    } else {
        crate::i18n::tr(
            "The file was published, but its folder could not be synced; it might not survive a power loss.",
        )
    }
}

fn import_message(report: &ImportReport) -> String {
    let mut message = if report.is_partial() {
        crate::i18n::trf(
            "Partial OpenPGP import: GnuPG incorporated %1 item(s) and left %2 unimported.",
            &[report.imported.to_string(), report.not_imported.to_string()],
        )
    } else {
        crate::i18n::trf(
            "OpenPGP import complete; GnuPG incorporated %1 item(s).",
            &[report.imported.to_string()],
        )
    };
    for (count, source) in [
        (
            report.unchanged,
            crate::i18n::tr_noop(" %1 item(s) were already up to date."),
        ),
        (
            report.failed,
            crate::i18n::tr_noop(" %1 record(s) with errors were detected."),
        ),
        (
            report.without_user_id,
            crate::i18n::tr_noop(" %1 key(s) had no user ID."),
        ),
        (
            report.skipped_v3,
            crate::i18n::tr_noop(" %1 v3 key(s) were skipped."),
        ),
        (
            report.new_user_ids,
            crate::i18n::tr_noop(" %1 identity(ies) were added."),
        ),
        (
            report.new_subkeys,
            crate::i18n::tr_noop(" %1 subkey(s) were added."),
        ),
        (
            report.new_signatures,
            crate::i18n::tr_noop(" %1 signature(s) were added."),
        ),
        (
            report.new_revocations,
            crate::i18n::tr_noop(" %1 revocation(s) were applied."),
        ),
    ] {
        if count > 0 {
            message.push_str(&crate::i18n::trf(source, &[count.to_string()]));
        }
    }
    message
}
