# UPSTREAM_PRS

PR gửi lên upstream (storytold) để giảm số patch của nhánh `nexus`.

| PR | Repo | Nội dung | Trạng thái | Đã cherry-pick vào nexus? |
|---|---|---|---|---|
| (chưa gửi) | storytold/wordcraft | `crates/pdf/src/tests.rs`: `right_to_left_text_extracts_in_logical_order` thất bại trên Linux (runner ubuntu-24.04): từ Ả Rập có dấu ("م\u{64e}ر\u{652}ح\u{64e}ب\u{64b}ا") được trích thành "…بب…" (lặp chữ ب). Cần sửa test/trích xuất hoặc ghi nhận phụ thuộc font | to file upstream, pending owner approval | Chưa |
