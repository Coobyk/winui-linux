//! `ref-terminal`: Windows Terminal with a PowerShell tab (docs/reference/terminal.png).

use rune::{El, TextStyle, el, row, text};
use winui_controls::style::rc;
use winui_icons::Symbol;
use winui_theme::{Color, Theme, ThemeMode, radii};

use super::{caption_buttons, icon, label};

/// Campbell scheme background / foreground.
const BG: u32 = 0xFF0C0C0C;
const FG: u32 = 0xFFCCCCCC;

pub fn view(t: &Theme) -> El {
    // Windows Terminal keeps its dark chrome and Campbell scheme whatever the app theme.
    let t = &Theme::new(ThemeMode::Dark, *t.accent());
    let k = t.tokens();
    let ramp = t.type_ramp();
    let ps = el()
        .size(16.0, 16.0)
        .radius(2.0)
        .fill(rc(Color::from_argb(0xFF2C5DCD)))
        .center()
        .child(icon(
            Symbol::ChevronRight,
            12.0,
            Color::from_argb(0xFFFFFFFF),
        ));
    let tab = row()
        .absolute(10.0, 8.0)
        .size(240.0, 33.0)
        .corners([radii::OVERLAY, radii::OVERLAY, 0.0, 0.0])
        .fill(rc(Color::from_argb(BG)))
        .pl(10.0)
        .pr(6.0)
        .gap(10.0)
        .center_y()
        .child(ps)
        .child(
            label(
                "Windows PowerShell",
                ramp.caption,
                k.text_fill_color_primary,
            )
            .grow(1.0),
        )
        .child(
            el().size(28.0, 24.0)
                .radius(radii::CONTROL)
                .center()
                .hover_fill(rc(k.subtle_fill_color_secondary))
                .child(icon(Symbol::Dismiss, 12.0, k.text_fill_color_primary)),
        );
    let small = |sym: Symbol, x: f32, w: f32| {
        el().absolute(x, 12.0)
            .size(w, 26.0)
            .radius(radii::CONTROL)
            .center()
            .hover_fill(rc(k.subtle_fill_color_secondary))
            .child(icon(sym, 12.0, k.text_fill_color_primary))
    };
    let strip = el()
        .w_full()
        .h(41.0)
        .fill(rc(Color::from_argb(0xFF2E2E2E)))
        .window_drag()
        .child(tab)
        .child(small(Symbol::Add, 256.0, 32.0))
        .child(small(Symbol::ChevronDown, 290.0, 24.0))
        .child(caption_buttons(t).pin(None, Some(4.0), Some(0.0), None));
    let mono = TextStyle::new(16.0, 400.0).mono().lh(20.5);
    let line = |s: &str| {
        text(s.to_owned())
            .font(mono)
            .color(rc(Color::from_argb(FG)))
    };
    let body = rune::col()
        .w_full()
        .grow(1.0)
        .fill(rc(Color::from_argb(BG)))
        .pl(9.0)
        .pt(6.0)
        .child(line("Windows PowerShell 5.1"))
        .child(line(
            "Copyright (C) Microsoft Corporation. All rights reserved.",
        ))
        .child(line(" "))
        .child(
            row()
                .center_y()
                .child(line("PS C:\\Users\\Coobyk> "))
                .child(el().size(1.0, 18.0).fill(rc(Color::from_argb(0xFFFFFFFF)))),
        );
    rune::col()
        .w_full()
        .h_full()
        .child(strip)
        .child(body)
        .clip_rounded(0.0)
}
