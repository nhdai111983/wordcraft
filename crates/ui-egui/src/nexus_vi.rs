//! NexusOffice: Vietnamese defaults that follow the interface language (QC 2026-10-26).
//!
//! When the interface is Vietnamese and the user hasn't picked a region (File › Options ›
//! Region: Automatic), "Automatic" means Vietnam whatever the system's locale says: new documents
//! start on A4 with the decree 30 margins, Times New Roman 13 pt (drawn with a metric-compatible
//! open substitute) and Vietnamese proofing, lengths are shown in centimetres with a decimal
//! comma (QC-DOC-02). Any other interface language keeps the system's region, as upstream.

use wordcraft_engine::Region;

use crate::WordApp;

/// The region "Automatic" stands for under the interface language `ui` (a catalog code such as
/// `vi` or `en`), given the system's own region.
pub fn region_for(system: &Region, ui: &str) -> Region {
    if ui == "vi" && system.language != "vi" { Region::from_tag("vi-VN") } else { system.clone() }
}

/// Every frame: keep the session's "Automatic" region in step with the interface language. The
/// region the host gave the session (from the system) is remembered, so switching the interface
/// back gives it back; a host that sets a new one later is followed too.
pub(crate) fn sync_region(app: &mut WordApp, ui: &str) {
    let current = app.session.system_region.clone();
    if app.nexus_region_set.as_ref() != Some(&current) {
        app.nexus_host_region = Some(current.clone());
    }
    let host = app.nexus_host_region.clone().unwrap_or(current);
    let want = region_for(&host, ui);
    app.nexus_region_set = Some(want.clone());
    app.session.set_system_region(want);
}

/// A plain number (line spacing 1,15, a pen 0,5 pt) written with the region's decimal separator
/// (QC-DOC-10).
pub fn num(v: f32) -> String {
    let sep = crate::options::decimal();
    let s = v.to_string();
    if sep == '.' { s } else { s.replace('.', &sep.to_string()) }
}

