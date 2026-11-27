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
fn app_scenes_render_the_clone_full_window() {
    let s = spec();
    for a in crate::apps::App::ALL {
        let (_, set) = s.scenes.iter().find(|(id, _)| *id == a.id()).expect(a.id());
        let mut st = State::default();
        set(&mut st);
        assert_eq!(st.page, Page::App(a));
        assert!(st.full_app, "{}", a.id());
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            st.theme = Some(mode);
            let _ = a.window(&st.theme());
        }
    }
    let mut st = State::default();
    (s.scenes.iter().find(|(id, _)| *id == "settings").unwrap().1)(&mut st);
    assert!(!st.full_app);
}
