//! NexusOffice: the templates in Vietnamese (QC-DOC-07). File › New's sample, letter, résumé and
//! report, written for a Vietnamese reader (our own text, not a translation of anyone's
//! template), on the region's blank page (A4, decree 30 margins, Times New Roman 13). `file.new`
//! uses them when the region is Vietnam; every other region keeps upstream's English templates.

use wordcraft_doc::para::InlineObject;
use wordcraft_doc::props::{Align, Border, BorderStyle, Borders, CharProps, NumRef, Rgb, TabAlign, TabLeader, TabStop, TextColor};
use wordcraft_doc::{Block, Document, ListKind, Paragraph, PartKind, Table, para_block};

fn para(text: &str) -> Paragraph {
    Paragraph::with_text(text, CharProps::default())
}

fn styled(text: &str, style: &str) -> Paragraph {
    para(text).styled(style)
}

fn add(p: &mut Paragraph, text: &str, props: CharProps) {
    let at = p.len();
    let _ = p.insert_text(at, text, &props);
}

fn bold() -> CharProps {
    CharProps { bold: Some(true), ..Default::default() }
}

fn italic() -> CharProps {
    CharProps { italic: Some(true), ..Default::default() }
}

fn with_body(mut d: Document, blocks: Vec<Block>) -> Document {
    d.body = blocks.into_iter().map(std::sync::Arc::new).collect();
    d
}

/// The width text runs across on `d`'s page, points.
fn text_width(d: &Document) -> f32 {
    d.last_section.text_width()
}

/// The template `name` (`sample`, `letter`, `resume`, `report`) in Vietnamese on `blank`'s page,
/// or `None` for a name without one.
pub fn template(name: &str, blank: Document) -> Option<Document> {
    match name {
        "sample" => Some(sample(blank)),
        "letter" => Some(letter(blank)),
        "resume" => Some(resume(blank)),
        "report" => Some(report(blank)),
        _ => None,
    }
}

