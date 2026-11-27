//! The WinUI 3 control set as Rune components.
//! See docs/ARCHITECTURE.md for the contract.
//!
//! Every control is a `view(&self, theme) -> El` builder (or a free `*_view`
//! function) reading Rest/Hover/Pressed/Disabled visuals from
//! `Theme::tokens()`. Interactivity is chained on the returned element —
//! `.on_press(|s: &mut S| ..)`, `.on_edit(..)` — so controls stay independent
//! of your app state type.

pub mod buttons;
pub mod content;
pub mod menus;
pub mod navigation;
pub mod selection;
pub mod status;
pub mod text;

pub use buttons::{Button, DropDownButton, HyperlinkButton, RepeatButton, SplitButton, ToggleButton};
pub use content::{Card, Expander, IconElement, InfoBadge, ListRow, PersonPicture};
pub use menus::{ComboBox, CommandBar, Flyout, MenuFlyout, MenuItem};
pub use navigation::{BreadcrumbBar, NavItem, NavigationView, Tab, TabView};
pub use selection::{CheckBox, CheckState, ColorSwatchGrid, RadioButton, RatingControl, Slider, ToggleSwitch};
pub use status::{InfoBar, ProgressBar, ProgressRing, Severity, TeachingTip, ToolTip};
pub use text::{AutoSuggestBox, NumberBox, PasswordBox, TextBox};

use rune::{Color, El, Shape, TextStyle, canvas};
use winui_materials::rc;
use winui_theme::{Theme, Tokens};

/// `winui_theme::TextStyle` → `rune::TextStyle`.
pub fn ts(s: &winui_theme::TextStyle) -> TextStyle {
    TextStyle::new(s.size, s.weight as f32).lh(s.line_height)
}

/// Body 14 / Regular — the size almost every control label uses.
pub fn body() -> TextStyle {
    TextStyle::new(14.0, 400.0)
}

/// Caption 12.
pub fn caption() -> TextStyle {
    TextStyle::new(12.0, 400.0)
}

/// A themed text element.
pub fn txt(theme: &Theme, style: TextStyle, s: impl Into<String>) -> El {
    rune::text(s).font(style).color(rc(theme.tokens().text_fill_color_primary))
}

/// Caption-styled secondary text.
pub fn caption_txt(theme: &Theme, s: impl Into<String>) -> El {
    rune::text(s).font(caption()).color(rc(theme.tokens().text_fill_color_secondary))
}

/// An icon element: a Fluent System Icons glyph tinted `color`.
/// Drawn from the vendored SVG path — identical outlines to Windows.
pub fn glyph(sym: winui_icons::Symbol, size: f32, color: Color) -> El {
    let rule = match sym.fill_rule() {
        winui_icons::FillRule::EvenOdd => rune::FillRule::EvenOdd,
        winui_icons::FillRule::NonZero => rune::FillRule::NonZero,
    };
    canvas(
        Shape::new()
            .fill_svg_with(sym.path(), rule, color)
            .view_box(winui_icons::VIEWBOX, winui_icons::VIEWBOX),
    )
    .size(size, size)
}

/// Glyph tinted `TextFillColorPrimary`.
pub fn icon(theme: &Theme, sym: winui_icons::Symbol, size: f32) -> El {
    glyph(sym, size, rc(theme.tokens().text_fill_color_primary))
}

/// The control corner radius.
pub fn radius() -> f32 {
    winui_theme::radii::CONTROL
}

/// Control fill for the given enabled state (standard, non-accent).
pub fn control_fill(t: &Tokens, enabled: bool) -> Color {
    rc(if enabled {
        t.control_fill_color_default
    } else {
        t.control_fill_color_disabled
    })
}

/// Text fill for the given enabled state.
pub fn text_fill(t: &Tokens, enabled: bool) -> Color {
    rc(if enabled {
        t.text_fill_color_primary
    } else {
        t.text_fill_color_disabled
    })
}

/// Standard button borders: `ControlElevationBorder` is a two-stop vertical
/// gradient (solid stroke on top, a band of the weaker stroke at the bottom).
pub fn elevation_border(t: &Tokens) -> (Color, Color) {
    (
        rc(t.control_elevation_border.top(32.0)),
        rc(t.control_elevation_border.bottom(32.0)),
    )
}

/// The accent equivalent for accent buttons.
pub fn accent_elevation_border(t: &Tokens) -> (Color, Color) {
    (
        rc(t.accent_control_elevation_border.top(32.0)),
        rc(t.accent_control_elevation_border.bottom(32.0)),
    )
}
