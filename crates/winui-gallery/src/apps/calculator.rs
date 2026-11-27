//! Calculator, Standard mode (`docs/reference/calculator.png`).

use rune::{El, col, el, row};
use winui_icons::Symbol;
use winui_theme::{Theme, radii};

use super::glyphs as g;
use super::{caption_buttons, path_icon, px_style, sym, txt};
use crate::fluent::c;

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Op,
    Digit,
    Equals,
}

fn key(t: &Theme, label: El, kind: Kind) -> El {
    let k = t.tokens();
    let fill = match kind {
        Kind::Op => k.control_fill_color_secondary,
        Kind::Digit => k.control_fill_color_tertiary.over(
            k.control_fill_color_secondary
                .over(k.solid_background_fill_color_base),
        ),
        Kind::Equals => k.accent_fill_color_default,
    };
    let stroke = if kind == Kind::Equals {
        k.control_stroke_color_on_accent_default
    } else {
        k.control_stroke_color_default
    };
    el().grow(1.0)
        .h_full()
        .radius(radii::CONTROL)
        .fill(c(fill))
        .border(1.0, c(stroke))
        .center()
        .child(label)
}

pub fn view(t: &Theme) -> El {
    let k = t.tokens();
    let ink = k.text_fill_color_primary;
    let on_accent = k.text_on_accent_fill_color_primary;
    let glyph = |s: &str| txt(s, px_style(14.0, 400, 20.0), ink);
    let math = |s: &str| txt(s, px_style(20.0, 400, 24.0), ink);
    let digit = |s: &str| txt(s, px_style(20.0, 400, 24.0), ink);
    let op = |l: El| key(t, l, Kind::Op);
    let num = |s: &str| key(t, digit(s), Kind::Digit);
    let rows: Vec<Vec<El>> = vec![
        vec![
            op(glyph("%")),
            op(glyph("CE")),
            op(glyph("C")),
            op(path_icon(g::BACKSPACE, 18.0, ink)),
        ],
        vec![
            op(glyph("¹/x")),
            op(glyph("x²")),
            op(glyph("²√x")),
            op(math("÷")),
        ],
        vec![num("7"), num("8"), num("9"), op(math("×"))],
        vec![num("4"), num("5"), num("6"), op(math("−"))],
        vec![num("1"), num("2"), num("3"), op(math("+"))],
        vec![
            num("+/−"),
            num("0"),
            num("."),
            key(
                t,
                txt("=", px_style(20.0, 400, 24.0), on_accent),
                Kind::Equals,
            ),
        ],
    ];
    let mut pad = col().w_full().grow(1.0).min_h(0.0).px(5.0).pb(3.0).gap(2.0);
    for r in rows {
        pad = pad.child(row().w_full().grow(1.0).gap(2.0).children(r));
    }
    let memory = ["MC", "MR", "M+", "M−", "MS", "M˅"];
    let mut mem = row().w_full().h(34.0).px(3.0);
    for (i, m) in memory.iter().enumerate() {
        let on = matches!(i, 2..=4);
        let color = if on { ink } else { k.text_fill_color_disabled };
        mem = mem.child(el().grow(1.0).h_full().center().child(txt(
            *m,
            t.type_ramp().caption,
            color,
        )));
    }
    let title = row()
        .w_full()
        .h(34.0)
        .items_center()
        .child(el().pl(14.0).child(sym(Symbol::Calculator, 16.0, ink)))
        .child(
            el().pl(13.0)
                .child(txt("Calculator", t.type_ramp().caption, ink)),
        )
        .child(el().grow(1.0))
        .child(caption_buttons(t, 34.0));
    let header = row()
        .w_full()
        .h(46.0)
        .items_center()
        .pl(5.0)
        .pr(5.0)
        .child(
            el().size(40.0, 36.0)
                .center()
                .child(sym(Symbol::GlobalNavigation, 16.0, ink)),
        )
        .child(
            el().pl(4.0)
                .child(txt("Standard", t.type_ramp().subtitle, ink)),
        )
        .child(
            el().size(40.0, 36.0)
                .ml(8.0)
                .center()
                .child(path_icon(g::PIP_ENTER, 16.0, ink)),
        )
        .child(el().grow(1.0))
        .child(
            el().size(40.0, 36.0)
                .center()
                .child(sym(Symbol::History, 16.0, ink)),
        );
    let display = row()
        .w_full()
        .h(88.0)
        .pr(15.0)
        .items_end()
        .justify_end()
        .child(txt("0", px_style(48.0, 600, 60.0), ink));
    col()
        .fill(c(k.solid_background_fill_color_base))
        .child(title)
        .child(header)
        .child(display)
        .child(el().h(7.0))
        .child(mem)
        .child(el().h(4.0))
        .child(pad)
}
