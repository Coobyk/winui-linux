//! `ref-clock-timer` / `ref-clock-settings`: the Windows Clock app
//! (docs/reference/clock-timer.png, clock-settings.png).

use rune::{El, col, el, row};
use winui_controls::style::rc;
use winui_controls::{Button, HyperlinkButton, ProgressRing, ToggleSwitch, selection_pill};
use winui_icons::Symbol;
use winui_theme::{Color, Theme, radii};

use super::{icon, label, settings_card, title_bar};

const NAV: [(Symbol, &str); 5] = [
    (Symbol::Clock, "Focus sessions"),
    (Symbol::Timer, "Timer"),
    (Symbol::Alert, "Alarm"),
    (Symbol::History, "Stopwatch"),
    (Symbol::Globe, "World clock"),
];

/// The Clock window: a 281px open pane on Mica, the page on a `LayerFillColorDefault` layer.
fn shell(t: &Theme, selected: usize, page: El) -> El {
    let k = t.tokens();
    let ramp = t.type_ramp();
    let item = |i: usize, sym: Symbol, name: &str, top: f32| {
        let sel = i == selected;
        let mut r = row()
            .absolute(6.0, top)
            .size(271.0, 36.0)
            .radius(radii::CONTROL)
            .pl(15.0)
            .gap(14.0)
            .center_y()
            .fill(rc(if sel {
                k.subtle_fill_color_secondary
            } else {
                k.subtle_fill_color_transparent
            }))
            .hover_fill(rc(k.subtle_fill_color_secondary))
            .child(icon(sym, 16.0, k.text_fill_color_primary))
            .child(label(name, ramp.body, k.text_fill_color_primary));
        if sel {
            r = r.child(selection_pill(t, 0.0, 10.0));
        }
        r
    };
    let logo = el()
        .size(16.0, 16.0)
        .radius(8.0)
        .fill(rc(Color::from_argb(0xFFE6E6E6)))
        .center()
        .child(icon(Symbol::Clock, 12.0, Color::from_argb(0xFF3A3A3A)));
    let mut pane = el()
        .w(282.0)
        .h_full()
        .child(logo.absolute(16.0, 11.0))
        .child(label("Clock", ramp.caption, k.text_fill_color_primary).absolute(47.0, 11.0));
    for (i, (sym, name)) in NAV.into_iter().enumerate() {
        pane = pane.child(item(i, sym, name, 39.0 + 40.0 * i as f32));
    }
    pane = pane
        .child(item(5, Symbol::Person, "Sign in", 849.0))
        .child(item(6, Symbol::Settings, "Settings", 890.0));
    let layer = el()
        .grow(1.0)
        .h_full()
        .fill(rc(k.layer_fill_color_default))
        .border_sides([1.0, 0.0, 0.0, 0.0], rc(k.card_stroke_color_default))
        .child(page.pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0)))
        .child(title_bar(t, 32.0, el()).pin(Some(0.0), Some(0.0), Some(0.0), None));
    row()
        .w_full()
        .h_full()
        .child(pane)
        .child(layer)
        .clip_rounded(0.0)
}

pub fn timer(t: &Theme) -> El {
    let k = t.tokens();
    let mut page = el();
    for (i, (name, time)) in [
        ("1 min", "00:01:00"),
        ("3 min", "00:03:00"),
        ("5 min", "00:05:00"),
        ("10 min", "00:10:00"),
    ]
    .into_iter()
    .enumerate()
    {
        let (x, y) = (56.0 + 408.0 * (i % 2) as f32, 53.0 + 336.0 * (i / 2) as f32);
        page = page.child(timer_card(t, name, time).absolute(x, y));
    }
    let tools = row()
        .absolute(809.0, 864.0)
        .size(88.0, 49.0)
        .radius(radii::OVERLAY)
        .fill(rc(k.solid_background_fill_color_quarternary))
        .border(1.0, rc(k.surface_stroke_color_default))
        .shadow(4.0, 16.0, rc(Color::from_argb(0x42000000)))
        .px(4.0)
        .center_y()
        .child(el().size(40.0, 40.0).center().child(icon(
            Symbol::Edit,
            16.0,
            k.text_fill_color_primary,
        )))
        .child(el().size(40.0, 40.0).center().child(icon(
            Symbol::Add,
            16.0,
            k.text_fill_color_primary,
        )));
    shell(t, 1, page.child(tools))
}

