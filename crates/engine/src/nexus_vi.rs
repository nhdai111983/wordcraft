//! NexusOffice: text the editor writes into a Vietnamese document (QC 2026-10-26).
//!
//! A few commands put words of their own into the document: a content control's placeholder
//! ("Type here."), a bibliography's heading ("References"), a table of contents' or an index's
//! heading. That is document text, not interface text, so it follows the language of the text at
//! the caret (else the document's), not the interface catalog: a Vietnamese document gets
//! Vietnamese words whatever language the window speaks, and every other language keeps
//! upstream's English (QC-DOC-04). The wording is our own.

/// Is `lang` (a BCP 47 tag such as `vi-VN`, `vi`, `vi_VN`) Vietnamese?
pub fn is_vi(lang: Option<&str>) -> bool {
    lang.is_some_and(|l| l.get(..2).is_some_and(|p| p.eq_ignore_ascii_case("vi")) && matches!(l.as_bytes().get(2), None | Some(b'-' | b'_')))
}

/// English text the editor writes into documents, and its Vietnamese.
const VI: &[(&str, &str)] = &[
    ("Type here.", "Nhập nội dung."),
    ("Pick an option.", "Chọn một mục."),
    ("Pick a date.", "Chọn ngày."),
    ("References", "Tài liệu tham khảo"),
    ("Works Cited", "Tài liệu trích dẫn"),
    ("Bibliography", "Thư mục tài liệu"),
    ("Contents", "Mục lục"),
    ("Index", "Chỉ mục"),
];

/// `en` as written into text in `lang`: its Vietnamese for Vietnamese text, else `en` itself.
pub fn doc_text<'a>(lang: Option<&str>, en: &'a str) -> &'a str {
    if !is_vi(lang) {
        return en;
    }
    VI.iter().find(|(e, _)| *e == en).map_or(en, |(_, v)| v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;
    use serde_json::json;
    use wordcraft_doc::StoryRef;

    #[test]
    fn vietnamese_text_gets_vietnamese_words() {
        assert!(is_vi(Some("vi-VN")) && is_vi(Some("vi")) && is_vi(Some("VI_vn")));
        assert!(!is_vi(Some("vie")) && !is_vi(Some("en-US")) && !is_vi(None) && !is_vi(Some("v")));
        assert_eq!(doc_text(Some("vi-VN"), "Pick a date."), "Chọn ngày.");
        assert_eq!(doc_text(Some("en-US"), "Pick a date."), "Pick a date.");
        assert_eq!(doc_text(Some("vi-VN"), "Something else"), "Something else");
    }

    fn vi_session() -> Session {
        let mut s = Session::new(wordcraft_doc::Document::new());
        s.run("review.language", &json!({"lang": "vi-VN", "default": true})).unwrap();
        s
    }

    /// QC-DOC-04: Developer › content controls and References › Bibliography in a Vietnamese
    /// document write Vietnamese; an English document is unchanged.
    #[test]
    fn controls_and_headings_in_a_vietnamese_document() {
        for (cmd, vi, en) in [
            ("developer.plainText", "Nhập nội dung.", "Type here."),
            ("developer.richText", "Nhập nội dung.", "Type here."),
            ("developer.datePicker", "Chọn ngày.", "Pick a date."),
            ("developer.dropDown", "Chọn một mục.", "Pick an option."),
            ("developer.comboBox", "Chọn một mục.", "Pick an option."),
        ] {
            let mut s = vi_session();
            s.run(cmd, &json!({})).unwrap();
            assert_eq!(s.doc.plain_text(StoryRef::Body), vi, "{cmd}");
            let mut e = Session::new(wordcraft_doc::Document::new());
            e.run(cmd, &json!({})).unwrap();
            assert_eq!(e.doc.plain_text(StoryRef::Body), en, "{cmd}");
        }
        let mut s = vi_session();
        s.run("references.bibliography", &json!({})).unwrap();
        assert!(s.doc.plain_text(StoryRef::Body).starts_with("Tài liệu tham khảo"), "{}", s.doc.plain_text(StoryRef::Body));
        assert!(!s.doc.plain_text(StoryRef::Body).contains("References"));
        // A title the caller gives is kept.
        let mut s = vi_session();
        s.run("references.bibliography", &json!({"title": "Nguồn"})).unwrap();
        assert!(s.doc.plain_text(StoryRef::Body).starts_with("Nguồn"));
    }
}