/// The showcase: a short handbook of a shared workshop, with headings, lists, a table, a quote, a
/// header and page numbers.
pub fn sample(mut d: Document) -> Document {
    d.core.title = "Sổ tay Xưởng sáng tác chung".into();
    d.core.creator = "Nexus Docs".into();
    let width = text_width(&d);
    let mut b: Vec<Block> = Vec::new();
    b.push(Block::Para(styled("Sổ tay Xưởng sáng tác chung", "Title")));
    b.push(Block::Para(styled("Cách một nhóm nhỏ chia sẻ chỗ làm, dụng cụ và thời gian", "Subtitle")));
    let mut intro = para("Tối thứ Năm nào, căn nhà in cũ trên phố Hàng Mã cũng kín người đến làm đồ. ");
    add(&mut intro, "Người vẽ", bold());
    add(&mut intro, " ngồi bên cửa sổ hướng bắc, nhóm ", CharProps::default());
    add(&mut intro, "in khắc gỗ", italic());
    add(
        &mut intro,
        " chiếm phòng sau, và lúc nào cũng có người mang bánh mì tới. Cuốn sổ này ghi lại những gì chúng tôi học được khi cùng nhau giữ một xưởng chung: nếp sinh hoạt giúp mọi người dễ chịu, dụng cụ dùng chung, và những thỏa thuận nhỏ biến ba mươi người lạ thành một nhóm.",
        CharProps::default(),
    );
    b.push(Block::Para(intro));
    b.push(Block::Para(styled("Vì sao nên dùng chung xưởng?", "Heading1")));
    b.push(Block::Para(para(
        "Tiết kiệm tiền thuê là lý do dễ thấy nhất, nhưng không phải lý do hay nhất. Ở chung một phòng, bạn được thấy những việc mình chưa từng nghĩ tới: người làm gốm giảng về men cho người vẽ truyện tranh, người chụp ảnh mượn khung dệt để dựng một bức tĩnh vật.",
    )));
    let mut quote = styled("Xưởng là nơi những việc còn dang dở luôn được chào đón.", "Quote");
    quote.props.space_before = Some(12.0);
    b.push(Block::Para(quote));
    b.push(Block::Para(styled("Những điều chúng ta thống nhất", "Heading2")));
    let mut num = d.numbering.clone();
    let bullets = num.add_list(ListKind::Bullet);
    let steps = num.add_list(ListKind::Numbered);
    d.numbering = num;
    for (lead, rest) in [
        ("Dùng xong dọn ngay. ", "Để lại mặt bàn như cách bạn muốn thấy nó."),
        ("Ghi tên đồ của mình. ", "Vật liệu không ghi tên trên kệ chung là của chung."),
        ("Hỏi trước khi mượn. ", "Một mảnh giấy nhắn nhỏ giúp mọi người cùng nắm được."),
        ("Giờ yên lặng ", "từ 9 đến 11 giờ sáng các ngày trong tuần."),
    ] {
        let mut p = Paragraph::new().styled("ListParagraph");
        add(&mut p, lead, bold());
        add(&mut p, rest, CharProps::default());
        p.props.numbering = Some(NumRef { num: bullets, level: 0 });
        b.push(Block::Para(p));
    }
    b.push(Block::Para(styled("Dụng cụ dùng chung", "Heading2")));
    b.push(Block::Para(para("Bảng dưới đây liệt kê những thứ cả nhóm cùng sở hữu và người giữ cho từng thứ luôn dùng được.")));
    let rows = [
        ["Dụng cụ", "Vị trí", "Người phụ trách", "Đặt lịch"],
        ["Máy in khắc", "Phòng sau", "Chị Mai", "Sổ đăng ký"],
        ["Lò nung gốm", "Nhà kho ngoài sân", "Anh Tuấn", "Báo trước 48 giờ"],
        ["Máy in khổ lớn", "Văn phòng", "Chị Hằng", "Lịch chung"],
        ["Bàn đèn", "Cửa sổ hướng bắc", "Mọi người", "Ai đến trước dùng trước"],
    ];
    let mut t = Table::new(rows.len(), 4, width);
    t.props.style = Some("GridTable4AccentBlue".into());
    for (r, row) in rows.iter().enumerate() {
        for (c, txt) in row.iter().enumerate() {
            if let Some(cell) = t.rows.get_mut(r).and_then(|x| x.cells.get_mut(c)) {
                cell.blocks = vec![para_block(para(txt))];
            }
        }
    }
    if let Some(r) = t.rows.first_mut() {
        r.props.header = true;
    }
    b.push(Block::Table(t));
    b.push(Block::Para(styled("Mở cửa xưởng", "Heading2")));
    for s in [
        "Bật cầu dao tổng cạnh cửa ra vào.",
        "Mở hai cửa mái, trừ khi trời mưa.",
        "Đun nước pha trà. Bước này không được bỏ.",
        "Xem sổ đăng ký lò nung và máy in.",
    ] {
        let mut p = para(s).styled("ListParagraph");
        p.props.numbering = Some(NumRef { num: steps, level: 0 });
        b.push(Block::Para(p));
    }
    b.push(Block::Para(styled("Thành viên", "Heading1")));
    let mut m = para("Phí thành viên là ");
    add(&mut m, "tùy khả năng", CharProps { bold: Some(true), color: Some(TextColor::Rgb(Rgb(0x15, 0x60, 0x82))), ..Default::default() });
    add(
        &mut m,
        ", xem lại mỗi mùa. Thành viên mới đi cùng một người quen xưởng trong hai buổi đầu, và mỗi người nhận một ca dọn dẹp mỗi tháng. Nếu bạn chưa từng tự tay làm ra thứ gì, bạn chính là người chúng tôi mong gặp.",
        CharProps::default(),
    );
    b.push(Block::Para(m));
    let mut h = para("Có câu hỏi, hãy hỏi ở bàn lễ tân hoặc ghi lên bảng tin cạnh bồn rửa. ");
    add(
        &mut h,
        "Xem bảng tin cộng đồng",
        CharProps { style: Some("Hyperlink".into()), link: Some("https://example.org/".into()), ..Default::default() },
    );
    add(&mut h, " để biết các buổi gặp mặt.", CharProps::default());
    b.push(Block::Para(h));
    let mut d = with_body(d, b);
    let mut f = Paragraph::new().styled("Footer");
    f.props.align = Some(Align::Center);
    let _ = f.insert_object(
        0,
        InlineObject::Field { instr: "PAGE".into(), result: "1".into(), locked: false, dirty: false, code: false, form: String::new() },
        &CharProps::default(),
    );
    let fid = d.add_part(PartKind::Footer, vec![para_block(f)]);
    d.last_section.footers.default = Some(fid);
    let mut hd = styled("Sổ tay Xưởng sáng tác chung\t\tẤn bản mùa xuân", "Header");
    hd.props.borders = Some(Borders {
        bottom: Some(Border { style: BorderStyle::Single, width: 0.5, color: Some(Rgb(0xBF, 0xBF, 0xBF)), space: 4.0 }),
        ..Default::default()
    });
    let hid = d.add_part(PartKind::Header, vec![para_block(hd)]);
    d.last_section.headers.default = Some(hid);
    d
}

