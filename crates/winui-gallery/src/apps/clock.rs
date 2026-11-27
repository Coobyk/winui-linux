//! Clock > Timer (`clock-timer.png`) and Clock > Settings (`clock-settings.png`).

use super::{ACCENT, WHITE, art, caption_buttons, icons, label, path_icon, rgb, rgba};
use crate::State;
use rune::{El, Shape, canvas, el, row};

const NAV_BG: u32 = 0x202020;
const PANE: u32 = 0x272727;
const CARD: u32 = 0x323232;
const STROKE: u32 = 0x232323;
const SUB: u32 = 0xD0D0D0;

fn logo() -> El {
    art(
        14.0,
        14.0,
        [14.0, 14.0],
        &[
            ("M7 0.5 L13 3.5 V10.5 L7 13.5 L1 10.5 V3.5Z", 0xD6D6D8, 0.0),
            ("M7 0.5 L13 3.5 L7 6.5 L1 3.5Z", 0xFAFAFB, 0.0),
            ("M7 6.5 V13.5 L1 10.5 V3.5Z", 0x9C9CA0, 0.0),
            ("M7 3.6 V6.5 L9.2 5.4", 0x3A3A3A, 1.0),
        ],
    )
}

fn nav_row(icon: &'static str, t: &'static str, y: f32, sel: bool) -> El {
    let mut r = row()
        .absolute(5.0, y)
        .size(272.0, 36.0)
        .items_center()
        .radius(4.0)
        .hover(rgba(WHITE, 0.0605))
        .press(rgba(WHITE, 0.0419))
        .role("button")
        .label(t)
        .child(path_icon(icon, 20.0, WHITE).ml(10.0))
        .child(label(t, 14.0, 400.0, WHITE).ml(14.0));
    if sel {
        r = r.fill(rgba(WHITE, 0.0512)).child(
            el().absolute(0.0, 10.0)
                .size(3.0, 16.0)
                .radius(1.5)
                .fill(rgb(ACCENT)),
        );
    }
    r
}

/// The NavigationView pane shared by both Clock pages; `page` 1 = Timer, 7 = Settings.
fn nav(page: usize) -> El {
    let items = [
        (icons::CIRCLE_HALF_FILL, "Focus sessions"),
        (icons::HOURGLASS, "Timer"),
        (icons::ALERT, "Alarm"),
        (icons::TIMER, "Stopwatch"),
        (icons::GLOBE, "World clock"),
    ];
    let mut n = el()
        .absolute(0.0, 0.0)
        .size(282.0, 933.0)
        .fill(rgb(NAV_BG))
        .border_sides([0.0, 1.0, 0.0, 0.0], rgb(0x1D1D1D))
        .child(logo().absolute(17.5, 11.5))
        .child(label("Clock", 12.0, 400.0, WHITE).absolute(47.0, 12.0));
    for (i, (ic, t)) in items.into_iter().enumerate() {
        n = n.child(nav_row(ic, t, 39.0 + 40.0 * i as f32, page == i));
    }
    n.child(nav_row(icons::PERSON, "Sign in", 849.0, false))
        .child(nav_row(icons::SETTINGS, "Settings", 890.0, page == 7))
}

fn frame(page: usize, content: El) -> El {
    el().w_full()
        .h_full()
        .fill(rgb(PANE))
        .child(content)
        .child(nav(page))
        .child(el().absolute(282.0, 0.0).size(919.0, 33.0).window_drag())
        .child(caption_buttons(33.0, 0x737373).absolute(1063.0, 0.0))
}

fn ring(d: f32, stroke: f32, color: u32) -> El {
    let r = (d - stroke) / 2.0;
    let c = d / 2.0;
    let p = format!(
        "M{} {c} a{r} {r} 0 1 0 {} 0 a{r} {r} 0 1 0 {} 0",
        c - r,
        2.0 * r,
        -2.0 * r
    );
    canvas(
        Shape::new()
            .stroke_svg(&p, stroke, rune::LineCap::Butt, rgb(color))
            .view_box(d, d),
    )
    .size(d, d)
}

fn round_button(d: f32, fill: u32, icon: El, name: &'static str) -> El {
    el().size(d, d)
        .radius(d / 2.0)
        .fill(rgb(fill))
        .hover(rgba(fill, 0.9))
        .center()
        .role("button")
        .label(name)
        .child(icon)
}

