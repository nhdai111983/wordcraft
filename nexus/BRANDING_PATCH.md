# BRANDING_PATCH

Danh sách file mà nhánh `nexus` sửa so với upstream vì lý do thương hiệu.
Mỗi mục là đường dẫn file chính xác trong dấu backtick (không dùng glob/thư mục).

Giá trị: sản phẩm **Nexus Docs**, bộ **NexusOffices**, nhà phát hành **Sonnytech**, app id
`vn.sonnytech.nexus.docs`, tài sản phát hành `NexusDocs-*`, binary cài đặt `nexus-docs`/`nexus-docs-cli`
(Windows MSI, Linux; tên build Cargo vẫn là `wordcraft`/`wordcraft-cli`).

Không sửa catalog `.tsv` nào: chữ mới của About (en/vi) nằm trong `nexus.rs` qua `i18n::nexus_text(en, vi)`
(không phải key catalog, nên không bắt 7 catalog "đầy đủ" của upstream phải dịch thêm).

Cơ chế chính: `crates/ui-egui/src/i18n/brand.rs` (file mới) — `pub fn brand(s) -> Cow<str>` thay
`WordCraft → Nexus Docs`, `ArtCraft → NexusOffices` và tên lệnh CLI khi được trích dạng mã
(`` `wordcraft-cli `` → `` `nexus-docs-cli ``, vd. Options › Agent control, đủ 23 catalog), được gọi **một lần** trong `i18n::t`/`i18n::tc`
(đường `tl!`), nên mọi ngôn ngữ (cả `en`) hiện tên Nexus mà không sửa key/catalog. `i18n::tr` giữ
nguyên (test catalog upstream dựa vào nó; About dùng nó cho câu ghi công). Hằng và trang About ở
`crates/ui-egui/src/nexus.rs` (file mới).

Văn bản người dùng không bao giờ qua `brand()`: trạng thái engine chỉ được tra catalog (và đổi tên)
khi nó là key catalog (`i18n::is_catalog_key`); tên kiểu và nhãn chú thích của tài liệu dùng
`i18n::tr` (không đổi tên). Thông báo engine/Zotero mà CLI/MCP cũng thấy được đổi tên **tại nguồn**
("Nexus Docs can open"…), nên CLI, MCP và giao diện hiện cùng một chữ.

## File thương hiệu