fn timer_card(t: &Theme, name: &str, time: &str) -> El {
    let k = t.tokens();
    let ramp = t.type_ramp();
    let round = |fill: winui_theme::Color, sym: Symbol, fg: winui_theme::Color| {
        el().size(28.0, 28.0)
            .radius(14.0)
            .fill(rc(fill))
            .center()
            .child(icon(sym, 12.0, fg))
    };
    el().size(398.0, 323.0)
        .radius(radii::OVERLAY)
        .fill(rc(k.card_background_fill_color_default))
        .border(1.0, rc(k.card_stroke_color_default))
        .child(label(name, ramp.body_strong, k.text_fill_color_primary).absolute(15.0, 11.0))
        .child(icon(Symbol::ArrowMaximize, 16.0, k.text_fill_color_primary).absolute(328.0, 13.0))
        .child(icon(Symbol::OpenInNew, 16.0, k.text_fill_color_primary).absolute(368.0, 13.0))
        .child(
            ProgressRing::new()
                .value(0.0)
                .size(209.0)
                .stroke(13.0)
                .view(t)
                .absolute(94.5, 46.0),
        )
        .child(el().absolute(0.0, 126.0).w(398.0).center().child(label(
            time,
            winui_theme::TextStyle::new(40.0, 400, 52.0),
            k.text_fill_color_secondary,
        )))
        .child(
            el().absolute(164.0, 273.0)
                .size(28.0, 28.0)
                .radius(14.0)
                .fill(rc(k.accent_fill_color_default))
                .center()
                .child(
                    rune::canvas(
                        rune::Shape::new()
                            .fill(
                                [[1.0, 0.0], [10.0, 5.5], [1.0, 11.0]],
                                rc(k.text_on_accent_fill_color_primary),
                            )
                            .view_box(11.0, 11.0),
                    )
                    .size(11.0, 11.0),
                ),
        )
        .child(
            round(
                k.control_fill_color_default,
                Symbol::ArrowCounterclockwise,
                k.text_fill_color_disabled,
            )
            .absolute(206.0, 273.0),
        )
}

pub fn settings(t: &Theme) -> El {
    let k = t.tokens();
    let ramp = t.type_ramp();
    let w = 806.0;
    let x = 56.0;
    let header =
        |s: &str, y: f32| label(s, ramp.body_strong, k.text_fill_color_primary).absolute(x, y);
    let chevron = || {
        el().size(32.0, 32.0).center().child(icon(
            Symbol::ChevronDown,
            12.0,
            k.text_fill_color_primary,
        ))
    };
    let toggle = |chev: bool| {
        let mut r = row()
            .gap(12.0)
            .center_y()
            .child(ToggleSwitch::new().on(true).right_to_left(true).view(t));
        if chev {
            r = r.child(chevron());
        }
        r
    };
    let page = el()
        .child(label("Settings", ramp.title_large, k.text_fill_color_primary).absolute(x, 37.0))
        .child(header("Account", 140.0))
        .child(
            settings_card(t, w, 68.0, Symbol::Person, "Sign in with a work, school, or Microsoft account", None, Button::new("Sign in").width(117.0).view(t))
                .absolute(x, 167.0),
        )
        .child(header("Focus sessions", 267.0))
        .child(
            settings_card(t, w, 70.0, Symbol::Clock, "Focus periods", Some("Adjust the lengths of your focus time, or breaks to fit your needs."), chevron())
                .absolute(x, 294.0),
        )
        .child(settings_card(t, w, 70.0, Symbol::Alert, "End of session sound", Some("Play an alarm when focus period ends"), toggle(true)).absolute(x, 367.0))
        .child(settings_card(t, w, 70.0, Symbol::Alert, "End of break sound", Some("Play an alarm when breaks end"), toggle(true)).absolute(x, 440.0))
        .child(
            settings_card(t, w, 70.0, Symbol::MusicNote, "Spotify", Some("You need the Spotify app to enhance your focus sessions with music and podcasts"), toggle(false))
                .absolute(x, 513.0),
        )
        .child(header("General", 617.0))
        .child(
            settings_card(
                t,
                w,
                70.0,
                Symbol::Color,
                "App theme",
                Some("Choose your preferred app theme"),
                row().gap(14.0).center_y().child(label("Use system setting", ramp.body, k.text_fill_color_primary)).child(chevron()),
            )
            .absolute(x, 645.0),
        )
        .child(
            settings_card(t, w, 70.0, Symbol::Alert, "Notifications", Some("Modify your notification settings"), HyperlinkButton::new("Change notification settings").view(t))
                .absolute(x, 718.0),
        )
        .child(
            settings_card(
                t,
                w,
                70.0,
                Symbol::Lock,
                "Privacy",
                Some("Your data is stored on your device for 90 days. Select Clear history to remove this data."),
                Button::new("Clear history").width(121.0).view(t),
            )
            .absolute(x, 791.0),
        )
        .child(header("About", 890.0))
        .child(settings_card(t, w, 70.0, Symbol::Clock, "Clock", Some("© Microsoft Corporation. All rights reserved."), chevron()).absolute(x, 918.0));
    shell(t, 6, col().w_full().h_full().child(page.grow(1.0)))
}
