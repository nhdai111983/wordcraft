//! NexusOffice: "Lưu thay đổi?" as a tab of the NexusOffice window (`nexus_save_prompt`).

use egui::accesskit::Role;
use egui_kittest::kittest::Queryable;
use serde_json::json;
use wordcraft_engine::Session;

use crate::{Services, WordApp, dialogs, theme};

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("nexus-docs-save-prompt-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// An embedded Vietnamese app with `dir/Báo cáo.docx` open and changed, asking "Lưu thay đổi?".
fn asking(dir: &std::path::Path) -> (WordApp, std::path::PathBuf) {
    let path = dir.join("Báo cáo.docx");
    let mut s = Session::new(wordcraft_doc::Document::from_text("Bản gốc"));
    s.run("file.save", &json!({"path": path.to_string_lossy()})).unwrap();
    let mut a = WordApp::new(Session::new(wordcraft_doc::Document::new()), Services::default());
    a.wps = true;
    a.embedded = true;
    a.ui.language = "vi".into();
    a.run("file.open", json!({"path": path.to_string_lossy()})).unwrap();
    a.run("text.insert", json!({"text": "Sửa "})).unwrap();
    a.run("file.close", json!({})).unwrap();
    assert!(matches!(a.dialog, Some(dialogs::Dialog::SaveChanges { .. })));
    (a, path)
}

fn harness(a: WordApp) -> egui_kittest::Harness<'static, WordApp> {
    let mut fonts = false;
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1000.0, 700.0)).build_ui_state(
        move |ui, app: &mut WordApp| {
            let ctx = ui.ctx().clone();
            crate::i18n::set_current(crate::i18n::Lang::from_pref(&app.ui.language));
            if fonts {
                dialogs::show(app, &ctx);
            } else {
                theme::install_fonts_for(&ctx, false);
                fonts = true;
            }
        },
        a,
    );
    for _ in 0..4 {
        h.step();
    }
    h
}

/// The title, the question naming the document, Lưu · Không lưu · Hủy right-aligned in that
/// order at the shared size, and the keyboard on Lưu.
#[test]
fn embedded_prompt_reads_like_every_nexus_app() {
    let dir = scratch("look");
    let (a, _) = asking(&dir);
    let name = a
        .dialog
        .as_ref()
        .and_then(|d| match d {
            dialogs::Dialog::SaveChanges { name, .. } => Some(name.clone()),
            _ => None,
        })
        .unwrap();
    let h = harness(a);
    h.get_by_label("Lưu thay đổi?");
    h.get_by_label(&format!("Lưu thay đổi của “{name}” trước khi đóng?"));
    assert!(h.query_by_label("Nexus Docs").is_none(), "the app's name is not the title");
    let save = h.get_by_role_and_label(Role::Button, "Lưu").rect();
    let dont = h.get_by_role_and_label(Role::Button, "Không lưu").rect();
    let cancel = h.get_by_role_and_label(Role::Button, "Hủy").rect();
    assert!(save.max.x <= dont.min.x && dont.max.x <= cancel.min.x, "Lưu, Không lưu, Hủy: {save:?} {dont:?} {cancel:?}");
    assert!((save.center().y - cancel.center().y).abs() < 0.5, "one row");
    for b in [save, dont, cancel] {
        assert!(b.width() >= crate::nexus_save_prompt::BUTTON_SIZE.x - 0.5 && b.height() >= crate::nexus_save_prompt::BUTTON_SIZE.y - 0.5);
    }
    // A dialog named by its title; the buttons end at its right padding.
    let card = h.get_by_role_and_label(Role::Dialog, "Lưu thay đổi?").rect();
    let pad = f32::from(crate::nexus_save_prompt::PADDING);
    assert!((cancel.max.x - (card.max.x - pad)).abs() < 1.5, "right-aligned: {cancel:?} {card:?}");
    assert!(h.get_by_role_and_label(Role::Button, "Lưu").is_focused(), "the keyboard starts on Lưu");
    let _ = std::fs::remove_dir_all(&dir);
}

