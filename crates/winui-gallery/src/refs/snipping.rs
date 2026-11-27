//! `ref-snippets`: the Snipping Tool toolbar (docs/reference/snipping-tool.png).

use rune::{El, el, row};
use winui_controls::Button;
use winui_controls::style::rc;
use winui_icons::Symbol;
use winui_theme::{Color, TextStyle, Theme, radii};

use super::{icon, label, title_bar};

pub fn view(t: &Theme) -> El {
    let k = t.tokens();
    let ramp = t.type_ramp();
    let logo = el()
        .size(16.0, 16.0)
        .radius(3.0)
        .fill(rc(Color::from_argb(0xFFF3F3F3)))
        .center()
        .child(icon(Symbol::Cut, 12.0, Color::from_argb(0xFFC42B1C)));
    let bar = el().child(logo.absolute(8.0, 8.0)).child(
        label("Snipping Tool", ramp.caption, k.text_fill_color_secondary).absolute(31.0, 8.0),
    );
    let mode = |sym: Symbol, sel: bool| {
        let mut b = el()
            .size(36.0, 32.0)
            .radius(radii::CONTROL)
            .center()
            .fill(rc(if sel {
                k.control_fill_color_default
            } else {
                k.subtle_fill_color_transparent
            }))
            .hover_fill(rc(k.subtle_fill_color_secondary))
            .child(icon(sym, 20.0, k.text_fill_color_primary));
        if sel {
            b = b.border(1.0, rc(k.control_stroke_color_default)).child(
                el().absolute(10.0, 27.0)
                    .size(16.0, 3.0)
                    .radius(1.5)
                    .fill(rc(k.accent_fill_color_default)),
            );
        }
        b
    };
    let toggles = row()
        .absolute(110.0, 39.0)
        .gap(2.0)
        .child(mode(Symbol::Camera, true))
        .child(mode(Symbol::Video, false));
    let new = Button::new("New")
        .icon(Symbol::Add)
        .width(76.0)
        .view(t)
        .absolute(17.0, 39.0);
    let dropdown = |sym: Symbol, x: f32| {
        row()
            .absolute(x, 39.0)
            .size(52.0, 32.0)
            .radius(radii::CONTROL)
            .pl(8.0)
            .gap(6.0)
            .center_y()
            .hover_fill(rc(k.subtle_fill_color_secondary))
            .child(icon(sym, 20.0, k.text_fill_color_primary))
            .child(icon(Symbol::ChevronDown, 12.0, k.text_fill_color_primary))
    };
    let shape = dropdown(Symbol::Window, 202.0);
    let delay = dropdown(Symbol::Timer, 258.0);
    let text_btn = el()
        .absolute(322.0, 39.0)
        .size(36.0, 32.0)
        .radius(radii::CONTROL)
        .center()
        .hover_fill(rc(k.subtle_fill_color_secondary))
        .child(icon(Symbol::TextField, 20.0, k.text_fill_color_primary));
    let more = el()
        .absolute(458.0, 39.0)
        .size(36.0, 32.0)
        .radius(radii::CONTROL)
        .center()
        .hover_fill(rc(k.subtle_fill_color_secondary))
        .child(icon(
            Symbol::MoreHorizontal,
            20.0,
            k.text_fill_color_primary,
        ));
    let strong = TextStyle::new(14.0, 600, 20.0);
    let win = rune::col()
        .gap(1.0)
        .child(
            row()
                .gap(1.0)
                .child(el().size(5.5, 5.5).fill(rc(k.text_fill_color_primary)))
                .child(el().size(5.5, 5.5).fill(rc(k.text_fill_color_primary))),
        )
        .child(
            row()
                .gap(1.0)
                .child(el().size(5.5, 5.5).fill(rc(k.text_fill_color_primary)))
                .child(el().size(5.5, 5.5).fill(rc(k.text_fill_color_primary))),
        );
    let hint = row()
        .absolute(0.0, 50.0)
        .w(502.0)
        .justify_center()
        .center_y()
        .gap(5.0)
        .child(label("Press", ramp.body, k.text_fill_color_primary))
        .child(win)
        .child(label("+ Shift + S", strong, k.text_fill_color_primary))
        .child(label(
            "to start a snip",
            ramp.body,
            k.text_fill_color_primary,
        ));
    let body = el()
        .absolute(0.0, 80.0)
        .w(502.0)
        .h(152.0)
        .fill(rc(k.layer_fill_color_default))
        .border_sides([0.0, 1.0, 0.0, 0.0], rc(k.card_stroke_color_default))
        .child(hint);
    el().w_full()
        .h_full()
        .child(title_bar(t, 32.0, bar).absolute(0.0, 0.0))
        .child(new)
        .child(toggles)
        .child(shape)
        .child(delay)
        .child(
            el().absolute(314.0, 45.0)
                .size(1.0, 20.0)
                .fill(rc(k.divider_stroke_color_default)),
        )
        .child(text_btn)
        .child(more)
        .child(body)
        .clip_rounded(0.0)
}
