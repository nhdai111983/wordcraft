//! File tab (Backstage): Home/New, Open, Info, Save As, Print, Export, Close, Options, About.

use egui::{Align2, Rect, Sense, Stroke, Ui, pos2, vec2};
use serde_json::json;

use crate::theme::{APP_COLOR, Tokens, medium, regular, semibold};
use crate::{WordApp, icons};

const PAGES: [(&str, &str); 11] = [
    ("home", "Home"),
    ("new", "New"),
    ("open", "Open"),
    ("info", "Info"),
    ("recover", "Recover"),
    ("save", "Save"),
    ("saveAs", "Save As"),
    ("print", "Print"),
    ("export", "Export"),
    ("close", "Close"),
    ("options", "Options"),
];

/// The Backstage's colours. Upstream draws the side menu in the app colour (black on the Recover
/// page) with white text. NexusOffice's WPS layout ([`WordApp::wps`]) keeps the window's own
/// chrome instead, light or dark with the theme: the side menu on the title bar's grey, the open
/// page on the accent's tint with an accent bar and accent text, the pages on the ribbon's white
/// with the body text.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Look {
    /// The side menu's fill.
    pub nav: egui::Color32,
    /// The page's fill.
    pub page: egui::Color32,
    pub text: egui::Color32,
    pub text_off: egui::Color32,
    /// The open page's row, and its text.
    pub active: egui::Color32,
    pub active_text: egui::Color32,
    pub hover: egui::Color32,
    /// WPS: the accent bar beside the open page, and the line between the menu and the page.
    pub bar: Option<egui::Color32>,
    pub divider: Option<egui::Color32>,
}

impl Look {
    pub(crate) fn new(t: &Tokens, wps: bool, recovery: bool) -> Self {
        if wps {
            return Look {
                nav: t.title_bar,
                page: t.ribbon,
                text: t.text,
                text_off: t.text_disabled,
                active: t.checked,
                active_text: crate::nexus_wps::file_ink(t, t.checked),
                // The plain hover grey barely shows on the light menu's own grey.
                hover: if t.dark { t.hover } else { t.pressed },
                bar: Some(t.accent),
                divider: Some(t.border),
            };
        }
        let fill = if recovery { egui::Color32::BLACK } else { APP_COLOR };
        Look {
            nav: fill,
            page: if recovery { egui::Color32::BLACK } else { t.ribbon },
            text: egui::Color32::WHITE,
            text_off: egui::Color32::from_white_alpha(110),
            active: egui::Color32::from_white_alpha(46),
            active_text: egui::Color32::WHITE,
            hover: egui::Color32::from_white_alpha(26),
            bar: None,
            divider: None,
        }
    }
}

/// The side menu's back button (it closes the Backstage). Upstream: a white chevron. WPS: a
/// bordered "‹ Back" button, so the way back to the document is plain to see.
fn back_button(ui: &mut Ui, look: &Look, band: Rect, x: f32) -> egui::Response {
    let id = ui.id().with("backstage_back");
    if look.bar.is_none() {
        let c = pos2(x, band.center().y);
        let resp = ui.interact(Rect::from_center_size(c, vec2(22.0, band.height())), id, Sense::click());
        icons::paint(ui.painter(), Rect::from_center_size(c, vec2(18.0, 18.0)), "chevronLeft", look.text, look.text);
        return resp;
    }
    let t = Tokens::get(ui.ctx());
    let label = tl!("Back");
    let font = medium(13.5);
    let tw = ui.painter().layout_no_wrap(label.to_string(), font.clone(), look.text).size().x;
    let r = Rect::from_min_size(pos2(x - 12.0, band.center().y - 15.0), vec2(12.0 + 16.0 + 6.0 + tw + 14.0, 30.0));
    let resp = ui.interact(r, id, Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));
    let fill = if resp.hovered() { look.hover } else { t.ribbon };
    ui.painter().rect(r, 6.0, fill, Stroke::new(1.0, t.border_strong), egui::StrokeKind::Inside);
    icons::paint(ui.painter(), Rect::from_center_size(pos2(r.min.x + 18.0, r.center().y), vec2(16.0, 16.0)), "chevronLeft", look.text, look.text);
    ui.painter().text(pos2(r.min.x + 32.0, r.center().y), Align2::LEFT_CENTER, label, font, look.text);
    resp
}

/// A side-menu row's highlight: upstream fills the row; WPS an inset rounded tint, with the
/// accent bar on the open page.
fn paint_row(ui: &Ui, look: &Look, r: Rect, fill: egui::Color32, active: bool) {
    match look.bar {
        None => {
            ui.painter().rect_filled(r, 0.0, fill);
        }
        Some(bar) => {
            let r = Rect::from_min_max(pos2(r.min.x + 8.0, r.min.y + 2.0), pos2(r.max.x - 8.0, r.max.y - 2.0));
            ui.painter().rect_filled(r, 6.0, fill);
            if active {
                ui.painter().rect_filled(Rect::from_min_max(pos2(r.min.x, r.min.y + 8.0), pos2(r.min.x + 3.0, r.max.y - 8.0)), 1.5, bar);
            }
        }
    }
}

