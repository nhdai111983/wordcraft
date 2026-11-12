//! Date & Time insertion dialog (Word-style format and language selection).
use egui::{Stroke, Ui};
use serde::Serialize;
use serde_json::{Value, json};

use super::buttons;
use super::field::DATE_FORMATS;
use crate::WordApp;
use crate::theme::Tokens;

/// Predefined Portuguese (Brazil) date and time format pictures.
const PT_DATE_FORMATS: &[&str] = &[
    "dd/MM/yyyy",
    "dddd, d 'de' MMMM 'de' yyyy",
    "d 'de' MMMM 'de' yyyy",
    "dd/MM/yy",
    "yyyy-MM-dd",
    "d-MMM-yy",
    "dd.MM.yyyy",
    "d MMMM yyyy",
    "MMM-yy",
    "dd/MM/yyyy HH:mm",
    "dd/MM/yyyy HH:mm:ss",
    "HH:mm",
    "HH:mm:ss",
];

/// The date preview list consists of document values (not interface strings).
/// The i18n audit recognizes these dynamic values instead of treating month names as UI labels.
#[cfg(test)]
pub(crate) fn date_time_preview_matches(value: &str) -> bool {
    PT_DATE_FORMATS.iter().chain(DATE_FORMATS.iter()).any(|picture| {
        wordcraft_engine::cmd::insert::format_date_in(picture, Some("pt-BR")) == value
            || wordcraft_engine::cmd::insert::format_date_in(picture, Some("en-US")) == value
    }) || VI_DATE_FORMATS.iter().any(|picture| wordcraft_engine::cmd::insert::format_date_in(picture, Some("vi-VN")) == value)
}

/// NexusOffice (QC-DOC-03): Vietnamese text gets the Vietnamese pictures of Insert › Date & Time's
/// menu (26/10/2026, "Thứ Hai, 26 tháng 10 năm 2026"…).
const VI_DATE_FORMATS: &[&str] = wordcraft_engine::cmd::insert::DATE_FORMATS_VI;

/// NexusOffice (QC-DOC-03): the dialog's language. A saved choice (`pt…` is Portuguese (Brazil),
/// `vi…` Vietnamese, anything else English (United States)); without one, Vietnamese text at the
/// caret ([`caret_lang`](wordcraft_engine::cmd::insert::caret_lang), the Date & Time menu's
/// choice) or, for text in no language, a Vietnamese interface opens it in Vietnamese.
pub(super) fn dialog_language(saved: &str, caret: Option<&str>, interface: &str) -> &'static str {
    let vi = |l: &str| wordcraft_engine::nexus_vi::is_vi(Some(l));
    let pt = |l: &str| l.to_ascii_lowercase().starts_with("pt");
    if !saved.is_empty() {
        return if pt(saved) {
            "pt-BR"
        } else if vi(saved) {
            "vi-VN"
        } else {
            "en-US"
        };
    }
    if caret.map_or(vi(interface), vi) {
        "vi-VN"
    } else if pt(interface) {
        "pt-BR"
    } else {
        "en-US"
    }
}

/// The pictures the dialog offers for `language`.
fn formats_for(language: &str) -> &'static [&'static str] {
    match language {
        "pt-BR" => PT_DATE_FORMATS,
        "vi-VN" => VI_DATE_FORMATS,
        _ => DATE_FORMATS,
    }
}

