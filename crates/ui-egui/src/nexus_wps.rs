//! NexusOffice layout in the style of WPS Office (its "Universal UI"; Henry's approved mockup,
//! 2026-10-24). Only the layout is borrowed: every icon is drawn in code (`icons.rs`) and the
//! colours are the app's own (`theme.rs`).
//!
//! There is no title bar: the NexusOffice window's tab strip (or the system's title bar in the
//! stand-alone app) names the document. The app draws two rows:
//!
//! - [`bar`] (36 pt): File (opens Backstage), the Quick Access Toolbar, the ribbon tabs left
//!   aligned; on the right the save state (with AutoSave in its menu), Search, Comments, Share and
//!   a "⋯" menu with what the title bar held besides (account, Editing/Reviewing, Customize).
//! - [`strip`] (56 pt): the current tab's groups as one flat row with no group titles, two rows of
//!   small buttons high (a large button is a small icon over one label line). A group's
//!   dialog launcher moves to its right-click menu, and the groups the window has no room for
//!   fold into a "…" menu at the end of the row instead of being cut off.
//!
//! The groups themselves are upstream's ([`crate::ribbon::tab_groups`]); [`group_begin`] and
//! [`group_end`], called by [`crate::widgets::group_titled`], decide which of them each row draws.

use egui::{Align2, Color32, Id, Rect, Response, Sense, Stroke, Ui, pos2, vec2};
use serde_json::json;

use crate::theme::{Tokens, regular, semibold};
use crate::{WordApp, icons};

/// Height of the top row (File, Quick Access, tabs, Share).
pub const BAR_H: f32 = 36.0;
/// Height of the command strip under it: two rows of small buttons, or a large button's 20 px
/// icon over its label ([`crate::widgets::COMPACT_H`] plus 3 pt above and below).
pub const STRIP_H: f32 = crate::widgets::COMPACT_H + 6.0;
/// Height of the top row's buttons and search box.
const CONTROL_H: f32 = 28.0;
/// Width kept at the end of the strip for the "…" button.
const OVERFLOW_W: f32 = 30.0;
/// Width of the » button that holds the tabs the top row has no room for.
const MORE_TABS_W: f32 = 26.0;
/// Below this window width the top row drops the word "Comments" (the icon stays) and narrows
/// the search box, so more tabs fit.
const NARROW_W: f32 = 1200.0;

/// The File (Tệp) button.
pub fn file_button_id() -> Id {
    Id::new("nexus_wps_file")
}
/// The strip's "…" button, shown when groups don't fit.
pub fn overflow_button_id() -> Id {
    Id::new("nexus_wps_overflow")
}
/// The save state ("Saved" / "Not saved" / "Saving…"), which opens AutoSave and Save.
pub fn save_state_id() -> Id {
    Id::new("nexus_wps_save_state")
}
/// The top row's "⋯" menu (account, Editing/Reviewing, Customize).
pub fn more_button_id() -> Id {
    Id::new("nexus_wps_more")
}
/// The search box.
pub fn search_id() -> Id {
    Id::new("nexus_wps_search")
}

/// Draw the layout's top panels: the top row always, the command strip when `ribbon` (not in
/// Focus). Reading mode keeps the top row and shows the reading controls under it.
pub fn show(app: &mut WordApp, ui: &mut Ui, ribbon: bool) {
    let t = Tokens::get(ui.ctx());
    let tabs = (!app.session.view.read_mode).then(|| crate::ribbon::sync_tabs(app));
    bar(app, ui, &t, tabs.as_ref().map_or(&[][..], |(tabs, _)| tabs.as_slice()));
    if !ribbon {
        return;
    }
    match tabs {
        None => crate::ribbon::reading_toolbar(app, ui, t),
        Some((_, user_tabs)) if !app.ui.ribbon_collapsed => strip(app, ui, &t, &user_tabs),
        Some(_) => {}
    }
}

/// The File button's fill: the accent, faint, over the row (a little stronger under the pointer).
pub(crate) fn file_fill(t: &Tokens, hovered: bool) -> Color32 {
    t.ribbon.lerp_to_gamma(t.accent, if hovered { 0.16 } else { 0.08 })
}

/// File's label on `fill`: the accent's ink, darkened toward the body text as far as 4.5:1 needs
/// (a light accent's own ink is close to the limit on white already).
pub(crate) fn file_ink(t: &Tokens, fill: Color32) -> Color32 {
    [0.0, 0.2, 0.4, 0.6, 0.8]
        .iter()
        .map(|k| t.accent_text.lerp_to_gamma(t.text, *k))
        .find(|ink| crate::theme::contrast_ratio(*ink, fill) >= 4.5)
        .unwrap_or(t.text)
}

/// The colour "Saved" is written in: green that keeps 4.5:1 on the row in either theme.
fn saved_color(dark: bool) -> Color32 {
    if dark { Color32::from_rgb(0x6E, 0xE7, 0xB7) } else { Color32::from_rgb(0x04, 0x78, 0x57) }
}

