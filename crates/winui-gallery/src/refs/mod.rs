//! Recreations of the Windows 11 inbox-app screenshots in `docs/reference/` as headless
//! scenes (`--scene ref-settings --theme dark --snap out.png`). Each scene is sized to its
//! reference capture and built from the real `winui-controls` / `winui-materials` APIs.

mod calculator;
mod clock;
mod settings;
mod snipping;
mod store;
mod terminal;

use rune::{El, LineCap, Shape, el, row, text};
use winui_controls::style::{font, glyph, rc};
use winui_icons::Symbol;
use winui_theme::{Color, TextStyle, Theme};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ref {
    Settings,
    Store,
    ClockTimer,
    ClockSettings,
    Calculator,
    Terminal,
    Snippets,
}

impl Ref {
    pub const ALL: [Ref; 7] = [
        Ref::Settings,
        Ref::Store,
        Ref::ClockTimer,
        Ref::ClockSettings,
        Ref::Calculator,
        Ref::Terminal,
        Ref::Snippets,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Ref::Settings => "ref-settings",
            Ref::Store => "ref-store",
            Ref::ClockTimer => "ref-clock-timer",
            Ref::ClockSettings => "ref-clock-settings",
            Ref::Calculator => "ref-calculator",
            Ref::Terminal => "ref-terminal",
            Ref::Snippets => "ref-snippets",
        }
    }

    pub fn from_id(id: &str) -> Option<Ref> {
        Self::ALL.into_iter().find(|r| r.id() == id)
    }

    /// The reference capture's size in points (the captures are at 100% scale).
    pub fn size(self) -> (f32, f32) {
        match self {
            Ref::Settings => (1201.0, 934.0),
            Ref::Store => (1201.0, 928.0),
            Ref::ClockTimer | Ref::ClockSettings => (1201.0, 933.0),
            Ref::Calculator => (328.0, 532.0),
            Ref::Terminal => (1153.0, 616.0),
            Ref::Snippets => (502.0, 232.0),
        }
    }
}

pub fn view(r: Ref, t: &Theme) -> El {
    let body = match r {
        Ref::Settings => settings::view(t),
        Ref::Store => store::view(t),
        Ref::ClockTimer => clock::timer(t),
        Ref::ClockSettings => clock::settings(t),
        Ref::Calculator => calculator::view(t),
        Ref::Terminal => terminal::view(t),
        Ref::Snippets => snipping::view(t),
    };
    el().w_full()
        .h_full()
        .child(winui_materials::mica(t).pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0)))
        .child(body.pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0)))
}

/// Text in a type-ramp style.
pub(crate) fn label(s: impl Into<String>, style: TextStyle, color: Color) -> El {
    text(s.into()).font(font(style)).color(rc(color))
}

pub(crate) fn icon(s: Symbol, size: f32, color: Color) -> El {
    glyph(s, size, color)
}

/// `TitleBar` caption buttons: 46×32 Minimize / Maximize / Close with the 10px
/// `ChromeMinimize` / `ChromeMaximize` / `ChromeClose` glyphs at 1px stroke.
pub(crate) fn caption_buttons(t: &Theme) -> El {
    let k = t.tokens();
    let fg = rc(k.text_fill_color_primary);
    let b = |shape: Shape, name: &'static str, hover: rune::Color| {
        el().size(46.0, 32.0)
            .center()
            .hover_fill(hover)
            .child(rune::canvas(shape.view_box(10.0, 10.0)).size(10.0, 10.0))
            .role("button")
            .label(name)
            .key(format!("caption:{name}"))
    };
    let hover = rc(k.subtle_fill_color_secondary);
    let min = Shape::new().line([0.0, 5.5], [10.0, 5.5], 1.0, LineCap::Butt, fg);
    let max = Shape::new().stroke_svg(
        "M2.5 0.5h5a2 2 0 0 1 2 2v5a2 2 0 0 1 -2 2h-5a2 2 0 0 1 -2 -2v-5a2 2 0 0 1 2 -2z",
        1.0,
        LineCap::Butt,
        fg,
    );
    let close = Shape::new()
        .line([0.5, 0.5], [9.5, 9.5], 1.0, LineCap::Butt, fg)
        .line([9.5, 0.5], [0.5, 9.5], 1.0, LineCap::Butt, fg);
    row()
        .child(b(min, "Minimize", hover))
        .child(b(max, "Maximize", hover))
        .child(b(close, "Close", rc(Color::from_argb(0xFFC42B1C))))
}

/// A custom title bar `h` tall: `content` fills it and the caption buttons sit top-right.
pub(crate) fn title_bar(t: &Theme, h: f32, content: El) -> El {
    el().w_full()
        .h(h)
        .window_drag()
        .child(content.pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0)))
        .child(caption_buttons(t).pin(None, Some(0.0), Some(0.0), None))
}

/// A rounded square app icon: brand `fill` with a white glyph.
pub(crate) fn app_tile(s: Symbol, size: f32, fill: u32) -> El {
    el().size(size, size)
        .radius(size * 0.18)
        .fill(rc(Color::from_argb(fill)))
        .center()
        .child(glyph(s, size * 0.6, Color::from_argb(0xFFFFFFFF)))
}

/// The four-square Microsoft logo `size` points square.
pub(crate) fn ms_logo(size: f32) -> El {
    let q = (size - 1.0) / 2.0;
    let sq = |c: u32| el().size(q, q).fill(rc(Color::from_argb(c)));
    rune::col()
        .gap(1.0)
        .child(row().gap(1.0).child(sq(0xFFF25022)).child(sq(0xFF7FBA00)))
        .child(row().gap(1.0).child(sq(0xFF00A4EF)).child(sq(0xFFFFB900)))
}

/// A Community Toolkit `SettingsCard` as Settings and Clock draw it: 20px icon, header +
/// optional description, trailing content right-aligned.
pub(crate) fn settings_card(
    t: &Theme,
    w: f32,
    h: f32,
    sym: Symbol,
    title: &str,
    desc: Option<&str>,
    trailing: El,
) -> El {
    let k = t.tokens();
    let ramp = t.type_ramp();
    let mut text = rune::col()
        .grow(1.0)
        .child(label(title, ramp.body, k.text_fill_color_primary));
    if let Some(d) = desc {
        text = text.child(label(d, ramp.caption, k.text_fill_color_secondary));
    }
    row()
        .size(w, h)
        .radius(winui_theme::radii::CONTROL)
        .fill(rc(k.card_background_fill_color_default))
        .border(1.0, rc(k.card_stroke_color_default))
        .pl(16.0)
        .pr(16.0)
        .gap(20.0)
        .center_y()
        .child(icon(sym, 20.0, k.text_fill_color_primary))
        .child(text)
        .child(trailing)
}
