//! NexusOffice: what Nexus Docs writes into people's files names Nexus Docs, never the upstream
//! product (spec §4.4): the PDF creator/producer, DOCX `<Application>`, the HTML/ODT/RTF generator,
//! and the default author of comments and tracked changes.
#![cfg(not(target_arch = "wasm32"))]

use std::io::Read;

use wordcraft_engine::{Session, io, sample};

fn zip_entry(bytes: &[u8], name: &str) -> String {
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
    let mut s = String::new();
    zip.by_name(name).unwrap().read_to_string(&mut s).unwrap();
    s
}

fn latin1(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| b as char).collect()
}

#[test]
fn saved_files_name_nexus_docs_as_the_generator() {
    let doc = sample::sample_document();
    let html = String::from_utf8(io::save_bytes("x.html", &doc).unwrap()).unwrap();
    assert!(html.contains(r#"<meta name="generator" content="Nexus Docs">"#), "html");
    let rtf = latin1(&io::save_bytes("x.rtf", &doc).unwrap());
    assert!(rtf.contains(r"{\*\generator Nexus Docs;}"), "rtf");
    let odt = zip_entry(&io::save_bytes("x.odt", &doc).unwrap(), "meta.xml");
    assert!(odt.contains("<meta:generator>Nexus Docs</meta:generator>"), "odt: {odt}");
    let app = zip_entry(&io::save_bytes("x.docx", &doc).unwrap(), "docProps/app.xml");
    assert!(app.contains("<Application>Nexus Docs</Application>"), "docx: {app}");
    let pdf = latin1(&io::save_bytes("x.pdf", &doc).unwrap());
    assert!(pdf.contains("Nexus Docs"), "pdf creator/producer");
    for (what, text) in [("html", &html), ("rtf", &rtf), ("odt", &odt), ("docx", &app), ("pdf", &pdf)] {
        assert!(!text.contains("WordCraft"), "{what} still names WordCraft");
    }
}

#[test]
fn the_default_author_is_a_nexus_docs_user() {
    let s = Session::new(sample::sample_document());
    assert_eq!(s.author, "Nexus Docs User");
}