fn timer_card(x: f32, y: f32, mins: u32) -> El {
    let play = art(
        10.0,
        10.0,
        [11.0, 11.0],
        &[(
            "M1.5 0.8 C1.5 0.2 2.1 -0.1 2.6 0.2 L9.2 4.3 C9.7 4.6 9.7 5.4 9.2 5.7 L2.6 9.8 C2.1 10.1 1.5 9.8 1.5 9.2Z",
            0x000000,
            0.0,
        )],
    );
    el().absolute(x, y)
        .size(396.0, 323.0)
        .radius(8.0)
        .fill(rgb(CARD))
        .border(1.0, rgb(STROKE))
        .shadow(2.0, 4.0, rgba(0x000000, 0.13))
        .child(label(format!("{mins} min"), 14.0, 600.0, WHITE).absolute(15.0, 9.5))
        .child(path_icon(icons::ARROW_MAXIMIZE, 18.0, WHITE).absolute(327.0, 9.5))
        .child(path_icon(icons::WINDOW_ARROW_UP, 18.0, WHITE).absolute(367.0, 9.5))
        .child(ring(210.0, 14.0, 0x3E3E3E).absolute(93.0, 46.0))
        .child(
            row()
                .absolute(93.0, 46.0)
                .size(210.0, 210.0)
                .center()
                .child(
                    label(format!("00:{mins:02}:00"), 38.0, 600.0, SUB)
                        .mt(-8.0)
                        .mr(10.0),
                ),
        )
        .child(round_button(30.0, ACCENT, play.ml(1.0), "Start").absolute(162.0, 271.0))
        .child(
            round_button(
                30.0,
                0x3B3B3B,
                path_icon(icons::ARROW_COUNTERCLOCKWISE, 14.0, 0x7F7F7F),
                "Reset",
            )
            .absolute(204.0, 271.0),
        )
}

pub fn timer(_s: &State) -> El {
    let fab = row()
        .absolute(1090.0, 864.0)
        .size(90.0, 48.0)
        .radius(8.0)
        .fill(rgb(0x2D2D2D))
        .border(1.0, rgb(0x3A3A3A))
        .shadow(4.0, 12.0, rgba(0x000000, 0.26))
        .items_center()
        .child(path_icon(icons::EDIT, 20.0, WHITE).ml(14.0))
        .child(path_icon(icons::ADD, 18.0, WHITE).ml(20.0));
    let content = el()
        .absolute(0.0, 0.0)
        .w_full()
        .h_full()
        .child(timer_card(339.0, 54.0, 1))
        .child(timer_card(748.0, 54.0, 3))
        .child(timer_card(339.0, 390.0, 5))
        .child(timer_card(748.0, 390.0, 10))
        .child(fab);
    frame(1, content)
}

fn toggle_on() -> El {
    el().size(40.0, 20.0)
        .radius(10.0)
        .fill(rgb(ACCENT))
        .hover(rgba(ACCENT, 0.9))
        .role("switch")
        .child(
            el().absolute(24.0, 4.0)
                .size(12.0, 12.0)
                .radius(6.0)
                .fill(rgb(0x000000)),
        )
}

fn chevron_down() -> El {
    path_icon(icons::CHEVRON_DOWN, 17.5, 0xFDFDFD)
}

fn std_button(t: &'static str, w: f32) -> El {
    el().size(w, 32.0)
        .radius(4.0)
        .fill(rgb(0x3E3E3E))
        .border_sides([0.0, 1.0, 0.0, 1.0], rgb(0x444444))
        .hover(rgb(0x454545))
        .center()
        .role("button")
        .label(t)
        .child(label(t, 14.0, 400.0, WHITE).mt(-1.0))
}

