//! The NexusOffice layout (Henry, 2026-10-23): a menu bar and one row of tools in place of the
//! ribbon. The menus are built from the command registry (every command, grouped by where it
//! lives), so nothing the ribbon offers is lost; the tool row holds the everyday formatting
//! commands. The ribbon's code stays as upstream wrote it and draws when [`WordApp::toolbar`] is
//! off (upstream's own tests and builds).

use egui::{Align2, Rect, Sense, Stroke, Ui, pos2, vec2};
use serde_json::{Value, json};

use crate::WordApp;
use crate::theme::{Tokens, medium, regular};
use crate::widgets::{combo, font_combo, small, split};

/// Height of the menu bar and of the tool row.
pub const MENU_H: f32 = 28.0;
pub const TOOLS_H: f32 = 36.0;

/// One menu of the menu bar: a ribbon tab's name and its commands, grouped by the group they
/// sit in on the ribbon (in registry order), as `(group, [command ids])`.
#[derive(Debug, Clone, PartialEq)]
pub struct Menu {
    pub tab: String,
    pub groups: Vec<(String, Vec<&'static str>)>,
}

/// The menus for `tabs` (ribbon tab names, File excluded): every command whose location is the
/// tab or `Tab › Group…` goes into that tab's menu, under its group. Tabs with no command are left
/// out.
pub fn menus(commands: &[wordcraft_engine::CommandSpec], tabs: &[String]) -> Vec<Menu> {
    let mut out = Vec::new();
    for tab in tabs.iter().filter(|t| t.as_str() != "File") {
        let mut groups: Vec<(String, Vec<&'static str>)> = Vec::new();
        for c in commands {
            let mut parts = c.location.split(" › ");
            if parts.next() != Some(tab.as_str()) {
                continue;
            }
            let group = parts.next().unwrap_or("").to_string();
            match groups.iter_mut().find(|(g, _)| *g == group) {
                Some((_, ids)) => ids.push(c.id),
                None => groups.push((group, vec![c.id])),
            }
        }
        if !groups.is_empty() {
            out.push(Menu { tab: tab.clone(), groups });
        }
    }
    out
}

/// The tabs whose menus the bar shows: the user's ribbon tabs (Customize Ribbon order, hidden
/// ones left out, custom tabs too) and the contextual ones (Table Layout while in a table…).
fn tab_names(app: &WordApp) -> Vec<String> {
    let mut tabs: Vec<String> = app.session.ribbon.tabs().iter().filter(|e| !e.hidden && !e.custom).map(|e| e.name.clone()).collect();
    for ct in crate::ribbon::contextual_tabs(&app.session) {
        if !tabs.iter().any(|t| t == ct) {
            tabs.push(ct.to_string());
        }
    }
    tabs
}

/// The menu bar and the tool row.
pub fn show(app: &mut WordApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    egui::Panel::top("nexus_menu")
        .exact_size(MENU_H)
        .frame(egui::Frame::NONE.fill(t.tab_strip).inner_margin(egui::Margin { left: 8, right: 10, top: 0, bottom: 0 }))
        .show(ui, |ui| {
            menu_bar(app, ui);
            brand_line(ui);
        });
    if app.ui.ribbon_collapsed {
        return;
    }
    egui::Panel::top("nexus_tools")
        .exact_size(TOOLS_H)
        .frame(
            egui::Frame::NONE.fill(t.ribbon).inner_margin(egui::Margin { left: 8, right: 8, top: 0, bottom: 0 }).stroke(Stroke::new(1.0, t.border)),
        )
        .show(ui, |ui| {
            // No document open (#1129): the tools are greyed out.
            if !app.has_document() {
                ui.disable();
            }
            egui::ScrollArea::horizontal().scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden).show(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.spacing_mut().item_spacing = vec2(1.0, 0.0);
                    tools(app, ui);
                });
            });
        });
}

