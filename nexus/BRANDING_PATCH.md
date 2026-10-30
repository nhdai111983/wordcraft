# BRANDING_PATCH

Danh sách file mà nhánh `nexus` sửa so với upstream vì lý do thương hiệu.
Mỗi mục là đường dẫn file chính xác trong dấu backtick (không dùng glob/thư mục).

Giá trị: sản phẩm **Nexus Docs**, bộ **NexusOffice**, nhà phát hành **SonnyTech** (viết hoa S và T), app id
`vn.sonnytech.nexus.docs`, tài sản phát hành `NexusDocs-*`, binary cài đặt `nexus-docs`/`nexus-docs-cli`
(Windows MSI, Linux; tên build Cargo vẫn là `wordcraft`/`wordcraft-cli`).

Không sửa catalog nào: chữ mới của About (en/vi) nằm trong `nexus.rs` qua `i18n::nexus_text(en, vi)`
(không phải key catalog). Từ đồng bộ 2026-10-29 các catalog không còn trong repo: upstream chuyển chúng
sang `storytold/craft-i18n` (`locales/<mã>/wordcraft.tsv` + `common.tsv`), nhúng lúc build từ
`CRAFT_I18N_DIR` (checkout ở SHA trong `craft-i18n.rev`); thiếu biến này giao diện chỉ có tiếng Anh.
`nexus-ci`/`nexus-release` lấy craft-i18n theo `craft-i18n.rev` (release đặt `CRAFT_I18N_REQUIRED=1`).

Cơ chế chính: `crates/ui-egui/src/i18n/brand.rs` (file mới) — `pub fn brand(s) -> Cow<str>` thay
`WordCraft → Nexus Docs`, `ArtCraft → NexusOffice` và tên lệnh CLI khi được trích dạng mã
(`` `wordcraft-cli `` → `` `nexus-docs-cli ``, vd. Options › Agent control, đủ 23 catalog), được gọi **một lần** trong `i18n::t`/`i18n::tc`
(đường `tl!`), nên mọi ngôn ngữ (cả `en`) hiện tên Nexus mà không sửa key/catalog. `i18n::tr` giữ
nguyên (test catalog upstream dựa vào nó; About dùng nó cho câu ghi công). Hằng và trang About ở
`crates/ui-egui/src/nexus.rs` (file mới).

Văn bản người dùng không bao giờ qua `brand()`: trạng thái engine chỉ được tra catalog (và đổi tên)
khi nó là key catalog (`i18n::is_catalog_key`); tên kiểu và nhãn chú thích của tài liệu dùng
`i18n::tr` (không đổi tên). Thông báo engine/Zotero mà CLI/MCP cũng thấy được đổi tên **tại nguồn**
("Nexus Docs can open"…), nên CLI, MCP và giao diện hiện cùng một chữ.

Giữ nguyên định danh máy (không hiện cho người dùng; đổi tên hiển thị SonnyTech/NexusOffice ngày 2026-10-23 không đụng tới): app id
`vn.sonnytech.nexus.docs` (chữ thường; desktop id, metainfo, bundle id, StartupWMClass), WiX `UpgradeCode`/GUID component,
ProgId `Sonnytech.NexusDocs.*` và khoá registry `Software\Sonnytech\Nexus Docs\Capabilities`, `RegisteredApplications\NexusDocs`,
tên crate (`wordcraft*`), `serverInfo.name = "wordcraft"`, tên biến môi trường (`WORDCRAFT_*`), tên repo `nhdai111983/NexusOffices`
trong workflow CI/release (repo công cụ), `GITHUB_REPOSITORY` mặc định `nhdai111983/wordcraft` của zsync AppImage (kênh cập nhật,
đã có chuyển hướng khi repo sang `sonnytech-vn`), email maintainer `nhdai111983@users.noreply.github.com`.
Link nguồn hiển thị (About, README, metainfo, nfpm, `sign.ps1`, `Cargo.toml`, MCP `websiteUrl`) trỏ `https://github.com/sonnytech-vn/wordcraft`.

## File thương hiệu

