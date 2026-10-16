//! NexusOffices: the product names the interface shows.
//!
//! Upstream's English source strings (the catalog keys) and their translations say "WordCraft"
//! and "ArtCraft". Rather than rewrite thousands of keys in 23 catalogs, every lookup
//! ([`super::t`], [`super::tc`], which `tl!` uses) passes its result through [`brand`] once, so every language,
//! English included, shows Nexus Docs / NexusOffices. Keys, command ids, file keys
//! (`wordcraft:*`) and `WORDCRAFT_*` variables are untouched: only `WordCraft` and `ArtCraft`
//! spelled exactly so are replaced, plus the CLI's command name where an interface string quotes
//! it as a command (`` `wordcraft-cli `` → `` `nexus-docs-cli ``, the installed binary's name).
//! Only interface strings go through here: never document or other user text (see [`super::t`]).
//!
//! Text that must keep the upstream names (the attribution and the copyright notice in About)
//! is not looked up through `tl!` (see `crate::nexus`).

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Upstream name → the name NexusOffices shows, in the order they are replaced. The command name
/// is matched only as quoted code (a backtick, then `wordcraft-cli `), as in Options › Agent
/// control, so identifiers (`wordcraft_engine`, a bare `wordcraft-cli`) are never touched.
const NAMES: [(&str, &str); 3] = [("WordCraft", "Nexus Docs"), ("ArtCraft", "NexusOffices"), ("`wordcraft-cli ", "`nexus-docs-cli ")];

/// At most this many distinct branded strings are kept ([`branded`]). Interface strings number in
/// the low hundreds; the cap only bounds memory if something unexpected (a document's text) were
/// ever passed through the lookup, which then simply comes back unbranded.
const MAX_INTERNED: usize = 4096;

/// `s` with the upstream product names replaced (`WordCraft` → `Nexus Docs`, `ArtCraft` →
/// `NexusOffices`). Strings without them come back borrowed, without allocating.
pub fn brand(s: &str) -> Cow<'_, str> {
    if !NAMES.iter().any(|(from, _)| s.contains(from)) {
        return Cow::Borrowed(s);
    }
    let mut out = s.to_string();
    for (from, to) in NAMES {
        if out.contains(from) {
            out = out.replace(from, to);
        }
    }
    Cow::Owned(out)
}

/// [`brand`] for the lookup functions, which hand out `&str`: a string that needs branding is
/// branded once and kept for the life of the process (a bounded set: the interface strings).
pub(super) fn branded(s: &str) -> &str {
    if !NAMES.iter().any(|(from, _)| s.contains(from)) {
        return s;
    }
    static CACHE: OnceLock<Mutex<HashMap<String, &'static str>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    // A poisoned lock still holds a usable map (inserts are single statements).
    let mut map = cache.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(&hit) = map.get(s) {
        return hit;
    }
    if map.len() >= MAX_INTERNED {
        return s;
    }
    let leaked: &'static str = Box::leak(brand(s).into_owned().into_boxed_str());
    map.insert(s.to_string(), leaked);
    leaked
}

/// Upstream's engine messages that name the product. [`brand_status`] renames the product only
/// inside these phrases, so a file name or document text quoted in the same message is kept.
const STATUS_PHRASES: [&str; 7] = [
    "document WordCraft can open",
    "text file WordCraft can open",
    "WordCraft shows it but can't edit it",
    "which WordCraft can't open",
    "WordCraft can't decode its format",
    "making WordCraft the default app",
    "WordCraft can't change the default apps",
];

/// A status-bar message with the product renamed in upstream's own message phrases only
/// ([`STATUS_PHRASES`]); everything else, such as a quoted file name, is left as it is.
pub fn brand_status(s: String) -> String {
    let mut out = s;
    for phrase in STATUS_PHRASES {
        if out.contains(phrase) {
            out = out.replace(phrase, &phrase.replace("WordCraft", "Nexus Docs"));
        }
    }
    out
}

