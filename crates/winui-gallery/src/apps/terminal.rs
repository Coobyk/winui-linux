//! Windows Terminal running Windows PowerShell (`docs/reference/terminal.png`).

use rune::{El, col, el, row, text};
use winui_icons::Symbol;
use winui_theme::{Color, Theme, radii};

use super::{caption_buttons, sym, txt};
use crate::fluent::{c, font};

/// Terminal's default "Campbell" color scheme (microsoft/terminal `defaults.json`): the
/// terminal surface is the color scheme, not the XAML theme.
const CAMPBELL_BACKGROUND: Color = Color::from_argb(0xFF0C0C0C);
const CAMPBELL_FOREGROUND: Color = Color::from_argb(0xFFCCCCCC);
const CAMPBELL_BLUE: Color = Color::from_argb(0xFF0037DA);

fn powershell_icon(t: &Theme) -> El {
    let _ = t;
    el().size(16.0, 14.0)
        .radius(2.0)
        .fill(c(CAMPBELL_BLUE))
        .pl(3.0)
        .items_center()
        .child(
            text(">_")
                .font(font(super::px_style(9.0, 700, 10.0)).mono())
                .color(c(Color::from_argb(0xFFFFFFFF))),
        )
}

pub fn view(t: &Theme) -> El {
    let k = t.tokens();
    let ink = k.text_fill_color_primary;
    let tab = row()
        .w(240.0)
        .h(32.0)
        .corners([radii::OVERLAY, radii::OVERLAY, 0.0, 0.0])
        .fill(c(CAMPBELL_BACKGROUND))
        .items_center()
        .pl(12.0)
        .pr(8.0)
        .child(powershell_icon(t))
        .child(el().pl(10.0).grow(1.0).child(txt(
            "Windows PowerShell",
            t.type_ramp().caption.with_weight(600),
            CAMPBELL_FOREGROUND,
        )))
        .child(
            el().size(28.0, 24.0)
                .center()
                .child(sym(Symbol::Dismiss, 12.0, CAMPBELL_FOREGROUND)),
        );
    let strip = row()
        .w_full()
        .h(41.0)
        .fill(c(k.solid_background_fill_color_quarternary))
        .items_end()
        .child(el().pl(9.0).child(tab))
        .child(
            el().size(34.0, 32.0)
                .ml(6.0)
                .center()
                .child(sym(Symbol::Add, 14.0, ink)),
        )
        .child(
            el().size(26.0, 32.0)
                .center()
                .child(sym(Symbol::ChevronDown, 12.0, ink)),
        )
        .child(el().grow(1.0))
        .child(caption_buttons(t, 41.0).pin(None, Some(0.0), Some(0.0), None));
    let mono = font(super::px_style(15.0, 400, 20.0)).mono();
    let line = |s: &str| text(s.to_string()).font(mono).color(c(CAMPBELL_FOREGROUND));
    let cursor = el().size(1.0, 18.0).fill(c(CAMPBELL_FOREGROUND));
    let screen = col()
        .w_full()
        .grow(1.0)
        .fill(c(CAMPBELL_BACKGROUND))
        .pl(9.0)
        .pt(5.0)
        .child(line("Windows PowerShell 5.1"))
        .child(line(
            "Copyright (C) Microsoft Corporation. All rights reserved.",
        ))
        .child(el().h(20.0))
        .child(
            row()
                .items_center()
                .child(line("PS C:\\Users\\Coobyk> "))
                .child(el().pl(1.0).child(cursor)),
        );
    col().child(strip).child(screen)
}
