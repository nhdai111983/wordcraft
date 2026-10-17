//! NexusOffices branding for the egui front end, kept in one file so upstream's files change as
//! little as possible (the fork's `nexus/BRANDING_PATCH.md` lists every upstream file touched).
//!
//! Interface strings that name upstream's product go through [`crate::i18n::brand`] in the
//! lookup ([`crate::tl!`]); this module holds what the lookup can't express: the window title,
//! the About page and the switch that hides upstream's community (Discord) links.

use crate::theme::semibold;

/// The product name shown in the window title, the title bar and About.
pub const APP_NAME: &str = "Nexus Docs";
/// The suite the app belongs to.
pub const SUITE_NAME: &str = "NexusOffices";
/// The publisher.
pub const PUBLISHER: &str = "Sonnytech";
/// Desktop app id (Wayland app id, X11 WM_CLASS, macOS bundle id, Linux desktop file).
pub const APP_ID: &str = "vn.sonnytech.nexus.docs";
/// Where the source of this build lives.
pub const SOURCE_URL: &str = "https://github.com/nhdai111983/wordcraft";
/// The community link behind the title bar's button and Help › Community. NexusOffices has none
/// yet, so both are hidden; upstream's (ArtCraft's Discord) must never be shown.
pub const COMMUNITY_URL: Option<&str> = None;

/// The About page's attribution to upstream, which the MIT/Apache licences and ArtCraft's brand
/// licence ask for (text only, no logo, no endorsement implied). Never branded.
pub const ATTRIBUTION: (&str, &str) = (
    "NexusOffices is developed from the open-source apps of ArtCraft (MIT/Apache-2.0 licences). Nexus Docs is based on WordCraft.",
    "NexusOffices được phát triển dựa trên các ứng dụng mã nguồn mở của ArtCraft (giấy phép MIT/Apache-2.0). Nexus Docs dựa trên WordCraft.",
);
/// Upstream's copyright notice, kept as upstream wrote it (an existing catalog key, looked up
/// without branding).
pub const UPSTREAM_COPYRIGHT: &str = "MIT OR Apache-2.0. Copyright (c) 2026 ArtCraft Team and the WordCraft contributors.";

/// The About page's first tab: name, version, what it is, where the source is, the attribution
/// and the copyright notices.
pub fn about_ui(ui: &mut egui::Ui) {
    use crate::i18n::nexus_text;
    ui.label(egui::RichText::new(tl!("WordCraft")).font(semibold(22.0)));
    let build = option_env!("WORDCRAFT_BUILD_SHA").map(|s| s.get(..8).unwrap_or(s)).unwrap_or(tl!("development build"));
    ui.label(crate::i18n::fmt(tl!("Version {version} ({build})"), &[("version", env!("CARGO_PKG_VERSION")), ("build", build)]));
    ui.label(nexus_text(
        "A free, open-source word processor.\nPart of the NexusOffices office suite by Sonnytech.",
        "Trình soạn thảo văn bản miễn phí, mã nguồn mở.\nThuộc bộ ứng dụng văn phòng NexusOffices của Sonnytech.",
    ));
    ui.add_space(6.0);
    ui.hyperlink_to(nexus_text("Source code: github.com/nhdai111983/wordcraft", "Mã nguồn: github.com/nhdai111983/wordcraft"), SOURCE_URL);
    ui.add_space(6.0);
    ui.label(egui::RichText::new(nexus_text(ATTRIBUTION.0, ATTRIBUTION.1)).small().weak());
    ui.label(egui::RichText::new(crate::i18n::tr(crate::i18n::current(), UPSTREAM_COPYRIGHT)).small().weak());
    ui.label(egui::RichText::new(nexus_text("Nexus Docs © 2026 Sonnytech.", "Nexus Docs © 2026 Sonnytech.")).small().weak());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_follow_the_branding_table() {
        assert_eq!(APP_NAME, "Nexus Docs");
        assert_eq!(SUITE_NAME, "NexusOffices");
        assert_eq!(PUBLISHER, "Sonnytech");
        assert_eq!(APP_ID, "vn.sonnytech.nexus.docs");
        assert!(COMMUNITY_URL.is_none(), "upstream's community links stay hidden");
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