pub fn show(app: &mut WordApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let recovery = app.ui.backstage_page == "recover";
    let look = Look::new(&t, app.wps, recovery);
    // No document open (#1129): only the pages that open or create one, and Options.
    let has_document = app.has_document();
    if !has_document && !works_without_document(&app.ui.backstage_page) {
        app.ui.backstage_page = "home".into();
    }
    // Embedded in the NexusOffice window, the WPS layout has no title bar to share.
    let mac = app.integrated_titlebar && !(app.wps && app.embedded);
    egui::Panel::left("backstage_nav")
        .exact_size(200.0)
        // Fixed width, as in Word: without this egui shows a resize handle that does nothing (#502).
        .resizable(false)
        .frame(egui::Frame::NONE.fill(look.nav).inner_margin(egui::Margin { left: 0, right: 0, top: if mac { 0 } else { 12 }, bottom: 12 }))
        .show(ui, |ui| {
            if let Some(line) = look.divider {
                let full = ui.max_rect();
                let x = full.max.x - 0.5;
                ui.painter().vline(x, (full.min.y - 12.0)..=(full.max.y + 12.0), Stroke::new(1.0, line));
            }
            let back = if mac {
                // macOS (#222): the Backstage covers the title bar, so its top band is the title
                // bar: the back button sits beside the traffic lights, centred with them and as far
                // from them as the title bar's own controls, and the rest of the band drags the window.
                let (band, drag) = ui.allocate_exact_size(vec2(200.0, app.mac_titlebar.height), Sense::click_and_drag());
                if drag.drag_started() && !app.embedded {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                // The chevron's stroke starts at the content edge.
                back_button(ui, &look, band, band.min.x + app.mac_titlebar.content_left + 3.0)
            } else if look.bar.is_none() {
                // Upstream: the whole row is the back button.
                let (r, resp) = ui.allocate_exact_size(vec2(200.0, 40.0), Sense::click());
                let c = pos2(r.min.x + 26.0, r.center().y);
                icons::paint(ui.painter(), Rect::from_center_size(c, vec2(18.0, 18.0)), "chevronLeft", look.text, look.text);
                resp
            } else {
                let (r, _) = ui.allocate_exact_size(vec2(200.0, 44.0), Sense::hover());
                back_button(ui, &look, r, r.min.x + 26.0)
            };
            // Escape in the built-in file browser closes only the browser.
            if back.clicked() || (!app.file_dialog_open() && ui.input(|i| i.key_pressed(egui::Key::Escape))) {
                app.ui.backstage = false;
                app.canvas.want_focus = true;
            }
            ui.add_space(if mac { 12.0 } else { 6.0 });
            for (id, label) in PAGES {
                // No document open (#1129): Info, Save, Save As, Print and Export are greyed out.
                let on = has_document || works_without_document(id);
                let (r, resp) = ui.add_enabled_ui(on, |ui| ui.allocate_exact_size(vec2(200.0, 38.0), Sense::click())).inner;
                resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, on, tl!(label)));
                let active = app.ui.backstage_page == id;
                if active {
                    paint_row(ui, &look, r, look.active, true);
                } else if on && resp.hovered() {
                    paint_row(ui, &look, r, look.hover, false);
                }
                ui.painter().text(
                    pos2(r.min.x + 22.0, r.center().y),
                    Align2::LEFT_CENTER,
                    tl!(label),
                    if active { semibold(14.0) } else { medium(14.0) },
                    if !on {
                        look.text_off
                    } else if active {
                        look.active_text
                    } else {
                        look.text
                    },
                );
                if resp.clicked() {
                    match id {
                        // A document without a file goes to the Save As page (as in Word) instead
                        // of `file.save`, which would also open the native Save As dialog.
                        // In the browser a save is a download, so it saves right away.
                        "save" if app.session.path.is_none() && app.services.download.is_none() => {
                            app.ui.backstage_page = "saveAs".into();
                        }
                        "save" => {
                            let r = app.run("file.save", json!({}));
                            if r.is_ok_and(|v| v.get("saved").and_then(serde_json::Value::as_bool) == Some(true) || v.get("downloaded").is_some()) {
                                app.ui.backstage = false;
                                app.canvas.want_focus = true;
                            } else {
                                app.ui.backstage_page = "saveAs".into();
                            }
                        }
                        "saveAs" => {
                            app.ui.backstage_page = "saveAs".into();
                        }
                        // Not a page: `file.close` asks Save / Don't Save / Cancel first when there
                        // are unsaved changes, and Cancel leaves the Backstage where it was.
                        "close" => {
                            let _ = app.run("file.close", json!({}));
                        }
                        _ => app.ui.backstage_page = id.into(),
                    }
                }
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                for (label, id) in [("About", "about"), ("Discord community", "discord")] {
                    if id == "discord" && crate::nexus::COMMUNITY_URL.is_none() {
                        continue;
                    }
                    let (r, resp) = ui.allocate_exact_size(vec2(200.0, 32.0), Sense::click());
                    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), tl!(label)));
                    if resp.hovered() {
                        paint_row(ui, &look, r, look.hover, false);
                    }
                    ui.painter().text(pos2(r.min.x + 22.0, r.center().y), Align2::LEFT_CENTER, tl!(label), regular(13.0), look.text);
                    if resp.clicked() {
                        if id == "discord" {
                            let _ = app.run("ui.discord", json!({}));
                        } else {
                            app.dialog = crate::dialogs::Dialog::open("about", app);
                        }
                    }
                }
            });
        });
    egui::CentralPanel::default().frame(egui::Frame::NONE.fill(look.page).inner_margin(egui::Margin::symmetric(40, 30))).show(ui, |ui| {
        // The Print page fills the pane and scrolls its settings and its preview itself (#1156).
        if app.ui.backstage_page == "print" {
            export_page(app, ui);
            return;
        }
        egui::ScrollArea::vertical().show(ui, |ui| match app.ui.backstage_page.as_str() {
            "home" => home_page(app, ui),
            "new" => new_page(app, ui),
            "open" => open_page(app, ui),
            "saveAs" => save_as_page(app, ui),
            "info" => info_page(app, ui),
            "recover" => recovery_page(app, ui),
            "print" | "export" => export_page(app, ui),
            "options" => crate::options::page(app, ui),
            _ => home_page(app, ui),
        });
    });
}

/// The recovery copies a window offers to restore: every copy but the one this session keeps
/// for its own open document (NexusOffice: that one is seconds old and restoring it would only
/// replace the document with itself).
pub(crate) fn recovery_list(app: &WordApp) -> Vec<wordcraft_engine::recovery::RecoveryInfo> {
    let own = app.session.recovery_current();
    app.session.recovery_documents().into_iter().filter(|d| Some(d.id.as_str()) != own).collect()
}

/// A recovery copy's name as shown: the engine names a document without a file "Untitled draft"
/// in the copy itself, so that name is translated where it is shown.
pub(crate) fn recovery_name(name: &str) -> String {
    if name == "Untitled draft" { tl!("Untitled draft").to_string() } else { name.to_string() }
}

