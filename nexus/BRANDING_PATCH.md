# BRANDING_PATCH

Danh sách file mà nhánh `nexus` sửa so với upstream vì lý do thương hiệu.
Mỗi mục là đường dẫn file chính xác trong dấu backtick (không dùng glob/thư mục).

Giá trị: sản phẩm **Nexus Docs**, bộ **NexusOffices**, nhà phát hành **Sonnytech**, app id
`vn.sonnytech.nexus.docs`, tài sản phát hành `NexusDocs-*`, binary cài đặt `nexus-docs`/`nexus-docs-cli`
(Windows MSI, Linux; tên build Cargo vẫn là `wordcraft`/`wordcraft-cli`).

Không sửa catalog nào: chữ mới của About (en/vi) nằm trong `nexus.rs` qua `i18n::nexus_text(en, vi)`
(không phải key catalog). Từ đồng bộ 2026-10-29 các catalog không còn trong repo: upstream chuyển chúng
sang `storytold/craft-i18n` (`locales/<mã>/wordcraft.tsv` + `common.tsv`), nhúng lúc build từ
`CRAFT_I18N_DIR` (checkout ở SHA trong `craft-i18n.rev`); thiếu biến này giao diện chỉ có tiếng Anh.
`nexus-ci`/`nexus-release` lấy craft-i18n theo `craft-i18n.rev` (release đặt `CRAFT_I18N_REQUIRED=1`).

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
| `crates/ui-egui/src/i18n.rs` | `pub mod brand; pub use brand::{brand, nexus_text};` + gọi `brand::branded` trong `t()`/`tc()`; `is_catalog_key` (ngôn ngữ hiện tại hoặc vi); doc của `t`/`tc`/`tr` ghi bất biến "không bao giờ đưa văn bản tài liệu/người dùng vào `t`/`tc`/`tl!`" | Có — hook "display name" trong i18n |
| `crates/ui-egui/src/lib.rs` | `pub mod nexus;` | Có — module white-label |
| `crates/ui-egui/src/app/commands.rs` | `ui.discord` không mở Discord ArtCraft (`nexus::COMMUNITY_URL`); `ui.checkForUpdates` trả lỗi khi `nexus::UPDATES = false`; trạng thái engine chỉ qua `tl!` khi là key catalog (`i18n::is_catalog_key`), nên tên file/phông của người dùng giữ nguyên; lỗi "making Nexus Docs the default app…"/"Nexus Docs can't change the default apps…" đổi tại nguồn; `after_command` thành `pub(crate)` (test gọi trực tiếp) | Có — `COMMUNITY_URL: Option`, `UPDATES` |
| `crates/ui-egui/src/windows.rs` | Tiêu đề cửa sổ (kể cả cửa sổ mới chưa có tên, không có tài liệu) = `nexus::APP_NAME` | Có — `APP_NAME` |
| `crates/ui-egui/src/tests.rs` | Test `status_keeps_the_users_file_name` (phần key catalog chỉ chạy khi có craft-i18n); 3 dòng test theo nhãn mới (tiêu đề cửa sổ, nút "Make Nexus Docs the default…", trang Options nhận qua "Start-up") | Đi cùng `app/commands.rs` |
| `crates/ui-egui/src/dialogs/dispatch.rs` | Tab About → `nexus::about_ui` (bỏ link getartcraft/Discord, mô tả "Crafting Apps from the ArtCraft team") | Có |
| `crates/ui-egui/src/chrome.rs` | Ẩn nút Discord trên title bar; tiêu đề khi không có tài liệu qua `tl!` (→ Nexus Docs) | Có — `COMMUNITY_URL` |
| `crates/ui-egui/src/ribbon/review.rs` | Ẩn nút Help › Community Discord (`COMMUNITY_URL`) và Help › Check for Updates (`UPDATES`) | Có — `COMMUNITY_URL`, `UPDATES` |
| `crates/ui-egui/src/backstage.rs` | Ẩn mục "Discord community" trong File; trang Thông tin thiết kế lại (`section`/`stat`/`elide_middle`: tiêu đề có icon, số liệu lớn, đường dẫn rút gọn giữa). Icon cho cột File: **đã có ở upstream (#1714), bỏ patch** | Có — `COMMUNITY_URL` |
| `crates/ui-egui/src/options.rs` | Ẩn ô "Show the community button" (nút đã ẩn) | Có — `COMMUNITY_URL` |
| `crates/ui-egui/src/options/tests.rs` | Test kittest tra nhãn "…when Nexus Docs starts" và tác giả mặc định "Nexus Docs User" | Đi cùng `options.rs` / `session/lifecycle.rs` |
| `crates/ui-egui/src/backstage/tests.rs` | Test `elide_middle_cuts_on_character_boundaries` | Đi cùng `backstage.rs` |
| `crates/mcp/src/server.rs` | `serverInfo.title = "Nexus Docs"`, websiteUrl fork; bỏ link Discord/getartcraft khỏi instructions. **`serverInfo.name` giữ `"wordcraft"`** có chủ đích: là định danh máy (test MCP và cấu hình client dựa vào), không phải tên hiển thị | Có |
| `crates/engine/src/sample.rs` | Tài liệu mẫu: creator `Nexus Docs`, link `getartcraft.com` → `example.org` | Có |
| `crates/formats/src/latex/export.rs` | Dòng đầu LaTeX: bỏ URL getartcraft, "Written by Nexus Docs" | Có — tên app qua biến build |
| `crates/formats/src/tests.rs` | Test khẳng định dòng đầu LaTeX mới | Đi cùng `latex.rs` |
| `crates/control-key/src/lib.rs` | Thư mục cấu hình `Nexus Docs` (macOS/Windows) / `nexus-docs` (XDG) — không đụng cấu hình WordCraft gốc (+ test) | Có — tên thư mục qua biến build |
| `apps/wordcraft/src/main.rs` | Tiêu đề cửa sổ, app id `vn.sonnytech.nexus.docs`, tên eframe, icon cửa sổ từ `nexus/assets/`, USAGE/log; tên lệnh `nexus-docs` trong USAGE, `--version` và tiền tố stderr | Có |
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
| `apps/wordcraft/build.rs` | Resource Windows: ProductName, CompanyName Sonnytech, icon, LegalCopyright (ghi công), OriginalFilename `nexus-docs.exe` | Có |
| `apps/wordcraft/src/default_apps.rs` | Desktop id `vn.sonnytech.nexus.docs.desktop`, `registeredAppMachine=NexusDocs`, thông báo Flatpak (+ test) | Có |
| `apps/wordcraft-web/index.html` | `<title>Nexus Docs</title>`, favicon, chữ "Loading" | Có |
| `apps/wordcraft-web/src/web.rs` | Thông báo lỗi khởi động bản web | Có |
| `packaging/windows/wordcraft.wxs` | Đồng bộ 2026-10-29: **giữ nguyên bản Nexus** (MSI per-machine, UpgradeCode riêng); MSI hai phạm vi mới của upstream (`install-policy.wxi/json`, `check-install-policy.ps1`) là của WordCraft (UpgradeCode, thư mục ArtCraft, khoá registry) nên không dùng. MSI: Name/Manufacturer, **UpgradeCode + GUID component mới**, cài `nexus-docs.exe`/`nexus-docs-cli.exe` vào `Program Files\Nexus Docs`, ProgId `Sonnytech.NexusDocs.*`, khoá `Software\Sonnytech\Nexus Docs`, `RegisteredApplications\NexusDocs`, bỏ URL ARP của ArtCraft; cài `LICENSE-MIT`/`LICENSE-APACHE`/`NOTICE`/`ATTRIBUTION.md` cạnh exe (4 component mới, GUID tự sinh); vòng sửa 1: thư mục `licenses\` + ComponentGroup `WordcraftFontLicences` (`<Files Include="$(var.BinDir)\licenses\*">`) trong feature Main, có điều kiện `FontLicences` (mặc định `no`) | Một phần (biến WiX) |
| `packaging/windows/package.ps1` | Đồng bộ 2026-10-29: bỏ bước `app-notices\`/`font-licences\` và `check-install-policy.ps1` của MSI hai phạm vi upstream (MSI Nexus cài giấy phép từ `$Stage` và `licenses\`). Tài sản `NexusDocs-*.msi/.zip`, icon Nexus, trang giấy phép có ghi công; zip portable chứa `nexus-docs.exe`/`nexus-docs-cli.exe` như MSI; chép `LICENSE-MIT`/`LICENSE-APACHE`/`NOTICE`/`ATTRIBUTION.md` vào thư mục stage (MSI cài từ đó) và zip portable (Apache-2.0 §4(d)); vòng sửa 1: dựng `licenses\` = OFL craft-fonts (`OFL-<family>.txt`) + `assets\fonts\OFL-*.txt` (tên duy nhất không phân biệt hoa thường; ném lỗi khi `CRAFT_FONTS_REQUIRED=1` mà không có OFL), truyền `-d FontLicences=yes|no` cho WiX, zip portable lấy OFL từ `licenses\` | Một phần |
| `packaging/windows/sign.ps1` | Mô tả chữ ký `/d Nexus Docs`, `/du` repo fork (như 3 fork kia); bỏ mục allowlist cả file | Một phần |
| `packaging/macos/Info.plist.in` | Tên bundle, executable `NexusDocs`, id `vn.sonnytech.nexus.docs`, icon, copyright (ghi công) | Một phần |
| `packaging/macos/package.sh` | Đồng bộ 2026-10-29: upstream gọi `copy_docs` cho `Contents/Resources` (copy_docs của Nexus gồm `copy_font_licences` + `copy_notices`), bỏ 2 dòng gọi riêng. `Nexus Docs.app`, `NexusDocs-*.dmg/zip`, icns Nexus, tên volume; zip CLI chứa `nexus-docs-cli`; `copy_notices` chép `LICENSE-MIT`/`LICENSE-APACHE`/`NOTICE`/`ATTRIBUTION.md` vào `Contents/Resources` của app (trước khi ký) | Một phần |
| `packaging/linux/package.sh` | Giữ `usr/share/doc` trong AppImage: **đã có ở upstream** (upstream thôi bỏ thư mục doc), bỏ patch. `APP_ID=vn.sonnytech.nexus.docs`, binary `/usr/bin/nexus-docs(-cli)`, `NexusDocs-*`, icon `nexus/assets/hicolor`, maintainer Sonnytech, zsync trỏ fork; AppImage giữ `usr/share/doc/nexus-docs` (giấy phép, `NOTICE`, `ATTRIBUTION.md`) thay vì bỏ ra ngoài | Một phần |
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
| `README.md` | README Nexus Docs ngắn (tiếng Việt trước, có phần English): sản phẩm, NexusOffices/Sonnytech, câu ghi công "Nexus Docs được phát triển dựa trên WordCraft của ArtCraft (MIT/Apache-2.0)" chỉ bằng chữ, trỏ LICENSE-MIT/LICENSE-APACHE/NOTICE/ATTRIBUTION.md, link fork; được `copy_docs` và zip portable đóng gói | Không (README của fork) |
| `Cargo.toml` | `[workspace.package] repository` → `https://github.com/nhdai111983/wordcraft` (brand-check `brand-url`; metadata, không code nào đọc `CARGO_PKG_REPOSITORY`) | Có — metadata của fork |
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
| `crates/pdf/src/tests.rs` | Bỏ qua trên Linux test `right_to_left_text_extracts_in_logical_order` (chữ Ả Rập bị lặp glyph với font Linux, đỏ cả trên upstream main); không liên quan thương hiệu, chờ sửa upstream | Không |

## Rà soát gọi mạng (guard §5)
`rg -n -i "artcraft|storytold|storyteller|discord|telemetry|update|crash|sentry|posthog|https?://" --type rust`:
không có telemetry/crash report/cập nhật tự động trong app. Link ArtCraft duy nhất app tự mở là
`ui.discord` (đã chặn: `nexus::COMMUNITY_URL = None`) và link About (đã bỏ). AppImage zsync trỏ
GitHub Releases của fork (`GITHUB_REPOSITORY` hoặc `nhdai111983/wordcraft`).

## Đồng bộ upstream 2026-10-29 (egui 0.37)

- Upstream tách nhiều file lớn thành module (strict Clippy): các patch được đặt lại đúng chỗ mới
  (bảng trên đã cập nhật đường dẫn). Test tích hợp chuyển vào `tests/it/`; test của nexus
  (`tests/nexus_brand.rs`, `crates/doc/tests/collate_*.rs`) vẫn là file riêng, không sửa `it/main.rs`.
- **craft-update** (Help › Check for Updates, kiểm tra khi khởi động, tự cập nhật trên Windows) tìm bản
  phát hành `storytold/wordcraft` và sẽ thay Nexus Docs bằng WordCraft: tắt bằng `nexus::UPDATES = false`
  (`ribbon/review.rs` ẩn nút, `app/commands.rs` trả lỗi, `apps/wordcraft/src/main.rs` không bật kiểm tra
  khi khởi động). `craft_update::self_update::finish` (chỉ chạy với `--finish-update`) giữ nguyên.
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
