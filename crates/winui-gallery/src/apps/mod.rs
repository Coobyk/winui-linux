//! App clones: Windows 11 inbox apps rebuilt from the real theme tokens, icons, materials and
//! controls, each matched against its screenshot in `docs/reference/`.

mod calculator;
mod clock;
mod glyphs;
mod settings;
mod snipping;
mod store;
mod terminal;

use rune::{El, FillRule, Shape, canvas, col, el, row, text};
use winui_icons::Symbol;
use winui_theme::{Color, TextStyle as Ts, Theme};

use crate::fluent::{c, font};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum App {
    Settings,
    Calculator,
    Clock,
    ClockSettings,
    Store,
    Snipping,
    Terminal,
}

impl App {
    pub const ALL: [App; 7] = [
        App::Settings,
        App::Calculator,
        App::Clock,
        App::ClockSettings,
        App::Store,
        App::Snipping,
        App::Terminal,
    ];

    pub fn id(self) -> &'static str {
        match self {
            App::Settings => "app-settings",
            App::Calculator => "app-calculator",
            App::Clock => "app-clock",
            App::ClockSettings => "app-clock-settings",
            App::Store => "app-store",
            App::Snipping => "app-snipping",
            App::Terminal => "app-terminal",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            App::Settings => "Settings",
            App::Calculator => "Calculator",
            App::Clock => "Clock",
            App::ClockSettings => "Clock settings",
            App::Store => "Microsoft Store",
            App::Snipping => "Snipping Tool",
            App::Terminal => "Terminal",
        }
    }

    pub fn icon(self) -> Symbol {
        match self {
            App::Settings => Symbol::Settings,
            App::Calculator => Symbol::Calculator,
            App::Clock | App::ClockSettings => Symbol::Timer,
            App::Store => Symbol::Apps,
            App::Snipping => Symbol::Cut,
            App::Terminal => Symbol::Code,
        }
    }

    /// The window size of the matching screenshot in `docs/reference/`.
    pub fn size(self) -> (f32, f32) {
        match self {
            App::Settings => (1201.0, 934.0),
            App::Calculator => (328.0, 532.0),
            App::Clock | App::ClockSettings => (1201.0, 933.0),
            App::Store => (1201.0, 928.0),
            App::Snipping => (502.0, 232.0),
            App::Terminal => (1153.0, 616.0),
        }
    }

    pub fn from_id(id: &str) -> Option<App> {
        App::ALL.into_iter().find(|a| a.id() == id)
    }

    /// The whole app window, filling its parent.
    pub fn window(self, t: &Theme) -> El {
        let body = match self {
            App::Settings => settings::view(t),
            App::Calculator => calculator::view(t),
            App::Clock => clock::timer(t),
            App::ClockSettings => clock::settings(t),
            App::Store => store::view(t),
            App::Snipping => snipping::view(t),
            App::Terminal => terminal::view(t),
        };
        body.w_full().h_full()
    }
}

// Shared pieces every clone is built from.

pub(crate) fn txt(s: impl Into<String>, style: Ts, color: Color) -> El {
    text(s.into()).font(font(style)).color(c(color))
}

pub(crate) fn px_style(size: f32, weight: u16, line: f32) -> Ts {
    Ts {
        size,
        weight,
        line_height: line,
    }
}

/// A Fluent System Icon path (24x24 viewBox) drawn at `size`.
pub(crate) fn path_icon(d: &'static str, size: f32, color: Color) -> El {
    canvas(
        Shape::new()
            .fill_svg_with(d, FillRule::NonZero, c(color))
            .view_box(24.0, 24.0),
    )
    .size(size, size)
}

pub(crate) fn sym(s: Symbol, size: f32, color: Color) -> El {
    crate::fluent::icon(s, size, color)
}

/// Caption buttons (minimize, maximize, close): 46 px wide, Segoe Fluent's 10 px chrome glyphs
/// drawn at 1 px.
pub(crate) fn caption_buttons(t: &Theme, h: f32) -> El {
    let k = t.tokens();
    let ink = c(k.text_fill_color_primary);
    let b = |g: El| el().size(46.0, h).center().child(g);
    let min = canvas(
        Shape::new()
            .stroke(&[[0.0, 5.0], [10.0, 5.0]], 1.0, ink)
            .view_box(10.0, 10.0),
    )
    .size(10.0, 10.0);
    let max = el().size(10.0, 10.0).border(1.0, ink).radius(1.5);
    let close = canvas(
        Shape::new()
            .stroke(&[[0.0, 0.0], [10.0, 10.0]], 1.0, ink)
            .stroke(&[[10.0, 0.0], [0.0, 10.0]], 1.0, ink)
            .view_box(10.0, 10.0),
    )
    .size(10.0, 10.0);
    row().child(b(min)).child(b(max)).child(b(close))
}

pub(crate) fn chevron_right(t: &Theme) -> El {
    sym(
        Symbol::ChevronRight,
        12.0,
        t.tokens().text_fill_color_primary,
    )
}

/// The clone inside the gallery: a titled window card at the screenshot's size.
pub fn preview(a: App, t: &Theme) -> El {
    let k = t.tokens();
    let (w, h) = a.size();
    let window = el()
        .size(w, h)
        .radius(winui_theme::radii::OVERLAY)
        .border(1.0, c(k.surface_stroke_color_default))
        .clip_rounded(winui_theme::radii::OVERLAY)
        .child(a.window(t));
    col()
        .w_full()
        .gap(16.0)
        .child(txt(a.title(), t.type_ramp().title_large, k.text_fill_color_primary))
        .child(txt(
            format!("A static clone of docs/reference, rendered from theme tokens and controls. Scene: --scene {}", a.id()),
            t.type_ramp().body,
            k.text_fill_color_secondary,
        ))
        .child(el().w_full().clip_rounded(0.0).child(winui_materials::MaterialExt::theme_shadow(window, t, 32.0)))
}

/// The Windows logo: four squares with a 1 px gap.
pub(crate) fn windows_logo(t: &Theme, cell: f32, color: Color) -> El {
    let _ = t;
    let q = || el().size(cell, cell).fill(c(color));
    col()
        .gap(1.0)
        .child(row().gap(1.0).child(q()).child(q()))
        .child(row().gap(1.0).child(q()).child(q()))
}

/// The four-color Microsoft logo (Store, Microsoft 365) from the system fill colors.
pub(crate) fn settings_logo(t: &Theme, cell: f32) -> El {
    let k = t.tokens();
    let q = |col: Color| el().size(cell, cell).fill(c(col));
    col()
        .gap(1.0)
        .child(
            row()
                .gap(1.0)
                .child(q(k.system_fill_color_critical))
                .child(q(k.system_fill_color_success)),
        )
        .child(
            row()
                .gap(1.0)
                .child(q(t.accent().light1))
                .child(q(k.system_fill_color_caution)),
        )
}
