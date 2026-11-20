//! Text the backend shows to people, translated by the UI's catalog.
//!
//! Source texts are English. `tr`, `trf` and `tr_noop` are the names `Messages.sh`
//! extracts, so every user-facing literal goes through one of them.
//!
//! Until the backend returns typed results, the UI installs its translator at
//! start-up. Without one, texts come out in their source language.

use std::sync::OnceLock;

static TRANSLATOR: OnceLock<fn(&str) -> String> = OnceLock::new();

/// Installs the function that translates source texts. Only the first call counts.
pub fn set_translator(translator: fn(&str) -> String) {
    let _ = TRANSLATOR.set(translator);
}

pub fn tr(source: &str) -> String {
    match TRANSLATOR.get() {
        Some(translate) => translate(source),
        None => source.to_owned(),
    }
}

/// Marks a text that is translated later, where it is shown, and returns it unchanged.
pub const fn tr_noop(source: &'static str) -> &'static str {
    source
}

/// Substitute numbered numbered placeholders without interpreting argument contents.
pub fn trf(source: &str, arguments: &[String]) -> String {
    let translated = tr(source);
    let mut output = String::new();
    let mut remainder = translated.as_str();
    while let Some(start) = remainder.find('%') {
        output.push_str(&remainder[..start]);
        remainder = &remainder[start + 1..];
        let digits = remainder.bytes().take_while(u8::is_ascii_digit).count();
        let argument = remainder[..digits]
            .parse::<usize>()
            .ok()
            .and_then(|n| n.checked_sub(1))
            .and_then(|n| arguments.get(n));
        if let Some(argument) = argument {
            output.push_str(argument);
            remainder = &remainder[digits..];
        } else {
            output.push('%');
        }
    }
    output.push_str(remainder);
    output
}
