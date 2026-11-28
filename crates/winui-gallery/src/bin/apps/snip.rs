//! Snipping Tool replica (docs/reference shot, ~502×232).

use crate::chrome::{self, hex};
use crate::fluent::{c, font, icon};
use rune::{canvas, col, el, row, text, El, Shape, TextStyle};
use winui_icons::Symbol;
use winui_theme::{Theme, radii, weights};

fn tool_icon(t: &Theme, sym: Symbol, selected: bool, chevron: bool) -> El {
    let k = t.tokens();
    let mut b = row()
        .h(36.0)
        .items_center()
        .px(10.0)
        .gap(2.0)
        .radius(radii::CONTROL)
        .hover(c(k.subtle_fill_color_secondary));
    if selected {
        b = b.fill(c(k.subtle_fill_color_secondary)).child(
            el().w(16.0).h(2.0)
                .fill(c(k.accent_fill_color_default))
                .absolute(10.0, 32.0),
        );
    }
    b = b.child(icon(sym, 16.0, k.text_fill_color_primary));
    if chevron {
        b = b.child(icon(Symbol::ChevronDown, 10.0, k.text_fill_color_primary));
    }
    b
}

fn win_logo() -> El {
    // The ⊞ in "Press ⊞ + Shift + S": a 2×2 pane grid, slightly sheared.
    let sq = |s: Shape, x: f32, y: f32| {
        s.fill(
            [[x + 1.0, y], [x + 6.0, y], [x + 5.6, y + 5.0], [x + 0.6, y + 5.0]],
            c(hex(0xFFFFFF)),
        )
    };
    let s = sq(sq(sq(sq(Shape::new(), 0.0, 0.0), 6.6, 0.0), 0.0, 6.4), 6.6, 6.4);
    canvas(s).size(12.0, 12.0)
}

pub fn view(t: &Theme) -> El {
    let k = t.tokens();
    let divider = || {
        el().w(1.0)
            .h(16.0)
            .fill(c(k.divider_stroke_color_default))
            .mx(4.0)
    };
    let toolbar = row()
        .w_full()
        .h(48.0)
        .items_center()
        .px(12.0)
        .gap(4.0)
        .child(
            row().h(32.0).items_center().px(12.0).gap(8.0).radius(radii::CONTROL)
                .fill(c(k.control_fill_color_default))
                .border(1.0, c(k.control_stroke_color_default))
                .hover(c(k.control_fill_color_secondary))
                .child(icon(Symbol::Add, 14.0, k.accent_fill_color_default))
                .child(
                    text("New")
                        .font(font(t.type_ramp().body))
                        .color(c(k.text_fill_color_primary)),
                ),
        )
        .child(tool_icon(t, Symbol::Camera, true, false))
        .child(tool_icon(t, Symbol::Video, false, false))
        .child(divider())
        .child(tool_icon(t, Symbol::SelectAll, false, true))
        .child(tool_icon(t, Symbol::Timer, false, true))
        .child(divider())
        .child(tool_icon(t, Symbol::SquareMultiple, false, false))
        .child(el().grow(1.0).min_w(0.0))
        .child(tool_icon(t, Symbol::MoreHorizontal, false, false));

    let empty = el().w_full().h_full().center().child(
        row().items_center().gap(5.0)
            .child(
                text("Press")
                    .font(TextStyle::new(12.0, weights::REGULAR as f32))
                    .color(c(k.text_fill_color_secondary)),
            )
            .child(win_logo())
            .child(
                text("+ Shift + S to start a snip")
                    .font(TextStyle::new(12.0, weights::REGULAR as f32))
                    .color(c(k.text_fill_color_secondary)),
            ),
    );

    let body = col()
        .w_full()
        .h_full()
        .child(toolbar)
        .child(el().w_full().h(1.0).fill(c(k.divider_stroke_color_default)))
        .child(empty.grow(1.0).min_h(0.0));

    chrome::shell(
        t,
        chrome::title_bar(
            t,
            icon(Symbol::Cut, 14.0, hex(0xFFFFFF)),
            "Snipping Tool",
            8.0,
        )
        .child(el().grow(1.0))
        .child(chrome::captions(t)),
        body,
    )
}
