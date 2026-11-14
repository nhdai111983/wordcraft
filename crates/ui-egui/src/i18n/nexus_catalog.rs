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
    // Upstream sync 2026-11-12 (0.18.1), second pass: every `tl!` string of the editor that neither
    // craft-i18n's Vietnamese (1e515422) nor this overlay had. Drop a row once craft-i18n has it.
    ("*Bold* and _italic_ as real formatting", "*Đậm* và _nghiêng_ thành định dạng thật"),
    ("Add a period with double-space", "Thêm dấu chấm khi gõ hai dấu cách"),
    ("Add to AutoCorrect", "Thêm vào Tự động sửa"),
    ("Advanced Properties…", "Thuộc tính nâng cao…"),
    ("AutoCorrect Options…", "Tùy chọn tự động sửa…"),
    ("Automatically add words to list (First Letter)", "Tự động thêm từ vào danh sách (Viết hoa chữ cái đầu)"),
    ("Automatically add words to list (INitial CAps)", "Tự động thêm từ vào danh sách (Hai chữ hoa đầu)"),
    ("Bookmarks", "Dấu trang"),
    ("Built-in Heading styles", "Kiểu tiêu đề có sẵn"),
    ("Change to:", "Đổi thành:"),
    ("Change…", "Đổi…"),
    ("Check for Updates", "Kiểm tra bản cập nhật"),
    ("Clear all", "Xóa tất cả"),
    ("Corporate Author", "Tác giả là tổ chức"),
    ("Couldn't install: {error}", "Không cài được: {error}"),
    ("Couldn't remove: {error}", "Không gỡ được: {error}"),
    (
        "Couldn't save {name}, so it is still open and nothing was lost: {error}",
        "Không lưu được {name} nên tài liệu vẫn đang mở và không mất gì: {error}",
    ),
    ("Couldn't turn on downloading: {error}", "Không bật được tải xuống: {error}"),
    (
        "Crop on the page works on unturned pictures: set Rotation to 0° first, or use the crop boxes",
        "Cắt ngay trên trang chỉ dùng được với ảnh chưa xoay: hãy đặt Xoay về 0° trước, hoặc nhập số đo cắt trong các ô",
    ),
    ("Current Document", "Tài liệu hiện tại"),
    ("Dock Left", "Neo bên trái"),
    ("Dock Right", "Neo bên phải"),
    ("File type", "Loại tệp"),
    ("Float", "Thả nổi"),
    ("Folder path", "Đường dẫn thư mục"),
    ("Get the list of languages again", "Lấy lại danh sách ngôn ngữ"),
    ("Imported 1 source", "Đã nhập 1 nguồn"),
    ("Imported {n} sources", "Đã nhập {n} nguồn"),
    ("Install", "Cài đặt"),
    ("Installed", "Đã cài đặt"),
    ("Installing… {percent}%", "Đang cài… {percent}%"),
    ("Language packs aren't available in the browser.", "Gói ngôn ngữ không khả dụng trong trình duyệt."),
    ("Language packs come from a test source: {source}", "Gói ngôn ngữ đang được lấy từ nguồn thử nghiệm: {source}"),
    (
        "Language packs need the font store folder, and there is none on this computer.",
        "Gói ngôn ngữ cần thư mục kho phông chữ, nhưng máy này không có.",
    ),
    ("Licence: {licence}. Source: {source}", "Giấy phép: {licence}. Nguồn: {source}"),
    ("Macro name:", "Tên macro:"),
    ("Move Pane: {title}", "Di chuyển ngăn: {title}"),
    ("No new sources in that file", "Tệp đó không có nguồn nào mới"),
    ("Not in Dictionary", "Không có trong từ điển"),
    ("Nothing of this type in the document.", "Tài liệu không có mục nào thuộc loại này."),
    ("Only text properties can be changed.", "Chỉ đổi được thuộc tính dạng văn bản."),
    (
        "Only you can turn on downloading: click the button yourself. Agents can install once it is on.",
        "Chỉ bạn mới bật được tải xuống: hãy tự bấm nút. Sau khi bật, các tác tử mới cài được.",
    ),
    ("Page color", "Màu nền trang"),
    ("Pages in the range", "Các trang trong phạm vi"),
    ("Pane Options", "Tùy chọn ngăn"),
    ("Property names are 1–{max} characters.", "Tên thuộc tính dài từ 1 đến {max} ký tự."),
    ("Property values are at most {max} bytes.", "Giá trị thuộc tính dài tối đa {max} byte."),
    ("Record…", "Ghi macro…"),
    ("Resize Pane", "Đổi kích thước ngăn"),
    ("Run", "Chạy"),
    ("Scale to paper size", "Co giãn theo khổ giấy"),
    ("Select a place in this document:", "Chọn một vị trí trong tài liệu này:"),
    ("Select a property to change.", "Chọn một thuộc tính để đổi."),
    ("Set left- and first-indent with tabs and backspaces", "Đặt thụt lề trái và thụt dòng đầu bằng phím Tab và Backspace"),
    ("Spanish (Latin America)", "Tiếng Tây Ban Nha (Mỹ Latinh)"),
    ("Spanish (Uruguay)", "Tiếng Tây Ban Nha (Uruguay)"),
    ("Stop Recording", "Dừng ghi"),
    ("Suggestions", "Gợi ý"),
    ("Text Size", "Cỡ chữ"),
    (
        "Text properties only: Word's number, date and yes-no types aren't offered.",
        "Chỉ có thuộc tính dạng văn bản: chưa có các loại số, ngày và có/không.",
    ),
    ("The document or selection changed. Check again to continue.", "Tài liệu hoặc vùng chọn đã thay đổi. Hãy kiểm tra lại để tiếp tục."),
    ("The number must be a divisor of {max}.", "Số này phải là ước số của {max}."),
    (
        "The spelling and hyphenation data WordCraft uses, and their licences.",
        "Dữ liệu chính tả và ngắt từ mà Nexus Docs dùng, cùng giấy phép của chúng.",
    ),
    (
        "The spelling dictionary for {language} couldn't be read, so it isn't checked. File: {file}. Reason: {reason}",
        "Không đọc được từ điển chính tả {language} nên ngôn ngữ này không được kiểm tra. Tệp: {file}. Lý do: {reason}",
    ),
    ("There are no macros yet. Record one to add it here.", "Chưa có macro nào. Hãy ghi một macro để thêm vào đây."),
    ("This document has no custom properties.", "Tài liệu này chưa có thuộc tính tùy chỉnh nào."),
    ("Top of the Document", "Đầu tài liệu"),
    ("Update check is not supported in the web browser.", "Không kiểm tra được bản cập nhật trong trình duyệt."),
    ("View and edit the document's custom properties.", "Xem và sửa các thuộc tính tùy chỉnh của tài liệu."),
    ("{field}, record {n}", "{field}, bản ghi {n}"),
    ("{language} (dictionary unreadable)", "{language} (không đọc được từ điển)"),
    ("{language}: {file} (unreadable: {reason})", "{language}: {file} (không đọc được: {reason})"),
    ("{n} KB", "{n} KB"),
    ("{n} MB", "{n} MB"),
    ("{paper}   {width} × {height}", "{paper}   {width} × {height}"),
    // Counts (`tn`): keyed `one|other` like craft-i18n's `@plural` rows; Vietnamese has one form.
    ("{n} font not installed|{n} fonts not installed", "{n} phông chữ chưa cài"),
    ("…and {n} more|…and {n} more", "…và {n} phông chữ khác"),
    // Upstream sync 2026-11-14: Markdown Source view, scroll speeds, Fit One Page — interface
    // text craft-i18n's Vietnamese (1e51542) doesn't have yet.
    (
        "Saved. Edits in the formatted view meant writing the Markdown out again, so its own layout wasn't kept.",
        "Đã lưu. Vì có sửa trong chế độ xem định dạng nên Markdown được ghi lại từ đầu; bố cục riêng của tệp không được giữ.",
    ),
    (
        "This document has content Markdown can't hold. Changing the text here removes it; one Undo afterwards brings it back.",
        "Tài liệu này có nội dung Markdown không giữ được. Sửa chữ ở đây sẽ bỏ nội dung đó; Hoàn tác một lần ngay sau đó sẽ lấy lại.",
    ),
    ("Markdown Source", "Mã nguồn Markdown"),
    ("Markdown Source takes up to 2 MB of text", "Mã nguồn Markdown nhận tối đa 2 MB chữ"),
    ("Nothing to undo", "Không có gì để hoàn tác"),
    ("Nothing to redo", "Không có gì để làm lại"),
    ("Mouse wheel scroll speed:", "Tốc độ cuộn bằng con lăn chuột:"),
    ("Touchpad scroll speed:", "Tốc độ cuộn bằng bàn di chuột:"),
    ("On X11, the touchpad follows the mouse wheel speed.", "Trên X11, bàn di chuột theo tốc độ của con lăn chuột."),
    ("Fit\nOne Page", "Vừa\nmột trang"),
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

    /// Counts in the status bar's missing-fonts item are Vietnamese, with the number in.
    #[test]
    fn counts_show_in_a_vietnamese_interface() {
        use super::super::{Lang, catalogs_embedded, current, set_current, tn};
        if !catalogs_embedded() {
            return;
        }
        let Some(vi) = Lang::from_code("vi") else { panic!("the Vietnamese catalog is embedded") };
        let before = current();
        set_current(vi);
        // The number comes isolated (LRI…PDI) so a right-to-left sentence can't reorder it.
        let count = |n: u64, one: &str, other: &str| craft_i18n::strip_isolates(&tn(n, one, other));
        assert_eq!(count(1, "{n} font not installed", "{n} fonts not installed"), "1 phông chữ chưa cài");
        assert_eq!(count(3, "{n} font not installed", "{n} fonts not installed"), "3 phông chữ chưa cài");
        assert_eq!(count(5, "…and {n} more", "…and {n} more"), "…và 5 phông chữ khác");
        set_current(Lang::EN);
        assert_eq!(count(3, "{n} font not installed", "{n} fonts not installed"), "3 fonts not installed");
        set_current(before);
    }

    /// Each key once, each translation non-empty and with the key's `{placeholders}`.
    #[test]
    fn rows_are_unique_and_keep_their_placeholders() {
        let mut seen = std::collections::HashSet::new();
        for (en, vi) in ROWS {
            assert!(seen.insert(*en), "{en:?} is listed twice");
            assert!(!vi.trim().is_empty(), "{en:?} has no Vietnamese");
            // Each placeholder at least once (a count row's key has `{n}` in both its forms).
            let holes = |s: &str| {
                let mut v: Vec<String> = s.split('{').skip(1).filter_map(|p| p.split_once('}').map(|(h, _)| h.to_string())).collect();
                v.sort();
                v.dedup();
                v
            };
            assert_eq!(holes(en), holes(vi), "{en:?}: placeholders differ");
            assert_eq!(en.ends_with('…'), vi.ends_with('…'), "{en:?}: a trailing … in one needs one in the other");
        }
    }
}