fn menu_bar(app: &mut WordApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_centered(|ui| {
        ui.spacing_mut().item_spacing = vec2(2.0, 0.0);
        // File opens the File page (backstage), as the ribbon's File tab does.
        if menu_title(ui, tl!("File"), false).clicked() {
            let _ = app.run("ui.tab", json!({"tab": "File"}));
        }
        let tabs = tab_names(app);
        let registry = app.session.registry.clone();
        let menus = menus(registry.all(), &tabs);
        let contextual = crate::ribbon::contextual_tabs(&app.session);
        for m in &menus {
            let ctx_tab = contextual.contains(&m.tab.as_str());
            let resp = menu_title(ui, tl!(m.tab.as_str()), ctx_tab);
            egui::Popup::menu(&resp).show(|ui| {
                ui.set_min_width(220.0);
                menu_body(app, ui, m);
            });
        }
        // Menus the host program adds (the NexusOffice window's "Nexus AI"), after the app's own.
        let mut host = std::mem::take(&mut app.host_menus);
        for (title, contents) in &mut host {
            let resp = menu_title(ui, title, true);
            egui::Popup::menu(&resp).show(|ui| {
                ui.set_min_width(220.0);
                contents(ui);
            });
        }
        app.host_menus = host;
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if !app.has_document() {
                ui.disable();
            }
            let share = tl!("Share");
            let share_w = ui.ctx().fonts_mut(|f| f.layout_no_wrap(share.to_string(), medium(12.0), t.text).size().x);
            let (r, resp) = ui.allocate_exact_size(vec2((share_w + 30.0).max(64.0), 22.0), Sense::click());
            ui.painter().rect_filled(r, 4.0, if resp.hovered() { t.accent_text } else { t.accent });
            crate::icons::paint(ui.painter(), Rect::from_center_size(pos2(r.min.x + 12.0, r.center().y), vec2(13.0, 13.0)), "share", t.on_accent, t.on_accent);
            ui.painter().text(pos2(r.min.x + 23.0, r.center().y), Align2::LEFT_CENTER, share, medium(12.0), t.on_accent);
            if resp.on_hover_text(tl!("Export a copy to share (PDF, Word document)")).clicked() {
                let _ = app.run("ui.backstage", json!({"value": true, "page": "export"}));
            }
            ui.add_space(4.0);
            if crate::widgets::icon_text_button(ui, "comment", tl!("Comments"), regular(12.0)).clicked() {
                let _ = app.run("view.commentsPane", json!({}));
            }
        });
    });
}

/// The SonnyTech gradient (indigo → cyan → emerald), 2 pt under the menu bar.
fn brand_line(ui: &mut Ui) {
    let r = ui.max_rect();
    let line = Rect::from_min_max(pos2(r.min.x - 8.0, r.max.y - 2.0), pos2(r.max.x + 10.0, r.max.y));
    let [a, b, c] = crate::theme::BRAND_GRADIENT;
    let mid = line.center().x;
    let mut mesh = egui::Mesh::default();
    for (x0, x1, c0, c1) in [(line.min.x, mid, a, b), (mid, line.max.x, b, c)] {
        let i = mesh.vertices.len() as u32;
        mesh.colored_vertex(pos2(x0, line.min.y), c0);
        mesh.colored_vertex(pos2(x1, line.min.y), c1);
        mesh.colored_vertex(pos2(x1, line.max.y), c1);
        mesh.colored_vertex(pos2(x0, line.max.y), c0);
        mesh.add_triangle(i, i + 1, i + 2);
        mesh.add_triangle(i, i + 2, i + 3);
    }
    ui.painter().add(egui::Shape::mesh(mesh));
}

/// A menu bar title: text that highlights on hover; contextual menus (Table Layout…) in the
/// accent colour, as the ribbon draws contextual tabs.
fn menu_title(ui: &mut Ui, text: &str, contextual: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let w = ui.ctx().fonts_mut(|f| f.layout_no_wrap(text.to_string(), medium(12.5), t.text).size().x) + 16.0;
    let (r, resp) = ui.allocate_exact_size(vec2(w, MENU_H - 4.0), Sense::click());
    if resp.hovered() {
        ui.painter().rect_filled(r, 4.0, t.hover);
    }
    ui.painter().text(r.center(), Align2::CENTER_CENTER, text, medium(12.5), if contextual { t.accent_text } else { t.text });
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, text));
    resp
}

/// A menu's items: one submenu per ribbon group (a single group lists its commands directly).
fn menu_body(app: &mut WordApp, ui: &mut Ui, m: &Menu) {
    let single = m.groups.len() == 1;
    for (group, ids) in &m.groups {
        if single || group.is_empty() {
            items(app, ui, ids);
        } else {
            let label = crate::i18n::location(group);
            ui.menu_button(label, |ui| {
                ui.set_min_width(220.0);
                items(app, ui, ids);
            });
        }
    }
}

