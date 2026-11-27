//! Shared chrome for the app replicas: window title bars with caption
//! buttons, NavigationView panes, settings cards, toggle pills.

use crate::fluent::{c, font, icon, mica};
use rune::{canvas, col, el, row, text, El, LineCap, Path, Shape, TextStyle};
use winui_icons::Symbol;
use winui_theme::{Color, Theme, radii, weights};

pub fn hex(v: u32) -> Color {
    Color::rgb(
        ((v >> 16) & 0xFF) as u8,
        ((v >> 8) & 0xFF) as u8,
        (v & 0xFF) as u8,
    )
}

/// Quick text at an arbitrary size.
pub fn txt(s: &str, size: f32, weight: u16, color: Color) -> El {
    text(s)
        .font(TextStyle::new(size, weight as f32))
        .color(c(color))
}

pub fn body(t: &Theme, s: &str, color: Color) -> El {
    text(s).font(font(t.type_ramp().body)).color(c(color))
}

pub fn caption(t: &Theme, s: &str, color: Color) -> El {
    text(s).font(font(t.type_ramp().caption)).color(c(color))
}

// ---------------------------------------------------------------- title bar

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cap {
    Min,
    Max,
    Close,
}

fn cap_glyph(kind: Cap, fg: Color) -> El {
    let s = match kind {
        Cap::Min => Shape::new().line([1.0, 5.0], [11.0, 5.0], 1.0, LineCap::Butt, c(fg)),
        Cap::Max => Shape::new().stroke_path(
            &Path::new()
                .polygon([[1.5, 1.5], [10.5, 1.5], [10.5, 10.5], [1.5, 10.5]])
                .close(),
            1.0,
            LineCap::Butt,
            c(fg),
        ),
        Cap::Close => Shape::new()
            .line([1.0, 1.0], [11.0, 11.0], 1.0, LineCap::Butt, c(fg))
            .line([11.0, 1.0], [1.0, 11.0], 1.0, LineCap::Butt, c(fg)),
    };
    canvas(s).size(12.0, 12.0)
}

fn cap_button(t: &Theme, kind: Cap) -> El {
    let k = t.tokens();
    let fg = k.text_fill_color_primary;
    let b = el()
        .size(46.0, 32.0)
        .center()
        .child(cap_glyph(kind, fg));
    if kind == Cap::Close {
        b.hover(c(hex(0xE81123))).press(c(hex(0xF1707B)))
    } else {
        b.hover(c(k.subtle_fill_color_secondary))
            .press(c(k.subtle_fill_color_tertiary))
    }
}

/// The `─ ▢ ✕` cluster, flush right.
pub fn captions(t: &Theme) -> El {
    row()
        .h(32.0)
        .child(cap_button(t, Cap::Min))
        .child(cap_button(t, Cap::Max))
        .child(cap_button(t, Cap::Close))
}

/// Title bar: app icon + caption-sized title at the left, window caption
/// buttons at the right. `pad` is the row height (30 in Settings/Store).
pub fn title_bar(t: &Theme, app_icon: El, title: &str, pad_left: f32) -> El {
    row()
        .w_full()
        .h(32.0)
        .items_center()
        .pl(pad_left)
        .window_drag()
        .child(app_icon)
        .child(
            caption(t, title, t.tokens().text_fill_color_primary)
                .ml(8.0),
        )
}

/// Full window: mica under a title bar + content column.
pub fn shell(t: &Theme, title_row: El, content: El) -> El {
    el().w_full()
        .h_full()
        .child(mica(t).pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0)))
        .child(
            col()
                .w_full()
                .h_full()
                .child(title_row)
                .child(content.grow(1.0).min_h(0.0))
                .pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0)),
        )
}

// ------------------------------------------------------------- NavigationView

/// One left-pane row: icon + label, selection pill + accent bar.
pub fn nav_row(t: &Theme, sym: Symbol, label: &str, tint: Option<Color>, selected: bool) -> El {
    let k = t.tokens();
    let mut r = row()
        .h(36.0)
        .w_full()
        .items_center()
        .radius(radii::CONTROL)
        .hover(c(k.subtle_fill_color_secondary))
        .press(c(k.subtle_fill_color_tertiary));
    if selected {
        r = r
            .fill(c(k.subtle_fill_color_secondary))
            .child(
                el().size(3.0, 16.0)
                    .radius(radii::SELECTION_INDICATOR)
                    .fill(c(k.accent_fill_color_default))
                    .absolute(0.0, 10.0),
            );
    }
    r.child(
        el().size(36.0, 36.0)
            .center()
            .child(icon(
                sym,
                16.0,
                tint.unwrap_or(k.text_fill_color_primary),
            )),
    )
    .child(
        body(t, label, k.text_fill_color_primary).truncate(),
    )
}

// --------------------------------------------------------------- cards/toggle

/// Card container (CardBackgroundFillColorDefault + stroke, 8px radius).
pub fn card(t: &Theme) -> El {
    let k = t.tokens();
    col()
        .w_full()
        .radius(radii::OVERLAY)
        .fill(c(k.card_background_fill_color_default))
        .border(1.0, c(k.card_stroke_color_default))
}

/// ToggleSwitch track only (no label) — the pill WinUI draws.
pub fn pill(t: &Theme, on: bool) -> El {
    let k = t.tokens();
    if on {
        el().size(40.0, 20.0)
            .radius(10.0)
            .fill(c(k.accent_fill_color_default))
            .hover(c(k.accent_fill_color_secondary))
            .child(
                el().size(12.0, 12.0)
                    .radius(6.0)
                    .fill(c(k.text_on_accent_fill_color_primary))
                    .absolute(24.0, 4.0),
            )
    } else {
        el().size(40.0, 20.0)
            .radius(10.0)
            .fill(c(k.control_alt_fill_color_secondary))
            .border(1.0, c(k.control_strong_stroke_color_default))
            .child(
                el().size(12.0, 12.0)
                    .radius(6.0)
                    .fill(c(k.text_fill_color_secondary))
                    .absolute(4.0, 4.0),
            )
    }
}

/// `›` chevron used by settings cards and navigation rows.
pub fn chevron(t: &Theme) -> El {
    icon(
        Symbol::ChevronRight,
        12.0,
        t.tokens().text_fill_color_primary,
    )
}

/// Section heading above a group of settings cards ("Account", "General").
pub fn section_header(t: &Theme, s: &str) -> El {
    col().h(34.0).w_full().justify_end().pb(6.0).child(txt(
        s,
        13.0,
        weights::SEMIBOLD,
        t.tokens().text_fill_color_primary,
    ))
}
