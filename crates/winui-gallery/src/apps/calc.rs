//! Calculator, Standard mode (docs/reference/calculator.png, 328×532).

use super::{ACCENT, WHITE, caption_buttons, icons, label, path_icon, rgb, rgba, sym};
use crate::State;
use rune::{El, LineCap, Shape, canvas, col, el, row};
use winui_icons::Symbol;

const BG: u32 = 0x202020;
const NUM: u32 = 0x3B3B3B;
const OP: u32 = 0x323232;
const DIM: u32 = 0x717171;

/// Draws a Segoe Fluent operator glyph as 1px strokes in a `w`×`h` box.
fn strokes(w: f32, h: f32, lines: &[([f32; 2], [f32; 2])], dots: &[[f32; 2]], color: u32) -> El {
    let mut s = Shape::new();
    for (a, b) in lines {
        s = s.line(*a, *b, 1.0, LineCap::Butt, rgb(color));
    }
    for d in dots {
        s = s.line([d[0] - 0.75, d[1]], [d[0] + 0.75, d[1]], 1.5, LineCap::Round, rgb(color));
    }
    canvas(s).size(w, h)
}

enum Key {
    Text(&'static str),
    Glyph(El),
}

fn key(k: Key, fill: u32, accent: bool, id: &'static str) -> El {
    let fg = if accent { 0x000000 } else { WHITE };
    let (hover, press) = if accent {
        (rgba(ACCENT, 0.9), rgba(ACCENT, 0.8))
    } else if fill == NUM {
        (rgb(0x323232), rgb(0x282828))
    } else {
        (rgb(0x3B3B3B), rgb(0x282828))
    };
    let content = match k {
        Key::Text(t) => {
            let digit = t.len() == 1 && t.as_bytes()[0].is_ascii_digit();
            label(t, if digit { 20.0 } else { 14.0 }, 400.0, fg)
        }
        Key::Glyph(g) => g,
    };
    let mut b = el()
        .size(77.0, 50.0)
        .radius(4.0)
        .fill(rgb(fill))
        .hover(hover)
        .press(press)
        .center()
        .role("button")
        .label(id)
        .key(format!("calc-{id}"))
        .child(content);
    if id.len() == 1 && id.as_bytes()[0].is_ascii_digit() {
        b = b.on_press(move |s: &mut State| {
            if s.calc == "0" {
                s.calc = id.to_string();
            } else if s.calc.len() < 16 {
                s.calc.push_str(id);
            }
        });
    }
    if id == "C" || id == "CE" {
        b = b.on_press(|s: &mut State| s.calc = "0".into());
    }
    if id == "back" {
        b = b.on_press(|s: &mut State| {
            s.calc.pop();
            if s.calc.is_empty() {
                s.calc = "0".into();
            }
        });
    }
    b
}

/// `¹⁄ₓ`, `x²` and `²√x`, set the way Calculator's glyph font draws them.
fn fraction() -> El {
    row()
        .items_start()
        .child(label("1", 10.0, 400.0, WHITE).mt(-2.0))
        .child(label("/", 14.0, 300.0, WHITE))
        .child(label("x", 14.0, 400.0, WHITE).mt(2.0))
}

fn square() -> El {
    row()
        .items_start()
        .child(label("x", 15.0, 400.0, WHITE).mt(2.0))
        .child(label("2", 9.0, 400.0, WHITE))
}

fn root() -> El {
    row()
        .items_start()
        .child(label("2", 8.0, 400.0, WHITE).mt(-1.0).mr(-2.0))
        .child(label("√", 15.0, 300.0, WHITE))
        .child(label("x", 13.0, 400.0, WHITE).mt(3.0))
}

fn grid() -> El {
    let div = strokes(14.0, 10.0, &[([0.0, 5.0], [14.0, 5.0])], &[[7.0, 1.0], [7.0, 9.0]], WHITE);
    let times = strokes(
        9.0,
        9.0,
        &[([0.0, 0.0], [9.0, 9.0]), ([9.0, 0.0], [0.0, 9.0])],
        &[],
        WHITE,
    );
    let minus = strokes(14.0, 1.0, &[([0.0, 0.5], [14.0, 0.5])], &[], WHITE);
    let plus = strokes(
        14.0,
        14.0,
        &[([0.0, 7.0], [14.0, 7.0]), ([7.0, 0.0], [7.0, 14.0])],
        &[],
        WHITE,
    );
    let eq = strokes(
        14.0,
        6.0,
        &[([0.0, 0.5], [14.0, 0.5]), ([0.0, 5.5], [14.0, 5.5])],
        &[],
        0x000000,
    );
    let rows: Vec<Vec<El>> = vec![
        vec![
            key(Key::Text("%"), OP, false, "%"),
            key(Key::Text("CE"), OP, false, "CE"),
            key(Key::Text("C"), OP, false, "C"),
            key(Key::Glyph(path_icon(icons::BACKSPACE, 16.0, WHITE)), OP, false, "back"),
        ],
        vec![
            key(Key::Glyph(fraction()), OP, false, "inv"),
            key(Key::Glyph(square()), OP, false, "sq"),
            key(Key::Glyph(root()), OP, false, "sqrt"),
            key(Key::Glyph(div), OP, false, "div"),
        ],
        vec![
            key(Key::Text("7"), NUM, false, "7"),
            key(Key::Text("8"), NUM, false, "8"),
            key(Key::Text("9"), NUM, false, "9"),
            key(Key::Glyph(times), OP, false, "mul"),
        ],
        vec![
            key(Key::Text("4"), NUM, false, "4"),
            key(Key::Text("5"), NUM, false, "5"),
            key(Key::Text("6"), NUM, false, "6"),
            key(Key::Glyph(minus), OP, false, "sub"),
        ],
        vec![
            key(Key::Text("1"), NUM, false, "1"),
            key(Key::Text("2"), NUM, false, "2"),
            key(Key::Text("3"), NUM, false, "3"),
            key(Key::Glyph(plus), OP, false, "add"),
        ],
        vec![
            key(Key::Text("+/-"), NUM, false, "neg"),
            key(Key::Text("0"), NUM, false, "0"),
            key(Key::Text("."), NUM, false, "dot"),
            key(Key::Glyph(eq), ACCENT, true, "eq"),
        ],
    ];
    let mut g = col().gap(3.0).pl(5.0);
    for r in rows {
        g = g.child(row().gap(3.0).children(r));
    }
    g
}

/// The Calculator tile: grey body with a blue display strip.
fn app_icon() -> El {
    el().size(11.0, 14.0)
        .radius(1.5)
        .linear_gradient(
            std::f32::consts::PI,
            [(0.0, rgb(0xC6CED5)), (1.0, rgb(0x7E8A96))],
        )
        .child(
            el().size(7.0, 3.0)
                .radius(0.5)
                .fill(rgb(0x4DE2FD))
                .absolute(2.0, 2.0),
        )
}

fn memory_row() -> El {
    let m = |s: &'static str, on: bool| {
        row()
            .grow(1.0)
            .h(32.0)
            .center()
            .radius(4.0)
            .hover(rgba(WHITE, if on { 0.0605 } else { 0.0 }))
            .child(label(s, 12.0, 400.0, if on { WHITE } else { DIM }))
    };
    row()
        .w_full()
        .px(3.0)
        .child(m("MC", false))
        .child(m("MR", false))
        .child(m("M+", true))
        .child(m("M\u{2212}", true))
        .child(m("MS", true))
        .child(m("M\u{02C5}", false))
}