/// Recovery is a separate copy. Leaving this page never discards it.
fn recovery_page(app: &mut WordApp, ui: &mut Ui) {
    // NexusOffice: upstream draws the page on black, so its buttons use the dark look too; white
    // text alone left them white on white in the light theme. The WPS layout keeps the theme.
    if !app.wps {
        *ui.visuals_mut() = egui::Visuals::dark();
        ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);
    }
    heading(ui, "Recover documents");
    ui.label(tl!("Restore opens an unsaved copy. Your original file stays unchanged."));
    ui.add_space(20.0);
    if app.session.password.is_some() {
        ui.label(tl!("Recovery is disabled while this document has a password."));
    }
    if let Some(error) = app.session.recovery_error() {
        ui.label(error);
    }
    if !app.session.recovery_ready() {
        ui.label(tl!("Local recovery is unavailable or still loading."));
    }
    let documents = recovery_list(app);
    if documents.is_empty() && app.session.recovery_ready() {
        ui.label(tl!("No recovery copies."));
    }
    let busy = app.session.recovery_busy();
    egui::Grid::new("recovery_documents").min_col_width(180.0).spacing(vec2(24.0, 16.0)).show(ui, |ui| {
        if !documents.is_empty() {
            ui.strong(tl!("Document"));
            ui.strong(tl!("Recovery copy"));
            ui.end_row();
        }
        for doc in documents {
            ui.label(recovery_name(&doc.name));
            ui.label(&doc.saved_at);
            ui.add_enabled_ui(!busy, |ui| {
                ui.horizontal(|ui| {
                    // Whole labels: a button never shortens its text to fit the column.
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                    if ui.button(tl!("Restore")).clicked() {
                        let _ = app.run("file.recover", json!({"restore": doc.id}));
                    }
                    if ui.button(tl!("Discard…")).clicked() {
                        let _ = app.run("ui.recoveryDiscard", json!({"id": doc.id}));
                    }
                });
            });
            ui.end_row();
        }
    });
    ui.add_space(20.0);
    ui.horizontal(|ui| {
        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
        if ui.button(tl!("Continue without restoring")).clicked() {
            let _ = app.run("ui.backstage", json!({"value": false}));
            app.canvas.want_focus = true;
        }
        if ui.add_enabled(!busy && app.session.recovery_ready(), egui::Button::new(tl!("Refresh"))).clicked() {
            let _ = app.run("file.recover", json!({"refresh": true}));
        }
    });
}

/// The Backstage pages that work while no document is open (#1129).
fn works_without_document(page: &str) -> bool {
    matches!(page, "home" | "new" | "open" | "options")
}

fn heading(ui: &mut Ui, s: &str) {
    ui.label(egui::RichText::new(tl!(s)).font(semibold(26.0)));
    ui.add_space(16.0);
}

fn template_tile(ui: &mut Ui, app: &mut WordApp, label: &str, template: &str) {
    let t = Tokens::get(ui.ctx());
    ui.vertical(|ui| {
        // The label sits right under the thumbnail whatever row gap the gallery uses.
        ui.spacing_mut().item_spacing.y = 0.0;
        let (r, resp) = ui.allocate_exact_size(vec2(150.0, 194.0), Sense::click());
        ui.painter().rect(
            r,
            2.0,
            egui::Color32::WHITE,
            Stroke::new(if resp.hovered() { 2.0 } else { 1.0 }, if resp.hovered() { t.accent } else { t.border_strong }),
            egui::StrokeKind::Inside,
        );
        let ppp = ui.ctx().pixels_per_point();
        let key = format!("tpl:{template}:{ppp}");
        let region = app.session.region();
        let tex = ui.ctx().data(|d| d.get_temp::<egui::TextureHandle>(egui::Id::new(&key))).or_else(|| {
            let mut s = wordcraft_engine::Session::new(wordcraft_doc::Document::new());
            // The blank tile shows the region's paper (#838).
            s.system_region = region;
            let _ = s.run("file.new", &json!({"template": template}));
            let l = s.export_layout();
            let page = l.pages.first()?;
            let img = wordcraft_render::render_page(&s.doc, page, 150.0 / page.w * ppp, &crate::canvas::screen_render_options());
            let px = img.to_straight();
            let h = ui.ctx().load_texture(
                &key,
                egui::ColorImage::from_rgba_unmultiplied([img.width as usize, img.height as usize], &px),
                egui::TextureOptions::LINEAR,
            );
            ui.ctx().data_mut(|d| d.insert_temp(egui::Id::new(&key), h.clone()));
            Some(h)
        });
        if let Some(h) = tex {
            ui.painter().image(h.id(), r.shrink(1.0), Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), egui::Color32::WHITE);
        }
        // The caption opens the template too, as in Word.
        let caption = ui.add(egui::Label::new(egui::RichText::new(crate::i18n::tc("template", label)).font(regular(12.5))).sense(Sense::click()));
        if resp.clicked() || caption.clicked() {
            let _ = app.run("file.new", json!({"template": template}));
            app.ui.backstage = false;
            app.canvas.want_focus = true;
        }
    });
}

/// The built-in templates in gallery order: display label and the `file.new` template id.
const TEMPLATES: [(&str, &str); 5] =
    [("Blank document", "blank"), ("Studio handbook (sample)", "sample"), ("Letter", "letter"), ("Résumé", "resume"), ("Report", "report")];

/// How many templates Home shows before "More templates" (the rest are on New).
const HOME_TEMPLATES: usize = 4;

/// The templates whose English or translated label, or id, contains `query` (case-insensitive;
/// the id lets "resume" find "Résumé"); all of them for an empty query.
fn matching_templates(query: &str) -> Vec<(&'static str, &'static str)> {
    let q = query.trim().to_lowercase();
    TEMPLATES
        .into_iter()
        .filter(|(label, id)| {
            q.is_empty() || id.contains(&q) || label.to_lowercase().contains(&q) || crate::i18n::tc("template", label).to_lowercase().contains(&q)
        })
        .collect()
}

/// Home: a greeting, Blank plus a short row of templates with a link to New, then Recent.
fn home_page(app: &mut WordApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let hour = wordcraft_engine::cmd::now_iso().get(11..13).and_then(|h| h.parse::<u32>().ok()).unwrap_or(9);
    heading(
        ui,
        if hour < 12 {
            "Good morning"
        } else if hour < 18 {
            "Good afternoon"
        } else {
            "Good evening"
        },
    );
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(18.0, 0.0);
        for (label, template) in TEMPLATES.into_iter().take(HOME_TEMPLATES) {
            template_tile(ui, app, label, template);
        }
    });
    ui.add_space(8.0);
    let more = ui.add(egui::Label::new(egui::RichText::new(tl!("More templates →")).font(medium(13.5)).color(t.accent)).sense(Sense::click()));
    if more.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
        app.ui.backstage_page = "new".into();
    }
    ui.add_space(24.0);
    open_list(app, ui);
}