/// A SettingsCard row at `y`, `h` tall: icon, title (+ subtitle), trailing content.
fn card(y: f32, h: f32, icon: El, t: &'static str, sub: Option<&'static str>, trail: El) -> El {
    let mut c = el()
        .absolute(339.0, y)
        .size(805.0, h)
        .radius(4.0)
        .fill(rgb(CARD))
        .border(1.0, rgb(STROKE))
        .hover(rgb(0x363636))
        .child(icon.absolute(15.0, (h - 24.0) / 2.0));
    c = match sub {
        Some(s) => c
            .child(label(t, 14.0, 400.0, WHITE).absolute(58.0, (h / 2.0 - 20.0).round()))
            .child(label(s, 12.3, 400.0, SUB).absolute(58.0, (h / 2.0 + 2.0).round())),
        None => c.child(label(t, 14.0, 400.0, WHITE).absolute(58.0, (h / 2.0 - 9.0).round())),
    };
    c.child(
        row()
            .absolute(0.0, 0.0)
            .size(788.0, h)
            .justify_end()
            .items_center()
            .child(trail),
    )
}

fn on_chevron(chev: bool) -> El {
    let r = row()
        .items_center()
        .child(label("On", 14.0, 400.0, WHITE).mt(-2.0))
        .child(toggle_on().ml(12.0));
    if chev {
        r.child(chevron_down().ml(10.0).mr(0.5))
    } else {
        r.mr(0.0)
    }
}

fn spotify() -> El {
    art(
        20.0,
        20.0,
        [20.0, 20.0],
        &[
            ("M10 1 a9 9 0 1 1 0 18 a9 9 0 1 1 0 -18Z", WHITE, 0.0),
            (
                "M5 7.6 C8.4 6.5 12.2 6.8 15.2 8.4 M5.6 10.6 C8.4 9.8 11.6 10.1 14 11.4 M6.2 13.4 C8.4 12.8 10.8 13 12.8 14",
                0x323232,
                1.4,
            ),
        ],
    )
}

pub fn settings(_s: &State) -> El {
    let icon = |d: &'static str| path_icon(d, 24.0, WHITE);
    let combo = row()
        .items_center()
        .child(label("Use system setting", 14.0, 400.0, WHITE).mt(-2.0))
        .child(chevron_down().ml(11.0).mr(0.5));
    let content = el()
        .absolute(0.0, 0.0)
        .w_full()
        .h_full()
        .child(label("Settings", 40.0, 600.0, WHITE).absolute(338.0, 34.0))
        .child(label("Account", 14.0, 600.0, WHITE).absolute(339.0, 140.0))
        .child(card(167.0, 68.0, icon(icons::PERSON), "Sign in with a work, school, or Microsoft account", None, std_button("Sign in", 117.0)))
        .child(label("Focus sessions", 14.0, 600.0, WHITE).absolute(339.0, 266.0))
        .child(card(294.0, 70.0, icon(icons::CIRCLE_HALF_FILL), "Focus periods", Some("Adjust the lengths of your focus time, or breaks to fit your needs."), chevron_down().mr(0.5)))
        .child(card(367.0, 70.0, icon(icons::CLOCK_ALARM), "End of session sound", Some("Play an alarm when focus period ends"), on_chevron(true)))
        .child(card(440.0, 70.0, icon(icons::CLOCK_ALARM), "End of break sound", Some("Play an alarm when breaks end"), on_chevron(true)))
        .child(card(513.0, 70.0, spotify(), "Spotify", Some("You need the Spotify app to enhance your focus sessions with music and podcasts"), on_chevron(false)))
        .child(label("General", 14.0, 600.0, WHITE).absolute(339.0, 618.0))
        .child(card(645.0, 70.0, icon(icons::COLOR), "App theme", Some("Choose your preferred app theme"), combo))
        .child(card(718.0, 69.0, icon(icons::ALERT), "Notifications", Some("Modify your notification settings"), label("Change notification settings", 14.0, 400.0, 0x99EBFF).role("link").mt(-2.0).mr(1.0)))
        .child(card(790.0, 69.0, icon(icons::DOCUMENT_LOCK), "Privacy", Some("Your data is stored on your device for 90 days. Select Clear history to remove this data."), std_button("Clear history", 120.0)))
        .child(label("About", 14.0, 600.0, WHITE).absolute(339.0, 891.0))
        .child(el().absolute(339.0, 918.0).size(805.0, 60.0).radius(4.0).fill(rgb(CARD)).border(1.0, rgb(STROKE)))
        .child(el().absolute(1194.0, 128.0).size(2.0, 680.0).radius(1.0).fill(rgb(0x9D9D9D)));
    frame(7, content)
}
