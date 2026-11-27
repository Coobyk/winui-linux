//! Headless layout + input: controls build, carry their keys, sizes and states, and their
//! handlers update app state.
use rune::{col, App, AppSpec, Cx, El, Env};
use winui_controls::*;

#[derive(Default)]
struct S {
    presses: u32,
    on: bool,
    check: CheckState,
    combo: Option<usize>,
    open: bool,
    rating: u32,
    nav: usize,
}

fn view(s: &S, cx: &mut Cx<S>) -> El {
    let t = if cx.system_dark() { Theme::dark() } else { Theme::light() };
    col()
        .gap(8.0)
        .padding(8.0)
        .child(Button::new("Go").key("go").on_press(|s: &mut S| s.presses += 1).view(&t))
        .child(Button::new("Off").key("off").enabled(false).on_press(|s: &mut S| s.presses += 100).view(&t))
        .child(ToggleSwitch::new().on(s.on).key("sw").on_toggle(|s: &mut S, v| s.on = v).view(&t))
        .child(CheckBox::new("Check").state(s.check).three_state(true).key("cb").on_toggle(|s: &mut S, v| s.check = v).view(&t))
        .child(RatingControl::new().value(s.rating as f32).on_rate(|s: &mut S, v| s.rating = v).view(&t))
        .child(
            ComboBox::new(["One", "Two"]).selected(s.combo).open(s.open)
                .on_select(|s: &mut S, i| { s.combo = Some(i); s.open = false; })
                .on_toggle(|s: &mut S, v| s.open = v)
                .view(&t),
        )
        .child(
            NavigationView::new([NavItem::new(Symbol::Home, "Home"), NavItem::new(Symbol::Apps, "Apps")])
                .selected(s.nav)
                .on_select(|s: &mut S, i| s.nav = i)
                .view(&t)
                .h(300.0),
        )
}

fn rt() -> rune::Runtime<S> {
    App::new(AppSpec::new("t", (900.0, 900.0), S::default, view)).headless_runtime((900.0, 900.0), Env::new(false, 1.0))
}

fn settle(rt: &mut rune::Runtime<S>) {
    for _ in 0..30 {
        rt.frame(1.0 / 60.0);
    }
}

/// Presses the element keyed `key` once overlays have finished their entrance.
fn click(rt: &mut rune::Runtime<S>, key: &str) {
    settle(rt);
    let (x, y) = rt.find(key).unwrap_or_else(|| panic!("no element keyed {key}"));
    rt.pointer_down(x, y);
    rt.pointer_up(x, y);
    rt.frame(0.0);
}

fn rect(rt: &rune::Runtime<S>, key: &str) -> [f32; 4] {
    let e = rt.elements().into_iter().find(|e| e.key.as_deref().is_some_and(|k| k.ends_with(key))).unwrap_or_else(|| panic!("no {key}"));
    e.rect
}

#[test]
fn button_is_32_tall_and_presses() {
    let mut rt = rt();
    rt.frame(0.0);
    assert_eq!(rect(&rt, "go")[3], 32.0);
    click(&mut rt, "go");
    click(&mut rt, "off");
    rt.update(|s, _| assert_eq!(s.presses, 1));
}

#[test]
fn toggles_cycle() {
    let mut rt = rt();
    click(&mut rt, "sw");
    click(&mut rt, "cb");
    rt.update(|s, _| assert!(s.on && s.check == CheckState::Checked));
    click(&mut rt, "cb");
    click(&mut rt, "rating:4");
    rt.update(|s, _| assert!(s.check == CheckState::Indeterminate && s.rating == 4));
}

#[test]
fn combobox_opens_and_selects() {
    let mut rt = rt();
    click(&mut rt, "combobox");
    rt.update(|s, _| assert!(s.open));
    click(&mut rt, "combobox:item1");
    rt.update(|s, _| assert!(s.combo == Some(1) && !s.open));
}

#[test]
fn navigation_selects() {
    let mut rt = rt();
    click(&mut rt, "nav:Apps");
    rt.update(|s, _| assert_eq!(s.nav, 1));
    assert_eq!(rect(&rt, "nav:Apps")[3], 36.0);
}

#[test]
fn tokens_and_helpers() {
    let light = Theme::light();
    let dark = Theme::dark();
    assert_ne!(light.tokens().text_fill_color_primary, dark.tokens().text_fill_color_primary);
    assert_eq!(light.accent().base, shim::theme::Color::rgb(0x00, 0x78, 0xD4));
    assert_eq!(PersonPicture::initials_of("Jane van Doe"), "JD");
    assert_eq!(range::value_at(10.0, 220.0, 0.0, 100.0, 1.0), 0.0);
    assert_eq!(range::value_at(210.0, 220.0, 0.0, 100.0, 1.0), 100.0);
    assert_eq!(NumberBox::parse(" 12.5 ", 0.0, 10.0), Some(10.0));
    assert_eq!(AutoSuggestBox::filter(&["Card", "Canvas", "Slider"], "ca"), vec!["Card", "Canvas"]);
    for s in Symbol::ALL {
        assert!(matches!(s.glyph(), Glyph::Path(d) if !d.is_empty()), "{s:?}");
    }
}
