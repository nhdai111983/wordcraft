# Chính sách bảo mật / Security policy

## Báo lỗ hổng

Hãy báo lỗi bảo mật một cách riêng tư qua GitHub:
**[Báo lỗ hổng](https://github.com/sonnytech-vn/wordcraft/security/advisories/new)**
(thẻ Security › Advisories › Report a vulnerability).

Đừng mở issue hay pull request công khai về lỗ hổng cho tới khi bản sửa được phát hành.

Nên kèm theo:
- phiên bản hoặc commit của Nexus Docs, và nền tảng (Windows, macOS, Linux, web);
- kẻ tấn công làm được gì và cần gì (một tài liệu được dựng sẵn, quyền dùng MCP hay kênh điều khiển, một tài khoản trên máy…);
- các bước hoặc một file mẫu tái hiện được lỗi.

Lỗi nằm trong mã kế thừa từ upstream cũng được báo ở đây; SonnyTech sẽ chuyển tiếp khi cần.

## Phạm vi

Trong phạm vi: mọi thứ khiến một tài liệu, file, lệnh, yêu cầu MCP hay kênh điều khiển, hoặc phản hồi mạng đọc hay ghi file không được phép, chạy mã, làm lộ dữ liệu, hoặc làm Nexus Docs sập và mất việc của người dùng. Sập vì đầu vào độc hại cũng tính là lỗi (xem [Never crash](AGENTS.md)).

---

## Reporting a vulnerability

Please report security problems privately through GitHub:
**[Report a vulnerability](https://github.com/sonnytech-vn/wordcraft/security/advisories/new)**
(Security tab › Advisories › Report a vulnerability).

Don't open a public issue or pull request for a vulnerability until a fix is released.

Useful to include:
- the Nexus Docs version or commit, and the platform (Windows, macOS, Linux, web);
- what an attacker can do, and what they need (a crafted document, MCP or control-channel access, a local account…);
- steps or a sample file that reproduce it.

Problems in code inherited from upstream are reported here too; SonnyTech forwards them where needed.

## Supported versions

Fixes go into `nexus` and ship in the next NexusOffice release. Older releases don't get backported fixes.

## Scope

In scope: anything that lets a document, file, command, MCP or control-channel request, or network response read or write files it shouldn't, run code, leak data, or crash Nexus Docs and lose the user's work. Crashes from hostile input count (see [Never crash](AGENTS.md)).
