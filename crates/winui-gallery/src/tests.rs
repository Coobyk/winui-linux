use super::*;
use rune::{App, Env};

fn rt(dark: bool) -> rune::Runtime<State> {
    let mut rt = App::new(spec()).headless_runtime((1280.0, 800.0), Env::new(dark, 1.0));
    rt.frame(0.0);
    rt
}

#[test]
fn scenes_cover_every_page_id() {
    let s = spec();
    for p in Page::ALL {
        assert!(s.scenes.iter().any(|(id, _)| *id == p.id()), "{}", p.id());
        assert_eq!(Page::from_id(p.id()), Some(p));
    }
}

#[test]
fn every_page_builds_in_both_themes() {
    for dark in [false, true] {
        let mut rt = rt(dark);
        for p in Page::ALL {
            rt.update(move |s, _| s.page = p);
            rt.frame(0.0);
            assert!(!rt.elements().is_empty());
        }
    }
}

#[test]
fn nav_click_switches_page() {
    let mut rt = rt(false);
    let (x, y) = rt.find("nav-status").unwrap();
    rt.pointer_down(x, y);
    rt.pointer_up(x, y);
    rt.frame(0.0);
    assert_eq!(rt.state.page, Page::Status);
}

#[test]
fn settings_toggle_theme_and_accent() {
    let mut rt = rt(false);
    rt.update(|s, _| s.page = Page::Settings);
    rt.frame(0.0);
    assert_eq!(rt.state.mode(), ThemeMode::Light);
    let (x, y) = rt.find("theme-toggle").unwrap();
    rt.pointer_down(x, y);
    rt.pointer_up(x, y);
    rt.frame(0.0);
    assert_eq!(rt.state.mode(), ThemeMode::Dark);
    let (x, y) = rt.find("accent-3").unwrap();
    rt.pointer_down(x, y);
    rt.pointer_up(x, y);
    rt.frame(0.0);
    assert_eq!(rt.state.accent, 3);
    assert_ne!(
        rt.state.theme().tokens().accent_fill_color_default,
        Theme::dark().tokens().accent_fill_color_default
    );
}

#[test]
fn app_scenes_are_wired_and_build() {
    let s = spec();
    for app in apps::App::ALL {
        let (_, set) = s.scenes.iter().find(|(id, _)| *id == app.id()).expect(app.id());
        let mut rt = App::new(spec()).headless_runtime(app.size(), Env::new(true, 1.0));
        rt.update(move |st, _| set(st));
        rt.frame(0.0);
        assert_eq!(rt.state.app, Some(app));
        assert!(rt.find(&format!("app-{}", app.id())).is_some(), "{}", app.id());
        assert!(rt.find("nav-home").is_none(), "{} rendered the gallery shell", app.id());
    }
}

#[test]
fn calculator_keys_edit_the_display() {
    let mut rt = App::new(spec()).headless_runtime((328.0, 532.0), Env::new(true, 1.0));
    rt.update(|s, _| s.app = Some(apps::App::Calc));
    rt.frame(0.0);
    for id in ["calc-7", "calc-4", "calc-back", "calc-2"] {
        let (x, y) = rt.find(id).unwrap();
        rt.pointer_down(x, y);
        rt.pointer_up(x, y);
        rt.frame(0.0);
    }
    assert_eq!(rt.state.calc, "72");
}