/// The top row.
fn bar(app: &mut WordApp, ui: &mut Ui, t: &Tokens, tabs: &[(String, bool)]) {
    // Stand-alone on macOS the window has no title bar: the row sits beside the traffic lights and
    // moves the window. Embedded, the NexusOffice window's own tab strip does that.
    let mac = app.integrated_titlebar && !app.embedded;
    let height = if mac { BAR_H.max(app.mac_titlebar.height) } else { BAR_H };
    let left = if mac { app.mac_titlebar.content_left.clamp(0.0, f32::from(i8::MAX)) as i8 } else { 12 };
    egui::Panel::top("wps_bar")
        .exact_size(height)
        .frame(egui::Frame::NONE.fill(t.ribbon).inner_margin(egui::Margin { left, right: 10, top: 0, bottom: 0 }))
        .show(ui, |ui| {
            let full = ui.max_rect();
            if mac {
                let drag = ui.interact(full, ui.id().with("wps_drag"), Sense::click_and_drag());
                if drag.drag_started() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                if drag.double_clicked() {
                    let max = ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false));
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!max));
                }
            }
            let narrow = ui.ctx().content_rect().width() < NARROW_W;
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing = vec2(2.0, 0.0);
                file_button(ui, app, t);
                ui.add_space(8.0);
                crate::chrome::quick_access(ui, app);
                ui.add_space(6.0);
                let (d, _) = ui.allocate_exact_size(vec2(1.0, 20.0), Sense::hover());
                ui.painter().rect_filled(d, 0.0, t.border);
                ui.add_space(8.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing = vec2(6.0, 0.0);
                    more_menu(ui, app, t);
                    ui.scope(|ui| {
                        // No document open (#1129): nothing to share or comment on.
                        if !app.has_document() {
                            ui.disable();
                        }
                        share_button(ui, app, t);
                        comments_button(ui, app, t, narrow);
                    });
                    search_box(ui, app, t, narrow);
                    if app.has_document() {
                        ui.add_space(6.0);
                        save_state(ui, app, t);
                    }
                    ui.add_space(12.0);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                        tab_row(ui, app, t, tabs, full.height());
                    });
                });
            });
            ui.painter().hline(full.x_range(), full.max.y - 0.5, Stroke::new(1.0, t.border.gamma_multiply(0.6)));
        });
}

/// File (Tệp): the three bars and the word on a faint accent, opening Backstage as File does.
fn file_button(ui: &mut Ui, app: &mut WordApp, t: &Tokens) -> Response {
    let label = tl!("File");
    let font = semibold(13.0);
    let tw = ui.painter().layout_no_wrap(label.to_string(), font.clone(), t.accent_text).size().x;
    let (r, _) = ui.allocate_exact_size(vec2(10.0 + 16.0 + 6.0 + tw + 12.0, CONTROL_H), Sense::hover());
    let resp = ui.interact(r, file_button_id(), Sense::click());
    let fill = file_fill(t, resp.hovered());
    let ink = file_ink(t, fill);
    ui.painter().rect_filled(r, 6.0, fill);
    icons::paint(ui.painter(), Rect::from_center_size(pos2(r.min.x + 18.0, r.center().y), vec2(16.0, 16.0)), "hamburger", ink, ink);
    ui.painter().text(pos2(r.min.x + 32.0, r.center().y), Align2::LEFT_CENTER, label, font, ink);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));
    if app.ui.keytips == crate::keytips::Phase::Tabs {
        crate::keytips::record_tab(r, "File", &app.ui.keytip_prefix, &mut app.keytip_rects);
    }
    if resp.clicked() {
        let _ = app.run("ui.tab", json!({"tab": "File"}));
    }
    resp
}

/// The ribbon tabs, left aligned: the active one in the accent, semibold, underlined. Tabs that
/// don't fit go into a » menu (File's own button stands for File).
fn tab_row(ui: &mut Ui, app: &mut WordApp, t: &Tokens, tabs: &[(String, bool)], height: f32) {
    let tabs: Vec<&(String, bool)> = tabs.iter().filter(|(n, custom)| *custom || n != "File").collect();
    // A custom tab's name is the user's own text: shown as typed.
    let labels: Vec<&str> = tabs.iter().map(|(tab, custom)| if *custom { tab.as_str() } else { tl!(tab) }).collect();
    let contextual = |tab: &str, custom: bool| !custom && wordcraft_engine::ribbon::CONTEXTUAL_TABS.contains(&tab);
    let widths: Vec<(f32, bool)> = tabs
        .iter()
        .zip(&labels)
        .map(|((tab, custom), shown)| {
            let w = ui.painter().layout_no_wrap(shown.to_string(), semibold(13.0), t.text).size().x + 20.0;
            (w, *tab == app.ui.tab || contextual(tab, *custom))
        })
        .collect();
    let fits = crate::ribbon::fit_tabs(&widths, ui.available_width(), MORE_TABS_W);
    for ((((tab, custom), shown), (w, _)), fit) in tabs.iter().zip(&labels).zip(&widths).zip(&fits) {
        if !fit {
            continue;
        }
        let (r, resp) = ui.allocate_exact_size(vec2(*w, height), Sense::click());
        if app.ui.keytips == crate::keytips::Phase::Tabs {
            crate::keytips::record_tab(r, tab, &app.ui.keytip_prefix, &mut app.keytip_rects);
        }
        let active = app.ui.tab == *tab && !app.ui.backstage;
        resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::SelectableLabel, true, active, *shown));
        if resp.hovered() && !active {
            ui.painter().rect_filled(r.shrink2(vec2(2.0, 6.0)), 6.0, t.hover);
        }
        let color = if active || contextual(tab, *custom) { t.accent_text } else { t.text };
        ui.painter().text(r.center(), Align2::CENTER_CENTER, *shown, if active { semibold(13.0) } else { regular(13.0) }, color);
        if active {
            ui.painter().rect_filled(Rect::from_min_max(pos2(r.min.x + 10.0, r.max.y - 2.0), pos2(r.max.x - 10.0, r.max.y)), 1.0, t.accent);
        }
        if resp.clicked() {
            let _ = app.run("ui.tab", json!({"tab": tab}));
        }
        if resp.double_clicked() {
            let _ = app.run("ui.collapseRibbon", json!({}));
        }
        resp.context_menu(|ui| crate::widgets::customize_menu_items(ui, app));
    }
    let left_out: Vec<(&str, &str)> =
        tabs.iter().zip(&labels).zip(&fits).filter(|(_, fit)| !**fit).map(|(((tab, _), shown), _)| (tab.as_str(), *shown)).collect();
    if !left_out.is_empty() {
        let (r, resp) = ui.allocate_exact_size(vec2(MORE_TABS_W, height), Sense::click());
        if resp.hovered() {
            ui.painter().rect_filled(r.shrink2(vec2(0.0, 6.0)), 6.0, t.hover);
        }
        icons::paint(ui.painter(), Rect::from_center_size(r.center(), vec2(14.0, 14.0)), "chevronDoubleRight", t.icon, t.accent);
        resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, tl!("More tabs")));
        egui::Popup::menu(&resp).show(|ui| {
            for (tab, shown) in left_out {
                if ui.selectable_label(false, shown).clicked() {
                    let _ = app.run("ui.tab", json!({"tab": tab}));
                    ui.close();
                }
            }
        });
    }
}