/// New: the whole template gallery with a search box; no Recent list (that is on Home and Open).
fn new_page(app: &mut WordApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    heading(ui, "New");
    let id = egui::Id::new("backstage_template_search");
    let mut query = ui.ctx().data(|d| d.get_temp::<String>(id)).unwrap_or_default();
    let search = ui.add(egui::TextEdit::singleline(&mut query).hint_text(tl!("Search for templates")).desired_width(360.0));
    if search.changed() {
        ui.ctx().data_mut(|d| d.insert_temp(id, query.clone()));
    }
    ui.add_space(18.0);
    let found = matching_templates(&query);
    if found.is_empty() {
        ui.label(egui::RichText::new(tl!("No templates match your search.")).color(t.text_dim));
        return;
    }
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(18.0, 18.0);
        for (label, template) in found {
            template_tile(ui, app, label, template);
        }
    });
}

fn open_list(app: &mut WordApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.label(egui::RichText::new(tl!("Recent")).font(semibold(16.0)));
    ui.add_space(6.0);
    if app.ui.recent.is_empty() {
        ui.label(egui::RichText::new(tl!("Documents you open will show up here.")).color(t.text_dim));
    }
    for p in app.ui.recent.iter().take(app.ui.recent_shown()).cloned().collect::<Vec<_>>() {
        let name = std::path::Path::new(&p).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or(p.clone());
        let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width().min(700.0), 40.0), Sense::click());
        if resp.hovered() {
            ui.painter().rect_filled(r, 4.0, t.hover);
        }
        icons::paint(ui.painter(), Rect::from_center_size(pos2(r.min.x + 18.0, r.center().y), vec2(22.0, 22.0)), "printLayout", t.icon, t.accent);
        ui.painter().text(pos2(r.min.x + 40.0, r.min.y + 13.0), Align2::LEFT_CENTER, &name, medium(13.0), t.text);
        ui.painter().text(pos2(r.min.x + 40.0, r.min.y + 29.0), Align2::LEFT_CENTER, &p, regular(11.0), t.text_dim);
        if resp.clicked() {
            let _ = app.run("file.open", json!({"path": p}));
            app.ui.backstage = false;
            app.canvas.want_focus = true;
        }
    }
}

fn open_page(app: &mut WordApp, ui: &mut Ui) {
    heading(ui, "Open");
    if ui.button(egui::RichText::new(tl!("Browse…")).font(medium(14.0))).clicked() {
        let _ = app.run("ui.openFileDialog", json!({}));
    }
    // Linux and the BSDs (#576): the system dialog needs a file chooser portal or zenity; when it
    // doesn't show up, WordCraft's own browser is always one click away.
    #[cfg(all(unix, not(target_os = "macos"), not(target_arch = "wasm32")))]
    if ui
        .button(egui::RichText::new(tl!("Browse in WordCraft…")).font(medium(14.0)))
        .on_hover_text(tl!("Open a file with WordCraft's own browser when the system file dialog doesn't appear."))
        .clicked()
    {
        let _ = app.run("ui.fileBrowser", json!({}));
    }
    ui.add_space(18.0);
    open_list(app, ui);
}

/// Save As: Browse… asks where to save. It has no recent-documents list: clicking one of those
/// opens it, which is not what Save As is for.
fn save_as_page(app: &mut WordApp, ui: &mut Ui) {
    heading(ui, "Save As");
    if ui.button(egui::RichText::new(tl!("Browse…")).font(medium(14.0))).clicked() {
        app.save_as_dialog();
    }
}

fn info_page(app: &mut WordApp, ui: &mut Ui) {
    heading(ui, "Info");
    let info = app.session.run("file.info", &json!({})).unwrap_or_default();
    ui.columns(2, |cols| {
        let ui = &mut cols[0];
        ui.label(egui::RichText::new(tl!("Properties")).font(semibold(15.0)));
        // The fields edit a copy of the document's properties that is read afresh every frame,
        // so each change goes into the document as it is typed (#260): waiting for the field to
        // lose focus dropped the keystrokes, and leaving File with Escape never loses it at all.
        // The keystrokes of one visit to a field are a single undo step.
        let mut props = app.session.doc.core.clone();
        let mut edited = None;
        let mut focused = None;
        egui::Grid::new("props").num_columns(2).spacing(vec2(12.0, 6.0)).show(ui, |ui| {
            for (l, v) in [("Title", &mut props.title), ("Subject", &mut props.subject), ("Author", &mut props.creator), ("Keywords", &mut props.keywords), ("Category", &mut props.category)] {
                let label = ui.label(tl!(l));
                let r = ui.add(egui::TextEdit::singleline(v).id_salt(("info-prop", l))).labelled_by(label.id);
                if r.changed() {
                    edited = Some(l);
                }
                if r.has_focus() {
                    focused = Some(l);
                }
                ui.end_row();
            }
        });
        if let Some(field) = edited {
            if app.info_editing == Some((field, app.session.rev())) {
                app.session.join_next_undo();
            }
            let ok = app.run("file.properties", json!({"title": props.title, "subject": props.subject, "author": props.creator, "keywords": props.keywords, "category": props.category})).is_ok();
            app.info_editing = ok.then(|| (field, app.session.rev()));
        }
        if focused != app.info_editing.map(|(f, _)| f) {
            app.info_editing = None;
        }
        // Protect Document › Encrypt with Password (#55).
        ui.add_space(18.0);
        ui.label(egui::RichText::new(tl!("Protect Document")).font(semibold(15.0)));
        let encrypted = info.get("encrypted").and_then(|v| v.as_bool()).unwrap_or(false);
        ui.label(if encrypted {
            tl!("A password is required to open this document.")
        } else {
            tl!("Anyone can open this document. Encrypt it with a password to keep it private.")
        });
        ui.horizontal(|ui| {
            let label = if encrypted { tl!("Change Password…") } else { tl!("Encrypt with Password…") };
            if ui.button(egui::RichText::new(label).font(medium(13.5))).clicked() {
                let _ = app.run("file.encrypt", json!({}));
            }
            if encrypted && ui.button(egui::RichText::new(tl!("Remove Password")).font(medium(13.5))).clicked() {
                let _ = app.run("file.encrypt", json!({"password": null}));
            }
        });
        let ui = &mut cols[1];
        ui.label(egui::RichText::new(tl!("Statistics")).font(semibold(15.0)));
        for (l, k) in [("Pages", "pages"), ("Words", "words"), ("Paragraphs", "paragraphs"), ("Sections", "sections"), ("Comments", "comments")] {
            ui.label(format!("{}: {}", tl!(l), info.get(k).map(|v| v.to_string()).unwrap_or_default()));
        }
        let path = info.get("path").and_then(|v| v.as_str()).unwrap_or(tl!("Not saved yet"));
        ui.label(crate::i18n::fmt(tl!("Location: {path}"), &[("path", path)]));
    });
}

