//! Windows 11 Clock — Timer page and Settings page replicas.

use crate::chrome::{self, hex};
use crate::fluent::{c, font, icon};
use rune::{canvas, col, el, row, text, El, LineCap, ScrollSpec, Shape, TextStyle};
use winui_icons::Symbol;
use winui_theme::{Color, Theme, radii, weights};

fn clock_icon() -> El {
    icon(Symbol::Clock, 16.0, hex(0xFFFFFF))
}

fn nav(t: &Theme, selected: usize) -> El {
    let items = [
        (Symbol::Clock, "Focus sessions", None, selected == 0),
        (Symbol::Timer, "Timer", None, selected == 1),
        (Symbol::Alert, "Alarm", None, selected == 2),
        (Symbol::Timer, "Stopwatch", None, selected == 3),
        (Symbol::Globe, "World clock", None, selected == 4),
    ];
    let footer = [
        (Symbol::Person, "Sign in", None, selected == 5),
        (Symbol::Settings, "Settings", None, selected == 6),
    ];
    col()
        .w(236.0)
        .h_full()
        .min_h(0.0)
        .child(
            col()
                .w_full()
                .min_h(0.0)
                .grow(1.0)
                .child({
                    let mut l = col().w_full().px(4.0).gap(2.0).pt(4.0);
                    for (s, l_, tint, sel) in items {
                        l = l.child(chrome::nav_row(t, s, l_, tint, sel));
                    }
                    l
                }),
        )
        .child({
            let mut l = col().w_full().px(4.0).gap(2.0).pb(4.0);
            for (s, l_, tint, sel) in footer {
                l = l.child(chrome::nav_row(t, s, l_, tint, sel));
            }
            l
        })
        .fill(c(hex(0x1B1B1B)))
}

fn ring(size: f32, track: Color) -> El {
    let r = (size - 8.0) / 2.0;
    let cx = size / 2.0;
    let d = format!(
        "M {cx} {cy} m -{r} 0 a {r} {r} 0 1 1 {d} 0 a {r} {r} 0 1 1 -{d} 0",
        cx = cx,
        cy = cx,
        r = r,
        d = 2.0 * r
    );
    canvas(
        Shape::new().stroke_svg(&d, 6.0, LineCap::Round, c(track)),
    )
    .size(size, size)
}

fn timer_card(t: &Theme, label: &str, time: &str) -> El {
    let k = t.tokens();
    chrome::card(t)
        .h(268.0)
        .grow(1.0)
        .min_w(0.0)
        .clip_rounded(radii::OVERLAY)
        .child(
            row().w_full().h(40.0).items_center().px(16.0).child(
                chrome::body(t, label, k.text_fill_color_primary).grow(1.0),
            )
            .child(
                row()
                    .gap(4.0)
                    .child(icon(Symbol::OpenInNew, 12.0, k.text_fill_color_primary))
                    .child(icon(Symbol::SquareMultiple, 12.0, k.text_fill_color_primary)),
            ),
        )
        .child(
            el().w_full().grow(1.0).min_h(0.0).center().child(
                el().size(176.0, 176.0)
                    .child(ring(176.0, hex(0x464646)))
                    .child(
                        el().pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                            .center()
                            .child(
                                text(time)
                                    .font(TextStyle::new(30.0, weights::REGULAR as f32))
                                    .color(c(k.text_fill_color_primary)),
                            ),
                    ),
            ),
        )
        .child(
            row().w_full().h(56.0).justify_center().items_center().gap(8.0)
                .child(
                    el().size(36.0, 36.0)
                        .radius(18.0)
                        .fill(c(k.accent_fill_color_default))
                        .hover(c(k.accent_fill_color_secondary))
                        .center()
                        .child(icon(Symbol::Play, 16.0, k.text_on_accent_fill_color_primary)),
                )
                .child(
                    el().size(36.0, 36.0)
                        .radius(18.0)
                        .fill(c(k.control_fill_color_default))
                        .hover(c(k.control_fill_color_secondary))
                        .center()
                        .child(icon(
                            Symbol::ArrowCounterclockwise,
                            16.0,
                            k.text_fill_color_primary,
                        )),
                ),
        )
}

/// The floating edit/new pill at bottom-right of the Timer page.
fn fab(t: &Theme) -> El {
    let k = t.tokens();
    chrome::card(t)
        .w(72.0)
        .h(44.0)
        .radius(8.0)
        .clip_rounded(8.0)
        .child(
            row().w_full().h_full().items_center().justify_center().gap(10.0)
                .child(icon(Symbol::Edit, 16.0, k.text_fill_color_primary))
                .child(icon(Symbol::Add, 16.0, k.text_fill_color_primary)),
        )
        .pin(None, None, Some(48.0), Some(48.0))
}

pub fn timer(t: &Theme) -> El {
    let cards = [("1 min", "00:01:00"), ("3 min", "00:03:00"), ("5 min", "00:05:00"), ("10 min", "00:10:00")];
    let mut grid = col().w_full().px(48.0).pt(16.0).gap(10.0);
    for r in 0..2 {
        let mut line = row().w_full().gap(10.0);
        for c_i in 0..2 {
            let (label, time) = cards[r * 2 + c_i];
            line = line.child(timer_card(t, label, time));
        }
        grid = grid.child(line);
    }
    let content = row()
        .w_full()
        .h_full()
        .child(nav(t, 1))
        .child(
            el().grow(1.0).min_w(0.0).h_full().child(grid).child(fab(t)),
        );
    chrome::shell(
        t,
        chrome::title_bar(t, clock_icon(), "Clock", 12.0)
            .child(el().grow(1.0))
            .child(chrome::captions(t)),
        content,
    )
}

