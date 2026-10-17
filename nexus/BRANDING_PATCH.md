# BRANDING_PATCH

Danh sách file mà nhánh `nexus` sửa so với upstream vì lý do thương hiệu.
Mỗi mục là đường dẫn file chính xác trong dấu backtick (không dùng glob/thư mục).

## File thương hiệu

| File | Lý do | Thay được bằng white-label? |
|---|---|---|

## File khác

| File | Lý do | Thay được bằng white-label? |
|---|---|---|
| `crates/pdf/src/tests.rs` | Bỏ qua trên Linux test `right_to_left_text_extracts_in_logical_order` (chữ Ả Rập bị lặp glyph với font Linux, đỏ cả trên upstream main); không liên quan thương hiệu, chờ sửa upstream | Không |
