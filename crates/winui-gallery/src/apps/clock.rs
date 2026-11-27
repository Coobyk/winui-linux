//! Clock: Timer (`docs/reference/clock-timer.png`) and Settings (`clock-settings.png`).

use rune::{El, col, el, row};
use winui_controls::{Button, HyperlinkButton, ToggleSwitch};
use winui_icons::Symbol;
use winui_theme::{Theme, radii, sizes};

use super::glyphs as g;
use super::{caption_buttons, path_icon, sym, txt};
use crate::fluent::c;

const PANE_W: f32 = 283.0;

#[derive(Clone, Copy, PartialEq)]
enum Nav {
    Focus,
    Timer,
    Alarm,
    Stopwatch,
    World,
    SignIn,
    Settings,
}

fn nav_item(t: &Theme, icon: El, label: &str, selected: bool) -> El {
    let k = t.tokens();
    let mut r = row()
        .w(PANE_W - 12.0)
        .h(sizes::NAV_ITEM_HEIGHT)
        .radius(radii::CONTROL)
        .items_center()
        .child(el().w(42.0).pl(10.0).child(icon))
        .child(txt(label, t.type_ramp().body, k.text_fill_color_primary));
    if selected {
        let pill = el()
            .size(3.0, 16.0)
            .radius(radii::SELECTION_INDICATOR)
            .fill(c(k.accent_fill_color_default))
            .absolute(0.0, 10.0);
        r = r.fill(c(k.subtle_fill_color_secondary)).child(pill);
    }
    r
}

fn pane(t: &Theme, sel: Nav) -> El {
    let k = t.tokens();
    let ink = k.text_fill_color_primary;
    let item = |n: Nav, icon: El, label: &str| nav_item(t, icon, label, n == sel);
    let top = col()
        .gap(4.0)
        .child(item(
            Nav::Focus,
            path_icon(g::CIRCLE_HALF, 16.0, ink),
            "Focus sessions",
        ))
        .child(item(
            Nav::Timer,
            path_icon(g::HOURGLASS, 16.0, ink),
            "Timer",
        ))
        .child(item(Nav::Alarm, path_icon(g::ALARM, 16.0, ink), "Alarm"))
        .child(item(
            Nav::Stopwatch,
            sym(Symbol::Timer, 16.0, ink),
            "Stopwatch",
        ))
        .child(item(
            Nav::World,
            path_icon(g::GLOBE_CLOCK, 16.0, ink),
            "World clock",
        ));
    let bottom = col()
        .gap(4.0)
        .child(item(Nav::SignIn, sym(Symbol::Person, 16.0, ink), "Sign in"))
        .child(item(
            Nav::Settings,
            sym(Symbol::Settings, 16.0, ink),
            "Settings",
        ));
    let title = row()
        .h(38.0)
        .items_center()
        .pl(16.0)
        .child(sym(Symbol::Timer, 16.0, ink))
        .child(
            el().pl(12.0)
                .child(txt("Clock", t.type_ramp().caption, ink)),
        );
    col()
        .w(PANE_W)
        .h_full()
        .pl(6.0)
        .pb(8.0)
        .child(title)
        .child(top)
        .child(el().grow(1.0))
        .child(bottom)
}

fn shell(t: &Theme, sel: Nav, content: El) -> El {
    let k = t.tokens();
    let frame = el()
        .grow(1.0)
        .h_full()
        .min_w(0.0)
        .fill(c(k.layer_fill_color_default))
        .border_sides([1.0, 0.0, 0.0, 0.0], c(k.card_stroke_color_default))
        .child(content.pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0)))
        .child(caption_buttons(t, 32.0).pin(None, Some(0.0), Some(0.0), None));
    row()
        .fill(c(k.solid_background_fill_color_base))
        .child(pane(t, sel))
        .child(frame)
}

fn round_button(t: &Theme, accent: bool) -> El {
    let k = t.tokens();
    let (fill, glyph) = if accent {
        (
            k.accent_fill_color_default,
            path_icon(g::PLAY_FILLED, 12.0, k.text_on_accent_fill_color_primary),
        )
    } else {
        (
            k.control_fill_color_default,
            sym(
                Symbol::ArrowCounterclockwise,
                12.0,
                k.text_fill_color_disabled,
            ),
        )
    };
    el().size(30.0, 30.0)
        .radius(15.0)
        .fill(c(fill))
        .border(1.0, c(k.control_stroke_color_default))
        .center()
        .child(glyph)
}

fn timer_card(t: &Theme, name: &str, time: &str) -> El {
    let k = t.tokens();
    let ink = k.text_fill_color_primary;
    let head = row()
        .w_full()
        .h(36.0)
        .items_center()
        .pl(14.0)
        .pr(10.0)
        .child(txt(name, t.type_ramp().body_strong, ink))
        .child(el().grow(1.0))
        .child(
            el().size(36.0, 32.0)
                .center()
                .child(sym(Symbol::ArrowMaximize, 16.0, ink)),
        )
        .child(
            el().size(36.0, 32.0)
                .center()
                .child(path_icon(g::PIP_ENTER, 16.0, ink)),
        );
    let ring = el()
        .size(208.0, 208.0)
        .radius(104.0)
        .border(
            9.0,
            c(k.control_fill_color_default
                .over(k.card_background_fill_color_default)),
        )
        .center()
        .child(txt(
            time,
            super::px_style(36.0, 600, 48.0),
            k.text_fill_color_secondary,
        ));
    col()
        .w(395.0)
        .h(321.0)
        .radius(radii::OVERLAY)
        .fill(c(k.card_background_fill_color_default))
        .border(1.0, c(k.card_stroke_color_default))
        .pt(6.0)
        .items_center()
        .child(head)
        .child(el().pt(1.0).child(ring))
        .child(
            row()
                .pt(19.0)
                .gap(12.0)
                .child(round_button(t, true))
                .child(round_button(t, false)),
        )
}

