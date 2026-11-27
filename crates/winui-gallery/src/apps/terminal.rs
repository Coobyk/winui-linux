//! Windows Terminal (`terminal.png`).

use super::{WHITE, art, caption_buttons, icons, label, mono, path_icon, rgb, rgba};
use crate::State;
use rune::{El, el, row};

const STRIP: u32 = 0x2E2E2E;
const BODY: u32 = 0x0C0C0C;
const FG: u32 = 0xCCCCCC;

fn ps_icon() -> El {
    art(
        16.0,
        12.0,
        [16.0, 12.0],
        &[
            ("M3 0 h12.2 a0.8 0.8 0 0 1 0.8 1 L13.6 11.2 a1 1 0 0 1 -1 0.8 H0.8 a0.8 0.8 0 0 1 -0.8 -1 L2.1 0.7 a1 1 0 0 1 0.9 -0.7Z", 0x4980E1, 0.0),
            ("M4 2.6 L8 6 L3.4 9.4", WHITE, 1.3),
            ("M8 9.4 h3.5", WHITE, 1.3),
        ],
    )
}

fn x_glyph(size: f32, color: u32) -> El {
    art(10.0, 10.0, [size, size], &[("M0.6 0.6 L9.4 9.4 M9.4 0.6 L0.6 9.4", color, 1.1)])
}

fn tab_button(content: El, x: f32, name: &'static str) -> El {
    el().absolute(x, 8.0)
        .size(32.0, 32.0)
        .radius(4.0)
        .center()
        .hover(rgba(WHITE, 0.0605))
        .role("button")
        .label(name)
        .child(content)
}

pub fn view(_s: &State) -> El {
    let plus = path_icon(icons::ADD, 16.0, 0xF7F7F7);
    let tab = el()
        .absolute(10.0, 9.0)
        .size(241.0, 32.0)
        .corners([8.0, 8.0, 0.0, 0.0])
        .fill(rgb(BODY))
        .role("tab")
        .label("Windows PowerShell")
        .child(ps_icon().absolute(9.0, 10.0))
        .child(label("Windows PowerShell", 12.0, 600.0, WHITE).absolute(35.0, 9.0))
        .child(
            el().absolute(204.0, 4.0)
                .size(28.0, 24.0)
                .radius(4.0)
                .center()
                .hover(rgba(WHITE, 0.0605))
                .role("button")
                .label("Close tab")
                .child(x_glyph(9.0, WHITE).ml(1.0)),
        );
    let line = |t: &'static str, y: f32| mono(t, 15.3, FG).absolute(9.0, y);
    el().w_full()
        .h_full()
        .fill(rgb(BODY))
        .child(el().absolute(0.0, 0.0).size(1153.0, 40.0).fill(rgb(STRIP)).window_drag())
        .child(tab)
        .child(tab_button(plus, 257.0, "New tab"))
        .child(tab_button(path_icon(icons::CHEVRON_DOWN, 12.0, 0xDEDEDE), 287.0, "Open a profile"))
        .child(row().absolute(1015.0, 0.0).child(caption_buttons(40.0, WHITE).mt(1.0)))
        .child(line("Windows PowerShell 5.1", 48.0))
        .child(line("Copyright (C) Microsoft Corporation. All rights reserved.", 67.0))
        .child(line("PS C:\\Users\\Coobyk> ", 106.0))
        .child(el().absolute(189.0, 106.0).size(1.0, 19.0).fill(rgb(0xFFFFFF)))
}
