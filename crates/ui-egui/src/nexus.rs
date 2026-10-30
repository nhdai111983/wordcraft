//! NexusOffice branding for the egui front end, kept in one file so upstream's files change as
//! little as possible (the fork's `nexus/BRANDING_PATCH.md` lists every upstream file touched).
//!
//! Interface strings that name upstream's product go through [`crate::i18n::brand`] in the
//! lookup ([`crate::tl!`]); this module holds what the lookup can't express: the window title,
//! the About page and the switch that hides upstream's community (Discord) links.

use crate::theme::semibold;

/// The product name shown in the window title, the title bar and About.
pub const APP_NAME: &str = "Nexus Docs";
/// The suite the app belongs to.
pub const SUITE_NAME: &str = "NexusOffice";
/// The publisher.
pub const PUBLISHER: &str = "SonnyTech";
/// Desktop app id (Wayland app id, X11 WM_CLASS, macOS bundle id, Linux desktop file).
pub const APP_ID: &str = "vn.sonnytech.nexus.docs";
/// Where the source of this build lives.
pub const SOURCE_URL: &str = "https://github.com/sonnytech-vn/wordcraft";
/// The community link behind the title bar's button and Help › Community. NexusOffice has none
/// yet, so both are hidden; upstream's (ArtCraft's Discord) must never be shown.
pub const COMMUNITY_URL: Option<&str> = None;
/// Whether the app checks for and installs newer releases itself (upstream's `craft-update`,
/// Help › Check for Updates and the check at start). Off: upstream's updater looks for ArtCraft's
/// WordCraft releases and would replace Nexus Docs with WordCraft; Nexus Docs is updated with the
/// NexusOffices installer.
pub const UPDATES: bool = false;

/// What Help › Check for Updates says while [`UPDATES`] is off.
pub fn updates_message() -> &'static str {
    crate::i18n::nexus_text(
        "New versions of Nexus Docs come with the NexusOffices installer.",
        "Phiên bản mới của Nexus Docs được cập nhật qua bộ cài NexusOffices.",
    )
}

/// The About page's one-line description and suite sentence (English, Vietnamese).
pub const ABOUT_LINE: (&str, &str) = (
    "A free, open-source word processor.\nPart of the NexusOffice suite by SonnyTech.",
    "Trình soạn thảo văn bản miễn phí, mã nguồn mở.\nThuộc bộ ứng dụng văn phòng NexusOffice của SonnyTech.",
);

/// The About page's attribution to upstream, which the MIT/Apache licences and ArtCraft's brand
/// licence ask for (text only, no logo, no endorsement implied). Never branded.
pub const ATTRIBUTION: (&str, &str) = (
    "NexusOffice is developed from the open-source apps of ArtCraft (MIT/Apache-2.0 licences). Nexus Docs is based on WordCraft.",
    "NexusOffice được phát triển dựa trên các ứng dụng mã nguồn mở của ArtCraft (giấy phép MIT/Apache-2.0). Nexus Docs dựa trên WordCraft.",
);
/// Upstream's copyright notice, kept as upstream wrote it (an existing catalog key, looked up
/// without branding).
pub const UPSTREAM_COPYRIGHT: &str = "MIT OR Apache-2.0. Copyright (c) 2026 ArtCraft Team and the WordCraft contributors.";

/// The NexusOffices look (a lighter ribbon in the style of WPS Office, Henry 2026-10-23): what
/// differs from upstream's Word-style layout, in one place. Colours are in `theme.rs`.
pub mod look {
    /// Group titles under the ribbon groups ("Clipboard", "Font"…). Off: the ribbon is one row of
    /// controls with dividers, 18 pt shorter, and a group's dialog launcher sits at the bottom
    /// right of its controls. The title stays in the launcher's tooltip and the accessibility tree.
    pub const GROUP_TITLES: bool = false;
    /// Ribbon tabs centred in the tab strip (File stays at the left edge).
    pub const CENTRED_TABS: bool = true;
    /// The largest text size (points) the Home › Styles gallery draws a style's sample in, so a
    /// Title or Heading 1 sample fits its tile instead of being cut off. Font, weight, colour and
    /// the smaller sizes are drawn as the style has them.
    pub const STYLE_SAMPLE_MAX_PT: f32 = 15.0;
}

/// The About page's first tab: name, version, what it is, where the source is, the attribution
/// and the copyright notices.
pub fn about_ui(ui: &mut egui::Ui) {
    use crate::i18n::nexus_text;
    ui.label(egui::RichText::new(tl!("WordCraft")).font(semibold(22.0)));
    let build = option_env!("WORDCRAFT_BUILD_SHA").map(|s| s.get(..8).unwrap_or(s)).unwrap_or(tl!("development build"));
    ui.label(crate::i18n::fmt(tl!("Version {version} ({build})"), &[("version", env!("CARGO_PKG_VERSION")), ("build", build)]));
    ui.label(nexus_text(ABOUT_LINE.0, ABOUT_LINE.1));
    ui.add_space(6.0);
    ui.hyperlink_to(nexus_text("Source code: github.com/sonnytech-vn/wordcraft", "Mã nguồn: github.com/sonnytech-vn/wordcraft"), SOURCE_URL);
    ui.add_space(6.0);
    ui.label(egui::RichText::new(nexus_text(ATTRIBUTION.0, ATTRIBUTION.1)).small().weak());
    ui.label(egui::RichText::new(crate::i18n::tr(crate::i18n::current(), UPSTREAM_COPYRIGHT)).small().weak());
    ui.label(egui::RichText::new(nexus_text("Nexus Docs © 2026 SonnyTech.", "Nexus Docs © 2026 SonnyTech.")).small().weak());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_follow_the_branding_table() {
        assert_eq!(APP_NAME, "Nexus Docs");
        assert_eq!(SUITE_NAME, "NexusOffice");
        assert_eq!(PUBLISHER, "SonnyTech");
        assert_eq!(APP_ID, "vn.sonnytech.nexus.docs");
        assert!(COMMUNITY_URL.is_none(), "upstream's community links stay hidden");
        assert_eq!(SOURCE_URL, "https://github.com/sonnytech-vn/wordcraft");
        assert!(ABOUT_LINE.0.ends_with("Part of the NexusOffice suite by SonnyTech."));
        assert!(ABOUT_LINE.1.ends_with("Thuộc bộ ứng dụng văn phòng NexusOffice của SonnyTech."));
    }

    /// About keeps upstream's names in the attribution and the copyright notice (not branded)
    /// and shows Nexus Docs as the product, in English and Vietnamese.
    #[test]
    fn about_attributes_upstream_in_english_and_vietnamese() {
        use crate::i18n::{Lang, current, set_current, tr};
        let before = current();
        for code in ["en", "vi"] {
            set_current(Lang::from_code(code).unwrap());
            let att = crate::i18n::nexus_text(ATTRIBUTION.0, ATTRIBUTION.1);
            assert!(att.contains("ArtCraft") && att.contains("WordCraft") && att.contains("MIT/Apache-2.0"), "{code}: {att}");
            assert!(tr(current(), UPSTREAM_COPYRIGHT).contains("ArtCraft"), "{code}: copyright notice kept");
            assert_eq!(crate::tl!("WordCraft"), APP_NAME);
        }
        set_current(before);
    }
}