fn items(app: &mut WordApp, ui: &mut Ui, ids: &[&'static str]) {
    for id in ids {
        let Some(c) = app.session.registry.get(id) else { continue };
        let label = tl!(c.label).to_string();
        let on = crate::widgets::enabled(app, id);
        let sc = crate::widgets::shortcut_text(app, id);
        if ui.add_enabled(on, egui::Button::new(label).shortcut_text(sc)).clicked() {
            let _ = app.run(id, json!({}));
            ui.close();
        }
    }
}

/// A thin divider between tool clusters.
fn divider(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(4.0);
    let (r, _) = ui.allocate_exact_size(vec2(1.0, 20.0), Sense::hover());
    ui.painter().line_segment([r.center_top(), r.center_bottom()], Stroke::new(1.0, t.border));
    ui.add_space(4.0);
}

/// The tool row: clipboard, style, font, character formatting, paragraph, insert, find.
fn tools(app: &mut WordApp, ui: &mut Ui) {
    let st = app.session.run("format.state", &json!({})).unwrap_or_default();
    let flag = |k: &str| st.get(k).and_then(Value::as_bool).unwrap_or(false);

    small(ui, app, "paste", None, "Paste", "edit.paste", json!({}), false);
    small(ui, app, "painter", None, "Format Painter", "edit.formatPainter", json!({}), app.session.painter.is_some());
    divider(ui);

    // Style: the gallery's styles by name.
    let styles: Vec<(String, String)> = app.session.doc.styles.gallery().iter().map(|s| (s.id.clone(), crate::i18n::style_name(s))).collect();
    let current_id = st.get("style").and_then(Value::as_str).unwrap_or("Normal").to_string();
    let current = styles.iter().find(|(id, _)| *id == current_id).map(|(_, n)| n.clone()).unwrap_or(current_id);
    let names: Vec<String> = styles.iter().map(|(_, n)| n.clone()).collect();
    if let Some(n) = combo(ui, "nexus_style", 120.0, &current, &names, None)
        && let Some((id, _)) = styles.iter().find(|(_, name)| *name == n)
    {
        let _ = app.run("para.style", json!({"style": id}));
    }
    ui.add_space(4.0);
    let font = st.get("font").and_then(Value::as_str).unwrap_or("").to_string();
    let font = app.previews.font_shown(&font);
    let fams = app.previews.families();
    let prev = app.previews.font_preview_fn();
    if let Some(f) = font_combo(ui, "nexus_font", 140.0, &font, &fams, Some(&*prev)) {
        let _ = if f == crate::widgets::ADD_FONT {
            app.run("font.add", json!({}))
        } else if f == crate::widgets::LOAD_SYSTEM_FONTS {
            app.run("font.loadSystem", json!({}))
        } else {
            let f = app.previews.font_stored(&f);
            app.run("format.font", json!({"name": f}))
        };
    }
    let size = st.get("size").and_then(Value::as_f64).map(|s| if s.fract() == 0.0 { format!("{s:.0}") } else { format!("{s}") }).unwrap_or_default();
    let sizes: Vec<String> = wordcraft_engine::cmd::format::SIZES.iter().map(|s| if s.fract() == 0.0 { format!("{s:.0}") } else { s.to_string() }).collect();
    if let Some(v) = combo(ui, "nexus_size", 48.0, &size, &sizes, None)
        && let Ok(x) = v.trim().parse::<f64>()
        && x.is_finite()
    {
        let _ = app.run("format.size", json!({"size": x}));
    }
    divider(ui);

    small(ui, app, "bold", None, "Bold", "format.bold", json!({}), flag("bold"));
    small(ui, app, "italic", None, "Italic", "format.italic", json!({}), flag("italic"));
    small(ui, app, "underline", None, "Underline", "format.underline", json!({}), flag("underline"));
    small(ui, app, "strike", None, "Strikethrough", "format.strikethrough", json!({}), flag("strike"));
    let fc = app.canvas.last_font_color.clone();
    let sw = wordcraft_doc::Rgb::parse(&fc).map(crate::theme::c32);
    split(ui, app, "fontcolor", "Font Color", "format.color", json!({"color": fc}), false, sw, |ui, app| {
        if ui.button(tl!("Automatic")).clicked() {
            let _ = app.run("format.color", json!({"color": "auto"}));
            ui.close();
        }
        let theme = app.session.doc.settings.theme_colors.clone();
        crate::colors::menu(ui, app, &theme, crate::colors::Target::cmd("format.color").remember(crate::colors::Remember::FontColor));
    });
    let hl = app.canvas.last_highlight.clone();
    split(ui, app, "highlight", "Text Highlight Color", "format.highlight", json!({"color": hl}), false, None, |ui, app| {
        for h in wordcraft_doc::props::Highlight::ALL.iter().skip(1) {
            if ui.button(h.name()).clicked() {
                app.canvas.last_highlight = h.ooxml().to_string();
                let _ = app.run("format.highlight", json!({"color": h.ooxml()}));
                ui.close();
            }
        }
        if ui.button(tl!("No Color")).clicked() {
            let _ = app.run("format.highlight", json!({"color": "none"}));
            ui.close();
        }
    });
    small(ui, app, "clear", None, "Clear All Formatting", "format.clear", json!({}), false);
    divider(ui);

    let rp = app.session.doc.para_at(&app.session.sel.focus).map(|p| app.session.doc.styles.resolve_para(&p.props));
    let align = rp.as_ref().map(|r| r.align.visual(r.bidi)).unwrap_or_default();
    let lists = wordcraft_engine::cmd::para::list_buttons(&app.session.doc, rp.as_ref().and_then(|r| r.numbering));
    use wordcraft_doc::Align as A;
    small(ui, app, "alignLeft", None, "Align Left", "para.alignLeft", json!({}), align == A::Left);
    small(ui, app, "alignCenter", None, "Center", "para.alignCenter", json!({}), align == A::Center);
    small(ui, app, "alignRight", None, "Align Right", "para.alignRight", json!({}), align == A::Right);
    small(ui, app, "justify", None, "Justify", "para.justify", json!({}), align == A::Justify);
    divider(ui);
    small(ui, app, "bullets", None, "Bullets", "para.bullets", json!({}), lists.bullets);
    small(ui, app, "numbering", None, "Numbering", "para.numbering", json!({}), lists.numbering);
    small(ui, app, "outdent", None, "Decrease Indent", "para.outdent", json!({}), false);
    small(ui, app, "indent", None, "Increase Indent", "para.indent", json!({}), false);
    crate::widgets::menu_button(ui, app, "lineSpacing", None, "Line and Paragraph Spacing", false, "para.lineSpacing", |ui, app| {
        for v in [1.0, 1.15, 1.5, 2.0, 2.5, 3.0] {
            if ui.button(format!("{v}")).clicked() {
                let _ = app.run("para.lineSpacing", json!({"value": v}));
                ui.close();
            }
        }
    });
    divider(ui);
    small(ui, app, "link", None, "Link", "ui.dialog", json!({"name": "link"}), false);
    small(ui, app, "newComment", None, "Comment", "review.newComment", json!({}), false);
    crate::widgets::menu_button(ui, app, "table", None, "Add a Table", false, "insert.table", |ui, app| {
        crate::dialogs::table_grid_picker(ui, app);
    });
    small(ui, app, "picture", None, "Pictures", "insert.picture", json!({}), false);
    divider(ui);
    small(ui, app, "find", None, "Find", "edit.find", json!({}), false);
    small(ui, app, "replace", None, "Replace", "edit.replace", json!({}), false);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every command of a shown tab is in that tab's menu, under its ribbon group, so leaving the
    /// ribbon loses no command.
    #[test]
    fn menus_hold_every_command_of_their_tab() {
        let s = wordcraft_engine::Session::new(wordcraft_doc::Document::new());
        let tabs: Vec<String> = ["File", "Home", "Insert", "Review", "No Such Tab"].iter().map(|t| t.to_string()).collect();
        let ms = menus(s.registry.all(), &tabs);
        assert_eq!(ms.iter().map(|m| m.tab.as_str()).collect::<Vec<_>>(), ["Home", "Insert", "Review"], "File and empty tabs have no menu");
        for m in &ms {
            let listed: usize = m.groups.iter().map(|(_, ids)| ids.len()).sum();
            let expected =
                s.registry.all().iter().filter(|c| c.location == m.tab || c.location.starts_with(&format!("{} › ", m.tab))).count();
            assert_eq!(listed, expected, "{}", m.tab);
        }
        let home = &ms[0];
        assert!(home.groups.iter().any(|(g, ids)| g == "Font" && ids.contains(&"format.bold")), "{home:?}");
    }

    /// The menus are built from any registry slice, so an empty one gives no menu (no panic).
    #[test]
    fn no_commands_no_menus() {
        assert!(menus(&[], &["Home".to_string()]).is_empty());
    }
}
