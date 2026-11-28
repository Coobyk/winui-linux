//! `ref-calculator`: Windows Calculator, Standard mode (docs/reference/calculator.png).

use rune::{El, el, row};
use winui_controls::style::rc;
use winui_icons::Symbol;
use winui_theme::{Color, TextStyle, Theme, radii};

use super::{icon, label, title_bar};

/// Calculator's digit-key fill: lighter than the operator keys' `ControlFillColorSecondary`
/// (sampled #3B3B3B over #202020 in the reference).
const DIGIT_FILL: u32 = 0x1FFFFFFF;

const KEYS: [[&str; 4]; 6] = [
    ["%", "CE", "C", "⌫"],
    ["¹/x", "x²", "²√x", "÷"],
    ["7", "8", "9", "×"],
    ["4", "5", "6", "−"],
    ["1", "2", "3", "+"],
    ["+/−", "0", ".", "="],
];

pub fn view(t: &Theme) -> El {
    let k = t.tokens();
    let ramp = t.type_ramp();
    let logo = el()
        .size(12.0, 14.0)
        .radius(2.0)
        .fill(rc(Color::from_argb(0xFF5A6E8C)))
        .child(
            el().absolute(2.0, 2.0)
                .size(8.0, 3.0)
                .fill(rc(Color::from_argb(0xFFDDE6F0))),
        );
    let bar = el()
        .child(logo.absolute(12.0, 10.0))
        .child(label("Calculator", ramp.caption, k.text_fill_color_primary).absolute(49.0, 9.0));
    let mut root = el()
        .w_full()
        .h_full()
        .child(title_bar(t, 32.0, bar).absolute(0.0, 0.0))
        .child(
            el().absolute(5.0, 39.0)
                .size(40.0, 36.0)
                .radius(radii::CONTROL)
                .center()
                .hover_fill(rc(k.subtle_fill_color_secondary))
                .child(icon(
                    Symbol::GlobalNavigation,
                    16.0,
                    k.text_fill_color_primary,
                )),
        )
        .child(
            label(
                "Standard",
                TextStyle::new(20.0, 600, 28.0),
                k.text_fill_color_primary,
            )
            .absolute(49.0, 43.0),
        )
        .child(icon(Symbol::OpenInNew, 16.0, k.text_fill_color_primary).absolute(148.0, 49.0))
        .child(icon(Symbol::History, 16.0, k.text_fill_color_primary).absolute(294.0, 49.0))
        .child(
            row()
                .absolute(0.0, 108.0)
                .w(316.0)
                .justify_end()
                .child(label(
                    "0",
                    TextStyle::new(46.0, 600, 60.0),
                    k.text_fill_color_primary,
                )),
        );
    for (i, (m, on)) in [
        ("MC", false),
        ("MR", false),
        ("M+", true),
        ("M−", true),
        ("MS", true),
        ("M⌄", false),
    ]
    .into_iter()
    .enumerate()
    {
        let fg = if on {
            k.text_fill_color_primary
        } else {
            k.text_fill_color_disabled
        };
        root = root.child(
            el().absolute(4.0 + 53.0 * i as f32, 180.0)
                .size(53.0, 30.0)
                .center()
                .child(label(m, ramp.caption, fg)),
        );
    }
    for (r, keys) in KEYS.iter().enumerate() {
        for (c, key) in keys.iter().enumerate() {
            root = root.child(
                key_button(t, key, r, c).absolute(5.0 + 80.0 * c as f32, 213.0 + 53.0 * r as f32),
            );
        }
    }
    root.clip_rounded(0.0)
}

fn key_button(t: &Theme, key: &str, r: usize, c: usize) -> El {
    let k = t.tokens();
    let equals = r == 5 && c == 3;
    let digit = (2..6).contains(&r) && c < 3;
    let (fill, fg) = if equals {
        (
            k.accent_fill_color_default,
            k.text_on_accent_fill_color_primary,
        )
    } else if digit {
        (Color::from_argb(DIGIT_FILL), k.text_fill_color_primary)
    } else {
        (k.control_fill_color_secondary, k.text_fill_color_primary)
    };
    let style = if digit && key.chars().all(|ch| ch.is_ascii_digit()) {
        TextStyle::new(20.0, 400, 28.0)
    } else if c == 3 {
        TextStyle::new(20.0, 300, 28.0)
    } else {
        TextStyle::new(15.0, 400, 22.0)
    };
    let mut b = el()
        .size(77.0, 50.0)
        .radius(radii::CONTROL)
        .fill(rc(fill))
        .border(1.0, rc(k.control_stroke_color_default))
        .center()
        .role("button")
        .label(key.to_owned());
    b = if key == "⌫" {
        b.child(icon(Symbol::Back, 16.0, fg))
    } else {
        b.child(label(key, style, fg))
    };
    if equals {
        b.hover_fill(rc(k.accent_fill_color_secondary))
    } else {
        b.hover_fill(rc(k.control_fill_color_tertiary))
    }
}