/// A number field written and read with the region's decimal separator; typing either separator
/// works (QC-DOC-10).
pub fn regional(d: egui::DragValue<'_>) -> egui::DragValue<'_> {
    let sep = crate::options::decimal();
    d.custom_formatter(move |n, decimals| {
        let s = egui::emath::format_with_decimals_in_range(n, decimals);
        if sep == '.' { s } else { s.replace('.', &sep.to_string()) }
    })
    .custom_parser(|text| text.trim().replace(',', ".").parse::<f64>().ok())
}

/// The engine's placeholder author name, used until someone names themselves.
const DEFAULT_AUTHOR: &str = "Nexus Docs User";

/// QC-CHUNG-05: a session still on the placeholder author takes the computer's account name
/// (`USERNAME` on Windows, `USER` elsewhere), as Word falls back to the system's user. A name
/// saved in Options (applied later from the preferences) wins. Not in tests: they stay
/// independent of who runs them.
pub fn os_author(session: &mut wordcraft_engine::Session) {
    if cfg!(test) || session.author != DEFAULT_AUTHOR {
        return;
    }
    if let Some(name) = os_user_name(|k| std::env::var(k).ok()) {
        session.author = name;
    }
}

/// The account name from the environment `var`, trimmed, without control characters, at most 64
/// characters; `None` when there is none.
pub fn os_user_name(var: impl Fn(&str) -> Option<String>) -> Option<String> {
    ["USERNAME", "USER", "LOGNAME"].iter().find_map(|k| {
        let v: String = var(k)?.chars().filter(|c| !c.is_control()).take(64).collect();
        let v = v.trim().to_string();
        (!v.is_empty() && v != "root").then_some(v)
    })
}

/// The name of an accent preset (Options › Accent colour) in the interface language.
pub fn accent_name(name: &str) -> &str {
    use crate::i18n::nexus_text;
    match name {
        "Indigo" => nexus_text("Indigo", "Chàm"),
        "Violet" => nexus_text("Violet", "Tím"),
        "Mint" => nexus_text("Mint", "Xanh bạc hà"),
        "Amber" => nexus_text("Amber", "Hổ phách"),
        "Coral" => nexus_text("Coral", "San hô"),
        "Sky" => nexus_text("Sky", "Xanh da trời"),
        _ => name,
    }
}

/// A tooltip naming a command and its key, `Dán (Ctrl+V)`: `label` is in the interface language,
/// `key` as [`crate::widgets::shortcut_text`] writes it for this platform (QC-DOC-13).
pub fn with_key(label: &str, key: &str) -> String {
    if key.is_empty() { label.to_string() } else { format!("{label} ({key})") }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wordcraft_engine::Session;

    fn app() -> WordApp {
        WordApp::new(Session::new(wordcraft_doc::Document::new()), crate::Services::default())
    }

    fn near(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.05
    }

    #[test]
    fn region_follows_a_vietnamese_interface_only() {
        let us = Region::from_tag("en-US");
        assert_eq!(region_for(&us, "vi"), Region::from_tag("vi-VN"));
        assert_eq!(region_for(&us, "en"), us);
        assert_eq!(region_for(&Region::from_tag("de-DE"), "pl"), Region::from_tag("de-DE"));
        // A Vietnamese system keeps its own tag.
        assert_eq!(region_for(&Region::from_tag("vi"), "vi"), Region::from_tag("vi"));
    }

    /// QC-DOC-02: the window's blank document (made before the first frame, on the system's
    /// en-US region) moves to A4, decree 30 margins, Times New Roman 13 and vi-VN, and lengths
    /// read in centimetres, when the interface is Vietnamese. English keeps Letter and inches.
    #[test]
    fn a_vietnamese_interface_starts_on_a4_in_centimetres() {
        let mut a = app();
        a.ui.language = "vi".into();
        let blank = a.session.blank_document();
        a.session.set_document(blank);
        a.session.file_name = Some("Văn bản 1".into());
        let id = a.session.document_id();
        let ctx = egui::Context::default();
        a.logic(&ctx);
        let sp = &a.session.doc.last_section;
        assert!(near(sp.page_w, 595.28) && near(sp.page_h, 841.89), "A4: {}x{}", sp.page_w, sp.page_h);
        assert!(near(sp.margin_left, 85.04) && near(sp.margin_right, 42.52) && near(sp.margin_top, 56.69));
        let c = &a.session.doc.styles.default_chr;
        assert_eq!((c.font.as_deref(), c.size, c.lang.as_deref()), (Some("Times New Roman"), Some(13.0), Some("vi-VN")));
        assert_eq!(a.session.units(), wordcraft_geom::Unit::Centimeters);
        assert_eq!(a.session.format_length(wordcraft_geom::PT_PER_CM * 2.5), "2,5 cm");
        assert_eq!(a.session.document_id(), id, "the same document, re-laid");
        assert_eq!(a.session.file_name.as_deref(), Some("Văn bản 1"));
        assert!(!a.session.dirty && a.session.undo_depth() == 0);
        // File › New makes the same page.
        a.run("file.new", json!({"template": "blank"})).unwrap();
        assert!(near(a.session.doc.last_section.page_w, 595.28));

        // Back to English: the system's region again, and an untouched page follows it.
        a.ui.language = "en".into();
        a.logic(&ctx);
        assert_eq!(a.session.region(), Region::from_tag("en-US"));
        let sp = &a.session.doc.last_section;
        assert_eq!((sp.page_w, sp.page_h, sp.margin_left), (612.0, 792.0, 72.0));
        assert_eq!(a.session.doc.styles.default_chr.font.as_deref(), Some(wordcraft_doc::styles::BODY_FONT));
        assert_eq!(a.session.format_length(72.0), "1\"");
    }

    #[test]
    fn numbers_use_the_regions_decimal_separator() {
        let ctx = egui::Context::default();
        let mut a = app();
        a.ui.language = "vi".into();
        a.logic(&ctx);
        assert_eq!((num(1.15), num(0.5), num(2.0)), ("1,15".to_string(), "0,5".to_string(), "2".to_string()));
        a.ui.language = "en".into();
        a.logic(&ctx);
        assert_eq!(num(1.15), "1.15");
    }

    /// QC-DOC-13: keys are written the platform's way (Go To's tooltip), not the Mac's on Windows.
    #[test]
    fn paste_names_the_platforms_key() {
        let a = app();
        let key = crate::widgets::shortcut_text(&a, "edit.paste");
        if cfg!(target_os = "macos") {
            assert_eq!(key, "⌘V");
        } else {
            assert_eq!(key, "Ctrl+V");
        }
        assert_eq!(with_key("Dán", &key), format!("Dán ({key})"));
        assert_eq!(with_key("Đi tới", ""), "Đi tới");
    }

    /// QC-CHUNG-05: the computer's account name, not "Nexus Docs User"; Vietnamese colour names.
    #[test]
    fn account_name_and_colour_names() {
        let env = |pairs: &'static [(&'static str, &'static str)]| move |k: &str| pairs.iter().find(|(n, _)| *n == k).map(|(_, v)| v.to_string());
        assert_eq!(os_user_name(env(&[("USERNAME", " Đại ")])).as_deref(), Some("Đại"));
        assert_eq!(os_user_name(env(&[("USER", "henry")])).as_deref(), Some("henry"));
        assert_eq!(os_user_name(env(&[("USER", "root")])), None);
        assert_eq!(os_user_name(env(&[("USERNAME", "  ")])), None);
        let before = crate::i18n::current();
        crate::i18n::set_current(crate::i18n::Lang::from_pref("vi"));
        assert_eq!(accent_name("Amber"), "Hổ phách");
        crate::i18n::set_current(crate::i18n::Lang::from_pref("en"));
        assert_eq!(accent_name("Amber"), "Amber");
        crate::i18n::set_current(before);
        for (n, _) in crate::theme::ACCENT_PRESETS {
            assert_ne!(accent_name(n), "", "{n}");
        }
    }

    /// Text on the page, a file, or a region the user picked: nothing moves.
    #[test]
    fn a_touched_document_or_a_chosen_region_is_left_alone() {
        let ctx = egui::Context::default();
        let mut a = app();
        a.run("text.insert", json!({"text": "Letter"})).unwrap();
        a.ui.language = "vi".into();
        a.logic(&ctx);
        assert_eq!(a.session.doc.last_section.page_w, 612.0, "typed text keeps its page");
        assert_eq!(a.session.region(), Region::from_tag("vi-VN"), "new documents are Vietnamese");

        let mut b = app();
        b.session.prefs.region = Some("en-GB".into());
        b.ui.language = "vi".into();
        b.logic(&ctx);
        assert_eq!(b.session.region(), Region::from_tag("en-GB"), "the user's region wins");
        assert_eq!(b.session.doc.styles.default_chr.font.as_deref(), Some(wordcraft_doc::styles::BODY_FONT));
    }
}
