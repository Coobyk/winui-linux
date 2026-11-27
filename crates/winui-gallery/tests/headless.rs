use rune::{App, Env};
use winui_gallery::fallback::theme::ThemeMode;
use winui_gallery::{ACCENTS, Page, SCENES, State, pages, spec};

const SIZE: (f32, f32) = (1280.0, 1600.0);

fn runtime(dark: bool, f: impl FnOnce(&mut State)) -> rune::runtime::Runtime<State> {
    let mut sp = spec();
    sp.init = State::default;
    let mut rt = App::new(sp).headless_runtime(SIZE, Env::new(dark, 1.0));
    rt.update(|s, _| f(s));
    rt.frame(0.0);
    rt.frame(0.0);
    rt
}

fn has_key(rt: &rune::runtime::Runtime<State>, key: &str) -> bool {
    rt.elements().iter().any(|e| e.key.as_deref() == Some(key))
}

fn click(rt: &mut rune::runtime::Runtime<State>, key: &str) {
    let (x, y) = rt.find(key).unwrap_or_else(|| panic!("no element {key}"));
    rt.pointer_down(x, y);
    rt.pointer_up(x, y);
    // Settle enter/exit transitions so the next lookup sees the new layout.
    rt.frame(0.0);
    rt.frame(1.0);
}

#[test]
fn every_documented_scene_exists() {
    let ids: Vec<&str> = SCENES.iter().map(|(n, _)| *n).collect();
    for id in ["home", "buttons", "input", "collections", "menus", "navigation", "status", "media", "styles-icons", "styles-colors", "styles-type", "settings"] {
        assert!(ids.contains(&id), "missing scene {id}");
        assert_eq!(Page::from_id(id).map(|p| p.id()), Some(id));
    }
}

#[test]
fn every_page_renders_shell_and_samples() {
    for (name, f) in SCENES.iter().filter(|(n, _)| *n != "search") {
        let rt = runtime(false, |s| f(s));
        let page = Page::from_id(name).unwrap();
        assert!(has_key(&rt, "nav-pane") && has_key(&rt, "content") && has_key(&rt, "search"), "{name}: shell");
        assert!(has_key(&rt, &format!("page-{name}")), "{name}: page");
        for sample in pages::samples_for(page).filter(|_| page != Page::Home) {
            assert!(has_key(&rt, &format!("sample-{}", sample.id)), "{name}: sample {}", sample.id);
        }
    }
}

#[test]
fn nav_click_switches_page() {
    let mut rt = runtime(false, |_| {});
    click(&mut rt, "nav-status");
    assert_eq!(rt.state.page, Page::Status);
    assert!(has_key(&rt, "infobar-0"));
}

#[test]
fn search_filters_samples() {
    let rt = runtime(false, |s| s.set_search("toggle"));
    assert!(has_key(&rt, "page-search"));
    assert!(has_key(&rt, "sample-toggle-button") && has_key(&rt, "sample-toggle-switch"));
    assert!(!has_key(&rt, "sample-listview"));
    assert!(pages::search("zzzz-nothing").is_empty());
    let rt = runtime(false, |s| s.set_search("zzzz-nothing"));
    assert!(has_key(&rt, "no-results"));
}

#[test]
fn settings_toggle_theme_and_accent() {
    let mut rt = runtime(false, |s| s.page = Page::Settings);
    assert_eq!(rt.state.mode(false), ThemeMode::Light);
    click(&mut rt, "theme-toggle");
    assert_eq!(rt.state.theme_override, Some(ThemeMode::Dark));
    click(&mut rt, "accent-4");
    assert_eq!(rt.state.accent, ACCENTS[4].1);
    let theme = rt.state.theme(false);
    assert!(theme.is_dark());
}

#[test]
fn system_theme_followed_without_override() {
    let s = State::default();
    assert_eq!(s.mode(true), ThemeMode::Dark);
    assert_eq!(s.mode(false), ThemeMode::Light);
}

#[test]
fn sample_interactions() {
    let mut rt = runtime(false, |s| s.page = Page::Buttons);
    click(&mut rt, "btn-standard");
    assert_eq!(rt.state.clicks, 1);
    click(&mut rt, "btn-toggle");
    assert!(rt.state.toggled);
    let mut rt = runtime(false, |s| s.page = Page::Input);
    let before = rt.state.check;
    click(&mut rt, "cb-two");
    assert_ne!(rt.state.check, before);
    click(&mut rt, "radio-2");
    assert_eq!(rt.state.radio, 2);
    click(&mut rt, "slider-step20");
    assert!((rt.state.slider - 1.0).abs() < 1e-6);
}

#[test]
fn rune_args_strip_separator() {
    let a = winui_gallery::rune_args(["g", "--scene", "home", "--", "--size", "800x600", "--"].map(String::from));
    assert_eq!(a, ["--scene", "home", "--size", "800x600", "--"]);
}

#[test]
fn closing_earlier_tab_keeps_active_document() {
    let mut rt = runtime(false, |s| {
        s.page = Page::Navigation;
        s.tab_sel = 2;
    });
    click(&mut rt, "tab-0-close");
    assert_eq!(rt.state.tabs[rt.state.tab_sel], "Document 2");
    for _ in 0..5 {
        click(&mut rt, "tab-add");
    }
    assert_eq!(rt.state.tabs.len(), winui_gallery::pages::MAX_TABS);
}

#[test]
fn flyouts_open_and_close() {
    let mut rt = runtime(false, |s| s.page = Page::Buttons);
    assert!(!has_key(&rt, "dropdown-menu"));
    click(&mut rt, "btn-dropdown");
    assert!(has_key(&rt, "dropdown-menu"));
    click(&mut rt, "dropdown-send");
    assert!(!rt.state.dropdown_open);
    let mut rt = runtime(false, |s| s.page = Page::Menus);
    assert!(!has_key(&rt, "flyout"));
    click(&mut rt, "flyout-open");
    click(&mut rt, "flyout-confirm");
    assert!(!rt.state.cart_open);
}

#[test]
fn icon_search_by_name_and_keyword() {
    assert!(pages::search("star").iter().any(|s| s.id == "icons"));
    let rt = runtime(false, |s| s.set_search("glyph"));
    assert!(has_key(&rt, "icon-Home"));
}

#[test]
fn each_page_has_its_own_scroller() {
    let mut rt = runtime(false, |_| {});
    assert!(has_key(&rt, "scroll-home"));
    click(&mut rt, "nav-media");
    for i in 1..=4 {
        rt.frame(i as f64);
    }
    assert!(has_key(&rt, "scroll-media") && !has_key(&rt, "scroll-home"));
}