/// The save state: "Saved" with a check, "Not saved", or "Saving…" while the browser's save
/// picker works. A click opens AutoSave, Save, Save As and Rename.
fn save_state(ui: &mut Ui, app: &mut WordApp, t: &Tokens) {
    let saving = app.services.async_saves.as_ref().is_some_and(|s| s.pending() > 0);
    let saved = !saving && !app.session.dirty && app.session.path.is_some();
    let (text, color) = if saving {
        (tl!("Saving…"), t.text_dim)
    } else if saved {
        (tl!("Saved"), saved_color(ui.visuals().dark_mode))
    } else {
        (tl!("Not saved"), t.text_dim)
    };
    let font = regular(12.0);
    let tw = ui.painter().layout_no_wrap(text.to_string(), font.clone(), color).size().x;
    let icon_w = if saved { 20.0 } else { 0.0 };
    let (r, _) = ui.allocate_exact_size(vec2(8.0 + icon_w + tw + 4.0 + crate::widgets::DROPDOWN + 8.0, CONTROL_H), Sense::hover());
    let resp = ui.interact(r, save_state_id(), Sense::click());
    if resp.hovered() {
        ui.painter().rect_filled(r, 6.0, t.hover);
    }
    if saved {
        icons::paint(ui.painter(), Rect::from_center_size(pos2(r.min.x + 16.0, r.center().y), vec2(15.0, 15.0)), "check", color, color);
    }
    let x = r.min.x + 8.0 + icon_w;
    ui.painter().text(pos2(x, r.center().y), Align2::LEFT_CENTER, text, font, color);
    let c = pos2(x + tw + 4.0 + crate::widgets::DROPDOWN / 2.0, r.center().y);
    icons::paint(
        ui.painter(),
        Rect::from_center_size(c, vec2(crate::widgets::DROPDOWN, crate::widgets::DROPDOWN)),
        "dropdown",
        t.text_dim,
        t.text_dim,
    );
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, text));
    egui::Popup::menu(&resp).show(|ui| {
        ui.set_min_width(220.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = vec2(6.0, 0.0);
            crate::chrome::autosave_switch(ui, app);
        });
        ui.separator();
        crate::ribbon::mi(ui, app, "Save", "file.save", json!({}));
        crate::ribbon::mi(ui, app, "Save As", "file.saveAs", json!({}));
        crate::ribbon::mi(ui, app, "Rename Document", "file.rename", json!({}));
    });
}

/// The search box ("Tell me"): a click (or its shortcut, `tools.search`) opens Search. Narrow
/// windows show the magnifier alone, so the tabs keep their room.
fn search_box(ui: &mut Ui, app: &mut WordApp, t: &Tokens, narrow: bool) {
    let (r, _) = ui.allocate_exact_size(vec2(if narrow { CONTROL_H + 4.0 } else { 220.0 }, CONTROL_H), Sense::hover());
    let resp = ui.interact(r, search_id(), Sense::click());
    ui.painter().rect(r, 8.0, if resp.hovered() { t.input } else { t.panel }, Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
    let icon_x = if narrow { r.center().x } else { r.min.x + 16.0 };
    icons::paint(ui.painter(), Rect::from_center_size(pos2(icon_x, r.center().y), vec2(14.0, 14.0)), "search", t.text_dim, t.accent);
    let ph = tl!("Search commands and help");
    if !narrow {
        ui.painter().with_clip_rect(r.shrink2(vec2(8.0, 0.0))).text(
            pos2(r.min.x + 30.0, r.center().y),
            Align2::LEFT_CENTER,
            ph,
            regular(12.0),
            t.text_dim,
        );
    }
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, ph));
    if app.ui.keytips == crate::keytips::Phase::Tabs {
        app.keytip_rects.push((r, crate::keytips::SEARCH_LETTER.to_string()));
    }
    let tip = match crate::widgets::shortcut_text(app, "tools.search") {
        k if k.is_empty() => tl!("Search").to_string(),
        k => format!("{} ({k})", tl!("Search")),
    };
    if resp.on_hover_text(tip).clicked() {
        let _ = app.run("tools.search", json!({}));
    }
}