pub fn view(s: &State) -> El {
    let title = row()
        .w_full()
        .h(32.0)
        .items_center()
        .window_drag()
        .child(app_icon().ml(21.0).mt(2.0))
        .child(label("Calculator", 12.0, 400.0, WHITE).ml(17.0).mt(4.0))
        .child(rune::spacer())
        .child(caption_buttons(32.0, 0x858585).mt(1.0).mr(1.0));
    let header = row()
        .w_full()
        .h(48.0)
        .items_center()
        .child(
            row()
                .size(40.0, 36.0)
                .ml(5.0)
                .center()
                .radius(4.0)
                .hover(rgba(WHITE, 0.0605))
                .child(sym(Symbol::GlobalNavigation, 16.0, 0xE3E3E3).mt(2.0)),
        )
        .child(label("Standard", 20.0, 600.0, WHITE).ml(4.0))
        .child(path_icon(icons::WINDOW_ARROW_UP, 16.0, WHITE).ml(16.0))
        .child(rune::spacer())
        .child(path_icon(icons::HISTORY, 20.0, WHITE).mr(16.0));
    let display = row()
        .w_full()
        .h(99.0)
        .justify_end()
        .items_center()
        .pr(13.0)
        .child(label(s.calc.clone(), 48.0, 600.0, WHITE).mt(14.0));
    col()
        .w_full()
        .h_full()
        .fill(rgb(BG))
        .child(title)
        .child(header)
        .child(display)
        .child(memory_row())
        .child(el().h(2.0))
        .child(grid())
}
