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
    // Upstream sync 2026-11-05: interface text upstream added whose Vietnamese craft-i18n
    // (9d6d54d6) doesn't have yet. Drop a row once craft-i18n has it.
    (
        "Booklets: duplex on the short edge. Blank pages are added at the end.",
        "Sách gấp: in hai mặt, lật theo cạnh ngắn. Trang trống được thêm vào cuối.",
    ),
    ("Couldn't get the list of languages: {error}", "Không lấy được danh sách ngôn ngữ: {error}"),
    ("Getting the list of languages…", "Đang lấy danh sách ngôn ngữ…"),
    ("Install ({size})", "Cài đặt ({size})"),
    ("Installing turns on downloading of fonts and Language packs from GitHub.", "Cài đặt sẽ bật tải phông chữ và gói ngôn ngữ từ GitHub."),
    ("Inverse", "Đảo màu"),
    ("Next sheet", "Tờ tiếp theo"),
    ("Not installed — shown as", "Chưa cài — hiển thị bằng"),
    ("Pages per sheet", "Số trang mỗi tờ"),
    ("Previous sheet", "Tờ trước"),
    ("Repeated word", "Từ bị lặp"),
    ("Sepia", "Nâu đỏ"),
    ("Set the colors in the Text Pane to use custom colors.", "Đặt màu trong Ngăn văn bản để dùng màu tùy chỉnh."),
    ("Shape color", "Màu hình"),
    ("Sheet {page} of {pages}", "Tờ {page} trên {pages}"),
    ("Shown in substitute fonts. Click for details.", "Đang hiển thị bằng phông chữ thay thế. Bấm để xem chi tiết."),
    ("Style Color", "Màu theo kiểu"),
    ("{language} proofing isn't installed.", "Chưa cài hiệu đính cho {language}."),
    ("Emphasis Mark", "Dấu nhấn mạnh"),
    ("Flat OpenDocument Text (*.fodt)", "Tệp văn bản OpenDocument phẳng (*.fodt)"),
    ("Item Color", "Màu mục"),
    ("Missing Fonts", "Phông chữ bị thiếu"),
    ("OpenDocument Text Template (*.ott)", "Mẫu văn bản OpenDocument (*.ott)"),
    (
        "{font} isn't installed on this computer, so its text is shown in {shown}. Line breaks and page count can differ from where the document was made. If you have the font file, add it with Add Font… at the end of this list.",
        "{font} chưa được cài trên máy này nên chữ dùng phông này được hiển thị bằng {shown}. Ngắt dòng và số trang có thể khác với máy đã tạo tài liệu. Nếu có tệp phông chữ, hãy thêm bằng Thêm phông chữ… ở cuối danh sách này.",
    ),
    // Upstream sync 2026-11-12: interface text upstream added whose Vietnamese craft-i18n
    // (1e515422) doesn't have yet. Drop a row once craft-i18n has it.
    ("Applies the next time WordCraft starts.", "Có hiệu lực từ lần khởi động Nexus Docs tiếp theo."),
    ("Cancel printing", "Hủy in"),
    (
        "Cancellation could not be confirmed here. The printer driver may still be processing the job. If it appears in the system's print queue, cancel it there.",
        "Không xác nhận được việc hủy tại đây. Trình điều khiển máy in có thể vẫn đang xử lý lệnh in. Nếu lệnh in còn trong hàng đợi in của hệ thống, hãy hủy ở đó.",
    ),
    ("Column Edge", "Mép cột"),
    ("Could not save the preferences: {error}", "Không lưu được tùy chọn: {error}"),
    ("Default format saved.", "Đã lưu định dạng mặc định."),
    ("Display server:", "Máy chủ hiển thị:"),
    ("Hidden text is not shown unless you display it.", "Chữ ẩn không hiện ra trừ khi bạn bật hiển thị."),
    ("Language data", "Dữ liệu ngôn ngữ"),
    ("No other WordCraft windows are open.", "Không có cửa sổ Nexus Docs nào khác đang mở."),
    ("no other Nexus Docs window is open", "không có cửa sổ Nexus Docs nào khác đang mở"),
    ("only the desktop app has several windows", "chỉ ứng dụng trên máy tính mới có nhiều cửa sổ"),
    ("turn on View Side by Side first", "hãy bật Xem song song trước"),
    ("Paragraph Tools", "Công cụ đoạn văn"),
    ("Paragraph\nTools", "Công cụ\nđoạn văn"),
    ("Printing cancelled. Nothing was sent to the printer.", "Đã hủy in. Chưa có gì được gửi tới máy in."),
    ("Remove from list", "Xóa khỏi danh sách"),
    ("Removed {count} documents from the recent list", "Đã xóa {count} tài liệu khỏi danh sách gần đây"),
    ("Removed {name} from the recent list", "Đã xóa {name} khỏi danh sách gần đây"),
    ("Screen paper color:", "Màu giấy trên màn hình:"),
    ("Status bar count:", "Số đếm trên thanh trạng thái:"),
    ("You can also right-click the count in the status bar to change it.", "Bạn cũng có thể bấm chuột phải vào số đếm trên thanh trạng thái để đổi."),
    ("{chars} characters", "Ký tự: {chars}"),
    ("{selected} of {chars} characters", "Ký tự: {selected} trên {chars}"),
    ("{chars} characters (no spaces)", "Ký tự (không tính khoảng trắng): {chars}"),
    ("{selected} of {chars} characters (no spaces)", "Ký tự (không tính khoảng trắng): {selected} trên {chars}"),
    ("Synchronous Scrolling", "Cuộn đồng bộ"),
    ("View Side by Side", "Xem song song"),
    ("Type a size in any unit, such as 3 cm or 300 px.", "Nhập kích thước theo đơn vị bất kỳ, ví dụ 3 cm hoặc 300 px."),
    ("Update automatically", "Tự động cập nhật"),
    (
        "Updates are off in this window: WordCraft was started with another program's AppImage settings. Start WordCraft from its own menu entry to update it.",
        "Cập nhật bị tắt trong cửa sổ này: Nexus Docs được khởi động với thiết lập AppImage của chương trình khác. Hãy mở Nexus Docs từ mục menu riêng để cập nhật.",
    ),
    ("Warm cream", "Kem ấm"),
    // Home › Paragraph Tools (#5043) and the rest the i18n audit finds untranslated.
    ("Add Blank Paragraphs Between", "Chèn đoạn trống giữa các đoạn"),
    ("Insert Blank Paragraphs Between", "Chèn đoạn trống giữa các đoạn"),
    ("Convert Indents to Spaces", "Đổi thụt lề thành khoảng trắng"),
    ("Convert Line Breaks to Paragraphs", "Đổi ngắt dòng thành đoạn"),
    ("Convert Spaces to Indentation", "Đổi khoảng trắng thành thụt lề"),
    ("Delete Blank Paragraphs", "Xóa đoạn trống"),
    ("Delete Blank Paragraphs in the Whole Document", "Xóa đoạn trống trong toàn tài liệu"),
    ("Indent Leading Spaces", "Đổi khoảng trắng đầu dòng thành thụt lề"),
    ("Indents to Leading Spaces", "Đổi thụt lề thành khoảng trắng đầu dòng"),
    ("Join Lines", "Nối dòng"),
    ("Remove Extra Spaces", "Xóa khoảng trắng thừa"),
    ("Remove Extra Spaces in the Whole Document", "Xóa khoảng trắng thừa trong toàn tài liệu"),
    ("Remove Leading Spaces", "Xóa khoảng trắng đầu dòng"),
    ("Remove Leading Spaces in the Whole Document", "Xóa khoảng trắng đầu dòng trong toàn tài liệu"),
    ("Remove Line Breaks", "Xóa ngắt dòng"),
    ("Separate Paragraphs", "Tách đoạn"),
    ("Split Lines into Paragraphs", "Tách dòng thành đoạn"),
    ("Delete Previous Sentence", "Xóa câu trước"),
    ("Document background", "Nền tài liệu"),
    ("Linux display server", "Máy chủ hiển thị Linux"),
    ("Screen Paper Color", "Màu giấy trên màn hình"),
    (
        "Screen only. Document backgrounds, printing and exports keep their colours. Dark page and explicit Read Mode colours take precedence.",
        "Chỉ trên màn hình. Nền tài liệu, bản in và tệp xuất giữ màu của chúng. Trang tối và màu Chế độ đọc đặt riêng được ưu tiên.",
    ),
    ("Select Canvas Member", "Chọn thành phần trong khung vẽ"),
    ("Status bar", "Thanh trạng thái"),
    ("Strict Export Report", "Báo cáo xuất nghiêm ngặt"),
    ("a shape inside a drawing canvas is placed by the canvas", "hình nằm trong khung vẽ do khung vẽ đặt vị trí"),
    // Engine reasons Nexus names at the source (so craft-i18n's key no longer matches).
    (
        "this SmartArt was made in another program: Nexus Docs shows it but can't edit it",
        "SmartArt này được tạo bằng chương trình khác: Nexus Docs hiển thị được nhưng không sửa được",
    ),
    (
        "this chart was made in another program: Nexus Docs shows it but can't edit it",
        "biểu đồ này được tạo bằng chương trình khác: Nexus Docs hiển thị được nhưng không sửa được",
    ),
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
        // English keeps the key, with the product's name as installed (`brand`).
        for (en, _) in ROWS {
            assert_eq!(t(en), super::super::brand(en), "English keeps {en:?}");
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