/// State of the Word-like Insert > Date & Time dialog, including an optional validation error.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeForm {
    /// Selected Word field picture, not a Rust strftime format.
    pub format: String,
    /// Language of the resulting text or DATE field.
    pub language: String,
    /// Whether to insert a refreshable field rather than fixed text.
    pub update: bool,
    /// Last insertion failure; retain the window so the user can change the choice.
    #[serde(skip)]
    pub error: Option<String>,
    /// Brief status confirmation message (e.g. after setting default preferences).
    #[serde(skip)]
    pub status: Option<&'static str>,
}
impl DateTimeForm {
    /// Restore a saved format/language or choose a sensible default for the UI language.
    pub fn new(app: &WordApp) -> Self {
        let caret = wordcraft_engine::cmd::insert::caret_lang(&app.session);
        let language = dialog_language(&app.ui.date_time_language, caret.as_deref(), crate::i18n::current().code()).to_string();
        let list = formats_for(&language);
        let format = if list.contains(&app.ui.date_time_format.as_str()) { app.ui.date_time_format.clone() } else { list[0].to_string() };
        Self { format, language, update: app.ui.date_time_update, error: None, status: None }
    }
    /// Pictures available in the chosen language.
    pub(super) fn formats(&self) -> &'static [&'static str] {
        formats_for(&self.language)
    }
    /// Engine command parameters corresponding to the current controls.
    pub fn params(&self) -> Value {
        json!({"format": self.format, "language": self.language, "update": self.update})
    }
}
/// Draw the interactive format list, language picker and update checkbox; close on successful
/// insertion or Cancel, keeping errors visible rather than silently dismissing the dialog.
pub(super) fn date_time_ui(app: &mut WordApp, ui: &mut Ui, f: &mut DateTimeForm) -> bool {
    let tokens = Tokens::get(ui.ctx());
    ui.set_width(520.0);
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(350.0);
            ui.label(tl!("Date formats:"));
            egui::Frame::new().stroke(Stroke::new(1.0, tokens.border)).inner_margin(2.0).show(ui, |ui| {
                egui::ScrollArea::vertical().id_salt("date_time_formats").max_height(245.0).auto_shrink([false, false]).show(ui, |ui| {
                    for picture in f.formats() {
                        let sample = wordcraft_engine::cmd::insert::format_date_in(picture, Some(&f.language));
                        if ui.selectable_label(f.format == *picture, sample).on_hover_text(*picture).clicked() {
                            f.format = (*picture).to_string();
                        }
                    }
                });
            });
        });
        ui.add_space(12.0);
        ui.vertical(|ui| {
            ui.label(tl!("Language"));
            craft_ui::ComboBox::from_id_salt("date_time_lang")
                .selected_text(match f.language.as_str() {
                    "pt-BR" => tl!("Portuguese (Brazil)"),
                    "vi-VN" => tl!("Vietnamese"),
                    _ => tl!("English (United States)"),
                })
                .show_ui(ui, |ui| {
                    if ui.selectable_label(f.language == "vi-VN", tl!("Vietnamese")).clicked() {
                        f.language = "vi-VN".into();
                        f.format = VI_DATE_FORMATS.first().copied().unwrap_or("dd/MM/yyyy").into();
                    }
                    if ui.selectable_label(f.language == "pt-BR", tl!("Portuguese (Brazil)")).clicked() {
                        f.language = "pt-BR".into();
                        f.format = PT_DATE_FORMATS[0].into();
                    }
                    if ui.selectable_label(f.language == "en-US", tl!("English (United States)")).clicked() {
                        f.language = "en-US".into();
                        f.format = DATE_FORMATS[0].into();
                    }
                });
        });
    });
    ui.add_space(8.0);
    ui.checkbox(&mut f.update, tl!("Update automatically"));
    ui.add_space(6.0);
    if ui.button(tl!("Set as Default")).clicked() {
        app.ui.date_time_format = f.format.clone();
        app.ui.date_time_language = f.language.clone();
        app.ui.date_time_update = f.update;
        f.status = Some(tl!("Default format saved."));
    }
    if let Some(status) = f.status {
        ui.colored_label(tokens.text_dim, status);
    }
    if let Some(error) = &f.error {
        ui.colored_label(ui.visuals().error_fg_color, error);
    }
    let (ok, cancel) = buttons(ui, "OK", "Cancel", true);
    if ok {
        match app.run("insert.dateTime", f.params()) {
            Ok(_) => return true,
            Err(error) => {
                f.error = Some(error.to_string());
                return false;
            }
        }
    }
    cancel
}