/// Comments: opens the Comments pane. Narrow windows show the icon only.
fn comments_button(ui: &mut Ui, app: &mut WordApp, t: &Tokens, narrow: bool) {
    let label = tl!("Comments");
    let font = regular(12.0);
    let tw = if narrow { 0.0 } else { ui.painter().layout_no_wrap(label.to_string(), font.clone(), t.text).size().x + 6.0 };
    let (r, resp) = ui.allocate_exact_size(vec2(8.0 + 16.0 + tw + 8.0, CONTROL_H), Sense::click());
    let on = ui.is_enabled();
    if resp.hovered() && on {
        ui.painter().rect_filled(r, 6.0, t.hover);
    }
    let (ic, tc) = if on { (t.icon, t.text) } else { (t.text_disabled, t.text_disabled) };
    icons::paint(ui.painter(), Rect::from_center_size(pos2(r.min.x + 16.0, r.center().y), vec2(16.0, 16.0)), "comment", ic, t.accent);
    if !narrow {
        ui.painter().text(pos2(r.min.x + 30.0, r.center().y), Align2::LEFT_CENTER, label, font, tc);
    }
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, on, label));
    if resp.on_hover_text(label).clicked() {
        let _ = app.run("view.commentsPane", json!({}));
    }
}

/// Share: the accent fill and its text colour; exports a copy to share, as upstream's does.
fn share_button(ui: &mut Ui, app: &mut WordApp, t: &Tokens) {
    let label = tl!("Share");
    let font = semibold(12.0);
    let tw = ui.painter().layout_no_wrap(label.to_string(), font.clone(), t.on_accent).size().x;
    let (r, resp) = ui.allocate_exact_size(vec2(12.0 + 14.0 + 6.0 + tw + 12.0, CONTROL_H), Sense::click());
    let on = ui.is_enabled();
    let fill = if !on {
        t.accent.gamma_multiply(0.45)
    } else if resp.hovered() {
        t.accent_text
    } else {
        t.accent
    };
    ui.painter().rect_filled(r, 8.0, fill);
    icons::paint(ui.painter(), Rect::from_center_size(pos2(r.min.x + 19.0, r.center().y), vec2(14.0, 14.0)), "share", t.on_accent, t.on_accent);
    ui.painter().text(pos2(r.min.x + 32.0, r.center().y), Align2::LEFT_CENTER, label, font, t.on_accent);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, on, label));
    if resp.on_hover_text(tl!("Export a copy to share (PDF, Word document)")).clicked() {
        let _ = app.run("ui.backstage", json!({"value": true, "page": "export"}));
    }
}

/// "⋯": what the title bar and the tab strip held besides — the account (your name, set in
/// Options), Editing or Reviewing, and Customize.
fn more_menu(ui: &mut Ui, app: &mut WordApp, t: &Tokens) {
    let (r, _) = ui.allocate_exact_size(vec2(CONTROL_H, CONTROL_H), Sense::hover());
    let resp = ui.interact(r, more_button_id(), Sense::click());
    if resp.hovered() {
        ui.painter().rect_filled(r, 6.0, t.hover);
    }
    icons::paint(ui.painter(), Rect::from_center_size(r.center(), vec2(18.0, 18.0)), "more", t.icon, t.accent);
    let tip = tl!("More options");
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, tip));
    let resp = resp.on_hover_text(tip);
    egui::Popup::menu(&resp).show(|ui| {
        ui.set_min_width(240.0);
        // The account: initials as the title bar drew them, then the name.
        let initials: String = app.session.user_initials().chars().take(2).collect();
        let name = format!("{initials}  {}", app.session.author);
        if ui.button(name).on_hover_text(tl!("Account")).clicked() {
            let _ = app.run("ui.backstage", json!({"value": true, "page": "options", "pane": "general"}));
            ui.close();
        }
        ui.separator();
        ui.add_enabled_ui(app.has_document(), |ui| {
            let track = app.session.doc.settings.track_changes;
            if ui.selectable_label(!track, tl!("Editing — edit the document directly")).clicked() {
                let _ = app.run("review.trackChanges", json!({"value": false}));
                ui.close();
            }
            if ui.selectable_label(track, tl!("Reviewing — edits become suggestions")).clicked() {
                let _ = app.run("review.trackChanges", json!({"value": true}));
                ui.close();
            }
        });
        ui.separator();
        crate::widgets::customize_menu_items(ui, app);
        if app.ui.show_discord && crate::nexus::COMMUNITY_URL.is_some() && ui.button(tl!("Discord")).clicked() {
            let _ = app.run("ui.discord", json!({}));
            ui.close();
        }
    });
}

