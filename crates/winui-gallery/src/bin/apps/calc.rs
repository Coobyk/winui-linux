//! Windows 11 Calculator — Standard mode (docs/reference calc shot, 328×532).

use crate::chrome::{self, hex};
use crate::fluent::{c, icon};
use rune::{canvas, col, el, row, text, El, LineCap, Path, Shape, TextStyle};
use winui_icons::Symbol;
use winui_theme::{Color, Theme, radii, weights};

const OPS_FILL: u32 = 0x323232;
const OPS_HOVER: u32 = 0x3B3B3B;
const DIGIT_FILL: u32 = 0x3B3B3B;
const DIGIT_HOVER: u32 = 0x464646;

fn calc_icon() -> El {
    icon(Symbol::Calculator, 16.0, hex(0xFFFFFF))
}

/// Backspace glyph (no Fluent equivalent): rounded tag with an ✕.
fn backspace(fg: Color) -> El {
    let s = Shape::new()
        .stroke_path(
            &Path::new()
                .move_to([6.0, 2.5])
                .line_to([13.0, 2.5])
                .quad_to([14.5, 2.5], [14.5, 4.0])
                .line_to([14.5, 12.0])
                .quad_to([14.5, 13.5], [13.0, 13.5])
                .line_to([6.0, 13.5])
                .line_to([1.0, 8.0])
                .close(),
            1.0,
            LineCap::Round,
            c(fg),
        )
        .line([6.5, 5.5], [11.5, 10.5], 1.0, LineCap::Round, c(fg))
        .line([11.5, 5.5], [6.5, 10.5], 1.0, LineCap::Round, c(fg));
    canvas(s).size(16.0, 16.0)
}

fn key(t: &Theme, label: &str, fill: u32, hover: u32, fg: Color, weight: u16, size: f32) -> El {
    let mut b = row()
        .grow(1.0)
        .h_full()
        .min_w(0.0)
        .center()
        .radius(radii::CONTROL)
        .fill(c(hex(fill)))
        .hover(c(hex(hover)));
    let _ = t;
    if label == "⌫" {
        b = b.child(backspace(fg));
    } else {
        b = b.child(
            text(label)
                .font(TextStyle::new(size, weight as f32))
                .color(c(fg)),
        );
    }
    b
}

fn grid(t: &Theme) -> El {
    let k = t.tokens();
    let fg = k.text_fill_color_primary;
    let accent_fg = k.text_on_accent_fill_color_primary;
    let mut g = col().w_full().grow(1.0).min_h(0.0).gap(3.0);
    let rows: &[&[&str]] = &[
        &["%", "CE", "C", "⌫"],
        &["¹∕ₓ", "x²", "²√x", "÷"],
        &["7", "8", "9", "×"],
        &["4", "5", "6", "−"],
        &["1", "2", "3", "+"],
        &["+/−", "0", ".", "="],
    ];
    for (ri, r) in rows.iter().enumerate() {
        let mut line = row().w_full().grow(1.0).min_h(0.0).gap(3.0);
        for (ci, label) in r.iter().enumerate() {
            let digit = ri >= 2 && ci < 3;
            let b = if *label == "=" {
                key(t, label, 0x60CDFF, 0x7DD6FF, accent_fg, weights::SEMIBOLD, 20.0)
            } else if digit {
                key(t, label, DIGIT_FILL, DIGIT_HOVER, fg, weights::SEMIBOLD, 20.0)
            } else {
                key(t, label, OPS_FILL, OPS_HOVER, fg, weights::REGULAR, 16.0)
            };
            line = line.child(b);
        }
        g = g.child(line);
    }
    g
}

pub fn view(t: &Theme) -> El {
    let k = t.tokens();
    let mem = ["MC", "MR", "M+", "M−", "MS", "Mˇ"];
    let mut mem_row = row().w_full().h(32.0);
    for (i, m) in mem.iter().enumerate() {
        let disabled = i < 2;
        mem_row = mem_row.child(
            row()
                .grow(1.0)
                .min_w(0.0)
                .h_full()
                .center()
                .radius(radii::CONTROL)
                .hover(c(k.subtle_fill_color_secondary))
                .child(chrome::txt(
                    m,
                    13.0,
                    weights::REGULAR,
                    if disabled {
                        k.text_fill_color_disabled
                    } else {
                        k.text_fill_color_primary
                    },
                )),
        );
    }

    let header = row()
        .w_full()
        .h(44.0)
        .items_center()
        .pl(8.0)
        .child(
            el().size(40.0, 40.0)
                .center()
                .radius(radii::CONTROL)
                .hover(c(k.subtle_fill_color_secondary))
                .child(icon(
                    Symbol::GlobalNavigation,
                    16.0,
                    k.text_fill_color_primary,
                )),
        )
        .child(
            text("Standard")
                .font(TextStyle::new(20.0, weights::SEMIBOLD as f32))
                .color(c(k.text_fill_color_primary))
                .ml(4.0),
        )
        .child(
            el().size(32.0, 32.0)
                .center()
                .ml(4.0)
                .child(icon(Symbol::Pin, 14.0, k.text_fill_color_secondary)),
        )
        .child(el().grow(1.0).min_w(0.0))
        .child(
            el().size(40.0, 40.0)
                .center()
                .radius(radii::CONTROL)
                .hover(c(k.subtle_fill_color_secondary))
                .child(icon(Symbol::History, 16.0, k.text_fill_color_primary)),
        );

    let display = row()
        .w_full()
        .grow(1.0)
        .min_h(0.0)
        .items_end()
        .justify_end()
        .pr(12.0)
        .pb(8.0)
        .child(
            text("0")
                .font(TextStyle::new(50.0, 400.0))
                .color(c(k.text_fill_color_primary)),
        );

    let body = col()
        .w_full()
        .h_full()
        .child(header)
        .child(display)
        .child(mem_row)
        .child(el().h(4.0))
        .child(grid(t).px(4.0).pb(4.0));

    chrome::shell(
        t,
        chrome::title_bar(t, calc_icon(), "Calculator", 12.0)
            .child(el().grow(1.0))
            .child(chrome::captions(t)),
        body,
    )
}
