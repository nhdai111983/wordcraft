//! NexusOffice: the HTML, LaTeX, RTF and ODT writers (craft-libs since the 2026-11-15 sync) name
//! WordCraft in what they write; Nexus Docs writes its own name (spec §4.4,
//! `crates/engine/tests/nexus_brand.rs`). See `wordcraft_docx::nexus` for the DOCX side and the
//! zip rewrite these share.

/// The HTML generator meta.
pub(crate) fn html(s: String) -> String {
    s.replacen(r#"<meta name="generator" content="WordCraft">"#, r#"<meta name="generator" content="Nexus Docs">"#, 1)
}

/// The LaTeX header comment.
pub(crate) fn latex(s: String) -> String {
    s.replacen("% Written by WordCraft (https://getartcraft.com/apps/wordcraft).\n", "% Written by Nexus Docs (NexusOffice).\n", 1)
}

/// The RTF generator group.
pub(crate) fn rtf(s: String) -> String {
    s.replacen(r"{\*\generator WordCraft;}", r"{\*\generator Nexus Docs;}", 1)
}

/// ODF XML (a part of a package, or a whole Flat ODT): the `<meta:generator>`, and the names of
/// the arrowhead and other drawing styles craft-odf invents ("WordCraft_20_Arrow", shown as
/// "WordCraft Arrow" in other apps' style lists). Names in the user's own styles are left alone:
/// only the exact forms craft-odf writes change.
pub(crate) fn odf_xml(s: &str) -> String {
    s.replace("<meta:generator>WordCraft</meta:generator>", "<meta:generator>Nexus Docs</meta:generator>")
        .replace("draw:name=\"WordCraft_20_", "draw:name=\"Nexus_20_Docs_20_")
        .replace("=\"WordCraft_20_", "=\"Nexus_20_Docs_20_")
        .replace("draw:display-name=\"WordCraft ", "draw:display-name=\"Nexus Docs ")
}

/// A zipped ODT package with [`odf_xml`] applied to its metadata, styles and content.
pub(crate) fn odt_package(bytes: Vec<u8>) -> Vec<u8> {
    wordcraft_docx::nexus::rewrite_parts(bytes, |name, xml| matches!(name, "meta.xml" | "styles.xml" | "content.xml").then(|| odf_xml(xml)))
}

/// A Flat ODT (one XML file).
pub(crate) fn odt_flat(bytes: Vec<u8>) -> Vec<u8> {
    match String::from_utf8(bytes) {
        Ok(s) => odf_xml(&s).into_bytes(),
        Err(e) => e.into_bytes(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_writer_s_name_becomes_nexus_docs() {
        assert!(html(r#"<head><meta name="generator" content="WordCraft">"#.into()).contains(r#"content="Nexus Docs""#));
        assert_eq!(
            latex("% Written by WordCraft (https://getartcraft.com/apps/wordcraft).\n\\begin".into()),
            "% Written by Nexus Docs (NexusOffice).\n\\begin"
        );
        assert_eq!(rtf(r"{\rtf1}{\*\generator WordCraft;}".into()), r"{\rtf1}{\*\generator Nexus Docs;}");
        let xml = odf_xml(concat!(
            "<meta:generator>WordCraft</meta:generator>",
            "<draw:marker draw:name=\"WordCraft_20_Arrow\" draw:display-name=\"WordCraft Arrow\"/>",
            "<style:graphic-properties draw:marker-end=\"WordCraft_20_Arrow\" draw:stroke-dash=\"WordCraft_20_Dash_20_d1\"/>",
            "<text:p>WordCraft_20_ in the user's text stays</text:p>",
        ));
        assert!(!xml.contains("=\"WordCraft"), "{xml}");
        assert!(xml.contains("<meta:generator>Nexus Docs</meta:generator>"), "{xml}");
        assert!(xml.contains("draw:display-name=\"Nexus Docs Arrow\""), "{xml}");
        assert!(xml.contains("draw:marker-end=\"Nexus_20_Docs_20_Arrow\""), "{xml}");
        assert!(xml.contains("<text:p>WordCraft_20_ in the user's text stays</text:p>"), "{xml}");
    }
}