| File | Lý do | Thay được bằng white-label? |
|---|---|---|
| `crates/ui-egui/src/i18n/mod.rs` | `mod brand; pub use brand::{brand, nexus_text};` + gọi `brand::branded` trong `t()`/`tc()`; `is_catalog_key` (ngôn ngữ hiện tại hoặc vi); doc của `t`/`tc`/`tr` ghi bất biến "không bao giờ đưa văn bản tài liệu/người dùng vào `t`/`tc`/`tl!`" | Có — hook "display name" trong i18n |
| `crates/ui-egui/src/lib.rs` | `pub mod nexus;`; tiêu đề cửa sổ = `nexus::APP_NAME`; `ui.discord` không mở Discord ArtCraft; trạng thái engine chỉ qua `tl!` khi là key catalog (`i18n::is_catalog_key`), nên tên file/phông của người dùng giữ nguyên; lỗi "making Nexus Docs the default app…"/"Nexus Docs can't change the default apps…" đổi tại nguồn; test `status_keeps_the_users_file_name`; 3 dòng test theo nhãn mới | Có — `APP_NAME`, `COMMUNITY_URL: Option` |
| `crates/ui-egui/src/dialogs.rs` | Tab About → `nexus::about_ui` (bỏ link getartcraft/Discord, mô tả "Crafting Apps from the ArtCraft team") | Có |
| `crates/ui-egui/src/chrome.rs` | Ẩn nút Discord trên title bar; tiêu đề khi không có tài liệu qua `tl!` (→ Nexus Docs) | Có — `COMMUNITY_URL` |
| `crates/ui-egui/src/ribbon.rs` | Ẩn nút Help › Community Discord | Có — `COMMUNITY_URL` |
| `crates/ui-egui/src/backstage.rs` | Ẩn mục "Discord community" trong File | Có — `COMMUNITY_URL` |
| `crates/ui-egui/src/options.rs` | Ẩn ô "Show the community button" (nút đã ẩn); test kittest tra nhãn "…when Nexus Docs starts" | Có — `COMMUNITY_URL` |
| `crates/mcp/src/server.rs` | `serverInfo.title = "Nexus Docs"`, websiteUrl fork; bỏ link Discord/getartcraft khỏi instructions. **`serverInfo.name` giữ `"wordcraft"`** có chủ đích: là định danh máy (test MCP và cấu hình client dựa vào), không phải tên hiển thị | Có |
| `crates/engine/src/sample.rs` | Tài liệu mẫu: creator `Nexus Docs`, link `getartcraft.com` → `example.org` | Có |
| `crates/formats/src/latex.rs` | Dòng đầu LaTeX: bỏ URL getartcraft, "Written by Nexus Docs" | Có — tên app qua biến build |
| `crates/formats/src/tests.rs` | Test khẳng định dòng đầu LaTeX mới | Đi cùng `latex.rs` |
| `crates/control-key/src/lib.rs` | Thư mục cấu hình `Nexus Docs` (macOS/Windows) / `nexus-docs` (XDG) — không đụng cấu hình WordCraft gốc (+ test) | Có — tên thư mục qua biến build |
| `apps/wordcraft/src/main.rs` | Tiêu đề cửa sổ, app id `vn.sonnytech.nexus.docs`, tên eframe, icon cửa sổ từ `nexus/assets/`, USAGE/log; tên lệnh `nexus-docs` trong USAGE, `--version` và tiền tố stderr | Có |
| `apps/wordcraft/src/logging.rs` | Tiền tố stderr của panic hook `nexus-docs:` (tên file log `wordcraft.log`, log target giữ nguyên) | Có — tên binary qua biến build |
| `apps/wordcraft/src/print.rs` | Tên lệnh in trong hàng đợi máy in `"Nexus Docs document"` | Có — tên app qua biến build |
| `apps/wordcraft/tests/help.rs` | Assert USAGE bắt đầu bằng `nexus-docs — ` và không chứa `wordcraft`/`WordCraft` | Đi cùng `main.rs` |
| `apps/wordcraft/tests/control_arg.rs` | Assert tiền tố lỗi `nexus-docs:` và `--version` `nexus-docs …` | Đi cùng `main.rs` |
| `apps/wordcraft/tests/non_utf8_args.rs` | Assert tiền tố lỗi `nexus-docs:` | Đi cùng `main.rs` |
| `apps/wordcraft-cli/src/main.rs` | USAGE (dòng đầu "Nexus Docs from the command line", tên lệnh `nexus-docs-cli`, giữ cột mô tả), `--version`, tiền tố lỗi `nexus-docs-cli:`, lỗi "can't reach the Nexus Docs app"; test `usage_lists_exactly_the_subcommands` theo tên mới. Đầu trang `parity --markdown` "# Nexus Docs feature parity … (`nexus-docs-cli parity --markdown`)" | Có — tên binary qua biến build |
| `apps/wordcraft-cli/tests/help.rs` | Assert USAGE bắt đầu bằng `nexus-docs-cli — ` | Đi cùng `main.rs` |
| `apps/wordcraft-cli/tests/non_utf8_args.rs` | Assert tiền tố lỗi `nexus-docs-cli:` | Đi cùng `main.rs` |
| `crates/mcp/src/lib.rs` | `NEEDS_APP` (trả lời headless cho công cụ chỉ có trên app): `nexus-docs --control 7981` / `nexus-docs-cli mcp --connect 127.0.0.1:7981` | Có — tên binary qua biến build |
| `crates/mcp/src/tools.rs` | Mô tả công cụ MCP `list_commands`, `parity`, `screenshot`: "Nexus Docs" | Có — tên app qua biến build |
| `crates/engine/src/io.rs` | Lỗi mở tệp "not a document (or text file) Nexus Docs can open" — CLI/MCP/giao diện đều thấy | Có — tên app qua biến build |
| `crates/engine/src/cmd/chart.rs` | "this chart was made in another program: Nexus Docs shows it but can't edit it" | Có — tên app qua biến build |
| `crates/engine/src/cmd/smart_art.rs` | Như trên cho SmartArt | Có — tên app qua biến build |
| `crates/docx/src/crypt.rs` | Lỗi mã hoá "…which Nexus Docs can't open (yet)" | Có — tên app qua biến build |
| `apps/wordcraft/src/clipboard.rs` | "Nexus Docs can't decode its format" (ảnh trong clipboard) | Có — tên app qua biến build |
| `crates/zotero/src/bridge.rs` | Trả lời plugin Zotero "Nexus Docs doesn't support …" | Có — tên app qua biến build |
| `crates/ui-egui/src/zotero.rs` | Trả lời Zotero "Nexus Docs closed/replaced the document…", "Zotero works with the Nexus Docs desktop app" (wasm) | Có — tên app qua biến build |
| `crates/ui-egui/src/dialogs_review.rs` | Restrict Editing: tên kiểu của tài liệu hiện qua `i18n::tr` (dịch, không đổi tên) thay vì `tl!` | Có — nếu upstream tách tra catalog khỏi văn bản người dùng |
| `crates/ui-egui/src/dialogs_refs.rs` | Insert Caption: nhãn chú thích (có thể do người dùng đặt) qua `i18n::tr` thay vì `tl!` | Có — như trên |
| `crates/mcp/src/backend.rs` | Thông báo của backend nối app: "Nexus Docs app at … is not reachable", "unauthorized: the Nexus Docs app …", `describe()` (biến `WORDCRAFT_CONTROL_KEY` giữ nguyên); test `errors_name_nexus_docs` | Có — tên app qua biến build |
| `apps/wordcraft/build.rs` | Resource Windows: ProductName, CompanyName Sonnytech, icon, LegalCopyright (ghi công), OriginalFilename `nexus-docs.exe` | Có |
| `apps/wordcraft/src/default_apps.rs` | Desktop id `vn.sonnytech.nexus.docs.desktop`, `registeredAppMachine=NexusDocs`, thông báo Flatpak (+ test) | Có |
| `apps/wordcraft-web/index.html` | `<title>Nexus Docs</title>`, favicon, chữ "Loading" | Có |
| `apps/wordcraft-web/src/web.rs` | Thông báo lỗi khởi động bản web | Có |
| `packaging/windows/wordcraft.wxs` | MSI: Name/Manufacturer, **UpgradeCode + GUID component mới**, cài `nexus-docs.exe`/`nexus-docs-cli.exe` vào `Program Files\Nexus Docs`, ProgId `Sonnytech.NexusDocs.*`, khoá `Software\Sonnytech\Nexus Docs`, `RegisteredApplications\NexusDocs`, bỏ URL ARP của ArtCraft; cài `LICENSE-MIT`/`LICENSE-APACHE`/`NOTICE`/`ATTRIBUTION.md` cạnh exe (4 component mới, GUID tự sinh); vòng sửa 1: thư mục `licenses\` + ComponentGroup `WordcraftFontLicences` (`<Files Include="$(var.BinDir)\licenses\*">`) trong feature Main, có điều kiện `FontLicences` (mặc định `no`) | Một phần (biến WiX) |
| `packaging/windows/package.ps1` | Tài sản `NexusDocs-*.msi/.zip`, icon Nexus, trang giấy phép có ghi công; zip portable chứa `nexus-docs.exe`/`nexus-docs-cli.exe` như MSI; chép `LICENSE-MIT`/`LICENSE-APACHE`/`NOTICE`/`ATTRIBUTION.md` vào thư mục stage (MSI cài từ đó) và zip portable (Apache-2.0 §4(d)); vòng sửa 1: dựng `licenses\` = OFL craft-fonts (`OFL-<family>.txt`) + `assets\fonts\OFL-*.txt` (tên duy nhất không phân biệt hoa thường; ném lỗi khi `CRAFT_FONTS_REQUIRED=1` mà không có OFL), truyền `-d FontLicences=yes|no` cho WiX, zip portable lấy OFL từ `licenses\` | Một phần |
| `packaging/windows/sign.ps1` | Mô tả chữ ký `/d Nexus Docs`, `/du` repo fork (như 3 fork kia); bỏ mục allowlist cả file | Một phần |
| `packaging/macos/Info.plist.in` | Tên bundle, executable `NexusDocs`, id `vn.sonnytech.nexus.docs`, icon, copyright (ghi công) | Một phần |
| `packaging/macos/package.sh` | `Nexus Docs.app`, `NexusDocs-*.dmg/zip`, icns Nexus, tên volume; zip CLI chứa `nexus-docs-cli`; `copy_notices` chép `LICENSE-MIT`/`LICENSE-APACHE`/`NOTICE`/`ATTRIBUTION.md` vào `Contents/Resources` của app (trước khi ký) | Một phần |
| `packaging/linux/package.sh` | `APP_ID=vn.sonnytech.nexus.docs`, binary `/usr/bin/nexus-docs(-cli)`, `NexusDocs-*`, icon `nexus/assets/hicolor`, maintainer Sonnytech, zsync trỏ fork; AppImage giữ `usr/share/doc/nexus-docs` (giấy phép, `NOTICE`, `ATTRIBUTION.md`) thay vì bỏ ra ngoài | Một phần |
| `packaging/linux/nfpm.yaml` | Gói `nexus-docs`, vendor/homepage, binary, ghi công | Một phần |
| `packaging/env.sh` | `copy_docs` chép thêm `NOTICE` (Apache-2.0 §4(d)) và `ATTRIBUTION.md` (README Nexus trỏ tới cả hai) qua hàm mới `copy_notices` — bắt buộc, thiếu file thì lỗi; dùng cho deb/AppImage/tar, zip CLI macOS, zip web; vòng sửa 1: chép thêm `assets/fonts/OFL-*.txt` (font của repo nhúng trong binary, nếu có) sau OFL craft-fonts và báo lỗi khi trùng tên; mọi `cp` có `|| return 1` (gọi qua `||` thì errexit tắt); `copy_font_licences` báo lỗi khi `CRAFT_FONTS_REQUIRED=1` mà không có OFL | Nên PR upstream (gói upstream cũng thiếu NOTICE) |
| `crates/engine/src/session.rs` | Tác giả mặc định `"Nexus Docs User"` (ghi vào nhận xét/theo dõi thay đổi; avatar "ND") | Có — tên app qua biến build |
| `crates/engine/src/tests.rs` | Assert tác giả mặc định mới | Đi cùng `session.rs` |
| `crates/pdf/src/lib.rs` | PDF creator/producer `Nexus Docs` | Có — tên app qua biến build |
| `crates/docx/src/write/mod.rs` | DOCX `docProps/app.xml` `<Application>Nexus Docs</Application>` | Có — tên app qua biến build |
| `crates/docx/tests/passthrough.rs` | Assert `<Application>` mới | Đi cùng `write/mod.rs` |
| `crates/formats/src/html.rs` | `<meta name="generator" content="Nexus Docs">` | Có — tên app qua biến build |
| `crates/formats/src/odt.rs` | `<meta:generator>Nexus Docs</meta:generator>` | Có — tên app qua biến build |
| `crates/formats/src/rtf.rs` | `{\*\generator Nexus Docs;}` | Có — tên app qua biến build |
| `README.md` | README Nexus Docs ngắn (tiếng Việt trước, có phần English): sản phẩm, NexusOffices/Sonnytech, câu ghi công "Nexus Docs được phát triển dựa trên WordCraft của ArtCraft (MIT/Apache-2.0)" chỉ bằng chữ, trỏ LICENSE-MIT/LICENSE-APACHE/NOTICE/ATTRIBUTION.md, link fork; được `copy_docs` và zip portable đóng gói | Không (README của fork) |
| `ATTRIBUTION.md` | 14 dòng cho icon Nexus (`nexus/assets/**`, Sonnytech, CC0-1.0) | Không (bắt buộc theo luật asset) |

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
`crates/ui-egui/src/i18n/brand.rs`, `crates/ui-egui/src/nexus.rs`, `apps/wordcraft-cli/tests/nexus_brand.rs`
(USAGE/`--version`/tiền tố lỗi, `parity --markdown`, lỗi engine qua `convert`), `crates/mcp/tests/nexus_brand.rs` (mô tả công cụ, trả lời headless, backend nối app),
`packaging/linux/vn.sonnytech.nexus.docs.desktop`, `packaging/linux/vn.sonnytech.nexus.docs.metainfo.xml.in`
(MIME dùng lại file upstream `ai.storyteller.wordcraft.mime.xml`), `nexus/assets/*` (logo tạm
`nexus-docs.svg` + PNG/ICO/ICNS/hicolor sinh bằng `nexus/assets/render-icons.py`).

## File khác

| File | Lý do | Thay được bằng white-label? |
|---|---|---|
| `crates/ui-egui/src/i18n/audit.rs` | Test audit i18n: danh sách miễn thêm "Nexus Docs" và "Nexus Docs User" (thay "WordCraft User") (tên creator của tài liệu mẫu hiện ở File › Info, là dữ liệu tài liệu) | Có — nếu tên app là hằng dùng chung |

## Rà soát gọi mạng (guard §5)
`rg -n -i "artcraft|storytold|storyteller|discord|telemetry|update|crash|sentry|posthog|https?://" --type rust`:
không có telemetry/crash report/cập nhật tự động trong app. Link ArtCraft duy nhất app tự mở là
`ui.discord` (đã chặn: `nexus::COMMUNITY_URL = None`) và link About (đã bỏ). AppImage zsync trỏ
GitHub Releases của fork (`GITHUB_REPOSITORY` hoặc `nhdai111983/wordcraft`).

## Việc còn nợ (đã allowlist có lý do, cần PR white-label lên upstream — UPSTREAM_PRS.md)
- `crates/engine/src/cmd/citations.rs`: trạng thái "Researcher needs an online service; WordCraft keeps…" là key catalog nên giữ nguyên; giao diện đổi tên qua `tl!`, nhưng CLI/MCP vẫn thấy "WordCraft".
- Chạy `cargo xtask parity` trong fork sẽ ghi đầu trang Nexus vào `docs/parity-checklist.md` (tài liệu upstream) → đừng commit file đó trên nhánh `nexus`.
- Đề xuất PR upstream: hằng `APP_NAME`/`SETTINGS_DIR`/`COMMUNITY_URL` lấy từ biến build (`WORDCRAFT_APP_NAME`…) → fork chỉ còn đặt biến, bỏ phần lớn patch ở trên.
- DMG macOS: `dmg-layout.DS_Store`/`background.tiff` gắn với volume "WordCraft" → cửa sổ DMG Nexus chưa có nền; làm lại trên macOS (Task 12).
- Flatpak (`packaging/linux/flatpak/*`, `flatpak-bundle.sh`) chưa đổi — chưa phát hành Flatpak.
- Đã biết: thông báo engine mới của upstream có "WordCraft" sẽ hiện nguyên (CLI/MCP/giao diện) cho tới khi đổi tại nguồn; brand-check bắt được chuỗi mới.
- Hunspell vẫn tìm từ điển ở `%APPDATA%\WordCraft\dictionaries` (chỉ đọc, dùng chung).
