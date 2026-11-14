//! Rich Text Format adapter using craft-rtf.

pub use craft_rtf::doc::esc;
pub use craft_rtf::{dttm_to_iso, iso_to_dttm};

/// Import RTF bytes into WordCraft's document model.
pub fn import(bytes: &[u8]) -> Result<wordcraft_doc::Document, String> {
    let shared = craft_rtf::doc::import(bytes)?;
    wordcraft_docx::from_shared_model(shared).map_err(|e| e.to_string())
}

/// Export a WordCraft document to RTF. NexusOffice: a model conversion that fails gives an empty
/// RTF document instead of panicking (never crash); saving uses [`try_export`], which reports it.
pub fn export(doc: &wordcraft_doc::Document) -> String {
    try_export(doc).unwrap_or_else(|_| String::from(r"{\rtf1\ansi}"))
}

/// Export a WordCraft document to RTF, or say why the document couldn't be converted.
pub fn try_export(doc: &wordcraft_doc::Document) -> Result<String, String> {
    let shared = wordcraft_docx::to_shared_model(doc).map_err(|e| e.to_string())?;
    // NexusOffice: craft-rtf names WordCraft as the generator; Nexus Docs writes its own name
    // (spec §4.4, `crates/engine/tests/nexus_brand.rs`).
    Ok(craft_rtf::doc::export(&shared).replacen(r"{\*\generator WordCraft;}", r"{\*\generator Nexus Docs;}", 1))
}

/// Parse RTF bytes into the flow model.
pub fn parse(bytes: &[u8]) -> Result<crate::model::Flow, String> {
    let doc = import(bytes)?;
    Ok(crate::model::from_doc(&doc))
}