/// A letter: sender, place and date on the right, recipient, greeting, body, signature.
pub fn letter(d: Document) -> Document {
    let date = crate::cmd::insert::format_date_in("'ngày' d 'tháng' M 'năm' yyyy", Some("vi-VN"));
    let mut dated = Paragraph::new();
    dated.props.align = Some(Align::Right);
    add(&mut dated, &format!("Hà Nội, {date}"), italic());
    let lines: Vec<Paragraph> = vec![
        styled("Họ và tên của bạn", "Title"),
        para("Số nhà, đường, phường/xã, tỉnh/thành phố · ban@example.com · 0900 000 000"),
        para(""),
        dated,
        para(""),
        para("Kính gửi: Ông/Bà Họ và tên người nhận"),
        para("Chức vụ, tên cơ quan"),
        para("Địa chỉ cơ quan"),
        para(""),
        para("Kính thưa Ông/Bà,"),
        para("Mở đầu bằng một câu nói rõ lý do bạn viết thư. Mỗi đoạn nên ngắn và cụ thể; đoạn cuối nêu điều bạn mong muốn diễn ra tiếp theo."),
        para("Xin chân thành cảm ơn Ông/Bà đã dành thời gian."),
        para("Trân trọng,"),
        para(""),
        para("Họ và tên của bạn"),
    ];
    with_body(d, lines.into_iter().map(Block::Para).collect())
}

/// A one-page CV.
pub fn resume(d: Document) -> Document {
    let width = text_width(&d);
    let mut b: Vec<Block> =
        vec![Block::Para(styled("Nguyễn Minh Anh", "Title")), Block::Para(para("Họa sĩ minh họa · minhanh@example.com · minhanh.example.com"))];
    for (head, items) in [
        ("Kinh nghiệm", vec![("Trưởng nhóm minh họa, Nhà xuất bản Sông Hồng", "2021 – nay"), ("Họa sĩ minh họa tự do", "2016 – 2021")]),
        ("Học vấn", vec![("Cử nhân Mỹ thuật ứng dụng, Trường Đại học Mỹ thuật Ven Biển", "2016")]),
        ("Kỹ năng", vec![("Mực nho, màu bột, vẽ kỹ thuật số, dàn trang sách, viết chữ đẹp", "")]),
    ] {
        b.push(Block::Para(styled(head, "Heading1")));
        for (what, when) in items {
            let mut p = para(what);
            p.props.tabs = Some(vec![TabStop { pos: width, align: TabAlign::Right, leader: TabLeader::None }]);
            if !when.is_empty() {
                add(&mut p, &format!("\t{when}"), italic());
            }
            b.push(Block::Para(p));
        }
    }
    with_body(d, b)
}

/// A report with a title and sections.
pub fn report(d: Document) -> Document {
    let b: Vec<Block> = vec![
        Block::Para(styled("Tên báo cáo", "Title")),
        Block::Para(styled("Phụ đề hoặc ngày lập", "Subtitle")),
        Block::Para(styled("Tóm tắt", "Heading1")),
        Block::Para(para("Tóm tắt kết quả trong hai hoặc ba câu.")),
        Block::Para(styled("Bối cảnh", "Heading1")),
        Block::Para(para("Nêu bối cảnh và câu hỏi mà báo cáo trả lời.")),
        Block::Para(styled("Kết quả", "Heading1")),
        Block::Para(styled("Kết quả thứ nhất", "Heading2")),
        Block::Para(para("Trình bày bằng chứng.")),
        Block::Para(styled("Kiến nghị", "Heading1")),
        Block::Para(para("Liệt kê những việc cần làm tiếp theo.")),
    ];
    with_body(d, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;
    use serde_json::json;
    use wordcraft_doc::StoryRef;

    fn vi() -> Session {
        let mut s = Session::new(Document::new());
        s.run("file.options", &json!({"region": "vi-VN"})).unwrap();
        s
    }

    /// QC-DOC-07: File › New's templates are Vietnamese on A4 when the region is Vietnam, and
    /// English elsewhere.
    #[test]
    fn templates_follow_the_vietnamese_region() {
        for (name, title) in
            [("sample", "Sổ tay Xưởng sáng tác chung"), ("letter", "Họ và tên của bạn"), ("resume", "Nguyễn Minh Anh"), ("report", "Tên báo cáo")]
        {
            let mut s = vi();
            s.run("file.new", &json!({"template": name})).unwrap();
            let text = s.doc.plain_text(StoryRef::Body);
            assert!(text.starts_with(title), "{name}: {text}");
            assert!((s.doc.last_section.page_w - 595.28).abs() < 0.05, "{name}: A4");
            assert_eq!(s.doc.styles.default_chr.lang.as_deref(), Some("vi-VN"), "{name}");
            let l = wordcraft_layout::layout(&s.doc, &mut wordcraft_layout::LayoutCache::new(), &Default::default());
            assert!(!l.pages.is_empty(), "{name}");
            let mut e = Session::new(Document::new());
            e.run("file.new", &json!({"template": name})).unwrap();
            assert!(!e.doc.plain_text(StoryRef::Body).starts_with(title), "{name}: English elsewhere");
        }
        let mut s = vi();
        s.run("file.new", &json!({"template": "letter"})).unwrap();
        assert!(s.doc.plain_text(StoryRef::Body).contains("Hà Nội, ngày "), "{}", s.doc.plain_text(StoryRef::Body));
    }
}