/// The command strip: the current tab's groups, then "…" with the ones that didn't fit.
fn strip(app: &mut WordApp, ui: &mut Ui, t: &Tokens, user_tabs: &[wordcraft_engine::ribbon::TabEntry]) {
    egui::Panel::top("ribbon")
        .exact_size(STRIP_H)
        .frame(
            egui::Frame::NONE.fill(t.ribbon).inner_margin(egui::Margin { left: 12, right: 8, top: 3, bottom: 3 }).stroke(Stroke::new(1.0, t.border)),
        )
        .show(ui, |ui| {
            // No document open (#1129): every command is greyed out but Help's.
            if !app.has_document() && app.ui.tab != "Help" {
                ui.disable();
            }
            // Right-click on the strip (outside a button): Customize Ribbon. Registered first, so
            // the buttons drawn over it keep their own clicks.
            let bg = ui.interact(ui.max_rect(), ui.id().with("ribbon_background"), Sense::click());
            bg.context_menu(|ui| crate::widgets::customize_menu_items(ui, app));
            let key = Id::new("nexus_wps_group_widths").with(app.ui.tab.as_str());
            let known: Vec<f32> = ui.data(|d| d.get_temp(key)).unwrap_or_default();
            let full = ui.available_rect_before_wrap();
            let room = (full.width() - OVERFLOW_W).max(0.0);
            let shown = fit_groups(&known, room);
            let content = Rect::from_min_size(full.min, vec2(room, full.height()));
            // Salted with the tab: every tab's controls keep their own state (a stack's rows).
            let builder = egui::UiBuilder::new().id_salt(("wps_strip", app.ui.tab.as_str())).max_rect(content);
            let mut row = ui.new_child(builder.layout(egui::Layout::left_to_right(egui::Align::Min)));
            row.set_clip_rect(content.intersect(ui.clip_rect()));
            row.spacing_mut().item_spacing = vec2(2.0, 2.0);
            let gate = with_gate(ui.ctx(), Mode::Strip { shown }, || crate::ribbon::tab_groups(app, &mut row, user_tabs));
            let mut widths = known;
            widths.resize(gate.next, 0.0);
            for (w, m) in widths.iter_mut().zip(&gate.widths) {
                if let Some(m) = m {
                    *w = *m;
                }
            }
            if ui.data(|d| d.get_temp::<Vec<f32>>(key)).as_ref() != Some(&widths) {
                ui.data_mut(|d| d.insert_temp(key, widths));
                ui.ctx().request_repaint();
            }
            if shown >= gate.next {
                return;
            }
            let r = Rect::from_min_size(pos2(full.max.x - OVERFLOW_W + 4.0, full.min.y), vec2(OVERFLOW_W - 4.0, crate::widgets::COMPACT_H));
            let resp = ui.interact(r, overflow_button_id(), Sense::click());
            if resp.hovered() {
                ui.painter().rect_filled(r, 6.0, t.hover);
            }
            icons::paint(ui.painter(), Rect::from_center_size(r.center(), vec2(20.0, 20.0)), "more", t.icon, t.accent);
            let tip = tl!("More commands");
            resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, tip));
            let resp = resp.on_hover_text(tip);
            overflow_panel(app, ui, &resp, shown, user_tabs);
        });
}

/// Where the "…" panel was drawn last frame, while it is open: (its tab, its rect).
fn overflow_open_id() -> Id {
    Id::new("nexus_wps_overflow_open")
}

/// The "…" panel: the groups from `from` on, under the button. Not an egui popup — only one of
/// those can be open, and the groups' own menus (Select, Shading…) are popups that must open
/// over it. It closes on Escape, on another tab, or on a click outside it (unless that click
/// was in one of the groups' menus).
fn overflow_panel(app: &mut WordApp, ui: &mut Ui, button: &Response, from: usize, user_tabs: &[wordcraft_engine::ribbon::TabEntry]) {
    let ctx = ui.ctx().clone();
    let mut open: Option<(String, Rect)> = ctx.data(|d| d.get_temp(overflow_open_id()));
    if button.clicked() {
        open = match open {
            Some(_) => None,
            None => Some((app.ui.tab.clone(), Rect::NOTHING)),
        };
    } else if let Some((tab, rect)) = &open {
        let outside =
            ctx.input(|i| i.pointer.primary_pressed() && i.pointer.interact_pos().is_some_and(|p| !rect.contains(p) && !button.rect.contains(p)));
        let escape = ctx.input(|i| i.key_pressed(egui::Key::Escape));
        if *tab != app.ui.tab || escape || (outside && !egui::Popup::is_any_open(&ctx)) {
            open = None;
        }
    }
    let Some((tab, _)) = open else {
        ctx.data_mut(|d| d.remove::<(String, Rect)>(overflow_open_id()));
        return;
    };
    let area = egui::Area::new(overflow_open_id().with("area"))
        .order(egui::Order::Foreground)
        .pivot(Align2::RIGHT_TOP)
        .fixed_pos(button.rect.right_bottom() + vec2(0.0, 6.0))
        .constrain(true)
        .show(&ctx, |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                // No document open (#1129): greyed out but for Help, as the strip is.
                if !app.has_document() && app.ui.tab != "Help" {
                    ui.disable();
                }
                let builder = egui::UiBuilder::new().id_salt(("wps_overflow", tab.as_str())).layout(egui::Layout::left_to_right(egui::Align::Min));
                ui.scope_builder(builder, |ui| {
                    ui.spacing_mut().item_spacing = vec2(2.0, 2.0);
                    with_gate(&ctx, Mode::Menu { from }, || crate::ribbon::tab_groups(app, ui, user_tabs));
                });
            });
        });
    let rect = area.response.rect;
    ctx.data_mut(|d| d.insert_temp(overflow_open_id(), (tab, rect)));
}

/// How many groups, from the first, fit in `room` points given their widths last drawn: all of
/// them (`usize::MAX`) when they fit. A group not measured yet counts as fitting, so it is drawn
/// (clipped at worst) and measured once.
pub fn fit_groups(widths: &[f32], room: f32) -> usize {
    let mut x = 0.0;
    for (i, w) in widths.iter().enumerate() {
        x += if w.is_finite() { w.max(0.0) } else { 0.0 };
        if x > room + 0.5 {
            return i;
        }
    }
    usize::MAX
}

/// Which groups a pass over the tab draws.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Mode {
    /// The strip: the first `shown` groups.
    Strip { shown: usize },
    /// The "…" menu: the groups from `from` on.
    Menu { from: usize },
}

/// A pass over the tab's groups: the mode, how many groups were met, and the width of each the
/// strip drew.
#[derive(Clone, Debug)]
struct Gate {
    mode: Mode,
    next: usize,
    widths: Vec<Option<f32>>,
    /// A stack in the group being drawn is still finding its layout, so the group's width this
    /// frame isn't its width: it is recorded as unknown (and the group drawn again).
    unsettled: bool,
}

