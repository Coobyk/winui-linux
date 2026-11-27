//! Windows Terminal replica: tab strip + PowerShell profile surface.

use crate::chrome::{self, hex};
use crate::fluent::{c, icon};
use rune::{col, el, row, text, El, Family, TextStyle};
use winui_icons::Symbol;
use winui_theme::{Theme, radii, weights};

fn ps_icon() -> El {
    el().size(16.0, 16.0)
        .radius(3.0)
        .fill(c(hex(0x5391FE)))
        .center()
        .child(
            text(">_")
                .font(TextStyle::new(9.0, weights::BOLD as f32).mono())
                .color(c(hex(0x0C0C0C))),
        )
}

fn tab(t: &Theme, title: &str, selected: bool) -> El {
    let k = t.tokens();
    row()
        .h(32.0)
        .w(220.0)
        .items_center()
        .px(10.0)
        .radius(radii::CONTROL)
        .fill(c(if selected {
            hex(0x333333)
        } else {
            hex(0x2B2B2B)
        }))
        .child(ps_icon())
        .child(
            chrome::caption(t, title, k.text_fill_color_primary)
                .ml(8.0)
                .grow(1.0)
                .min_w(0.0)
                .truncate(),
        )
        .child(
            el().size(24.0, 24.0)
                .center()
                .radius(radii::CONTROL)
                .hover(c(k.subtle_fill_color_secondary))
                .child(icon(Symbol::Dismiss, 10.0, k.text_fill_color_primary)),
        )
}

fn mono(s: &str, color: u32) -> El {
    let mut st = TextStyle::new(13.5, weights::REGULAR as f32);
    st.family = Family::SystemMono;
    st.line_height = 20.0;
    text(s).font(st).color(c(hex(color)))
}

pub fn view(t: &Theme) -> El {
    let k = t.tokens();
    let strip = row()
        .w_full()
        .h(40.0)
        .items_center()
        .px(6.0)
        .gap(4.0)
        .window_drag()
        .child(tab(t, "Windows PowerShell", true))
        .child(
            el().size(32.0, 32.0)
                .center()
                .radius(radii::CONTROL)
                .hover(c(k.subtle_fill_color_secondary))
                .child(icon(Symbol::Add, 14.0, k.text_fill_color_primary)),
        )
        .child(
            el().size(32.0, 32.0)
                .center()
                .radius(radii::CONTROL)
                .hover(c(k.subtle_fill_color_secondary))
                .child(icon(Symbol::ChevronDown, 12.0, k.text_fill_color_primary)),
        )
        .child(el().grow(1.0).min_w(0.0))
        .child(chrome::captions(t));

    let cursor = el()
        .size(8.0, 16.0)
        .fill(c(hex(0xCCCCCC)));

    let surface = col()
        .w_full()
        .h_full()
        .fill(c(hex(0x0C0C0C)))
        .px(8.0)
        .pt(4.0)
        .child(mono("Windows PowerShell 5.1", 0xCCCCCC))
        .child(mono("Copyright (C) Microsoft Corporation. All rights reserved.", 0xCCCCCC))
        .child(el().h(20.0))
        .child(
            row().items_center().child(mono("PS C:\\Users\\Coobyk> ", 0xCCCCCC)).child(cursor),
        );

    let body = col()
        .w_full()
        .h_full()
        .child(strip)
        .child(surface.grow(1.0).min_h(0.0));

    // No extra title bar: the tab strip *is* the title bar in Terminal.
    el().w_full()
        .h_full()
        .fill(c(hex(0x202020)))
        .child(body)
}
