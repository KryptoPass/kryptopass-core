//! Spanish wording for typed keyring failures; the crate itself never words them.

use kryptopass_keyring::{CommandError, Error, FileError, InvalidInput, Step, Unconfirmed};

pub(super) fn describe(error: &Error) -> String {
    let fingerprint =
        |message: &str, value: &dyn ToString| crate::i18n::trf(message, &[value.to_string()]);
    match error {
        Error::InvalidInput(input) => invalid_input(*input),
        Error::UnsupportedEngine { found, minimum } => crate::i18n::trf(
            "GnuPG %1 is not supported; %2 or later is required.",
            &[found.clone(), (*minimum).to_owned()],
        ),
        Error::Canceled => crate::i18n::tr("The operation in pinentry was canceled."),
        Error::KeyNotFound(value) => fingerprint(
            crate::i18n::tr_noop("There is no local key with primary fingerprint %1."),
            value,
        ),
        Error::SubkeyNotFound { key, subkey } => crate::i18n::trf(
            "Subkey %1 does not belong to primary key %2.",
            &[subkey.to_string(), key.to_string()],
        ),
        Error::UserIdNotFound(_) => {
            crate::i18n::tr("The given identity does not belong to this key.")
        }
        Error::UserIdAlreadyPresent(_) => {
            crate::i18n::tr("The identity is already present in this key.")
        }
        Error::UserIdRevoked => crate::i18n::tr("The identity is revoked."),
        Error::UserIdInvalid => crate::i18n::tr("The identity is not valid."),
        Error::AlreadyRevoked(value) => {
            fingerprint(crate::i18n::tr_noop("%1 is already revoked."), value)
        }
        Error::KeyUnusable(value) => fingerprint(
            crate::i18n::tr_noop("Key %1 is revoked, disabled or not valid."),
            value,
        ),
        Error::KeyExpired(value) => fingerprint(crate::i18n::tr_noop("Key %1 is expired."), value),
        Error::NotCertifier(_) => crate::i18n::tr(
            "The operation requires your own primary private key with certification capability.",
        ),
        Error::NoSecretMaterial(_) => crate::i18n::tr("The key has no local private material."),
        Error::NoSecretKey => crate::i18n::tr("No OpenPGP private key is available."),
        Error::SecretMaterialPresent(_) => crate::i18n::tr(
            "GnuPG does not allow deleting only the public part while secret material exists; delete that material first or delete the whole key.",
        ),
        Error::NothingToImport => crate::i18n::tr(
            "The file contains no OpenPGP keys or revocations that GnuPG can import.",
        ),
        Error::ImportChangedNothing {
            not_imported,
            failed,
        } => crate::i18n::trf(
            "The import applied no changes: %1 item(s) not imported and %2 record(s) with errors.",
            &[not_imported.to_string(), failed.to_string()],
        ),
        Error::EncryptionSubkeyFailed {
            fingerprint: value,
            source,
        } => crate::i18n::trf(
            "Primary key %1 was created, but the encryption subkey failed: %2",
            &[value.to_string(), source.to_string()],
        ),
        Error::MalformedRevocation(_) => {
            crate::i18n::tr("The revocation certificate does not have a valid format.")
        }
        Error::Unconfirmed(unconfirmed) => self::unconfirmed(unconfirmed),
        Error::Gpg { step, source } => crate::i18n::trf(
            "GnuPG could not %1: %2",
            &[step_name(*step), source.to_string()],
        ),
        Error::File(file) => file_error(file),
        Error::Command(command) => command_error(command),
        _ => error.to_string(),
    }
}

pub(super) fn invalid_input(input: InvalidInput) -> String {
    crate::i18n::tr(match input {
        InvalidInput::Fingerprint => {
            crate::i18n::tr_noop("Enter a full OpenPGP fingerprint of 40 or 64 hexadecimal digits.")
        }
        InvalidInput::UserId => crate::i18n::tr_noop(
            "The identity must be between 1 and 255 bytes and contain no control characters.",
        ),
        InvalidInput::ExpiryTooFar => {
            crate::i18n::tr_noop("The expiration date exceeds the limit GnuPG can represent.")
        }
        InvalidInput::Keyserver => {
            crate::i18n::tr_noop("Enter a valid URL with a host and without credentials or spaces.")
        }
        InvalidInput::Path => {
            crate::i18n::tr_noop("The local path is empty or exceeds the allowed size.")
        }
        InvalidInput::TooManySubkeys => {
            crate::i18n::tr_noop("An expiry request can include at most 64 subkeys.")
        }
        InvalidInput::DuplicateSubkey => {
            crate::i18n::tr_noop("The list of subkey fingerprints contains duplicates.")
        }
        InvalidInput::UserIdCount => {
            crate::i18n::tr_noop("Select between 1 and 64 identities to certify.")
        }
        InvalidInput::DuplicateUserId => {
            crate::i18n::tr_noop("The list of identities contains duplicates.")
        }
        InvalidInput::SelfCertification => {
            crate::i18n::tr_noop("The certification must target a key other than the signing one.")
        }
        _ => crate::i18n::tr_noop("The OpenPGP request contains an invalid value."),
    })
}