fn gate_id() -> Id {
    Id::new("nexus_wps_gate")
}

/// Run `draw` with groups gated by `mode`; returns what the pass met.
fn with_gate(ctx: &egui::Context, mode: Mode, draw: impl FnOnce()) -> Gate {
    ctx.data_mut(|d| {
        d.insert_temp(gate_id(), Gate { mode, next: 0, widths: Vec::new(), unsettled: false });
        d.insert_temp(compact_id(), true);
    });
    draw();
    let gate = ctx.data_mut(|d| {
        d.remove::<bool>(compact_id());
        let gate = d.get_temp::<Gate>(gate_id());
        d.remove::<Gate>(gate_id());
        gate
    });
    gate.unwrap_or(Gate { mode, next: 0, widths: Vec::new(), unsettled: false })
}

fn compact_id() -> Id {
    Id::new("nexus_wps_compact")
}

/// Whether the ribbon being drawn is the WPS strip (shorter: see [`crate::widgets::COMPACT_H`]).
pub(crate) fn compact(ui: &Ui) -> bool {
    ui.data(|d| d.get_temp::<bool>(compact_id())).unwrap_or(false)
}

/// A stack of controls (upstream draws them one under another) in the WPS strip. Two rows fit
/// as they are (the Font group's two rows). Three or more flow left to right in two rows (Cut and
/// Copy, then Format Painter), as narrow as two rows allow. What fits is found over a few frames
/// — one column, then one row, then from half that width up until the controls take two rows —
/// and kept per interface language.
pub(crate) fn compact_stack(ui: &mut Ui, add: impl FnOnce(&mut Ui)) {
    let key = ui.id().with(("wps_stack", ui.next_auto_id(), crate::i18n::current().code()));
    let state: Option<StackFit> = ui.data(|d| d.get_temp(key));
    let spacing = |ui: &mut Ui| ui.spacing_mut().item_spacing = vec2(6.0, 2.0);
    // Two rows run a point or two past the strip's content height at most; three can't fit.
    let two_rows = crate::widgets::COMPACT_H + 8.0;
    let (layout, max) = match state {
        None | Some(StackFit::Column) => (egui::Layout::top_down(egui::Align::Min), None),
        Some(StackFit::MeasureRow) => (egui::Layout::left_to_right(egui::Align::Min), None),
        Some(StackFit::Rows { width, .. }) => (
            egui::Layout::left_to_right(egui::Align::Min).with_main_wrap(true),
            Some(Rect::from_min_size(ui.cursor().min, vec2(width.max(1.0), crate::widgets::COMPACT_H))),
        ),
    };
    let mut builder = egui::UiBuilder::new().layout(layout);
    if let Some(max) = max {
        builder = builder.max_rect(max);
    }
    let r = ui.scope_builder(builder, |ui| {
        spacing(ui);
        if matches!(state, Some(StackFit::Column)) {
            ui.spacing_mut().item_spacing.y = 0.0;
        }
        add(ui);
    });
    let size = r.response.rect.size();
    if !size.x.is_finite() || !size.y.is_finite() {
        return;
    }
    let next = match state {
        None if size.y <= two_rows => Some(StackFit::Column),
        None => Some(StackFit::MeasureRow),
        Some(StackFit::MeasureRow) => Some(StackFit::Rows { total: size.x, width: (size.x / 2.0).ceil() }),
        // (A point past one row's width, so rounding never wraps its last control.)
        Some(StackFit::Rows { total, width }) if size.y > two_rows && width < total + 1.0 => {
            Some(StackFit::Rows { total, width: (width + (total * 0.15).max(12.0)).min(total + 1.0) })
        }
        _ => None,
    };
    if let Some(next) = next {
        ui.data_mut(|d| {
            d.insert_temp(key, next);
            if let Some(mut gate) = d.get_temp::<Gate>(gate_id()) {
                gate.unsettled = true;
                d.insert_temp(gate_id(), gate);
            }
        });
        ui.ctx().request_repaint();
    }
}

/// How a [`compact_stack`] lays its controls out.
#[derive(Clone, Copy, Debug, PartialEq)]
enum StackFit {
    /// One under another (they fit in two rows).
    Column,
    /// Being measured in one row.
    MeasureRow,
    /// Flowing in rows `width` wide; `total` is their width in one row.
    Rows { total: f32, width: f32 },
}

/// Where a group is being drawn.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Slot {
    /// Upstream's ribbon (or any other caller): drawn as upstream draws it.
    Free,
    /// The WPS strip, as its group number `n`.
    Strip(usize),
    /// The WPS "…" menu, as its group number `n`.
    Menu(usize),
}

impl Slot {
    /// Drawn by the WPS layout (no dialog launcher column).
    pub(crate) fn is_wps(self) -> bool {
        !matches!(self, Slot::Free)
    }
}

/// Called as a ribbon group starts: where it is drawn, or `None` when this pass leaves it out.
pub(crate) fn group_begin(ui: &Ui) -> Option<Slot> {
    ui.ctx().data_mut(|d| {
        let Some(mut gate) = d.get_temp::<Gate>(gate_id()) else { return Some(Slot::Free) };
        let n = gate.next;
        gate.next = n.saturating_add(1);
        gate.unsettled = false;
        d.insert_temp(gate_id(), gate.clone());
        match gate.mode {
            Mode::Strip { shown } if n < shown => Some(Slot::Strip(n)),
            Mode::Menu { from } if n >= from => Some(Slot::Menu(n)),
            _ => None,
        }
    })
}

