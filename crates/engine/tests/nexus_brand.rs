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

/// A font store whose downloading is off: every call that would download says so.
struct OfflineStore;

impl wordcraft_engine::packs::Backend for OfflineStore {
    fn index(&self) -> Result<Vec<wordcraft_engine::packs::Pack>, String> {
        Ok(Vec::new())
    }
    fn fetch_index(&self, _: &wordcraft_engine::packs::Cancel) -> Result<Vec<wordcraft_engine::packs::Pack>, String> {
        Err("no request while downloading is off".into())
    }
    fn network_on(&self) -> bool {
        false
    }
    fn enable_network(&self) -> Result<(), String> {
        Err("refused".into())
    }
    fn install(&self, _: &wordcraft_engine::packs::Pack, _: &wordcraft_engine::packs::Cancel, _: &mut dyn FnMut(u64, u64)) -> Result<(), String> {
        Err("no request while downloading is off".into())
    }
    fn remove(&self, _: &str) -> Result<(), String> {
        Ok(())
    }
    fn packs_dir(&self) -> Option<std::path::PathBuf> {
        None
    }
}

/// The "downloading is off" error reaches the person unbranded (Options › Language shows engine
/// errors as they are, and the CLI/MCP print them), so it names Nexus Docs at the source.
#[test]
fn the_downloading_is_off_error_names_nexus_docs() {
    use wordcraft_engine::packs;
    packs::set_backend(Some(std::sync::Arc::new(OfflineStore)));
    let e = packs::refresh(&packs::Cancel::new()).unwrap_err();
    packs::set_backend(None);
    assert!(e.contains("downloading is turned off"), "{e}");
    assert!(e.contains("Nexus Docs"), "{e}");
    assert!(!e.contains("WordCraft"), "{e}");
}
