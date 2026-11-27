//! App replicas: Windows 11 inbox apps rebuilt from `docs/reference/*.png` (dark theme).
//! Colours are sampled from those screenshots, sizes measured at 1x.

mod calc;
mod clock;
mod icons;
mod settings;
mod snipping;
mod store;
mod terminal;

use crate::State;
use crate::fluent::c;
use rune::{Color, El, Family, FillRule, LineCap, Shape, TextStyle, canvas, el, row, text};
use winui_icons::{Glyph, Symbol, VIEWBOX};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum App {
    Calc,
    WinSettings,
    Store,
    Clock,
    ClockSettings,
    Snipping,
    Terminal,
}

impl App {
    pub const ALL: [App; 7] = [
        App::Calc,
        App::WinSettings,
        App::Store,
        App::Clock,
        App::ClockSettings,
        App::Snipping,
        App::Terminal,
    ];
    pub fn id(self) -> &'static str {
        match self {
            App::Calc => "calc",
            App::WinSettings => "winsettings",
            App::Store => "store",
            App::Clock => "clock",
            App::ClockSettings => "clocksettings",
            App::Snipping => "snipping",
            App::Terminal => "terminal",
        }
    }
    /// The reference screenshot's size in points (snap with `--size WxH --scale 1`).
    pub fn size(self) -> (f32, f32) {
        match self {
            App::Calc => (328.0, 532.0),
            App::WinSettings => (1201.0, 934.0),
            App::Store => (1201.0, 928.0),
            App::Clock | App::ClockSettings => (1201.0, 933.0),
            App::Snipping => (502.0, 232.0),
            App::Terminal => (1153.0, 616.0),
        }
    }
}

pub fn view(s: &State, app: App) -> El {
    let body = match app {
        App::Calc => calc::view(s),
        App::WinSettings => settings::view(s),
        App::Store => store::view(s),
        App::Clock => clock::timer(s),
        App::ClockSettings => clock::settings(s),
        App::Snipping => snipping::view(s),
        App::Terminal => terminal::view(s),
    };
    window(body).key(format!("app-{}", app.id()))
}

/// `#RRGGBB` → Rune colour.
pub fn rgb(hex: u32) -> Color {
    c(winui_theme::Color::from_argb(0xFF00_0000 | hex))
}

pub fn rgba(hex: u32, a: f32) -> Color {
    c(winui_theme::Color::from_argb(0xFF00_0000 | hex).with_alpha(a))
}

pub const WHITE: u32 = 0xFFFFFF;
/// AccentFillColorDefault (dark): SystemAccentColorLight2 for the default blue.
pub const ACCENT: u32 = 0x4CC2FF;

/// Selawik (Segoe UI metrics) at `size` px, `weight`.
pub fn label(s: impl Into<String>, size: f32, weight: f32, color: u32) -> El {
    let mut f = TextStyle::new(size, weight);
    f.line_height = (size * 1.36).round();
    text(s.into()).font(f).color(rgb(color)).nowrap()
}

pub fn mono(s: impl Into<String>, size: f32, color: u32) -> El {
    let mut f = TextStyle::new(size, 400.0);
    f.family = Family::Named("Cascadia Mono");
    f.line_height = (size * 1.32).round();
    text(s.into()).font(f).color(rgb(color)).nowrap()
}

/// A 24px-viewbox Fluent path drawn `size` px square.
pub fn path_icon(d: &str, size: f32, color: u32) -> El {
    canvas(
        Shape::new()
            .fill_svg_with(d, FillRule::NonZero, rgb(color))
            .view_box(VIEWBOX, VIEWBOX),
    )
    .size(size, size)
}

pub fn sym(s: Symbol, size: f32, color: u32) -> El {
    match s.glyph() {
        Glyph::Path(d) => canvas(
            Shape::new()
                .fill_svg_with(
                    d,
                    match s.fill_rule() {
                        winui_icons::FillRule::EvenOdd => FillRule::EvenOdd,
                        winui_icons::FillRule::NonZero => FillRule::NonZero,
                    },
                    rgb(color),
                )
                .view_box(VIEWBOX, VIEWBOX),
        )
        .size(size, size),
        _ => el().size(size, size),
    }
}

/// Min / max / close caption buttons (46×`h`), Segoe Fluent ChromeMinimize/Maximize/Close
/// glyphs: 10px, 1px strokes.
pub fn caption_buttons(h: f32, fg: u32) -> El {
    let b = |g: El, close: bool| {
        el().size(46.0, h)
            .center()
            .hover(if close {
                rgb(0xC42B1C)
            } else {
                rgba(WHITE, 0.0605)
            })
            .press(if close {
                rgb(0x9E2B21)
            } else {
                rgba(WHITE, 0.0419)
            })
            .role("button")
            .child(g)
    };
    let col = rgb(fg);
    let min = canvas(Shape::new().line([0.0, 0.5], [10.0, 0.5], 1.0, LineCap::Butt, col)).size(10.0, 1.0);
    let max = el().size(10.0, 10.0).radius(1.5).border(1.0, col);
    let close = canvas(
        Shape::new()
            .line([0.5, 0.5], [9.5, 9.5], 1.0, LineCap::Butt, col)
            .line([9.5, 0.5], [0.5, 9.5], 1.0, LineCap::Butt, col),
    )
    .size(10.0, 10.0);
    row()
        .h(h)
        .child(b(min, false).label("Minimize"))
        .child(b(max, false).label("Maximize"))
        .child(b(close, true).label("Close"))
}

/// The window frame: desktop behind 8px rounded corners, 1px border.
fn window(body: El) -> El {
    el().w_full().h_full().fill(rgb(0x020D1C)).child(
        el().pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
            .radius(8.0)
            .clip_corners([8.0; 4])
            .child(body.pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0)))
            .child(
                el().pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                    .radius(8.0)
                    .border(1.0, rgb(0x32373F)),
            ),
    )
}

/// A multi-colour icon in a `w`×`h` viewbox: each layer is `(svg path, colour, stroke)`,
/// filled when `stroke` is 0, otherwise stroked with round caps.
pub fn art(w: f32, h: f32, size: [f32; 2], layers: &[(&str, u32, f32)]) -> El {
    let mut sh = Shape::new();
    for &(d, color, stroke) in layers {
        sh = if stroke > 0.0 {
            sh.stroke_svg(d, stroke, LineCap::Round, rgb(color))
        } else {
            sh.fill_svg_with(d, FillRule::NonZero, rgb(color))
        };
    }
    canvas(sh.view_box(w, h)).size(size[0], size[1])
}