/// Called as a ribbon group ends, with the width it took (its divider included).
pub(crate) fn group_end(ui: &Ui, slot: Slot, width: f32) {
    let Slot::Strip(n) = slot else { return };
    ui.ctx().data_mut(|d| {
        let Some(mut gate) = d.get_temp::<Gate>(gate_id()) else { return };
        if gate.widths.len() <= n {
            gate.widths.resize(n.saturating_add(1), None);
        }
        let unsettled = std::mem::take(&mut gate.unsettled);
        if let Some(w) = gate.widths.get_mut(n) {
            // 0 = not known: the strip draws it (clipped at worst) and measures it again.
            *w = Some(if unsettled { 0.0 } else { width });
        }
        d.insert_temp(gate_id(), gate);
    });
}

/// A group's right-click area, as large as the group was last frame. Registered before the
/// group's controls so they keep their own clicks.
pub(crate) fn group_menu_area(ui: &mut Ui, id_title: &str) -> Response {
    let id = ui.id().with(("wps_group", id_title));
    let r = ui.data(|d| d.get_temp::<Rect>(id)).unwrap_or(Rect::NOTHING);
    ui.interact(r, id, Sense::click())
}

/// The group's right-click menu: its dialog (the launcher upstream draws in the corner), then
/// Customize. Remembers where the group is for [`group_menu_area`].
pub(crate) fn group_menu(ui: &mut Ui, area: Response, rect: Rect, title: &str, cmd: &str, app: &mut WordApp) {
    ui.data_mut(|d| d.insert_temp(area.id, rect));
    area.context_menu(|ui| {
        if ui.button(crate::i18n::fmt(tl!("{group} settings"), &[("group", title)])).clicked() {
            let _ = app.run(cmd, json!({}));
            ui.close();
        }
        ui.separator();
        crate::widgets::customize_menu_items(ui, app);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::kittest::Queryable;

    type H = egui_kittest::Harness<'static, WordApp>;

    /// The sample document in the WPS layout, embedded as a tab of the NexusOffice window.
    fn harness(size: (f32, f32), lang: &str, setup: impl FnOnce(&mut WordApp)) -> H {
        let mut a = WordApp::new(wordcraft_engine::Session::new(wordcraft_engine::sample::sample_document()), crate::Services::default());
        a.wps = true;
        a.embedded = true;
        a.ui.language = lang.into();
        setup(&mut a);
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(size.0, size.1)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        // Stacks and the "…" fold settle over a few frames.
        h.run_steps(12);
        h
    }

    fn click_id(h: &mut H, id: Id) {
        let r = h.ctx.read_response(id).map(|r| r.rect);
        let r = r.unwrap_or_else(|| panic!("{id:?} is not drawn"));
        h.input_mut().events.push(egui::Event::PointerMoved(r.center()));
        h.run_steps(1);
        for pressed in [true, false] {
            h.input_mut().events.push(egui::Event::PointerButton {
                pos: r.center(),
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            });
            h.run_steps(1);
        }
        h.run_steps(4);
    }

    /// Embedded with the WPS layout there is no title bar (upstream's names the document, "…
    /// — Not saved", in the middle); File (Tệp) opens Backstage.
    #[test]
    fn no_title_bar_and_file_opens_backstage() {
        // Upstream's layout shows the document name in its title bar: the check below sees it.
        let up = harness((1440.0, 900.0), "en", |a| a.wps = false);
        assert!(up.query_by_label_contains(" — ").is_some(), "the title bar's document name is found when it is there");

        let mut h = harness((1440.0, 900.0), "vi", |_| {});
        assert!(h.query_by_label_contains(" — ").is_none(), "no title bar");
        assert!(h.query_by_label("Tự động lưu").is_none(), "AutoSave moved into the save state's menu");
        h.get_by_label("Tệp").click();
        h.run_steps(4);
        assert!(h.state().ui.backstage, "File opens Backstage");
    }

    /// The Quick Access Toolbar keeps working in the top row: Undo and Redo, and Save writes the
    /// document.
    #[test]
    fn quick_access_save_undo_and_redo_work() {
        let path = std::env::temp_dir().join(format!("nexus-wps-qat-{}.docx", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let mut h = harness((1440.0, 900.0), "en", |a| {
            a.run("text.insert", serde_json::json!({"text": "Xin chào "})).ok();
        });
        assert!(h.state().session.can_undo());
        h.get_by_label_contains("Undo").click();
        h.run_steps(4);
        assert!(!h.state().session.can_undo(), "Undo undid the typing");
        h.get_by_label("Redo").click();
        h.run_steps(4);
        assert!(h.state().session.can_undo(), "Redo redid it");
        h.state_mut().session.path = Some(path.clone());
        h.state_mut().session.dirty = true;
        h.run_steps(2);
        h.get_by_label("Save").click();
        h.run_steps(6);
        assert!(path.exists(), "Save wrote the file");
        assert!(!h.state().session.dirty);
        // The save state says so, and its menu holds AutoSave.
        assert!(h.query_by_label("Saved").is_some());
        click_id(&mut h, save_state_id());
        assert!(h.query_by_label("AutoSave").is_some(), "AutoSave is in the save state's menu");
        let _ = std::fs::remove_file(&path);
    }

    /// Every tab, the contextual table tabs included, draws its strip at both window sizes
    /// without panicking, and no group is taller than two rows of the strip.
    #[test]
    fn every_tab_fits_the_strip_at_both_sizes() {
        let tabs = ["Home", "Insert", "Draw", "Design", "Layout", "References", "Mailings", "Review", "View", "Developer", "Help"];
        for size in [(1440.0, 900.0), (1024.0, 700.0)] {
            let mut h = harness(size, "vi", |a| {
                a.run("select.text", serde_json::json!({"text": "Equipment"})).ok();
            });
            for tab in tabs.iter().chain(&["Table Design", "Table Layout"]) {
                h.state_mut().run("ui.tab", serde_json::json!({"tab": tab})).ok();
                // The first look at a tab settles its stacks' rows over a few frames.
                h.run_steps(30);
                crate::widgets::TALL_GROUPS.with(|g| g.borrow_mut().clear());
                h.run_steps(1);
                let tall: Vec<String> = crate::widgets::TALL_GROUPS.with(|g| g.take());
                let too_tall: Vec<&String> = tall
                    .iter()
                    .filter(|g| g.rsplit(": ").next().and_then(|v| v.parse::<f32>().ok()).is_some_and(|v| v > crate::widgets::COMPACT_H + 8.0))
                    .collect();
                assert!(too_tall.is_empty(), "{tab} at {size:?}: {too_tall:?}");
                assert_eq!(h.state().ui.tab, *tab);
            }
        }
    }

    /// At 1024 pt the Home tab's last groups fold into "…": its menu shows them and they run.
    #[test]
    fn narrow_windows_fold_groups_into_the_overflow_menu() {
        let wide = harness((1920.0, 900.0), "en", |_| {});
        assert!(wide.ctx.read_response(overflow_button_id()).is_none(), "everything fits a wide window");

        let mut h = harness((1024.0, 700.0), "en", |_| {});
        assert!(h.query_by_label("Select").is_none(), "Editing folded away");
        click_id(&mut h, overflow_button_id());
        h.get_by_label("Select").click();
        h.run_steps(4);
        h.get_by_label_contains("Select All").click();
        h.run_steps(4);
        assert!(h.state().session.selects_whole_story(), "the folded group's command ran");
    }

    /// Search keeps its shortcut and its box; Comments and Share stay in the top row.
    #[test]
    fn search_comments_and_share_are_reachable() {
        let mut h = harness((1440.0, 900.0), "en", |_| {});
        let key = if cfg!(target_os = "macos") { None } else { Some((egui::Modifiers::ALT, egui::Key::Q)) };
        if let Some((m, k)) = key {
            h.key_press_modifiers(m, k);
            h.run_steps(4);
            assert!(matches!(h.state().dialog, Some(crate::dialogs::Dialog::Commands { .. })), "Alt+Q opens Search");
            h.state_mut().dialog = None;
            h.run_steps(2);
        }
        click_id(&mut h, search_id());
        assert!(matches!(h.state().dialog, Some(crate::dialogs::Dialog::Commands { .. })), "the box opens Search");
        h.state_mut().dialog = None;
        h.run_steps(2);
        let before = h.state().session.view.comments_pane;
        h.get_by_label("Comments").click();
        h.run_steps(4);
        assert_ne!(h.state().session.view.comments_pane, before, "Comments toggles the pane");
        h.get_by_label("Share").click();
        h.run_steps(4);
        assert!(h.state().ui.backstage && h.state().ui.backstage_page == "export", "Share exports a copy");
    }

    /// The "⋯" menu keeps what the title bar held: the account (the user's name).
    #[test]
    fn the_more_menu_names_the_account() {
        let mut h = harness((1440.0, 900.0), "en", |a| a.session.author = "Nguyễn Hữu Đại".into());
        click_id(&mut h, more_button_id());
        assert!(h.query_by_label_contains("Nguyễn Hữu Đại").is_some());
    }

    #[test]
    fn groups_fit_from_the_first_and_unknown_widths_are_drawn() {
        assert_eq!(fit_groups(&[], 100.0), usize::MAX);
        assert_eq!(fit_groups(&[40.0, 40.0], 100.0), usize::MAX);
        assert_eq!(fit_groups(&[40.0, 40.0, 40.0], 100.0), 2);
        assert_eq!(fit_groups(&[120.0, 10.0], 100.0), 0);
        assert_eq!(fit_groups(&[40.0, 0.0, 40.0], 100.0), usize::MAX, "0 = not measured yet");
        assert_eq!(fit_groups(&[f32::NAN, f32::INFINITY, 40.0], 100.0), usize::MAX, "junk widths count as 0");
    }

    /// File's label keeps 4.5:1 on its faint accent fill, in both themes and with a chosen accent.
    #[test]
    fn file_label_is_readable_on_its_fill() {
        let accents = [
            None,
            Some(Color32::from_rgb(0x04, 0x78, 0x57)),
            Some(Color32::from_rgb(0xE1, 0x1D, 0x48)),
            Some(Color32::from_rgb(0xFA, 0xCC, 0x15)),
            Some(Color32::from_rgb(0xB4, 0x53, 0x09)),
        ];
        for base in [Tokens::light(), Tokens::dark()] {
            for accent in accents {
                let t = accent.map_or(base, |c| crate::theme::with_accent(base, c));
                for hovered in [false, true] {
                    let fill = file_fill(&t, hovered);
                    assert_ne!(fill, t.ribbon, "File keeps its tint");
                    let ratio = crate::theme::contrast_ratio(file_ink(&t, fill), fill);
                    assert!(ratio >= 4.5, "{accent:?} hovered {hovered}: {ratio}");
                }
                assert!(crate::theme::contrast_ratio(t.on_accent, t.accent) >= 4.5, "Share's text on the accent");
            }
        }
        for dark in [false, true] {
            let row = if dark { Tokens::dark().ribbon } else { Tokens::light().ribbon };
            assert!(crate::theme::contrast_ratio(saved_color(dark), row) >= 4.5, "Saved, dark {dark}");
        }
    }
}
