//! NexusOffice: what Nexus Docs writes and says names Nexus Docs, never the upstream product
//! (spec §4.4, `crates/engine/tests/nexus_brand.rs`). Since the 2026-11-14 sync the DOCX reader
//! and writer live in craft-libs' `craft-docx`, which names WordCraft in `docProps/app.xml` and in
//! its encryption errors and has no option for the name, so this module renames them on the way
//! out.

use std::io::{Cursor, Read, Write};

use craft_docx::DocxError;

const UPSTREAM: &str = "WordCraft";
const NAME: &str = "Nexus Docs";
const APP_PART: &str = "docProps/app.xml";
const UPSTREAM_APP: &str = "<Application>WordCraft</Application>";
const NEXUS_APP: &str = "<Application>Nexus Docs</Application>";
/// `docProps/app.xml` is a few hundred bytes; a bigger one is left as written.
const MAX_APP_XML: u64 = 1 << 20;

/// An error whose text names WordCraft, naming Nexus Docs instead.
pub(crate) fn error(e: DocxError) -> DocxError {
    match e {
        DocxError::Encryption(s) => DocxError::Encryption(s.replace(UPSTREAM, NAME)),
        other => other,
    }
}

/// A zipped package whose `docProps/app.xml` names Nexus Docs as the application. Any trouble
/// reading the package back leaves it as written: a save never fails over the name.
pub(crate) fn package(bytes: Vec<u8>) -> Vec<u8> {
    match renamed(&bytes) {
        Some(out) => out,
        None => bytes,
    }
}

/// A Flat OPC document (one XML file holding every part) naming Nexus Docs as the application.
pub(crate) fn flat(bytes: Vec<u8>) -> Vec<u8> {
    match String::from_utf8(bytes) {
        Ok(s) => s.replacen(UPSTREAM_APP, NEXUS_APP, 1).into_bytes(),
        Err(e) => e.into_bytes(),
    }
}

fn renamed(bytes: &[u8]) -> Option<Vec<u8>> {
    let mut zin = zip::ZipArchive::new(Cursor::new(bytes)).ok()?;
    let app = {
        let f = zin.by_name(APP_PART).ok()?;
        if f.size() > MAX_APP_XML {
            return None;
        }
        let mut s = String::new();
        f.take(MAX_APP_XML).read_to_string(&mut s).ok()?;
        s
    };
    if !app.contains(UPSTREAM_APP) {
        return None;
    }
    let app = app.replacen(UPSTREAM_APP, NEXUS_APP, 1);
    let mut out = zip::ZipWriter::new(Cursor::new(Vec::with_capacity(bytes.len().saturating_add(64))));
    for i in 0..zin.len() {
        let f = zin.by_index_raw(i).ok()?;
        if f.name() == APP_PART {
            // Same compression and time as the part it replaces, so saves stay reproducible.
            let mut opts = zip::write::SimpleFileOptions::default().compression_method(f.compression());
            if let Some(t) = f.last_modified() {
                opts = opts.last_modified_time(t);
            }
            drop(f);
            out.start_file(APP_PART, opts).ok()?;
            out.write_all(app.as_bytes()).ok()?;
        } else {
            out.raw_copy_file(f).ok()?;
        }
    }
    Some(out.finish().ok()?.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn zip_of(parts: &[(&str, &str)]) -> Vec<u8> {
        let mut w = zip::ZipWriter::new(Cursor::new(Vec::new()));
        for (name, text) in parts {
            w.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
            w.write_all(text.as_bytes()).unwrap();
        }
        w.finish().unwrap().into_inner()
    }

    fn part(bytes: &[u8], name: &str) -> String {
        let mut z = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
        let mut s = String::new();
        z.by_name(name).unwrap().read_to_string(&mut s).unwrap();
        s
    }

    #[test]
    fn the_package_names_nexus_docs_and_keeps_every_other_part_in_order() {
        let pkg = zip_of(&[
            ("[Content_Types].xml", "<Types/>"),
            (APP_PART, "<Properties><Application>WordCraft</Application><Words>3</Words></Properties>"),
            ("word/document.xml", "<w:document>WordCraft stays in text</w:document>"),
        ]);
        let out = package(pkg);
        assert_eq!(part(&out, APP_PART), "<Properties><Application>Nexus Docs</Application><Words>3</Words></Properties>");
        assert_eq!(part(&out, "word/document.xml"), "<w:document>WordCraft stays in text</w:document>");
        let z = zip::ZipArchive::new(Cursor::new(&out)).unwrap();
        let names: Vec<_> = z.file_names().collect();
        assert_eq!(names.first(), Some(&"[Content_Types].xml"), "the content types stay first");
        assert_eq!(z.len(), 3);
    }

    #[test]
    fn a_damaged_package_or_one_without_the_name_is_left_as_written() {
        assert_eq!(package(b"not a zip".to_vec()), b"not a zip");
        let pkg = zip_of(&[(APP_PART, "<Properties><Application>Other</Application></Properties>")]);
        assert_eq!(package(pkg.clone()), pkg);
    }

    #[test]
    fn flat_documents_and_encryption_errors_name_nexus_docs() {
        let flat_doc = flat(b"<pkg:xmlData><Properties><Application>WordCraft</Application></Properties></pkg:xmlData>".to_vec());
        assert_eq!(flat_doc, b"<pkg:xmlData><Properties><Application>Nexus Docs</Application></Properties></pkg:xmlData>");
        let e = error(DocxError::Encryption("this file uses extensible encryption, which WordCraft can't open".into()));
        assert_eq!(e.to_string(), "encrypted document: this file uses extensible encryption, which Nexus Docs can't open");
    }
}