| File | Lý do | Thay được bằng white-label? |
|---|---|---|
| `crates/ui-egui/src/i18n.rs` | `pub mod brand; pub use brand::{brand, nexus_text};` + gọi `brand::branded` trong `t()`/`tc()`; `is_catalog_key` (ngôn ngữ hiện tại hoặc vi); doc của `t`/`tc`/`tr` ghi bất biến "không bao giờ đưa văn bản tài liệu/người dùng vào `t`/`tc`/`tl!`" | Có — hook "display name" trong i18n |
| `crates/ui-egui/src/lib.rs` | `pub mod nexus;` | Có — module white-label |
| `crates/ui-egui/src/app/commands.rs` | `ui.discord` không mở Discord ArtCraft (`nexus::COMMUNITY_URL`); `ui.checkForUpdates`/`help.checkForUpdates` không chạy updater khi `nexus::UPDATES = false` mà mở cửa sổ Check for Updates với `nexus::updates_message()` (en/vi: bản mới đến qua bộ cài NexusOffices); trạng thái engine chỉ qua `tl!` khi là key catalog (`i18n::is_catalog_key`), nên tên file/phông của người dùng giữ nguyên; lỗi "making Nexus Docs the default app…"/"Nexus Docs can't change the default apps…" đổi tại nguồn; `after_command` thành `pub(crate)` (test gọi trực tiếp) | Có — `COMMUNITY_URL: Option`, `UPDATES` |
| `crates/ui-egui/src/windows.rs` | Tiêu đề cửa sổ (kể cả cửa sổ mới chưa có tên, không có tài liệu) = `nexus::APP_NAME` | Có — `APP_NAME` |
| `crates/ui-egui/src/tests.rs` | Test `status_keeps_the_users_file_name` (phần key catalog chỉ chạy khi có craft-i18n); 3 dòng test theo nhãn mới (tiêu đề cửa sổ, nút "Make Nexus Docs the default…", trang Options nhận qua "Start-up") | Đi cùng `app/commands.rs` |
| `crates/ui-egui/src/dialogs/dispatch.rs` | Tab About → `nexus::about_ui` (bỏ link getartcraft/Discord, mô tả "Crafting Apps from the ArtCraft team") | Có |
| `crates/ui-egui/src/chrome.rs` | Ẩn nút Discord trên title bar; tiêu đề khi không có tài liệu qua `tl!` (→ Nexus Docs) | Có — `COMMUNITY_URL` |
| `crates/ui-egui/src/ribbon/review.rs` | Ẩn nút Help › Community Discord (`COMMUNITY_URL`) | Có — `COMMUNITY_URL` |
| `crates/ui-egui/src/updates_ui.rs` | Khi `nexus::UPDATES = false`: cửa sổ Check for Updates không đồng bộ theo trạng thái updater (chỉ hiện thông báo NexusOffices) | Có — `UPDATES` |
| `.github/workflows/sonarcloud.yml` | `if: github.repository == 'storytold/wordcraft'`: workflow upstream truyền mọi secret (`secrets: inherit`) cho `storytold/craft-common-actions@main`; không chạy trong fork | Không (CI của upstream) |
| `crates/ui-egui/src/backstage.rs` | Ẩn mục "Discord community" trong File; trang Thông tin thiết kế lại (`section`/`stat`/`elide_middle`: tiêu đề có icon, số liệu lớn, đường dẫn rút gọn giữa). Icon cho cột File: **đã có ở upstream (#1714), bỏ patch** | Có — `COMMUNITY_URL` |
| `crates/ui-egui/src/options.rs` | Ẩn ô "Show the community button" (nút đã ẩn) | Có — `COMMUNITY_URL` |
| `crates/ui-egui/src/options/tests.rs` | Test kittest tra nhãn "…when Nexus Docs starts" và tác giả mặc định "Nexus Docs User" | Đi cùng `options.rs` / `session/lifecycle.rs` |
| `crates/ui-egui/src/backstage/tests.rs` | Test `elide_middle_cuts_on_character_boundaries` | Đi cùng `backstage.rs` |
| `crates/ui-egui/src/ribbon.rs` | Giao diện NexusOffice: tab ribbon căn giữa (`nexus::look::CENTRED_TABS`) | Có — cờ giao diện |
| `crates/ui-egui/src/widgets/buttons.rs`, `widgets/groups.rs` | Giao diện NexusOffices: không có hàng tiêu đề nhóm ribbon, nút mở hộp thoại nhóm ở góc dưới phải (`nexus::look::GROUP_TITLES`) | Có — cờ giao diện |
| `crates/ui-egui/src/previews.rs` | Giao diện NexusOffices: mẫu Kiểu tối đa `STYLE_SAMPLE_MAX_PT` cho vừa ô; số ô theo chiều rộng còn lại | Một phần (cỡ mẫu có thể PR upstream) |
| `crates/ui-egui/src/nexus_toolbar.rs` | File mới: bố cục NexusOffice — thanh menu (sinh từ registry lệnh, đủ mọi lệnh theo nhóm) + một hàng công cụ thay ribbon | — (file mới) |
| `crates/ui-egui/src/lib.rs`, `app.rs`, `app/frame.rs` (thêm) | `pub mod nexus_toolbar;`, trường `WordApp::toolbar` (mặc định tắt = ribbon upstream), chọn `nexus_toolbar::show` thay `ribbon::show` khi bật | Có — cờ giao diện |
| `crates/ui-egui/src/nexus_wps.rs` | File mới: bố cục NexusOffice kiểu WPS (mockup Henry duyệt 2026-10-24) — không title bar; hàng trên 36 pt: nút Tệp (mở Backstage), thanh truy cập nhanh, các thẻ ribbon căn trái (thẻ đang chọn màu chủ đạo, gạch chân 2 pt, » cho thẻ không vừa), bên phải trạng thái lưu (menu: Tự động lưu, Lưu, Lưu thành, Đổi tên), ô tìm lệnh, Nhận xét, Chia sẻ, menu "⋯" (tài khoản, Chỉnh sửa/Góp ý, tùy biến); dải lệnh 56 pt: nhóm của thẻ hiện tại một hàng phẳng, hai tầng nút nhỏ, nút lớn = icon 20 px trên một dòng nhãn, nhóm không vừa gấp vào bảng "…" (không phải popup egui, để menu con của nhóm vẫn mở được); 8 test kittest | — (file mới) |
| `crates/ui-egui/src/lib.rs`, `app/frame.rs` (thêm) | `pub mod nexus_wps;`, trường `WordApp::wps` (mặc định tắt = upstream; thắng `toolbar`), `ui()` gọi `nexus_wps::show` thay title bar + ribbon (Tập trung giữ hàng trên) | Có — cờ giao diện |
| `crates/ui-egui/src/ribbon.rs`, `ribbon/{tabs,common,home,layout,table,design,draw}.rs` (thêm) | Tách `show` thành `sync_tabs` / `tab_strip` / `ribbon_panel` / `tab_groups` (không đổi hành vi) để bố cục WPS dùng lại nhóm của mọi thẻ; `reading_toolbar`, `two_lines` thành `pub(crate)`; `stack` trong dải WPS gọi `nexus_wps::compact_stack` (hai tầng); Trang chủ › Chỉnh sửa: Tìm và thay thế, Chọn là nút lớn khi `app.wps`; Bố trí › Đoạn: dải WPS bỏ hàng tiêu đề, ô được gọi tên bằng icon + tooltip; Thiết kế bảng: tùy chọn kiểu bảng 2 hàng × 3, thanh cuộn kiểu bảng nổi; `CONTENT_H` của nút bút/bộ kiểu → `widgets::content_h` | Có — cờ giao diện |
| `crates/ui-egui/src/widgets/buttons.rs`, `widgets/groups.rs` (thêm) | `COMPACT_H` (50 pt), `content_h(ui)`, `big_layout(ui)`: nút lớn trong dải WPS là icon 20 px trên một dòng nhãn; `group_titled` hỏi `nexus_wps::group_begin/group_end` (nhóm nào vẽ ở dải, nhóm nào vào "…") và chuyển nút mở hộp thoại nhóm vào menu chuột phải của nhóm | Có — cờ giao diện |
| `crates/ui-egui/src/chrome.rs` (thêm) | `quick_access` thành `pub(crate)`; công tắc Tự động lưu tách thành `autosave_switch` (title bar và menu trạng thái lưu dùng chung); nút truy cập nhanh có nhãn trợ năng (`widget_info`); thanh trạng thái 28 pt khi `app.wps` | Một phần (nhãn trợ năng nên PR upstream) |
| `crates/ui-egui/src/previews.rs` (thêm) | Ô Kiểu và ô bộ kiểu theo `widgets::content_h` (mẫu nhỏ hơn trong dải WPS) | Có — cờ giao diện |
| `crates/ui-egui/src/equation_tab.rs` | Nút lớn của thẻ Phương trình theo `widgets::big_layout`/`content_h` (dải WPS) | Có — cờ giao diện |
| `crates/ui-egui/src/icons.rs` | Icon mới `hamburger` (ba vạch, vẽ bằng code) cho nút Tệp | Có (icon gốc, có thể PR upstream) |
| `crates/ui-egui/src/i18n/nexus_catalog.rs` (thêm) | Khóa tiếng Việt mới của các nhánh nx (bố cục WPS, trang Khôi phục…): upstream đã xoá `i18n/*.tsv` (catalog sang craft-i18n), nên bản dịch Nexus nằm trong bảng phủ `NEXUS_VI`, tra trước catalog vi | Không — chuỗi giao diện |
| `crates/ui-egui/src/lib.rs`, `app/frame.rs`, `chrome.rs`, `backstage.rs` (thêm) | Trường `WordApp::embedded` (mặc định tắt): khi app được nhúng làm một tab của cửa sổ NexusOffice, không đặt tiêu đề cửa sổ, không kéo/phóng to cửa sổ từ title bar; test `an_embedded_app_leaves_the_window_title_alone` | Có — cờ nhúng |
| `crates/ui-egui/src/services.rs`, `app/commands.rs` (thêm) | `Services::new_document`: chủ nhà có tab (cửa sổ NexusOffice) nhận File › Mới, Ctrl+N và mẫu văn bản để mở tab mới, tài liệu đang soạn giữ nguyên; không có thì như upstream. Test `new_documents_go_to_the_host_when_it_takes_them` | Có — dịch vụ của chủ nhà |
| `crates/ui-egui/src/backstage.rs`, `backstage/tests.rs`, `dialogs/dispatch.rs`, `app.rs`, `app/commands.rs`, `i18n/nexus_catalog.rs` (thêm) | NX-UI.11 (test tay trên Windows 2026-10-24): khi `app.wps` bật, Backstage (mọi trang) dùng chrome của cửa sổ thay màu upstream: cột trái xám nhạt `title_bar`, trang đang mở nền `checked` + vạch và chữ màu chủ đạo, nội dung trắng `ribbon`, chữ thường; giao diện tối theo token tối; nút "‹ Quay lại" có viền và nhãn. Tắt `wps` giữ nguyên upstream (`backstage::Look`). Trang Khôi phục: tên "Untitled draft" được dịch ("Bản nháp chưa đặt tên"), "Discard…" thành "Loại bỏ…" (trước "Bỏ…" trông như bị cắt), nút không bao giờ rút gọn nhãn, hộp thoại Loại bỏ có tiếng Việt; danh sách bỏ bản khôi phục của chính tài liệu đang mở (`backstage::recovery_list`); Tệp sau khi trang Khôi phục tự mở lúc khởi động mở Thông tin (Trang chủ khi chưa có tài liệu). Test `wps_backstage_uses_the_light_chrome_on_every_page`, `wps_recover_page_is_translated_and_its_buttons_are_whole`, `recover_page_leaves_out_the_windows_own_copy`, `file_opens_info_after_the_start_up_recover_page` | Có — cờ giao diện; phần Khôi phục là sửa lỗi, ứng viên gửi upstream |
| `crates/engine/src/recovery.rs`, `recovery/desktop.rs` (thêm) | `Session::recovery_current()`: id bản khôi phục của tài liệu đang mở (bản tự lưu của chính phiên này), để cửa sổ không liệt kê nó | Không — sửa lỗi, ứng viên gửi upstream |
| `crates/engine/src/recovery.rs`, `recovery/desktop.rs`, `recovery/desktop/tests.rs`, `crates/ui-egui/src/app/files.rs`, `panes.rs`, `panes/comments.rs` (NX-UI.13, thêm) | Chuyển tài liệu chưa lưu sang cửa sổ NexusOffice khác (F1.6 T1): `Session::recovery_save_soon()` (ghi bản khôi phục của tài liệu đang mở ở nhịp tới, không chờ 10 s — đặt `recovery_slot.due`), `recovery_written()` (bản trên đĩa giữ thay đổi mới nhất: đã ghi, không chỉ xếp hàng, lần ghi cuối không lỗi), `recovery_discard_current()` (xoá bản của tài liệu đang mở, chờ luồng ghi tối đa 2 s mỗi lần qua `Recovery::receive_within`/`discard_within` rồi để luồng tự xoá; không ghi lại cho bản sửa đó); `WordApp::has_drafts(ctx)`: nhận xét đang sửa/trả lời làm đổi tài liệu (bản nháp `comment_drafts` của upstream #1348; trả lời trống hay sửa về đúng chữ cũ không tính). Test `a_handed_over_document_is_written_soon_and_discarded_now`, `comment_typing_not_in_the_document_is_a_draft` | Có — chỉ cửa sổ chủ gọi |
| `crates/ui-egui/src/dialogs/dialogs_layout/text.rs`, `i18n/nexus_catalog.rs` | Tiếng Việt (`vi-VN`) trong danh sách ngôn ngữ (Xem lại › Ngôn ngữ) và tên trên thanh trạng thái ("Tiếng Việt" thay vì `vi-VN`); test `vietnamese_is_a_named_document_language`. Ứng viên gửi upstream | Không — tính năng tiếng Việt |
| `apps/wordcraft-web/src/web.rs` | Bật `app.wps` (bố cục NexusOffice kiểu WPS) cho bản web | Có |
| `crates/ui-egui/examples/ui_shot.rs` | `{"toolbar": true}` (thanh menu), `{"wps": true}` (bố cục kiểu WPS), `{"embedded": true}` (như một tab của cửa sổ NexusOffice), `{"language": "vi"}`, `{"recovery": "/thư/mục"}` (chụp Tệp › Khôi phục với bản khôi phục tổng hợp) | Có |
| `crates/ui-egui/src/theme.rs` | Màu NexusOffice (DESIGN_SYSTEM.md): ink indigo `#4F46E5`, gradient `#6366F1`→`#06B6D4`→`#10B981`, nền `#F8FAFC`/`#F1F5F9`, viền `#E2E8F0`; màu chủ đạo người dùng chọn (`set_accent`, `with_accent` giữ tương phản 4.5:1) | Có — bảng màu qua biến build |
| `crates/mcp/src/server.rs` | `serverInfo.title = "Nexus Docs"`, websiteUrl fork; bỏ link Discord/getartcraft khỏi instructions. **`serverInfo.name` giữ `"wordcraft"`** có chủ đích: là định danh máy (test MCP và cấu hình client dựa vào), không phải tên hiển thị | Có |
| `crates/engine/src/sample.rs` | Tài liệu mẫu: creator `Nexus Docs`, link `getartcraft.com` → `example.org` | Có |
| `crates/formats/src/latex/export.rs` | Dòng đầu LaTeX: bỏ URL getartcraft, "Written by Nexus Docs" | Có — tên app qua biến build |
| `crates/formats/src/tests.rs` | Test khẳng định dòng đầu LaTeX mới | Đi cùng `latex.rs` |
| `crates/control-key/src/lib.rs` | Thư mục cấu hình `Nexus Docs` (macOS/Windows) / `nexus-docs` (XDG) — không đụng cấu hình WordCraft gốc (+ test) | Có — tên thư mục qua biến build |
| `apps/wordcraft/src/main.rs` | Không bật kiểm tra cập nhật khi khởi động (`nexus::UPDATES`). Bật `app.wps` (bố cục NexusOffice kiểu WPS; thanh menu `app.toolbar` vẫn giữ sau cờ); Tiêu đề cửa sổ, app id `vn.sonnytech.nexus.docs`, tên eframe, icon cửa sổ từ `nexus/assets/`, USAGE/log; tên lệnh `nexus-docs` trong USAGE, `--version` và tiền tố stderr | Có |
| `apps/wordcraft/src/logging.rs` | Tiền tố stderr của panic hook `nexus-docs:` (tên file log `wordcraft.log`, log target giữ nguyên) | Có — tên binary qua biến build |
| `apps/wordcraft/src/print.rs` | Tên lệnh in trong hàng đợi máy in `"Nexus Docs document"` | Có — tên app qua biến build |
| `apps/wordcraft/tests/it/help.rs` | Assert USAGE bắt đầu bằng `nexus-docs — ` và không chứa `wordcraft`/`WordCraft` | Đi cùng `main.rs` |
| `apps/wordcraft/tests/it/control_arg.rs` | Assert tiền tố lỗi `nexus-docs:` và `--version` `nexus-docs …` | Đi cùng `main.rs` |
| `apps/wordcraft/tests/it/non_utf8_args.rs` | Assert tiền tố lỗi `nexus-docs:` | Đi cùng `main.rs` |
| `apps/wordcraft-cli/src/main.rs` | USAGE (dòng đầu "Nexus Docs from the command line", tên lệnh `nexus-docs-cli`, giữ cột mô tả), `--version`, tiền tố lỗi `nexus-docs-cli:`, lỗi "can't reach the Nexus Docs app". Đầu trang `parity --markdown` "# Nexus Docs feature parity … (`nexus-docs-cli parity --markdown`)" | Có — tên binary qua biến build |
| `apps/wordcraft-cli/src/tests.rs` | Test `usage_lists_exactly_the_subcommands` theo tên mới; `run_without_template_follows_the_region` (Nghị định 30 qua `file.new`). `blank_session` của nexus: **đã có ở upstream** (`run` không file/mẫu đi qua `file.new`, giấy theo vùng #3152), bỏ patch | Đi cùng `main.rs` |
| `apps/wordcraft-cli/tests/it/help.rs` | Assert USAGE bắt đầu bằng `nexus-docs-cli — ` | Đi cùng `main.rs` |
| `apps/wordcraft-cli/tests/it/non_utf8_args.rs` | Assert tiền tố lỗi `nexus-docs-cli:` | Đi cùng `main.rs` |
| `crates/mcp/src/lib.rs` | `NEEDS_APP` (trả lời headless cho công cụ chỉ có trên app): `nexus-docs --control 7981` / `nexus-docs-cli mcp --connect 127.0.0.1:7981` | Có — tên binary qua biến build |
| `crates/mcp/src/tools.rs` | Mô tả công cụ MCP `list_commands`, `parity`, `screenshot`: "Nexus Docs" | Có — tên app qua biến build |
| `crates/engine/src/io.rs` | Lỗi mở tệp "not a document (or text file) Nexus Docs can open" — CLI/MCP/giao diện đều thấy | Có — tên app qua biến build |
| `crates/engine/src/cmd/chart.rs` | "this chart was made in another program: Nexus Docs shows it but can't edit it" | Có — tên app qua biến build |
| `crates/engine/src/cmd/smart_art.rs` | Như trên cho SmartArt | Có — tên app qua biến build |
| `crates/docx/src/crypt.rs` | Lỗi mã hoá "…which Nexus Docs can't open" (3 chỗ). Thông báo "Standard encryption (Office 2007)…can't open yet": **đã có ở upstream** (upstream đọc được Standard encryption), bỏ patch | Có — tên app qua biến build |
| `apps/wordcraft/src/clipboard.rs` | "Nexus Docs can't decode its format" (ảnh trong clipboard) | Có — tên app qua biến build |
| `crates/zotero/src/bridge.rs` | Trả lời plugin Zotero "Nexus Docs doesn't support …" | Có — tên app qua biến build |
| `crates/ui-egui/src/zotero.rs` | Trả lời Zotero "Nexus Docs closed/replaced the document…", "Zotero works with the Nexus Docs desktop app" (wasm) | Có — tên app qua biến build |
| `crates/ui-egui/src/dialogs/dialogs_review.rs` | Restrict Editing: tên kiểu của tài liệu hiện qua `i18n::tr` (dịch, không đổi tên) thay vì `tl!` | Có — nếu upstream tách tra catalog khỏi văn bản người dùng |
| `crates/ui-egui/src/dialogs/dialogs_refs/caption.rs` | Insert Caption: nhãn chú thích (có thể do người dùng đặt) qua `i18n::tr` thay vì `tl!` | Có — như trên |
| `crates/mcp/src/backend.rs` | Thông báo của backend nối app: "Nexus Docs app at … is not reachable", "will connect to the Nexus Docs app at … when it opens", "unauthorized: the Nexus Docs app …", `describe()` (biến `WORDCRAFT_CONTROL_KEY` giữ nguyên); test `errors_name_nexus_docs` | Có — tên app qua biến build |
| `crates/engine/src/io/safe_save.rs` | Lỗi lưu an toàn "This folder doesn't let Nexus Docs create…", "Nexus Docs can't create files in this folder.", gợi ý macOS "allow Nexus Docs to use this folder…", "macOS may be blocking Nexus Docs…" (file mới của upstream, đồng bộ 2026-10-29) | Có — tên app qua biến build |
| `crates/zotero/src/lib.rs` | Lỗi Zotero "Nexus Docs won't let this program edit the document" | Có — tên app qua biến build |
| `crates/zotero/src/peer.rs` | Lỗi Zotero "…Nexus Docs only talks to Zotero on 127.0.0.1" | Có — tên app qua biến build |
| `apps/wordcraft/build.rs` | Resource Windows: ProductName, CompanyName SonnyTech, icon, LegalCopyright (ghi công), OriginalFilename `nexus-docs.exe` | Có |
| `apps/wordcraft/src/default_apps.rs` | Desktop id `vn.sonnytech.nexus.docs.desktop`, `registeredAppMachine=NexusDocs`, thông báo Flatpak (+ test) | Có |
| `apps/wordcraft-web/index.html` | `<title>Nexus Docs</title>`, favicon, chữ "Loading" | Có |
| `apps/wordcraft-web/src/web.rs` | Thông báo lỗi khởi động bản web | Có |
| `packaging/windows/wordcraft.wxs` | Đồng bộ 2026-10-29: **giữ nguyên bản Nexus** (MSI per-machine, UpgradeCode riêng); MSI hai phạm vi mới của upstream (`install-policy.wxi/json`, `check-install-policy.ps1`) là của WordCraft (UpgradeCode, thư mục ArtCraft, khoá registry) nên không dùng. MSI: Name/Manufacturer, **UpgradeCode + GUID component mới**, cài `nexus-docs.exe`/`nexus-docs-cli.exe` vào `Program Files\Nexus Docs`, ProgId `Sonnytech.NexusDocs.*`, khoá `Software\Sonnytech\Nexus Docs` (giữ nguyên, định danh máy), `RegisteredApplications\NexusDocs`, bỏ URL ARP của ArtCraft; cài `LICENSE-MIT`/`LICENSE-APACHE`/`NOTICE`/`ATTRIBUTION.md` cạnh exe (4 component mới, GUID tự sinh); vòng sửa 1: thư mục `licenses\` + ComponentGroup `WordcraftFontLicences` (`<Files Include="$(var.BinDir)\licenses\*">`) trong feature Main, có điều kiện `FontLicences` (mặc định `no`) | Một phần (biến WiX) |
| `packaging/windows/package.ps1` | Đồng bộ 2026-10-29: bỏ bước `app-notices\`/`font-licences\` và `check-install-policy.ps1` của MSI hai phạm vi upstream (MSI Nexus cài giấy phép từ `$Stage` và `licenses\`). Tài sản `NexusDocs-*.msi/.zip`, icon Nexus, trang giấy phép có ghi công; zip portable chứa `nexus-docs.exe`/`nexus-docs-cli.exe` như MSI; chép `LICENSE-MIT`/`LICENSE-APACHE`/`NOTICE`/`ATTRIBUTION.md` vào thư mục stage (MSI cài từ đó) và zip portable (Apache-2.0 §4(d)); vòng sửa 1: dựng `licenses\` = OFL craft-fonts (`OFL-<family>.txt`) + `assets\fonts\OFL-*.txt` (tên duy nhất không phân biệt hoa thường; ném lỗi khi `CRAFT_FONTS_REQUIRED=1` mà không có OFL), truyền `-d FontLicences=yes|no` cho WiX, zip portable lấy OFL từ `licenses\` | Một phần |
| `packaging/windows/sign.ps1` | Mô tả chữ ký `/d Nexus Docs`, `/du` repo fork (như 3 fork kia); bỏ mục allowlist cả file | Một phần |
| `packaging/macos/Info.plist.in` | Tên bundle, executable `NexusDocs`, id `vn.sonnytech.nexus.docs`, icon, copyright (ghi công) | Một phần |
| `packaging/macos/package.sh` | Đồng bộ 2026-10-29: upstream gọi `copy_docs` cho `Contents/Resources` (copy_docs của Nexus gồm `copy_font_licences` + `copy_notices`), bỏ 2 dòng gọi riêng. `Nexus Docs.app`, `NexusDocs-*.dmg/zip`, icns Nexus, tên volume; zip CLI chứa `nexus-docs-cli`; `copy_notices` chép `LICENSE-MIT`/`LICENSE-APACHE`/`NOTICE`/`ATTRIBUTION.md` vào `Contents/Resources` của app (trước khi ký) | Một phần |
| `packaging/linux/package.sh` | Giữ `usr/share/doc` trong AppImage: **đã có ở upstream** (upstream thôi bỏ thư mục doc), bỏ patch. `APP_ID=vn.sonnytech.nexus.docs`, binary `/usr/bin/nexus-docs(-cli)`, `NexusDocs-*`, icon `nexus/assets/hicolor`, maintainer SonnyTech, zsync trỏ fork; AppImage giữ `usr/share/doc/nexus-docs` (giấy phép, `NOTICE`, `ATTRIBUTION.md`) thay vì bỏ ra ngoài | Một phần |
| `packaging/linux/nfpm.yaml` | Gói `nexus-docs`, vendor/homepage, binary, ghi công | Một phần |
| `packaging/env.sh` | Đồng bộ 2026-10-29: upstream đã tự chép `LICENSE-*`/`NOTICE`/`ATTRIBUTION.md` trong `copy_docs` (không bắt buộc có file); giữ bản Nexus vì kiểm tra bắt buộc, OFL font của repo và `CRAFT_FONTS_REQUIRED`. `copy_docs` chép thêm `NOTICE` (Apache-2.0 §4(d)) và `ATTRIBUTION.md` (README Nexus trỏ tới cả hai) qua hàm mới `copy_notices` — bắt buộc, thiếu file thì lỗi; dùng cho deb/AppImage/tar, zip CLI macOS, zip web; vòng sửa 1: chép thêm `assets/fonts/OFL-*.txt` (font của repo nhúng trong binary, nếu có) sau OFL craft-fonts và báo lỗi khi trùng tên; mọi `cp` có `|| return 1` (gọi qua `||` thì errexit tắt); `copy_font_licences` báo lỗi khi `CRAFT_FONTS_REQUIRED=1` mà không có OFL | Nên PR upstream (gói upstream cũng thiếu NOTICE) |
| `crates/engine/src/session/lifecycle.rs` | Tác giả mặc định `"Nexus Docs User"` (ghi vào nhận xét/theo dõi thay đổi; avatar "ND") | Có — tên app qua biến build |
| `crates/engine/src/tests.rs` | Assert tác giả mặc định mới | Đi cùng `session/lifecycle.rs` |
| `crates/pdf/src/metadata.rs` | PDF creator/producer `Nexus Docs` | Có — tên app qua biến build |
| `crates/docx/src/write/settings.rs` | DOCX `docProps/app.xml` `<Application>Nexus Docs</Application>` | Có — tên app qua biến build |
| `crates/docx/tests/it/passthrough.rs` | Assert `<Application>` mới | Đi cùng `write/settings.rs` |
| `crates/formats/src/html/export.rs` | `<meta name="generator" content="Nexus Docs">` | Có — tên app qua biến build |
| `crates/formats/src/odt/package.rs` | `<meta:generator>Nexus Docs</meta:generator>` | Có — tên app qua biến build |
| `crates/formats/src/rtf/export.rs` | `{\*\generator Nexus Docs;}` | Có — tên app qua biến build |
| `README.md` | README Nexus Docs ngắn (tiếng Việt trước, có phần English): sản phẩm, NexusOffice/SonnyTech, câu ghi công "Nexus Docs được phát triển dựa trên WordCraft của ArtCraft (MIT/Apache-2.0)" chỉ bằng chữ, trỏ LICENSE-MIT/LICENSE-APACHE/NOTICE/ATTRIBUTION.md, link fork; được `copy_docs` và zip portable đóng gói | Không (README của fork) |
| `Cargo.toml` | `[workspace.package] repository` → `https://github.com/sonnytech-vn/wordcraft` (brand-check `brand-url`; metadata, không code nào đọc `CARGO_PKG_REPOSITORY`) | Có — metadata của fork |
| `ATTRIBUTION.md` | 14 dòng cho icon Nexus (`nexus/assets/**`, SonnyTech, CC0-1.0) | Không (bắt buộc theo luật asset) |
| `crates/ui-egui/src/nexus_save_prompt.rs` | File mới, giống hệt ở 4 fork: hộp «Lưu thay đổi?» chung của NexusOffice khi chạy nhúng (quyết định Tech Lead 2026-10-25) — tiêu đề «Lưu thay đổi?», câu «Lưu thay đổi của “<tên>” trước khi đóng?», nút Lưu (màu chủ đạo) · Không lưu · Hủy căn phải, Enter = Lưu (nút đang có tiêu điểm), Esc = Hủy, tiêu điểm bắt đầu ở Lưu, modal trên tab (làm mờ tab, chặn chuột/phím bên dưới); số đo theo DESIGN_SYSTEM.md §3 (bo góc hộp 12, nút 6, lề 20, nút tối thiểu 88×32, tiêu đề 22, chữ 13), màu thẻ theo §2.4 (nền, viền, chữ giống nhau ở mọi app; chỉ nút Lưu theo màu chủ đạo); Tab/Shift+Tab chỉ đi qua ba nút theo thứ tự đọc; AccessKit: thẻ là `Dialog` mang tên «Lưu thay đổi?» | — (file mới) |
| `crates/ui-egui/src/dialogs/chrome.rs`, `crates/ui-egui/src/lib.rs`, `app.rs` (thêm) | NX-UI.12: khi `app.embedded`, Save Changes vẽ bằng `nexus_save_prompt` (trả lời qua `ui.saveChanges` như bản upstream; lý do lưu lỗi hiện thành dòng đỏ); trường `WordApp::drawn_in` (vùng tab, nơi hộp làm mờ và căn giữa). Không nhúng giữ nguyên hộp upstream. Test `nexus_save_prompt_tests.rs` (file mới) | Có — cờ nhúng |

### GUID WiX mới (Nexus Docs riêng, không dùng chung với WordCraft hay app Nexus khác)

| Mục | GUID cũ (upstream WordCraft) | GUID mới (Nexus Docs) |
|---|---|---|
| `UpgradeCode` | `075497F9-E861-45D0-9C7F-A74F8C34E6BA` | `B8F9CA5A-7CB4-4C4D-A260-DFC010020560` |
| Component `WordcraftApp` | `855DCEC6-9285-444F-9382-9654B83B34F6` | `817F98F7-FA47-45DC-B404-2CB8E089B5CD` |
| Component `WordcraftFileTypes` | `EFF89E6A-F9B8-49F9-B3AE-E243347484B4` | `18759204-C532-417D-B43D-41C0A4D0376E` |
| Component `WordcraftCli` | (tự sinh theo đường dẫn) | tự sinh — thư mục cài và tên file đã khác |
| Component `WordcraftMitLicense`, `WordcraftApacheLicense`, `WordcraftNotice`, `WordcraftAttribution` | (không có ở upstream) | tự sinh — file giấy phép/NOTICE/ATTRIBUTION.md cài cạnh exe |
| ComponentGroup `WordcraftFontLicences` (mỗi file trong `licenses\` một component) | (không có ở upstream) | tự sinh — OFL của font nhúng, cài vào `licenses\` |

Sinh bằng `python3 -c "import uuid; print(uuid.uuid4())"` ngày 2026-10-16.

### File mới (không tính là patch)
`crates/ui-egui/src/nexus_vi.rs`, `crates/engine/src/nexus_vi.rs`, `crates/engine/src/nexus_samples_vi.rs` (QC 2026-10-26),
`crates/ui-egui/src/i18n/brand.rs`, `crates/ui-egui/src/nexus.rs`, `apps/wordcraft-cli/tests/nexus_brand.rs`
(USAGE/`--version`/tiền tố lỗi, `parity --markdown`, lỗi engine qua `convert`), `crates/mcp/tests/nexus_brand.rs` (mô tả công cụ, trả lời headless, backend nối app),
`packaging/linux/vn.sonnytech.nexus.docs.desktop`, `packaging/linux/vn.sonnytech.nexus.docs.metainfo.xml.in`
(MIME dùng lại file upstream `ai.storyteller.wordcraft.mime.xml`), `nexus/assets/*` (logo tạm
`nexus-docs.svg` + PNG/ICO/ICNS/hicolor sinh bằng `nexus/assets/render-icons.py`).

## File khác

| File | Lý do | Thay được bằng white-label? |
|---|---|---|
| `crates/ui-egui/src/i18n/audit.rs` | Test audit i18n: danh sách miễn thêm "Nexus Docs" và "Nexus Docs User" (thay "WordCraft User") (tên creator của tài liệu mẫu hiện ở File › Info, là dữ liệu tài liệu) | Có — nếu tên app là hằng dùng chung |
| `crates/ui-egui/src/windows.rs`, `app.rs`, `windows/tests.rs` (nhánh `nx/embed-one-window`, 2026-10-31) | Upstream nay mở một cửa sổ native cho mỗi tài liệu (`craft_window::Viewports`). Nhúng vào tab NexusOffice, bộ điều khiển ấy coi cửa sổ NexusOffice là cửa sổ gốc của nó: đổi tiêu đề, đọc nút đóng, và View › New Window mở cửa sổ OS lạc. `WordApp::embed()` (và `embedded = true` từ khung kế) cho app một cửa sổ như bản web; test `embedded_in_a_host_window_the_app_opens_no_window_and_leaves_the_hosts_alone` | Có — nếu upstream nhận một chế độ "nhúng" cho app host |
| `crates/pdf/src/tests.rs` | Bỏ qua trên Linux test `right_to_left_text_extracts_in_logical_order` (chữ Ả Rập bị lặp glyph với font Linux, đỏ cả trên upstream main); không liên quan thương hiệu, chờ sửa upstream | Không |
| `crates/ui-egui/src/canvas.rs`, `canvas/pages.rs`, `canvas/tests.rs`, `crates/render/src/lib.rs` (`render_band`), `crates/ui-egui/src/tests/perf.rs` (hiệu năng, nhánh `nx/perf-typing`) | Gõ phím chỉ vẽ lại dải trang đã đổi (`page_prints`/`changed_band`/`band_rows`, `TextureHandle::set_partial`) thay vì cả trang (~7 MB điểm ảnh mỗi phím ở 150 %); trang có nội dung xoay/hiệu ứng, chế độ markup, đang kéo đối tượng, xem trước Chèn › Bảng, hoặc dải > nửa trang vẫn vẽ cả trang; test so dải vẽ lại với trang vẽ lại nguyên | Không — đáng gửi upstream (lợi cho mọi người dùng) |
| `crates/ui-egui/src/dialogs/forms.rs`, `crates/ui-egui/src/app.rs`, `app/frame.rs`, `app/input.rs`, `tests.rs`, `crates/engine/src/tests/typing.rs`, `docs/typing-parity.md` (QC-DOC-01, 2026-10-26) | Chèn › Bảng bấm lưới rồi gõ: chữ mất vì cú bấm trong menu lấy tiêu điểm bàn phím của trang. Lưới chạy `insert.table` qua `run_from_ribbon` (trả tiêu điểm cho trang như mọi nút ribbon); `refocus_after_chrome_click` coi cú bấm vào mục menu/popup làm menu đóng như cú bấm vào chrome (Ký hiệu, Dấu đầu dòng… cũng hết mất phím). Mỗi ô lưới có tên trợ năng "3×3 Table". Test `typing_after_insert_table_fills_the_first_cells` (engine) và `typing_after_inserting_a_table_from_the_grid_goes_into_its_cells` (kittest, ribbon và WPS) | Không — sửa lỗi, ứng viên gửi upstream |
| `crates/engine/src/session/query.rs`, `crates/ui-egui/src/lib.rs`, `app.rs`, `window_ui.rs`, `nexus_vi.rs` (QC-DOC-02) | `Session::set_system_region`: đổi vùng "Tự động" và đặt lại trang trắng chưa ai chạm (khổ giấy, lề, phông, cỡ, ngôn ngữ) theo vùng mới, giữ `document_id`. `WordApp::logic` gọi `nexus_vi::sync_region` (file mới): giao diện tiếng Việt mà người dùng chưa chọn vùng → vùng vi-VN (A4, lề NĐ 30, Times New Roman 13, cm, dấu phẩy thập phân); đổi lại tiếng Anh thì trả vùng hệ thống | Có — hook "vùng mặc định theo ngôn ngữ giao diện" |
| `crates/engine/src/cmd/insert.rs`, `cmd/insert/items.rs`, `cmd/insert/tests.rs`, `cmd.rs`, `cmd/references/fields.rs`, `crates/engine/Cargo.toml` (chrono `clock`), `crates/ui-egui/src/ribbon/insert.rs`, `i18n/audit.rs` (QC-DOC-03) | `insert.dateTime` viết ngày theo ngôn ngữ chữ ở con trỏ (tham số `lang`): tiếng Việt mặc định `dd/MM/yyyy`, tên thứ/tháng tiếng Việt ("Thứ Hai, 26 tháng 10 năm 2026"), SA/CH; chuỗi trong `'…'` là chữ nguyên văn (cú pháp ảnh ngày của Word). Ngày giờ lấy theo **giờ máy** (`local_offset_secs`, chrono trên desktop, `Date.getTimezoneOffset` trên web) thay vì UTC (trước 7 giờ sáng ở Việt Nam ra ngày hôm qua). Trường DATE cập nhật theo ngôn ngữ tài liệu. Nút Ngày & giờ thành menu các định dạng của ngôn ngữ đó | Một phần — phần giờ máy và `'…'` nên gửi upstream |
| `crates/engine/src/cmd/controls.rs`, `cmd/citations.rs`, `cmd/references/toc.rs`, `cmd/index.rs`, `crates/engine/src/lib.rs` (QC-DOC-04) | Chữ chương trình tự chèn vào tài liệu theo ngôn ngữ chữ ở con trỏ (`nexus_vi::doc_text`, file mới): "Type here." → "Nhập nội dung.", "Pick a date." → "Chọn ngày.", "Pick an option." → "Chọn một mục.", tiêu đề "References"/"Works Cited"/"Bibliography" → "Tài liệu tham khảo"…, "Contents" → "Mục lục", "Index" → "Chỉ mục". Tài liệu tiếng Anh giữ nguyên | Có — hook "chữ tài liệu theo ngôn ngữ" |
| `crates/ui-egui/src/ribbon/layout.rs`, `crates/ui-egui/src/i18n/nexus_catalog.rs` (QC-DOC-05) | Mục lề "Office 2003 Default" hiện "Classic  3,17 cm left and right" / "Cổ điển  trái, phải 3,17 cm" (không có tên sản phẩm Microsoft); số đo theo đơn vị vùng | Có |
| `crates/engine/src/cmd/design.rs`, `crates/ui-egui/src/i18n/nexus_catalog.rs` (QC-DOC-06) | Phông chủ đề là phông Windows có đủ chữ Việt (Arial, Palatino Linotype, Segoe UI, Tahoma; trước là phông macOS Helvetica Neue/Palatino/Futura/Avenir Next/Gill Sans). Tên chủ đề tiếng Việt (Cơ bản, Xưởng vẽ, Phòng tranh, Xưởng thủ công, Phòng tối, Sổ phác thảo); "Studio cover" → "Bìa đơn giản" | Một phần |
| `crates/engine/src/cmd/file.rs`, `crates/engine/src/lib.rs` (QC-DOC-07) | `file.new` với vùng Việt Nam dùng mẫu tiếng Việt (`nexus_samples_vi.rs`, file mới: sổ tay mẫu, thư, CV, báo cáo) trên trang trắng của vùng (A4, NĐ 30) | Có — hook mẫu theo vùng |
| `crates/ui-egui/src/i18n/nexus_catalog.rs` (QC-DOC-08) | "Auto orientation" → "Hướng tự động" | Không — chuỗi giao diện |
| `crates/ui-egui/src/ribbon/home.rs`, `ribbon/draw.rs`, `dialogs/dispatch.rs`, `options.rs` (QC-DOC-10) | Số thập phân theo vùng: giãn dòng 1,15, độ dày bút 0,5 pt, ô Hướng dẫn phát âm (`nexus_vi::num`, `nexus_vi::regional`); thêm vi-VN vào danh sách Vùng trong Tùy chọn | Có — ứng viên gửi upstream |
| `crates/ui-egui/src/backstage.rs` (QC-DOC-11) | Lời chào ở Tệp theo giờ máy (`now_local_iso`), không theo UTC | Không — sửa lỗi, ứng viên gửi upstream |
| `crates/ui-egui/src/widgets/buttons.rs`, `widgets/combo.rs`, `equation_tab.rs` (QC-DOC-12) | Tên trợ năng cho nút lớn trong bố cục WPS (Ảnh, Nhận xét, Hộp văn bản, mọi nút Zotero/Trợ giúp), nút Phương trình, ô Phông/Cỡ/Kiểu không có chú giải; nút tách và ô có chú giải: **đã có ở upstream (#3172)** | Không — ứng viên gửi upstream |
| `crates/ui-egui/src/ribbon/home.rs`, `chrome.rs` (QC-DOC-13) | Chú giải Dán / Đi tới ghi phím của nền tảng (`Ctrl+V` trên Windows) thay vì ký hiệu Mac viết cứng | Không — sửa lỗi, ứng viên gửi upstream |
| `crates/ui-egui/src/dialogs/dialogs_insert.rs`, `ribbon/insert.rs` (QC-DOC-14) | Hàng nút OK/Hủy cao đúng bằng nút (trước chiếm hết chiều cao còn lại: hộp SmartArt cao tới đáy màn hình); "Vẽ bảng" thẳng hàng các mục khác (không còn cột dấu ✓ ẩn) | Không — sửa lỗi, ứng viên gửi upstream |
| `crates/engine/src/ribbon.rs` (QC-DOC-16) | Thẻ Zotero ẩn mặc định (`HIDDEN_BY_DEFAULT`), bật lại ở Tùy chỉnh Ribbon | Có — cấu hình |
| `crates/ui-egui/src/canvas/context_menu.rs` (QC-DOC-17) | Menu chuột phải trên chữ: các cách dán **đã có ở upstream (#2583)**; thêm Dấu đầu dòng, Đánh số, Kiểu (Thường, Tiêu đề, Đầu đề 1–3) | Có |
| `crates/ui-egui/src/app.rs`, `options.rs` (QC-CHUNG-05) | Tác giả mặc định là tên tài khoản máy (`USERNAME`/`USER`) thay vì "Nexus Docs User"; tên màu nhấn tiếng Việt (`nexus_vi::accent_name`) | Có |

## Rà soát gọi mạng (guard §5)
`rg -n -i "artcraft|storytold|storyteller|discord|telemetry|update|crash|sentry|posthog|https?://" --type rust`:
không có telemetry/crash report/cập nhật tự động trong app. Link ArtCraft duy nhất app tự mở là
`ui.discord` (đã chặn: `nexus::COMMUNITY_URL = None`) và link About (đã bỏ). AppImage zsync trỏ
GitHub Releases của fork (`GITHUB_REPOSITORY` hoặc `nhdai111983/wordcraft` — kênh cập nhật, GitHub chuyển hướng sau khi chuyển repo sang `sonnytech-vn`).

## Đồng bộ upstream 2026-10-29 (egui 0.37)

- Upstream tách nhiều file lớn thành module (strict Clippy): các patch được đặt lại đúng chỗ mới
  (bảng trên đã cập nhật đường dẫn). Test tích hợp chuyển vào `tests/it/`; test của nexus
  (`tests/nexus_brand.rs`, `crates/doc/tests/collate_*.rs`) vẫn là file riêng, không sửa `it/main.rs`.
- **craft-update** (Help › Check for Updates, kiểm tra khi khởi động, tự cập nhật trên Windows) tìm bản
  phát hành `storytold/wordcraft` và sẽ thay Nexus Docs bằng WordCraft: tắt bằng `nexus::UPDATES = false`
  (`apps/wordcraft/src/main.rs` không bật kiểm tra khi khởi động; Help › Check for Updates mở cửa sổ nói
  bản mới đến qua bộ cài NexusOffices, en/vi, `nexus::updates_message`; test
  `check_for_updates_points_to_the_nexusoffices_installer`).
- **sonarcloud.yml** (mới ở upstream) chạy trên PR với `secrets: inherit` tới
  `storytold/craft-common-actions@main`: thêm điều kiện chỉ chạy ở `storytold/wordcraft`. `craft_update::self_update::finish` (chỉ chạy với `--finish-update`) giữ nguyên.
- **Đã có ở upstream, bỏ patch:** icon cột File (#1714); thông báo Standard encryption (upstream đọc được);
  `blank_session` của CLI (`run` đi qua `file.new`, #3152); AppImage giữ `usr/share/doc`; macOS gọi
  `copy_docs` cho `Contents/Resources`.
- egui/winit được upstream vá qua `[patch.crates-io]` từ `storytold/egui`, `storytold/winit` (allowlist
  brand-url); chưa có mirror `nhdai111983` cho hai repo này, nexus-ci lấy thẳng từ storytold.

## Việc còn nợ (đã allowlist có lý do, cần PR white-label lên upstream — UPSTREAM_PRS.md)
- `crates/engine/src/cmd/citations.rs`: trạng thái "Researcher needs an online service; WordCraft keeps…" là key catalog nên giữ nguyên; giao diện đổi tên qua `tl!`, nhưng CLI/MCP vẫn thấy "WordCraft".
- Chạy `cargo xtask parity` trong fork sẽ ghi đầu trang Nexus vào `docs/parity-checklist.md` (tài liệu upstream) → đừng commit file đó trên nhánh `nexus`.
- Đề xuất PR upstream: hằng `APP_NAME`/`SETTINGS_DIR`/`COMMUNITY_URL` lấy từ biến build (`WORDCRAFT_APP_NAME`…) → fork chỉ còn đặt biến, bỏ phần lớn patch ở trên.
- DMG macOS: `dmg-layout.DS_Store`/`background.tiff` gắn với volume "WordCraft" → cửa sổ DMG Nexus chưa có nền; làm lại trên macOS (Task 12).
- Flatpak (`packaging/linux/flatpak/*`, `flatpak-bundle.sh`) chưa đổi — chưa phát hành Flatpak.
- Đã biết: thông báo engine mới của upstream có "WordCraft" sẽ hiện nguyên (CLI/MCP/giao diện) cho tới khi đổi tại nguồn; brand-check bắt được chuỗi mới.
- Hunspell vẫn tìm từ điển ở `%APPDATA%\WordCraft\dictionaries` (chỉ đọc, dùng chung).
- Nhánh đang làm (nx/ui-wps-polish, nx/save-prompt, nx/detach-hooks, nx/perf-typing, nx/qc-doc-1…) thêm
  dòng vào `crates/ui-egui/src/i18n/vi.tsv`/`hr.tsv`, file đã bị upstream xoá: chữ mới phải chuyển sang
  `nexus_text` (hoặc một catalog phủ của Nexus / PR lên craft-i18n) khi gộp `nexus-sync` vào các nhánh đó.
- Thông điệp log (`log::warn`) mới của upstream có "WordCraft" (docbin, docx glossary, zotero fields) để
  nguyên (allowlist), như `docbin/src/media.rs`.
