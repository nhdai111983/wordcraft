//! NexusOffice: what Nexus Docs writes names Nexus Docs, never the upstream product (spec §4.4,
//! `crates/engine/tests/nexus_brand.rs`). Since the 2026-11-15 sync the DOCX, ODT, RTF, HTML and
//! LaTeX writers live in craft-libs, which write "WordCraft" (the DOCX `<Application>`, the ODT
//! `<meta:generator>` and arrow/marker style names, the RTF `\generator`, the HTML generator meta
//! and the LaTeX header comment) and have no option for the name. This module renames them on
//! the way out. Reading needs nothing: the craft-libs readers don't depend on those names.

use craft_zip::{Archive, Compression, Limits, Writer};

const DOCX_APP_PART: &str = "docProps/app.xml";
const UPSTREAM_APP: &str = "<Application>WordCraft</Application>";
const NEXUS_APP: &str = "<Application>Nexus Docs</Application>";
/// A part bigger than this leaves the package as written: the rewrite holds each part in memory.
const MAX_PART: u64 = 512 << 20;

/// A zipped DOCX package whose `docProps/app.xml` names Nexus Docs as the application.
pub(crate) fn docx_package(bytes: Vec<u8>) -> Vec<u8> {
    rewrite_parts(bytes, |name, xml| (name == DOCX_APP_PART).then(|| xml.replacen(UPSTREAM_APP, NEXUS_APP, 1)))
}

/// A Flat OPC document (one XML file holding every part) naming Nexus Docs as the application.
pub(crate) fn docx_flat(bytes: Vec<u8>) -> Vec<u8> {
    match String::from_utf8(bytes) {
        Ok(s) => s.replacen(UPSTREAM_APP, NEXUS_APP, 1).into_bytes(),
        Err(e) => e.into_bytes(),
    }
}

/// The zipped package `bytes` with each `.xml` part `edit` returns new text for replaced by it;
/// every other part is copied as it is, in the same order and with the same storage (a stored
/// `mimetype` stays stored and first). Any trouble reading the package back leaves it as
/// written: a save never fails over a name.
pub fn rewrite_parts(bytes: Vec<u8>, edit: impl Fn(&str, &str) -> Option<String>) -> Vec<u8> {
    match rewritten(&bytes, &edit) {
        Some(out) => out,
        None => bytes,
    }
}

fn rewritten(bytes: &[u8], edit: &dyn Fn(&str, &str) -> Option<String>) -> Option<Vec<u8>> {
    let mut zin = Archive::open(bytes, LIMITS).ok()?;
    let names: Vec<String> = zin.names().map(|n| n.into_owned()).collect();
    let mut out = Writer::new();
    let mut changed = false;
    for name in names {
        let compression = if zin.is_stored(&name)? { Compression::Stored } else { Compression::Deflated };
        let data = zin.read(&name, MAX_PART).ok()??;
        let edited =
            if name.ends_with(".xml") { std::str::from_utf8(&data).ok().and_then(|text| edit(&name, text).filter(|t| t != text)) } else { None };
        match edited {
            Some(text) => {
                out.add(&name, text.as_bytes(), compression).ok()?;
                changed = true;
            }
            None => out.add(&name, &data, compression).ok()?,
        }
    }
    if !changed {
        return None;
    }
    out.finish().ok()
}

/// What a package being renamed may hold: as many entries as a DOCX may
/// (`craft_docx` refuses more), expanding to no more than 1 GiB in all.
const LIMITS: Limits = Limits { max_entries: 100_000, max_total_bytes: 1 << 30 };

#[cfg(test)]
mod tests {
    use super::*;

    fn zip_of(parts: &[(&str, &str)]) -> Vec<u8> {
        let mut w = Writer::new();
        for (name, text) in parts {
            let c = if *name == "mimetype" { Compression::Stored } else { Compression::Deflated };
            w.add(name, text.as_bytes(), c).unwrap();
        }
        w.finish().unwrap()
    }

    fn part(bytes: &[u8], name: &str) -> String {
        let mut z = Archive::open(bytes, LIMITS).unwrap();
        String::from_utf8(z.read(name, MAX_PART).unwrap().unwrap()).unwrap()
    }

    fn names(bytes: &[u8]) -> Vec<String> {
        Archive::open(bytes, LIMITS).unwrap().names().map(|n| n.into_owned()).collect()
    }

    #[test]
    fn the_docx_package_names_nexus_docs_and_keeps_every_other_part_in_order() {
        let pkg = zip_of(&[
            ("[Content_Types].xml", "<Types/>"),
            (DOCX_APP_PART, "<Properties><Application>WordCraft</Application><Words>3</Words></Properties>"),
            ("word/document.xml", "<w:document>WordCraft stays in text</w:document>"),
        ]);
        let out = docx_package(pkg);
        assert_eq!(part(&out, DOCX_APP_PART), "<Properties><Application>Nexus Docs</Application><Words>3</Words></Properties>");
        assert_eq!(part(&out, "word/document.xml"), "<w:document>WordCraft stays in text</w:document>");
        assert_eq!(names(&out), ["[Content_Types].xml", DOCX_APP_PART, "word/document.xml"], "every part, in order");
    }

    #[test]
    fn a_stored_first_part_stays_stored_and_first() {
        let pkg = zip_of(&[("mimetype", "application/vnd.oasis.opendocument.text"), ("meta.xml", "<meta:generator>WordCraft</meta:generator>")]);
        let out = rewrite_parts(pkg, |name, xml| (name == "meta.xml").then(|| xml.replace("WordCraft", "Nexus Docs")));
        assert_eq!(names(&out).first().map(String::as_str), Some("mimetype"));
        assert_eq!(craft_zip::stored_mimetype(&out), Some(&b"application/vnd.oasis.opendocument.text"[..]), "still stored, readable in place");
        assert_eq!(part(&out, "meta.xml"), "<meta:generator>Nexus Docs</meta:generator>");
    }

    #[test]
    fn a_damaged_package_or_one_without_the_name_is_left_as_written() {
        assert_eq!(docx_package(b"not a zip".to_vec()), b"not a zip");
        let pkg = zip_of(&[(DOCX_APP_PART, "<Properties><Application>Other</Application></Properties>")]);
        assert_eq!(docx_package(pkg.clone()), pkg);
    }

    #[test]
    fn flat_documents_name_nexus_docs() {
        let flat = docx_flat(b"<pkg:xmlData><Properties><Application>WordCraft</Application></Properties></pkg:xmlData>".to_vec());
        assert_eq!(flat, b"<pkg:xmlData><Properties><Application>Nexus Docs</Application></Properties></pkg:xmlData>");
    }
}
