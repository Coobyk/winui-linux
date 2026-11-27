//! Snipping Tool (`docs/reference/snipping-tool.png`).

use rune::{El, col, el, row, text};
use winui_icons::Symbol;
use winui_theme::{Theme, radii};

use super::glyphs as g;
use super::{caption_buttons, path_icon, sym, txt};
use crate::fluent::{c, font};

fn tool(t: &Theme, icon: El, chevron: bool) -> El {
    let k = t.tokens();
    row()
        .h(32.0)
        .px(8.0)
        .gap(6.0)
        .radius(radii::CONTROL)
        .items_center()
        .child(icon)
        .children(chevron.then(|| sym(Symbol::ChevronDown, 10.0, k.text_fill_color_primary)))
}

fn divider(t: &Theme) -> El {
    el().size(1.0, 20.0)
        .ml(8.0)
        .mr(8.0)
        .fill(c(t.tokens().divider_stroke_color_default))
}

pub fn view(t: &Theme) -> El {
    let k = t.tokens();
    let ink = k.text_fill_color_primary;
    let accent = k.accent_fill_color_default;
    let title = row()
        .w_full()
        .h(32.0)
        .items_center()
        .child(
            el().pl(9.0)
                .child(path_icon(g::CUT_FILLED, 16.0, k.system_fill_color_critical)),
        )
        .child(el().pl(6.0).child(txt(
            "Snipping Tool",
            t.type_ramp().caption,
            k.text_fill_color_secondary,
        )))
        .child(el().grow(1.0))
        .child(caption_buttons(t, 32.0));
    let new = row()
        .h(32.0)
        .w(77.0)
        .radius(radii::CONTROL)
        .fill(c(k.control_fill_color_default))
        .border(1.0, c(k.control_stroke_color_default))
        .items_center()
        .pl(11.0)
        .gap(8.0)
        .child(sym(Symbol::Add, 16.0, accent))
        .child(txt("New", t.type_ramp().body, ink));
    let camera = el()
        .size(38.0, 32.0)
        .radius(radii::CONTROL)
        .fill(c(k.control_fill_color_default))
        .border(1.0, c(k.control_stroke_color_default))
        .center()
        .child(sym(Symbol::Camera, 20.0, ink))
        .child(
            el().size(16.0, 3.0)
                .radius(1.5)
                .fill(c(accent))
                .absolute(11.0, 28.0),
        );
    let modes = row()
        .h(32.0)
        .radius(radii::CONTROL)
        .fill(c(k.control_fill_color_tertiary))
        .items_center()
        .child(camera)
        .child(
            el().size(38.0, 32.0)
                .center()
                .child(sym(Symbol::Video, 20.0, ink)),
        );
    let toolbar = row()
        .w_full()
        .h(44.0)
        .items_center()
        .pl(17.0)
        .pr(9.0)
        .child(new)
        .child(el().w(15.0))
        .child(modes)
        .child(el().w(14.0))
        .child(tool(t, path_icon(g::SCREENSHOT, 20.0, ink), true))
        .child(el().w(8.0))
        .child(tool(t, path_icon(g::TIMER_OFF, 20.0, ink), true))
        .child(divider(t))
        .child(tool(t, path_icon(g::SCAN_TEXT, 20.0, ink), false))
        .child(el().grow(1.0))
        .child(
            el().size(36.0, 32.0)
                .center()
                .child(sym(Symbol::MoreHorizontal, 20.0, ink)),
        );
    let body = t.type_ramp().body;
    let bold = t.type_ramp().body_strong;
    let part = |s: &str, st| text(s.to_string()).font(font(st)).color(c(ink));
    let hint = row()
        .items_center()
        .gap(4.0)
        .child(part("Press", body))
        .child(super::windows_logo(t, 5.5, ink))
        .child(part("+ Shift + S", bold))
        .child(part("to start a snip", body));
    let canvas = el()
        .w_full()
        .grow(1.0)
        .fill(c(k.layer_fill_color_default))
        .border_sides([0.0, 1.0, 0.0, 0.0], c(k.card_stroke_color_default))
        .center()
        .child(el().pb(28.0).child(hint));
    col()
        .fill(c(k.solid_background_fill_color_base))
        .child(title)
        .child(toolbar)
        .child(el().h(4.0))
        .child(canvas)
}
