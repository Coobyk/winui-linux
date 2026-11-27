//! Snipping Tool (`snipping-tool.png`).

use super::{ACCENT, WHITE, art, caption_buttons, icons, label, path_icon, rgb, rgba};
use crate::State;
use rune::{El, el, row};

const BAR: u32 = 0x202020;
const BODY: u32 = 0x272727;

fn logo() -> El {
    art(
        16.0,
        16.0,
        [16.0, 16.0],
        &[
            (
                "M1 2 a1.5 1.5 0 0 1 1.5 -1.5 h9 a1.5 1.5 0 0 1 1.5 1.5 v9 a1.5 1.5 0 0 1 -1.5 1.5 h-9 a1.5 1.5 0 0 1 -1.5 -1.5Z",
                0xECEFF1,
                0.0,
            ),
            (
                "M3 3 h2 v2 h-2z M7 3 h2 v2 h-2z M3 7 h2 v2 h-2z",
                0xE8553F,
                0.0,
            ),
            ("M8 9 L14.5 15.5 M14.5 9 L8 15.5", 0x2493C7, 1.6),
        ],
    )
}

fn ghost(content: El, w: f32, name: &'static str) -> El {
    row()
        .size(w, 32.0)
        .radius(4.0)
        .center()
        .hover(rgba(WHITE, 0.0605))
        .press(rgba(WHITE, 0.0419))
        .role("button")
        .label(name)
        .child(content)
}

fn with_chevron(icon: &'static str) -> El {
    row()
        .items_center()
        .child(path_icon(icon, 20.0, WHITE))
        .child(
            path_icon(icons::CHEVRON_DOWN, 12.0, 0xCACACA)
                .ml(2.0)
                .mt(1.0),
        )
}

/// The ⊞ Windows key glyph.
fn win_key() -> El {
    art(
        10.0,
        10.0,
        [10.0, 10.0],
        &[(
            "M0 0 h4.6 v4.6 h-4.6z M5.4 0 h4.6 v4.6 h-4.6z M0 5.4 h4.6 v4.6 h-4.6z M5.4 5.4 h4.6 v4.6 h-4.6z",
            WHITE,
            0.0,
        )],
    )
}

pub fn view(_s: &State) -> El {
    let new = row()
        .absolute(17.0, 39.0)
        .size(77.0, 32.0)
        .radius(4.0)
        .fill(rgb(0x2D2D2D))
        .border_sides([1.0, 0.0, 1.0, 0.0], rgb(0x333333))
        .hover(rgb(0x323232))
        .items_center()
        .role("button")
        .label("New")
        .child(path_icon(icons::ADD, 20.0, ACCENT).ml(10.0).mt(-5.0))
        .child(label("New", 14.0, 400.0, WHITE).ml(8.0).mt(-1.0));
    let seg = el()
        .absolute(110.0, 39.0)
        .size(76.0, 32.0)
        .radius(4.0)
        .fill(rgb(0x1D1D1D))
        .border(1.0, rgb(0x2C2C2C))
        .child(
            el().absolute(0.0, 0.0)
                .size(38.0, 32.0)
                .radius(4.0)
                .fill(rgb(0x2A2A2A))
                .border_sides([1.0, 1.0, 1.0, 1.0], rgb(0x3B3B3B))
                .center()
                .role("button")
                .label("Snip")
                .child(path_icon(icons::CAMERA, 20.0, WHITE).mt(-2.0))
                .child(
                    el().absolute(12.0, 27.5)
                        .size(14.0, 2.5)
                        .radius(1.5)
                        .fill(rgb(ACCENT)),
                ),
        )
        .child(
            el().absolute(38.0, 0.0)
                .size(38.0, 32.0)
                .center()
                .role("button")
                .label("Record")
                .child(path_icon(icons::VIDEO, 20.0, WHITE).mt(-2.0)),
        );
    let msg = row()
        .absolute(0.0, 80.0)
        .size(502.0, 120.0)
        .justify_center()
        .items_center()
        .child(label("Press ", 14.0, 400.0, WHITE))
        .child(win_key().ml(1.0).mt(3.0))
        .child(label(" + Shift + S", 14.0, 600.0, WHITE))
        .child(label(" to start a snip", 14.0, 400.0, WHITE));
    el().w_full()
        .h_full()
        .fill(rgb(BODY))
        .child(
            el().absolute(0.0, 0.0)
                .size(502.0, 80.0)
                .fill(rgb(BAR))
                .border_sides([0.0, 0.0, 1.0, 0.0], rgb(0x313131))
                .window_drag(),
        )
        .child(logo().absolute(8.5, 6.5))
        .child(label("Snipping Tool", 12.0, 400.0, 0x797979).absolute(32.0, 8.0))
        .child(caption_buttons(30.0, 0x797979).absolute(364.0, 0.0))
        .child(new)
        .child(seg)
        .child(ghost(with_chevron(icons::SCREENSHOT), 48.0, "Snipping mode").absolute(201.0, 38.5))
        .child(ghost(with_chevron(icons::TIMER_OFF), 48.0, "Delay").absolute(257.0, 38.5))
        .child(
            el().absolute(314.0, 43.0)
                .size(1.0, 24.0)
                .fill(rgb(0x2B2B2B)),
        )
        .child(
            ghost(
                path_icon(icons::SCAN_TEXT, 18.0, WHITE),
                32.0,
                "Text actions",
            )
            .absolute(323.0, 39.0),
        )
        .child(
            ghost(
                path_icon(icons::MORE_HORIZONTAL, 26.0, WHITE),
                40.0,
                "See more",
            )
            .absolute(455.0, 39.0),
        )
        .child(msg)
}