fn unconfirmed(unconfirmed: &Unconfirmed) -> String {
    let with =
        |message: &str, value: &dyn ToString| crate::i18n::trf(message, &[value.to_string()]);
    match unconfirmed {
        Unconfirmed::CreatedKeyMissing(value) => with(
            crate::i18n::tr_noop(
                "Primary key %1 was created, but it does not appear in the keyring.",
            ),
            value,
        ),
        Unconfirmed::KeyNotRevoked(value) => with(
            crate::i18n::tr_noop(
                "GnuPG imported the certificate, but did not confirm the revocation of %1.",
            ),
            value,
        ),
        Unconfirmed::SubkeyNotRevoked(value) => with(
            crate::i18n::tr_noop("GnuPG did not confirm the revocation of subkey %1."),
            value,
        ),
        Unconfirmed::KeyStillPresent(value) => with(
            crate::i18n::tr_noop("GnuPG finished, but key %1 still appears in the keyring."),
            value,
        ),
        Unconfirmed::SecretStillPresent(value) => with(
            crate::i18n::tr_noop(
                "GnuPG finished, but the private material of %1 is still present.",
            ),
            value,
        ),
        Unconfirmed::PublicPartDeleted(value) => with(
            crate::i18n::tr_noop(
                "GnuPG also deleted the public part of %1; the operation was partially applied.",
            ),
            value,
        ),
        Unconfirmed::OwnerTrustNotApplied(value) => with(
            crate::i18n::tr_noop(
                "GnuPG did not record trust for key %1. It still cannot encrypt with it; check GnuPG and try again.",
            ),
            value,
        ),
        Unconfirmed::NotFetched(value) => with(
            crate::i18n::tr_noop("The external source did not return the requested key %1."),
            value,
        ),
        Unconfirmed::MissingFingerprint => {
            crate::i18n::tr("GnuPG created the key but did not return its fingerprint.")
        }
        Unconfirmed::EmptyOutput => {
            crate::i18n::tr("GnuPG produced no data for the requested export.")
        }
        Unconfirmed::NotUtf8 => {
            crate::i18n::tr("GnuPG returned a public key that is not ASCII/UTF-8.")
        }
        _ => unconfirmed.to_string(),
    }
}

fn step_name(step: Step) -> String {
    crate::i18n::tr(match step {
        Step::OpenContext => crate::i18n::tr_noop("open the context"),
        Step::ListKeys => crate::i18n::tr_noop("query the keyring"),
        Step::CreateKey => crate::i18n::tr_noop("create the key"),
        Step::Import => "importar",
        Step::Export => "exportar",
        Step::ChangePassphrase => crate::i18n::tr_noop("change the passphrase"),
        Step::SetExpiry => crate::i18n::tr_noop("update the expiry"),
        Step::AddUserId => crate::i18n::tr_noop("add the identity"),
        Step::SetPrimaryUserId => crate::i18n::tr_noop("change the primary identity"),
        Step::RevokeUserId => crate::i18n::tr_noop("revoke the identity"),
        Step::AddSubkey => crate::i18n::tr_noop("create the subkey"),
        Step::RevokeSubkey => crate::i18n::tr_noop("revoke the subkey"),
        Step::SelectSigner => crate::i18n::tr_noop("select the signing key"),
        Step::Certify => "certificar",
        Step::Sign => "firmar",
        Step::Delete => "eliminar",
        _ => crate::i18n::tr_noop("complete the operation"),
    })
}

fn file_error(error: &FileError) -> String {
    let path = |message: &str, value: &std::path::Path| {
        crate::i18n::trf(message, &[value.display().to_string()])
    };
    match error {
        FileError::Open { path: value, .. } | FileError::Read { path: value, .. } => path(
            crate::i18n::tr_noop("The file %1 could not be read."),
            value,
        ),
        FileError::NotRegular(value) => path(
            crate::i18n::tr_noop("%1 is not a regular local file."),
            value,
        ),
        FileError::TooLarge(value) => {
            path(crate::i18n::tr_noop("%1 exceeds the allowed size."), value)
        }
        FileError::NotAFile(value) => {
            path(crate::i18n::tr_noop("%1 does not point to a file."), value)
        }
        FileError::Destination { path: value, .. } => path(
            crate::i18n::tr_noop("%1 cannot be used as a destination folder."),
            value,
        ),
        FileError::DestinationExists(value) => path(
            crate::i18n::tr_noop("%1 already exists and will not be overwritten."),
            value,
        ),
        FileError::DatabaseExtension(value) => {
            path(crate::i18n::tr_noop("%1 has a database extension."), value)
        }
        FileError::InsideGnupgHome(value) => path(
            crate::i18n::tr_noop("%1 is inside the GnuPG folder."),
            value,
        ),
        FileError::Write { path: value, .. } => {
            path(crate::i18n::tr_noop("%1 could not be written."), value)
        }
        FileError::Temporary(_) => {
            crate::i18n::tr("A private temporary file could not be created.")
        }
        _ => error.to_string(),
    }
}

fn command_error(error: &CommandError) -> String {
    match error {
        CommandError::Spawn(_) => crate::i18n::tr("gpg could not be started."),
        CommandError::Io(_) => crate::i18n::tr("gpg could not be reached."),
        CommandError::Timeout => crate::i18n::tr("gpg timed out."),
        CommandError::Exit(status) => {
            crate::i18n::trf("gpg exited with %1.", &[status.to_string()])
        }
        CommandError::Protocol(_) => crate::i18n::tr("gpg responded unexpectedly."),
        _ => error.to_string(),
    }
}