pub fn timer(t: &Theme) -> El {
    let k = t.tokens();
    let ink = k.text_fill_color_primary;
    let grid = col()
        .gap(14.0)
        .child(
            row()
                .gap(14.0)
                .child(timer_card(t, "1 min", "00:01:00"))
                .child(timer_card(t, "3 min", "00:03:00")),
        )
        .child(
            row()
                .gap(14.0)
                .child(timer_card(t, "5 min", "00:05:00"))
                .child(timer_card(t, "10 min", "00:10:00")),
        );
    let bar = row()
        .size(88.0, 46.0)
        .radius(radii::OVERLAY)
        .fill(c(k.control_fill_color_default))
        .border(1.0, c(k.card_stroke_color_default))
        .items_center()
        .px(4.0)
        .child(
            el().size(40.0, 36.0)
                .center()
                .child(sym(Symbol::Edit, 16.0, ink)),
        )
        .child(
            el().size(40.0, 36.0)
                .center()
                .child(sym(Symbol::Add, 16.0, ink)),
        );
    let content = el()
        .child(grid.pin(Some(56.0), Some(54.0), None, None))
        .child(bar.pin(None, None, Some(22.0), Some(22.0)));
    shell(t, Nav::Timer, content)
}

fn spotify(t: &Theme) -> El {
    let k = t.tokens();
    let bar = |w: f32| {
        el().size(w, 1.5)
            .radius(1.0)
            .fill(c(k.solid_background_fill_color_base))
    };
    col()
        .size(16.0, 16.0)
        .radius(8.0)
        .fill(c(k.text_fill_color_primary))
        .items_center()
        .justify_center()
        .gap(2.0)
        .child(bar(9.0))
        .child(bar(7.0))
        .child(bar(5.0))
}

fn setting(t: &Theme, icon: El, title: &str, sub: &str, trailing: El) -> El {
    let k = t.tokens();
    row()
        .w_full()
        .h(68.0)
        .radius(radii::CONTROL)
        .fill(c(k.card_background_fill_color_default))
        .border(1.0, c(k.card_stroke_color_default))
        .items_center()
        .pl(16.0)
        .pr(14.0)
        .child(el().w(41.0).child(icon))
        .child(
            col()
                .grow(1.0)
                .gap(1.0)
                .child(txt(title, t.type_ramp().body, k.text_fill_color_primary))
                .children(
                    (!sub.is_empty())
                        .then(|| txt(sub, t.type_ramp().caption, k.text_fill_color_secondary)),
                ),
        )
        .child(trailing)
}

fn on_switch(t: &Theme, chevron: bool) -> El {
    let k = t.tokens();
    row()
        .items_center()
        .gap(12.0)
        .child(txt("On", t.type_ramp().body, k.text_fill_color_primary))
        .child(ToggleSwitch::new().on(true).content("", "").view(t))
        .children(chevron.then(|| {
            el().w(28.0)
                .center()
                .child(sym(Symbol::ChevronDown, 12.0, k.text_fill_color_primary))
        }))
}

pub fn settings(t: &Theme) -> El {
    let k = t.tokens();
    let ink = k.text_fill_color_primary;
    let head = |s: &str| {
        el().pt(30.0)
            .pb(9.0)
            .child(txt(s, t.type_ramp().body_strong, ink))
    };
    let chevron = || {
        el().w(28.0)
            .center()
            .child(sym(Symbol::ChevronDown, 12.0, ink))
    };
    let ico = |d: &'static str| path_icon(d, 20.0, ink);
    let list = col()
        .w(804.0)
        .gap(4.0)
        .child(txt("Settings", t.type_ramp().title_large, ink))
        .child(el().h(12.0))
        .child(head("Account"))
        .child(setting(t, sym(Symbol::Person, 20.0, ink), "Sign in with a work, school, or Microsoft account", "", Button::new("Sign in").width(100.0).view(t)).h(64.0))
        .child(head("Focus sessions"))
        .child(setting(t, ico(g::CIRCLE_HALF), "Focus periods", "Adjust the lengths of your focus time, or breaks to fit your needs.", chevron()))
        .child(setting(t, ico(g::ALARM), "End of session sound", "Play an alarm when focus period ends", on_switch(t, true)))
        .child(setting(t, ico(g::ALARM), "End of break sound", "Play an alarm when breaks end", on_switch(t, true)))
        .child(setting(t, spotify(t), "Spotify", "You need the Spotify app to enhance your focus sessions with music and podcasts", on_switch(t, false)))
        .child(head("General"))
        .child(setting(t, sym(Symbol::Color, 20.0, ink), "App theme", "Choose your preferred app theme", row().items_center().gap(4.0).child(txt("Use system setting", t.type_ramp().body, ink)).child(chevron())))
        .child(setting(t, sym(Symbol::Alert, 20.0, ink), "Notifications", "Modify your notification settings", HyperlinkButton::new("Change notification settings").view(t)))
        .child(setting(t, sym(Symbol::Shield, 20.0, ink), "Privacy", "Your data is stored on your device for 90 days. Select Clear history to remove this data.", Button::new("Clear history").view(t)))
        .child(head("About"))
        .child(setting(t, sym(Symbol::Timer, 20.0, ink), "Clock", "\u{a9} Microsoft Corporation. All rights reserved.", chevron()));
    let content = el()
        .clip_rounded(0.0)
        .child(list.pin(Some(56.0), Some(36.0), None, None));
    shell(t, Nav::Settings, content)
}