/// Enter saves (the file is written and the document closes); Escape cancels (nothing written,
/// the document stays open with its changes).
#[test]
fn embedded_prompt_enter_saves_and_escape_cancels() {
    for (key, saves) in [(egui::Key::Enter, true), (egui::Key::Escape, false)] {
        let dir = scratch(if saves { "enter" } else { "escape" });
        let (a, path) = asking(&dir);
        let before = std::fs::read(&path).unwrap();
        let mut h = harness(a);
        h.key_press(key);
        for _ in 0..3 {
            h.step();
        }
        let a = h.state();
        assert!(a.dialog.is_none(), "{key:?}: answered");
        assert_eq!(std::fs::read(&path).unwrap() != before, saves, "{key:?}: file written");
        assert_eq!(a.quit_requested, saves, "{key:?}: the document closes only after Lưu");
        assert_eq!(a.session.dirty, !saves, "{key:?}: unsaved changes kept on Hủy");
        let _ = std::fs::remove_dir_all(&dir);
    }
}

/// Không lưu closes without writing; a click on the dimmed tab outside the card does nothing.
#[test]
fn embedded_prompt_dont_save_and_the_dim_takes_clicks() {
    let dir = scratch("dont");
    let (a, path) = asking(&dir);
    let before = std::fs::read(&path).unwrap();
    let mut h = harness(a);
    h.get_by_role_and_label(Role::Button, "Lưu").hover();
    h.step();
    // Outside the card (the top-left corner of the window): still asking.
    let corner = egui::pos2(5.0, 5.0);
    h.event(egui::Event::PointerMoved(corner));
    h.event(egui::Event::PointerButton { pos: corner, button: egui::PointerButton::Primary, pressed: true, modifiers: egui::Modifiers::NONE });
    h.event(egui::Event::PointerButton { pos: corner, button: egui::PointerButton::Primary, pressed: false, modifiers: egui::Modifiers::NONE });
    h.step();
    h.step();
    assert!(h.state().dialog.is_some(), "a click on the dim doesn't answer");
    h.get_by_role_and_label(Role::Button, "Không lưu").click();
    for _ in 0..3 {
        h.step();
    }
    let a = h.state();
    assert!(a.dialog.is_none());
    assert!(a.quit_requested, "Không lưu closes");
    assert_eq!(std::fs::read(&path).unwrap(), before, "nothing written");
    let _ = std::fs::remove_dir_all(&dir);
}

/// Stand-alone (not embedded) keeps upstream's prompt.
#[test]
fn stand_alone_prompt_is_upstreams() {
    let dir = scratch("standalone");
    let (mut a, _) = asking(&dir);
    a.embedded = false;
    let h = harness(a);
    assert!(h.query_by_label("Lưu thay đổi?").is_none());
    h.get_by_role_and_label(Role::Button, "Không lưu");
    let _ = std::fs::remove_dir_all(&dir);
}

/// Tab and Shift+Tab go through the three buttons in the order they read (the dim and the card
/// never take the focus): Lưu → Không lưu → Hủy → Không lưu.
#[test]
fn tab_cycles_the_buttons_in_reading_order() {
    let dir = scratch("tab");
    let (a, _) = asking(&dir);
    let mut h = harness(a);
    let focused = |h: &egui_kittest::Harness<'static, WordApp>, l: &str| h.get_by_role_and_label(Role::Button, l).is_focused();
    assert!(focused(&h, "Lưu"));
    for (key, shift, want) in [(egui::Key::Tab, false, "Không lưu"), (egui::Key::Tab, false, "Hủy"), (egui::Key::Tab, true, "Không lưu")] {
        let m = if shift { egui::Modifiers::SHIFT } else { egui::Modifiers::NONE };
        h.key_press_modifiers(m, key);
        h.step();
        h.step();
        assert!(focused(&h, want), "{key:?} shift={shift}: {want} has the focus");
    }
    assert!(h.state().dialog.is_some(), "still asking");
    let _ = std::fs::remove_dir_all(&dir);
}