fn export_page(app: &mut WordApp, ui: &mut Ui) {
    if app.ui.backstage_page == "print" {
        heading(ui, "Print");
        // Print Markup (a print setting, as in Word): the comments beside each page.
        let mut markup = app.session.prefs.print_markup;
        if ui
            .checkbox(&mut markup, tl!("Print Markup"))
            .on_hover_text(tl!(
                "Comments beside each page and tracked changes in PDFs, prints and images; a comment list at the end of web pages, Markdown and text."
            ))
            .changed()
        {
            let _ = app.run("file.printMarkup", json!({"value": markup}));
        }
        ui.add_space(12.0);
        crate::print_page::show(app, ui);
        return;
    }
    heading(ui, "Export");
    ui.label(tl!("Save a copy in another format. Printing goes through a PDF you can print from any viewer."));
    ui.add_space(12.0);
    crate::print_page::export_formats(app, ui);
}

#[cfg(test)]
mod tests {
    use egui_kittest::kittest::Queryable;
    use serde_json::json;

    use super::matching_templates;
    use crate::{Services, WordApp};

    #[cfg(not(target_arch = "wasm32"))]
    fn recovery_app() -> (WordApp, std::path::PathBuf) {
        let root = std::env::temp_dir().join(format!(
            "wordcraft-recovery-ui-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        let id = "0123456789abcdef0123456789abcdef";
        let dir = root.join(id);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("snapshot-00000000000000000001.json"),
            serde_json::to_vec(&json!({
                "version": 1, "generation": 1, "info": {"id": id, "name": "Synthetic draft", "savedAt": "2026-10-11T10:42:00Z"},
                "document": wordcraft_doc::Document::from_text("Recovered synthetic work")
            }))
            .unwrap(),
        )
        .unwrap();
        let mut app = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::new()), Services::default());
        app.session.enable_recovery(root.clone(), std::sync::Arc::new(|| {})).unwrap();
        (app, root)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn recovery_harness(app: WordApp) -> egui_kittest::Harness<'static, WordApp> {
        egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            app,
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn settle_recovery(h: &mut egui_kittest::Harness<'_, WordApp>) {
        let end = std::time::Instant::now() + std::time::Duration::from_secs(5);
        loop {
            h.run_steps(2);
            if h.state().session.recovery_ready() && !h.state().session.recovery_busy() {
                break;
            }
            assert!(std::time::Instant::now() < end);
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
        // Worker completion is a state boundary; egui needs subsequent frames to finish
        // font installation/layout and expose the resulting buttons to AccessKit.
        h.run_steps(4);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn recovery_page_keeps_dismissed_copies_and_confirms_discard() {
        let (app, root) = recovery_app();
        let mut h = recovery_harness(app);
        settle_recovery(&mut h);
        assert!(h.state().ui.backstage && h.state().ui.backstage_page == "recover");
        h.get_by_label("Continue without restoring").click();
        h.run_steps(4);
        assert!(!h.state().ui.backstage);
        assert_eq!(h.state().session.recovery_documents().len(), 1);
        h.state_mut().run("file.recover", json!({})).unwrap();
        h.run_steps(4);
        h.get_by_label("Discard…").click();
        h.run_steps(4);
        assert!(matches!(h.state().dialog, Some(crate::dialogs::Dialog::DiscardRecovery { .. })));
        h.get_by_label("Cancel").click();
        h.run_steps(4);
        assert_eq!(h.state().session.recovery_documents().len(), 1);
        h.get_by_label("Discard…").click();
        h.run_steps(4);
        h.get_by_label("Discard").click();
        settle_recovery(&mut h);
        assert!(h.state().session.recovery_documents().is_empty());
        assert!(h.query_by_label("No recovery copies.").is_some());
        drop(h);
        let _ = std::fs::remove_dir_all(root);
    }

    /// NexusOffice: the Recover page is drawn on black, so its buttons are dark too and their
    /// white labels read (in the light theme they were white on white).
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn recovery_page_buttons_are_readable() {
        let (app, root) = recovery_app();
        let mut h = recovery_harness(app);
        settle_recovery(&mut h);
        let r = h.get_by_label("Continue without restoring").rect();
        let img = h.render().unwrap();
        let ppp = h.ctx.pixels_per_point();
        // Just inside the button's corner: its background, not its label.
        let px = img.get_pixel(((r.min.x + 4.0) * ppp) as u32, ((r.center().y) * ppp) as u32).0;
        let lum = (u32::from(px[0]) * 3 + u32::from(px[1]) * 6 + u32::from(px[2])) / 10;
        assert!(lum < 110, "button background {px:?} is too light for white text");
        drop(h);
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn recovery_restore_uses_save_changes_and_resets_autosave_permission() {
        let (app, root) = recovery_app();
        let mut h = recovery_harness(app);
        settle_recovery(&mut h);
        h.state_mut().execute("text.insert", json!({"text": "Current unsaved work"})).unwrap();
        h.state_mut().autosave_path = Some(root.join("must-not-write.docx"));
        h.state_mut().run("file.recover", json!({})).unwrap();
        h.run_steps(4);
        assert_eq!(h.state().session.recovery_documents().len(), 1);
        h.get_by_label("Restore").click();
        h.run_steps(4);
        assert!(matches!(h.state().dialog, Some(crate::dialogs::Dialog::SaveChanges { .. })));
        h.get_by_label("Cancel").click();
        h.run_steps(4);
        assert_eq!(h.state().session.doc.plain_text(wordcraft_doc::StoryRef::Body), "Current unsaved work");
        h.get_by_label("Restore").click();
        h.run_steps(4);
        h.get_by_label("Don't Save").click();
        settle_recovery(&mut h);
        assert_eq!(h.state().session.doc.plain_text(wordcraft_doc::StoryRef::Body), "Recovered synthetic work");
        assert!(h.state().session.dirty && h.state().session.path.is_none());
        assert!(h.state().autosave_path.is_none());
        assert!(!h.state().ui.backstage);
        assert!(!root.join("must-not-write.docx").exists());
        drop(h);
        let _ = std::fs::remove_dir_all(root);
    }

    /// A profile folder holding one recovery copy per `(name, text)`, like a crashed earlier
    /// session's, and a window with recovery on in the WPS layout (embedded, as in the
    /// NexusOffice window) in `lang`.
    #[cfg(not(target_arch = "wasm32"))]
    fn wps_recovery(lang: &str, size: (f32, f32), drafts: &[(&str, &str)]) -> (egui_kittest::Harness<'static, WordApp>, std::path::PathBuf) {
        let root = std::env::temp_dir().join(format!(
            "wordcraft-recovery-wps-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        for (i, (name, text)) in drafts.iter().enumerate() {
            let id = format!("{i:032x}");
            let dir = root.join(&id);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(
                dir.join("snapshot-00000000000000000001.json"),
                serde_json::to_vec(&json!({
                    "version": 1, "generation": 1, "info": {"id": id, "name": name, "savedAt": format!("2026-10-11T10:4{i}:00Z")},
                    "document": wordcraft_doc::Document::from_text(text)
                }))
                .unwrap(),
            )
            .unwrap();
        }
        let mut app = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::new()), Services::default());
        app.wps = true;
        app.embedded = true;
        app.ui.language = lang.into();
        app.session.enable_recovery(root.clone(), std::sync::Arc::new(|| {})).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(size.0, size.1)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            app,
        );
        settle_recovery(&mut h);
        (h, root)
    }

    /// Perceived brightness (0–255) of a pixel at `p` (points) of `img`.
    fn brightness(img: &image::RgbaImage, ppp: f32, p: egui::Pos2) -> u32 {
        let px = img.get_pixel((p.x * ppp) as u32, (p.y * ppp) as u32).0;
        (u32::from(px[0]) * 3 + u32::from(px[1]) * 6 + u32::from(px[2])) / 10
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Every text drawn in the last frame with where it was drawn and whether egui shortened it.
    fn drawn_texts(h: &egui_kittest::Harness<'_, WordApp>) -> Vec<(String, egui::Rect, bool)> {
        fn walk(shape: &egui::Shape, out: &mut Vec<(String, egui::Rect, bool)>) {
            match shape {
                egui::Shape::Text(t) => out.push((t.galley.text().to_string(), t.galley.rect.translate(t.pos.to_vec2()), t.galley.elided)),
                egui::Shape::Vec(v) => v.iter().for_each(|s| walk(s, out)),
                _ => {}
            }
        }
        let mut out = Vec::new();
        h.output().shapes.iter().for_each(|c| walk(&c.shape, &mut out));
        out
    }

    /// NexusOffice (Windows test, 2026-10-24): in the WPS layout File opened a Backstage in
    /// upstream's colours (and the Recover page on black). Every page now keeps the window's
    /// chrome: the side menu on the light grey of the title bar, the page white, the open page
    /// tinted, a labelled Back button; with the dark theme, the dark chrome. Upstream's layout
    /// keeps upstream's look.
    #[test]
    fn wps_backstage_uses_the_light_chrome_on_every_page() {
        for (wps, dark) in [(true, false), (true, true), (false, false)] {
            let mut a = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::from_text("Synthetic text")), Services::default());
            a.wps = wps;
            a.embedded = true;
            if dark {
                a.run("ui.theme", json!({"value": "dark"})).unwrap();
            }
            let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
                |ui, app: &mut WordApp| {
                    let ctx = ui.ctx().clone();
                    app.logic(&ctx);
                    app.ui(ui);
                },
                a,
            );
            h.run_steps(4);
            for page in ["home", "new", "open", "info", "recover", "saveAs", "print", "export", "options"] {
                h.state_mut().run("ui.backstage", json!({"value": true, "page": page})).unwrap();
                h.run_steps(4);
                let img = h.render().unwrap();
                let ppp = h.ctx.pixels_per_point();
                // Below the side menu's last page, and the page's bottom-right margin.
                let nav = brightness(&img, ppp, egui::pos2(100.0, 700.0));
                let content = brightness(&img, ppp, egui::pos2(1420.0, 880.0));
                match (wps, dark) {
                    (true, false) => {
                        assert!((215..250).contains(&nav), "{page}: side menu {nav} is the chrome's light grey");
                        assert!(content > 245, "{page}: page {content} is white");
                    }
                    (true, true) => assert!(nav < 60 && content < 70, "{page}: dark chrome, menu {nav} page {content}"),
                    _ => assert!(nav < 130, "{page}: upstream's side menu {nav} in the app colour or black"),
                }
            }
            if !wps {
                assert!(h.query_by_label("Back").is_none(), "upstream's back arrow has no label");
                continue;
            }
            // The open page's row is tinted, unlike the rest of the menu.
            h.state_mut().run("ui.backstage", json!({"value": true, "page": "info"})).unwrap();
            h.run_steps(4);
            // The side menu's Info, not the page's heading.
            let row = h.get_all_by_label("Info").map(|n| n.rect()).min_by(|a, b| a.min.x.total_cmp(&b.min.x)).unwrap();
            let img = h.render().unwrap();
            let ppp = h.ctx.pixels_per_point();
            let open = img.get_pixel(((row.max.x - 14.0) * ppp) as u32, (row.center().y * ppp) as u32).0;
            let menu = img.get_pixel((100.0 * ppp) as u32, (700.0 * ppp) as u32).0;
            assert_ne!(open, menu, "the open page stands out");
            h.get_by_label("Back").click();
            h.run_steps(4);
            assert!(!h.state().ui.backstage, "Back returns to the document");
        }
    }

    /// NexusOffice: the Recover page in Vietnamese names a copy without a file "Bản nháp chưa
    /// đặt tên" (it said "Untitled draft"), and its buttons show their whole labels, in Vietnamese
    /// and English, in a wide and a narrow window; in the WPS layout they are light.
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn wps_recover_page_is_translated_and_its_buttons_are_whole() {
        for lang in ["vi", "en"] {
            for width in [1440.0, 1024.0] {
                let (mut h, root) = wps_recovery(lang, (width, 900.0), &[("Untitled draft", "Synthetic work")]);
                assert!(h.state().ui.backstage && h.state().ui.backstage_page == "recover", "start-up opens Recover");
                h.run_steps(4);
                if lang == "vi" {
                    assert!(h.query_by_label("Bản nháp chưa đặt tên").is_some(), "the copy's name is translated");
                    assert!(h.query_by_label("Untitled draft").is_none());
                    assert!(h.query_by_label("Khôi phục tài liệu").is_some());
                }
                let l = crate::i18n::Lang::from_code(lang).unwrap();
                let texts = drawn_texts(&h);
                for label in ["Restore", "Discard…", "Continue without restoring"] {
                    let label = crate::i18n::tr(l, label);
                    let button = h.get_all_by_label(label).map(|n| n.rect()).find(|r| r.min.x > 210.0).unwrap();
                    let drawn = texts.iter().find(|(t, r, _)| t == label && button.expand(0.5).contains_rect(*r));
                    let (_, r, elided) = drawn.unwrap_or_else(|| panic!("{lang} {width}: {label:?} is not drawn whole inside its button {button:?}"));
                    assert!(!elided, "{lang} {width}: {label:?} is shortened");
                    assert!(r.width() + 8.0 <= button.width(), "{lang} {width}: {label:?} has padding");
                }
                if width == 1440.0 {
                    let r = h.get_by_label(crate::i18n::tr(l, "Continue without restoring")).rect();
                    let img = h.render().unwrap();
                    let lum = brightness(&img, h.ctx.pixels_per_point(), egui::pos2(r.min.x + 4.0, r.center().y));
                    assert!(lum > 180, "{lang}: the button {lum} is light on the light page");
                }
                drop(h);
                let _ = std::fs::remove_dir_all(root);
            }
        }
    }

    /// NexusOffice (Windows test): the Recover page listed this window's own copy of the open
    /// document. A restored copy becomes the window's own (its later autosaves go there), so it
    /// leaves the list; copies left by other sessions stay.
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn recover_page_leaves_out_the_windows_own_copy() {
        let (mut h, root) = wps_recovery("en", (1440.0, 900.0), &[("First synthetic draft", "First"), ("Second synthetic draft", "Second")]);
        assert!(h.query_by_label("First synthetic draft").is_some() && h.query_by_label("Second synthetic draft").is_some());
        let id = h.state().session.recovery_documents().into_iter().find(|d| d.name == "First synthetic draft").unwrap().id;
        h.state_mut().run("file.recover", json!({"restore": id})).unwrap();
        settle_recovery(&mut h);
        assert_eq!(h.state().session.doc.plain_text(wordcraft_doc::StoryRef::Body), "First");
        assert_eq!(h.state().session.recovery_current(), Some(id.as_str()), "the restored copy is the window's own");
        h.state_mut().run("file.recover", json!({})).unwrap();
        h.run_steps(4);
        assert_eq!(h.state().ui.backstage_page, "recover");
        assert!(h.query_by_label("First synthetic draft").is_none(), "the window's own copy isn't offered");
        assert!(h.query_by_label("Second synthetic draft").is_some(), "another session's copy is");
        drop(h);
        let _ = std::fs::remove_dir_all(root);
    }

    /// Start-up opens the Recover page once when an earlier session left copies (upstream's
    /// choice). File afterwards opens Info, not that page again; Recover stays one click away.
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn file_opens_info_after_the_start_up_recover_page() {
        let (mut h, root) = wps_recovery("en", (1440.0, 900.0), &[("Synthetic draft", "Synthetic work")]);
        assert!(h.state().ui.backstage && h.state().ui.backstage_page == "recover");
        h.get_by_label("Continue without restoring").click();
        h.run_steps(4);
        assert!(!h.state().ui.backstage);
        h.state_mut().run("ui.tab", json!({"tab": "File"})).unwrap();
        h.run_steps(4);
        assert!(h.state().ui.backstage);
        assert_eq!(h.state().ui.backstage_page, "info", "File opens Info");
        h.get_by_label("Recover").click();
        h.run_steps(4);
        assert_eq!(h.state().ui.backstage_page, "recover");
        // Leaving with Back and coming back with File: Info again.
        h.get_by_label("Back").click();
        h.run_steps(4);
        h.state_mut().run("ui.tab", json!({"tab": "File"})).unwrap();
        assert_eq!(h.state().ui.backstage_page, "info");
        drop(h);
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn template_search_matches_labels_case_insensitively() {
        let ids = |q: &str| matching_templates(q).into_iter().map(|(_, id)| id).collect::<Vec<_>>();
        assert_eq!(ids(""), ["blank", "sample", "letter", "resume", "report"]);
        assert_eq!(ids("  LETTER "), ["letter"]);
        assert_eq!(ids("re"), ["resume", "report"]);
        assert_eq!(ids("résumé"), ["resume"]);
        assert!(ids("no such template").is_empty());
    }

    /// Home and New are different pages (#141): Home has the short template row, the link to New
    /// and Recent; New has every template and no Recent list.
    #[test]
    fn home_and_new_show_different_sections() {
        let mut a = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::new()), Services::default());
        a.run("ui.backstage", json!({"value": true, "page": "home"})).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(4);
        assert!(h.query_by_label("Recent").is_some(), "Home lists recent documents");
        assert!(h.query_by_label("Letter").is_some(), "Home shows the first templates");
        assert!(h.query_by_label("Report").is_none(), "Home shows only a short row of templates");
        h.get_by_label("More templates →").click();
        h.run_steps(4);
        assert_eq!(h.state().ui.backstage_page, "new", "the link opens New");
        assert!(h.query_by_label("Report").is_some(), "New shows every template");
        assert!(h.query_by_label("Recent").is_none(), "New has no Recent list");
    }

    /// #260: File › Info's Title and Author fields dropped every keystroke (they edited a copy
    /// re-read each frame and only wrote it back when the field lost focus), so a save had no
    /// title or author. Typing now lands in the document as it happens, one undo step per field,
    /// and the browser's Save downloads it with the save stamps advanced (#262).
    #[test]
    fn info_fields_keep_typed_properties_through_a_web_save() {
        use egui::accesskit::Role;
        let got: std::rc::Rc<std::cell::RefCell<Vec<(String, Vec<u8>)>>> = Default::default();
        let sink = got.clone();
        let services = Services {
            download: Some(Box::new(move |n: &str, b: &[u8]| {
                sink.borrow_mut().push((n.to_string(), b.to_vec()));
                Ok(())
            })),
            ..Default::default()
        };
        let mut a = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::from_text("XYZ")), services);
        a.session.author = "GAMMA".into();
        a.run("ui.backstage", json!({"value": true, "page": "info"})).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(4);
        for (field, text) in [("Title", "ALPHA"), ("Author", "BETA")] {
            h.get_by_role_and_label(Role::TextInput, field).click();
            h.run_steps(2);
            for c in text.chars() {
                h.get_by_role_and_label(Role::TextInput, field).type_text(&c.to_string());
                h.run_steps(2);
            }
            assert_eq!(h.get_by_role_and_label(Role::TextInput, field).value().as_deref(), Some(text), "{field} shows what was typed");
        }
        let core = h.state().session.doc.core.clone();
        assert_eq!((core.title.as_str(), core.creator.as_str()), ("ALPHA", "BETA"));
        assert!(h.state().session.dirty, "the property edits are unsaved changes");

        // Save (web: a download) writes them into docProps/core.xml, with fresh save stamps.
        let a = h.state_mut();
        let created = a.session.doc.core.created.clone();
        a.run("file.save", json!({})).unwrap();
        a.run("file.save", json!({})).unwrap();
        let (name, bytes) = got.borrow_mut().pop().unwrap();
        let back = wordcraft_engine::io::open_bytes(&name, &bytes).unwrap();
        assert_eq!((back.core.title.as_str(), back.core.creator.as_str()), ("ALPHA", "BETA"));
        assert_eq!(back.core.last_modified_by, "GAMMA");
        assert_eq!(back.core.revision, core.revision + 2, "each save advances the revision");
        assert!(created.is_empty() && !back.core.created.is_empty(), "the first save sets the creation time");
        assert!(!back.core.modified.is_empty());
        assert!(!a.session.dirty);

        // Each field's typing is one undo step.
        a.session.run("edit.undo", &json!({})).unwrap();
        assert_eq!((a.session.doc.core.title.as_str(), a.session.doc.core.creator.as_str()), ("ALPHA", ""));
        a.session.run("edit.undo", &json!({})).unwrap();
        assert_eq!(a.session.doc.core.title, "");
    }

    #[test]
    fn save_as_page_shows_browse_and_no_open_list() {
        let mut a = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::new()), Services::default());
        a.run("ui.backstage", json!({"value": true, "page": "saveAs"})).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(4);
        assert!(h.query_by_label("Browse…").is_some(), "Save As has Browse button");
        assert!(h.query_by_label("Recent").is_none(), "Save As has no list that opens documents");
    }

    #[test]
    fn backstage_save_navigates_to_save_as_when_unnamed() {
        let mut a = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::new()), Services::default());
        a.run("ui.backstage", json!({"value": true, "page": "home"})).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(4);
        h.get_by_label("Save").click();
        h.run_steps(4);
        assert_eq!(h.state().ui.backstage_page, "saveAs", "unnamed Save opens Save As page");
        assert!(h.state().ui.backstage, "backstage remains open");
    }

    /// Save on a document without a file shows the Save As page only: no native dialog as well.
    #[test]
    fn backstage_save_when_unnamed_opens_no_file_dialog() {
        let shown = std::rc::Rc::new(std::cell::Cell::new(0));
        let count = shown.clone();
        let services = Services {
            file_dialog: Some(Box::new(move |_| {
                count.set(count.get() + 1);
                std::sync::mpsc::channel::<crate::FileDialogAnswer>().1
            })),
            ..Default::default()
        };
        let mut a = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::new()), services);
        a.run("ui.backstage", json!({"value": true, "page": "home"})).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(4);
        h.get_by_label("Save").click();
        h.run_steps(4);
        assert_eq!(h.state().ui.backstage_page, "saveAs");
        assert_eq!(shown.get(), 0, "no native Save As dialog");
        assert!(!h.state().file_dialog_open());
    }

    #[test]
    fn backstage_save_closes_when_saved() {
        let got: std::rc::Rc<std::cell::RefCell<Vec<(String, Vec<u8>)>>> = Default::default();
        let sink = got.clone();
        let services = Services {
            download: Some(Box::new(move |n: &str, b: &[u8]| {
                sink.borrow_mut().push((n.to_string(), b.to_vec()));
                Ok(())
            })),
            ..Default::default()
        };
        let mut a = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::new()), services);
        a.run("ui.backstage", json!({"value": true, "page": "home"})).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(4);
        h.get_by_label("Save").click();
        h.run_steps(4);
        assert!(!h.state().ui.backstage, "successful save closes backstage");
    }

    /// #1124: the side menu had no Close. It sits after Export and closes the document, asking
    /// first when there are unsaved changes; Cancel leaves the Backstage as it was.
    #[test]
    fn backstage_close_asks_then_closes() {
        let mut a = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::new()), Services::default());
        a.run("text.insert", json!({"text": "Draft"})).unwrap();
        a.run("ui.backstage", json!({"value": true, "page": "info"})).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(4);
        let export = h.get_by_label("Export").rect().min.y;
        assert!(h.get_by_label("Close").rect().min.y > export, "Close comes after Export");
        h.get_by_label("Close").click();
        h.run_steps(4);
        assert!(matches!(h.state().dialog, Some(crate::dialogs::Dialog::SaveChanges { .. })), "unsaved changes ask first");
        assert!(!h.state().quit_requested);

        h.state_mut().run("ui.saveChanges", json!({"answer": "cancel"})).unwrap();
        h.run_steps(4);
        assert!(!h.state().quit_requested, "Cancel keeps the document open");
        assert!(h.state().ui.backstage && h.state().ui.backstage_page == "info", "Cancel leaves the Backstage as it was");

        h.get_by_label("Close").click();
        h.run_steps(4);
        h.state_mut().run("ui.saveChanges", json!({"answer": "dontSave"})).unwrap();
        assert!(h.state().quit_requested, "Don't Save closes");
    }

    /// A document without unsaved changes closes without a prompt.
    #[test]
    fn backstage_close_on_a_clean_document_closes_at_once() {
        let mut a = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::new()), Services::default());
        a.run("ui.backstage", json!({"value": true, "page": "home"})).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(4);
        h.get_by_label("Close").click();
        h.run_steps(1);
        assert!(h.state().dialog.is_none(), "no prompt");
        assert!(h.state().quit_requested);
    }

    #[test]
    fn backstage_save_as_sidebar_opens_save_as_page() {
        let mut a = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::new()), Services::default());
        a.run("ui.backstage", json!({"value": true, "page": "home"})).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(4);
        h.get_by_label("Save As").click();
        h.run_steps(4);
        assert_eq!(h.state().ui.backstage_page, "saveAs");
    }
}
