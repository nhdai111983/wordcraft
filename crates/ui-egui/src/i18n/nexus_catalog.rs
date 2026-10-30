//! NexusOffice: Vietnamese for interface strings the Nexus branches added or re-worded.
//!
//! Upstream moved every interface catalog to craft-i18n (`CRAFT_I18N_DIR`) and deleted the
//! in-repo `i18n/*.tsv` files the Nexus branches used to add rows to. The rows live here instead,
//! one overlay over the Vietnamese catalog: [`super::tr`] (and so `tl!`) looks a key up here first
//! when the interface is in Vietnamese, then in craft-i18n. Keys are the English source strings,
//! exactly as the code passes them; `{name}` placeholders stay as they are. Rows that craft-i18n's
//! Vietnamese catalog gains with the same wording can be removed from here.

use std::collections::HashMap;
use std::sync::LazyLock;

/// (English key, Vietnamese).
const ROWS: &[(&str, &str)] = &[
    // Review › Language and the status bar name vi-VN documents (NX-UI.6).
    ("Vietnamese", "Tiếng Việt"),
    // The Recover page (File › Recover, NX-UI.8 / NX-UI.11).
    ("Recover documents", "Khôi phục tài liệu"),
    ("Restore opens an unsaved copy. Your original file stays unchanged.", "Khôi phục sẽ mở một bản sao chưa lưu. Tệp gốc của bạn giữ nguyên."),
    ("Recovery is disabled while this document has a password.", "Tính năng khôi phục tắt khi tài liệu này có mật khẩu."),
    ("Local recovery is unavailable or still loading.", "Bản khôi phục trên máy chưa dùng được hoặc đang tải."),
    ("No recovery copies.", "Không có bản khôi phục nào."),
    ("Recovery copy", "Bản khôi phục"),
    ("Discard…", "Loại bỏ…"),
    ("Continue without restoring", "Tiếp tục, không khôi phục"),
    ("Untitled draft", "Bản nháp chưa đặt tên"),
    ("Back", "Quay lại"),
    ("Discard recovery copy", "Loại bỏ bản khôi phục"),
    ("Discard the recovery copy of {name}?", "Loại bỏ bản khôi phục của {name}?"),
    ("This cannot be undone. Your original file stays unchanged.", "Không thể hoàn tác thao tác này. Tệp gốc của bạn giữ nguyên."),
    // The WPS layout (NX-UI.9 – NX-UI.11). "More tabs": the Nexus apps say "thẻ" for a tab.
    ("Saving…", "Đang lưu…"),
    ("More options", "Tùy chọn khác"),
    ("More commands", "Thêm lệnh"),
    ("More tabs", "Thêm thẻ"),
    ("Indent Left", "Thụt lề trái"),
    ("Indent Right", "Thụt lề phải"),
    ("Spacing Before", "Khoảng cách trước"),
    ("Spacing After", "Khoảng cách sau"),
    // QC sweep 2026-10-26 (nx/qc-doc-1): no Microsoft product name for the margins preset
    // (QC-DOC-05), Vietnamese theme and cover names (QC-DOC-06), Auto orientation (QC-DOC-08).
    ("Office 2003 Default", "Cổ điển"),
    ("Studio cover", "Bìa đơn giản"),
    ("Craft", "Cơ bản"),
    ("Studio", "Xưởng vẽ"),
    ("Atelier", "Xưởng thủ công"),
    ("Darkroom", "Phòng tối"),
    ("Gallery", "Phòng tranh"),
    ("Sketchbook", "Sổ phác thảo"),
    ("Auto orientation", "Hướng tự động"),
    // @@NEXUS_VI_MORE@@
];

static MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| ROWS.iter().copied().collect());

/// The Nexus Vietnamese for `key`, if this overlay has it.
pub(super) fn get(key: &str) -> Option<&'static str> {
    MAP.get(key).copied()
}

#[cfg(test)]
mod tests {
    use super::ROWS;

    /// With the Vietnamese catalog embedded, every Nexus row shows in Vietnamese through the
    /// interface lookups (`tl!`, contexts, the catalog-key check) and English stays English: the
    /// strings the nx branches added to the deleted `vi.tsv` still show after upstream moved the
    /// catalogs to craft-i18n.
    #[test]
    fn nexus_rows_show_in_a_vietnamese_interface() {
        use super::super::{Lang, catalogs_embedded, current, has, set_current, t, tc, tr};
        if !catalogs_embedded() {
            return;
        }
        let Some(vi) = Lang::from_code("vi") else { panic!("the Vietnamese catalog is embedded") };
        let before = current();
        set_current(vi);
        for (en, want) in ROWS {
            assert_eq!(tr(vi, en), *want, "{en:?}");
            assert_eq!(t(en), *want, "{en:?} through tl!");
            assert_eq!(tc("no such context", en), *want, "{en:?} with a context without its own row");
            assert!(has(vi, en), "{en:?} counts as translated");
        }
        set_current(Lang::EN);
        for (en, _) in ROWS {
            assert_eq!(t(en), *en, "English keeps {en:?}");
        }
        set_current(before);
        // A row of craft-i18n's own catalog still comes from there.
        assert_eq!(tr(vi, "Close"), "Đóng");
    }

    /// Each key once, each translation non-empty and with the key's `{placeholders}`.
    #[test]
    fn rows_are_unique_and_keep_their_placeholders() {
        let mut seen = std::collections::HashSet::new();
        for (en, vi) in ROWS {
            assert!(seen.insert(*en), "{en:?} is listed twice");
            assert!(!vi.trim().is_empty(), "{en:?} has no Vietnamese");
            let holes = |s: &str| {
                let mut v: Vec<String> = s.split('{').skip(1).filter_map(|p| p.split_once('}').map(|(h, _)| h.to_string())).collect();
                v.sort();
                v
            };
            assert_eq!(holes(en), holes(vi), "{en:?}: placeholders differ");
            assert_eq!(en.ends_with('…'), vi.ends_with('…'), "{en:?}: a trailing … in one needs one in the other");
        }
    }
}