/// Text NexusOffices adds to the interface (the About page), written in English and Vietnamese
/// only: `vi` gets `vi`, every other language `en`. Deliberately not a catalog key, so neither
/// branded nor asked of the other catalogs (upstream's complete-catalog checks).
pub fn nexus_text<'a>(en: &'a str, vi: &'a str) -> &'a str {
    #[cfg(test)]
    if super::pseudo::on() {
        // Marked like a catalog lookup, so the untranslated-text audit sees it as handled.
        return Box::leak(format!("⟦{}⟧", en.replace('\n', "⟧\n⟦")).into_boxed_str());
    }
    if super::current().code() == "vi" { vi } else { en }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brand_substitutes_product_names() {
        assert_eq!(brand("About WordCraft"), "About Nexus Docs");
        assert_eq!(brand("Join the ArtCraft community"), "Join the NexusOffices community");
        assert_eq!(brand("Giới thiệu về WordCraft"), "Giới thiệu về Nexus Docs");
        assert!(matches!(brand("Giới thiệu"), Cow::Borrowed("Giới thiệu")));
        // Identifiers and variables keep upstream's spelling.
        assert!(matches!(brand("wordcraft:canvas WORDCRAFT_LANGUAGE"), Cow::Borrowed(_)));
        assert!(matches!(brand("wordcraft-cli wordcraft_engine"), Cow::Borrowed(_)));
        // A quoted command uses the installed binary's name.
        assert_eq!(brand("run `wordcraft-cli mcp` for an MCP server"), "run `nexus-docs-cli mcp` for an MCP server");
    }

    /// Options › Agent control quotes the CLI command; every catalog shows the installed name.
    #[test]
    fn agent_control_hint_names_the_installed_cli() {
        use super::super::{Lang, current, set_current, t};
        const KEY: &str = "Every command is available to scripts and AI agents: run `wordcraft-cli mcp` for an MCP server, or start the app with `--control <port>` for the JSON control channel.";
        let before = current();
        for code in ["en", "vi", "de", "ko", "ja"] {
            set_current(Lang::from_code(code).unwrap());
            let shown = t(KEY);
            assert!(shown.contains("`nexus-docs-cli mcp`") && !shown.contains("wordcraft"), "{code}: {shown}");
        }
        set_current(before);
    }

    #[test]
    fn status_messages_rename_the_product_but_keep_quoted_names() {
        assert_eq!(
            brand_status("WordCraft notes.docx: not a document WordCraft can open (bad zip)".into()),
            "WordCraft notes.docx: not a document Nexus Docs can open (bad zip)"
        );
        assert_eq!(brand_status("Saved WordCraft.docx".into()), "Saved WordCraft.docx", "user text is never rewritten");
    }

    #[test]
    fn branded_lookups_are_stable_and_borrow_when_unbranded() {
        let a = branded("Make WordCraft the default");
        let b = branded("Make WordCraft the default");
        assert_eq!(a, "Make Nexus Docs the default");
        assert!(std::ptr::eq(a, b), "the branded string is made once");
        let plain = String::from("Close");
        assert!(std::ptr::eq(branded(&plain), plain.as_str()));
    }

    /// `tl!` ([`super::super::t`]) brands in every language, English included; [`super::super::tr`]
    /// keeps returning the catalog's own text (upstream's catalog tests rely on it).
    #[test]
    fn interface_lookups_show_nexus_names_in_english_and_vietnamese() {
        use super::super::{Lang, current, set_current, t, tc, tr};
        let before = current();
        set_current(Lang::from_code("en").unwrap());
        assert_eq!(t("About WordCraft"), "About Nexus Docs");
        assert_eq!(crate::tl!("WordCraft"), "Nexus Docs");
        set_current(Lang::from_code("vi").unwrap());
        assert_eq!(t("About WordCraft"), "Giới thiệu về Nexus Docs");
        assert_eq!(tc("preview", "About WordCraft"), "Giới thiệu về Nexus Docs");
        assert!(!t("Show the Start screen when WordCraft starts").contains("WordCraft"));
        assert_eq!(tr(Lang::from_code("vi").unwrap(), "WordCraft"), "WordCraft");
        assert_eq!(nexus_text("Source", "Mã nguồn"), "Mã nguồn");
        set_current(Lang::from_code("de").unwrap());
        assert_eq!(nexus_text("Source", "Mã nguồn"), "Source");
        set_current(before);
    }
}