// ----------------------------------------------------------------- settings

/// One settings card row: icon, title+description, trailing widgets.
fn setting_card(
    t: &Theme,
    sym: Symbol,
    title: &str,
    desc: &str,
    trailing: El,
) -> El {
    let k = t.tokens();
    chrome::card(t).child(
        row()
            .w_full()
            .min_h(56.0)
            .items_center()
            .pl(16.0)
            .pr(16.0)
            .py(7.0)
            .child(
                el().size(30.0, 30.0)
                    .center()
                    .child(icon(sym, 18.0, k.text_fill_color_primary)),
            )
            .child(
                col().grow(1.0).min_w(0.0).pl(8.0).gap(2.0)
                    .child(
                        text(title)
                            .font(font(t.type_ramp().body))
                            .color(c(k.text_fill_color_primary)),
                    )
                    .child(
                        text(desc)
                            .font(font(t.type_ramp().caption))
                            .color(c(k.text_fill_color_secondary))
                            .wrap(),
                    ),
            )
            .child(trailing),
    )
}

fn chevron_down(t: &Theme) -> El {
    icon(Symbol::ChevronDown, 12.0, t.tokens().text_fill_color_primary)
}

pub fn settings(t: &Theme) -> El {
    let k = t.tokens();
    let page = col()
        .w_full()
        .min_h(0.0)
        .grow(1.0)
        .scroll(ScrollSpec::default())
        .child(
            col()
                .w_full()
                .max_w(955.0)
                .pl(60.0)
                .pr(48.0)
                .pb(32.0)
                .child(
                    el().h(80.0).items_end().pb(10.0).child(
                        text("Settings")
                            .font(font(t.type_ramp().title))
                            .color(c(k.text_fill_color_primary)),
                    ),
                )
                .child(chrome::section_header(t, "Account"))
                .child(setting_card(
                    t,
                    Symbol::Person,
                    "Sign in with a work, school, or Microsoft account",
                    "",
                    crate::controls::button(t, "Sign in", false, true),
                ))
                .child(chrome::section_header(t, "Focus sessions"))
                .child(setting_card(
                    t,
                    Symbol::Timer,
                    "Focus periods",
                    "Adjust the lengths of your focus time, or breaks to fit your needs.",
                    chevron_down(t),
                ))
                .child(el().h(4.0))
                .child(setting_card(
                    t,
                    Symbol::Alert,
                    "End of session sound",
                    "Play an alarm when focus period ends",
                    row().items_center().gap(16.0)
                        .child(chrome::body(t, "On", k.text_fill_color_primary))
                        .child(chrome::pill(t, true))
                        .child(chevron_down(t)),
                ))
                .child(el().h(4.0))
                .child(setting_card(
                    t,
                    Symbol::Alert,
                    "End of break sound",
                    "Play an alarm when breaks end",
                    row().items_center().gap(16.0)
                        .child(chrome::body(t, "On", k.text_fill_color_primary))
                        .child(chrome::pill(t, true))
                        .child(chevron_down(t)),
                ))
                .child(el().h(4.0))
                .child(setting_card(
                    t,
                    Symbol::MusicNote,
                    "Spotify",
                    "You need the Spotify app to enhance your focus sessions with music and podcasts",
                    row().items_center().gap(16.0)
                        .child(chrome::body(t, "On", k.text_fill_color_primary))
                        .child(chrome::pill(t, true)),
                ))
                .child(chrome::section_header(t, "General"))
                .child(setting_card(
                    t,
                    Symbol::Color,
                    "App theme",
                    "Choose your preferred app theme",
                    row().items_center().gap(8.0)
                        .child(chrome::body(t, "Use system setting", k.text_fill_color_primary))
                        .child(chevron_down(t)),
                ))
                .child(el().h(4.0))
                .child(setting_card(
                    t,
                    Symbol::Alert,
                    "Notifications",
                    "Modify your notification settings",
                    chrome::body(t, "Change notification settings", k.text_fill_color_primary),
                ))
                .child(el().h(4.0))
                .child(setting_card(
                    t,
                    Symbol::Shield,
                    "Privacy",
                    "Your data is stored on your device for 90 days. Select Clear history to remove this data.",
                    crate::controls::button(t, "Clear history", false, true),
                ))
                .child(chrome::section_header(t, "About"))
                .child(setting_card(
                    t,
                    Symbol::Info,
                    "About",
                    "Version, system information, and app details",
                    chevron_down(t),
                )),
        );

    let content = row()
        .w_full()
        .h_full()
        .child(nav(t, 6))
        .child(el().grow(1.0).min_w(0.0).h_full().child(page));
    chrome::shell(
        t,
        chrome::title_bar(t, clock_icon(), "Clock", 12.0)
            .child(el().grow(1.0))
            .child(chrome::captions(t)),
        content,
    )
}
