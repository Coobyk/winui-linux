//! Buttons: Button (standard + Accent), HyperlinkButton, ToggleButton,
//! RepeatButton, SplitButton, DropDownButton.
//! Matches the WinUI 3 Gallery "Basic input > Button" samples.

use rune::{El, el, row, text};
use winui_icons::Symbol;
use winui_materials::rc;
use winui_theme::{Theme, sizes};

use crate::{accent_elevation_border, body, elevation_border, glyph};

/// A standard or accent WinUI Button.
///
/// Rest: ControlFill + ControlElevationBorder; hover lifts to
/// ControlFillColorSecondary; pressed drops to Tertiary and dims the label.
/// Accent uses the accent ramp and TextOnAccentFill.
#[derive(Clone, Debug)]
pub struct Button {
    pub label: String,
    pub accent: bool,
    pub enabled: bool,
    /// Optional leading glyph (buttons with icons, e.g. "Save" + floppy).
    pub symbol: Option<Symbol>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), accent: false, enabled: true, symbol: None }
    }
    pub fn accent(mut self, on: bool) -> Self {
        self.accent = on;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn symbol(mut self, s: Symbol) -> Self {
        self.symbol = Some(s);
        self
    }

    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let (fill, hover, pressed, text_c, border) = if self.accent {
            (
                rc(t.accent_fill_color_default),
                rc(t.accent_fill_color_secondary),
                rc(t.accent_fill_color_tertiary),
                if self.enabled {
                    rc(t.text_on_accent_fill_color_primary)
                } else {
                    rc(t.text_on_accent_fill_color_disabled)
                },
                accent_elevation_border(t),
            )
        } else {
            (
                if self.enabled {
                    rc(t.control_fill_color_default)
                } else {
                    rc(t.control_fill_color_disabled)
                },
                rc(t.control_fill_color_secondary),
                rc(t.control_fill_color_tertiary),
                if self.enabled {
                    rc(t.text_fill_color_primary)
                } else {
                    rc(t.text_fill_color_disabled)
                },
                elevation_border(t),
            )
        };
        let content = row()
            .items_center()
            .gap(6.0)
            .children(self.symbol.map(|s| glyph(s, 16.0, text_c)))
            .child(
                text(self.label.clone())
                    .font(body())
                    .color(text_c)
                    .nowrap(),
            );
        let mut e = el()
            .role("button")
            .label(self.label.clone())
            .min_h(sizes::CONTROL_HEIGHT)
            .min_w(40.0)
            .radius(4.0)
            .px(12.0)
            .center_y()
            .fill(fill)
            .child(content);
        if self.enabled {
            e = e.border_gradient(1.0, border.0, border.1)
                .hover_fill(hover)
                .press_fill(pressed)
                .cursor(rune::Cursor::Pointer);
        } else {
            e = e.border(1.0, rc(t.control_strong_stroke_color_disabled));
        }
        e
    }
}

/// A link-styled button: accent text, no chrome; underlines on hover.
#[derive(Clone, Debug)]
pub struct HyperlinkButton {
    pub label: String,
    pub enabled: bool,
}

impl HyperlinkButton {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), enabled: true }
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let c = if self.enabled {
            rc(t.accent_text_fill_color_primary)
        } else {
            rc(t.accent_text_fill_color_disabled)
        };
        el().role("link")
            .label(self.label.clone())
            .h_fit()
            .w_fit()
            .radius(4.0)
            .px(6.0)
            .py(4.0)
            .hover_fill(rc(t.subtle_fill_color_secondary))
            .cursor(rune::Cursor::Pointer)
            .child(text(self.label.clone()).font(body()).color(c).nowrap())
    }
}

/// A button that stays pressed while checked — like the Gallery's ToggleButton.
#[derive(Clone, Debug)]
pub struct ToggleButton {
    pub label: String,
    pub checked: bool,
    pub enabled: bool,
}

impl ToggleButton {
    pub fn new(label: impl Into<String>, checked: bool) -> Self {
        Self { label: label.into(), checked, enabled: true }
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn view(&self, theme: &Theme) -> El {
        // Checked toggles take the accent look.
        Button::new(self.label.clone())
            .accent(self.checked)
            .enabled(self.enabled)
            .view(theme)
    }
}

/// Press-and-hold repeats; visually identical to a standard Button.
/// (Gallery sample: "A button that repeats its command while pressed".)
#[derive(Clone, Debug)]
pub struct RepeatButton {
    pub label: String,
    pub enabled: bool,
}

impl RepeatButton {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), enabled: true }
    }
    pub fn view(&self, theme: &Theme) -> El {
        Button::new(self.label.clone()).enabled(self.enabled).view(theme)
    }
}

/// Two-segment button: [ action | ▾ ]. The divider and the chevron half's own
/// hover state are what distinguish it from a DropDownButton.
#[derive(Clone, Debug)]
pub struct SplitButton {
    pub label: String,
    pub enabled: bool,
}

impl SplitButton {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), enabled: true }
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let fill = if self.enabled {
            rc(t.control_fill_color_default)
        } else {
            rc(t.control_fill_color_disabled)
        };
        let text_c = if self.enabled {
            rc(t.text_fill_color_primary)
        } else {
            rc(t.text_fill_color_disabled)
        };
        let (top, bot) = elevation_border(t);
        el().role("button")
            .label(self.label.clone())
            .min_h(sizes::CONTROL_HEIGHT)
            .radius(4.0)
            .fill(fill)
            .border_gradient(1.0, top, bot)
            .child(
                row().items_center().child(
                    el().px(12.0)
                        .min_h(sizes::CONTROL_HEIGHT)
                        .center_y()
                        .hover_fill(rc(t.control_fill_color_secondary))
                        .corners([4.0, 0.0, 0.0, 4.0])
                        .child(text(self.label.clone()).font(body()).color(text_c).nowrap()),
                ),
            )
            .child(
                el().w(1.0)
                    .h(16.0)
                    .self_center()
                    .fill(rc(t.divider_stroke_color_default)),
            )
            .child(
                el().w(32.0)
                    .min_h(sizes::CONTROL_HEIGHT)
                    .center()
                    .corners([0.0, 4.0, 4.0, 0.0])
                    .hover_fill(rc(t.control_fill_color_secondary))
                    .child(glyph(Symbol::ChevronDown, 12.0, text_c)),
            )
    }
}

/// A button whose whole surface opens a menu — label + chevron, no divider.
#[derive(Clone, Debug)]
pub struct DropDownButton {
    pub label: String,
    pub enabled: bool,
}

impl DropDownButton {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), enabled: true }
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let (fill, text_c) = if self.enabled {
            (rc(t.control_fill_color_default), rc(t.text_fill_color_primary))
        } else {
            (rc(t.control_fill_color_disabled), rc(t.text_fill_color_disabled))
        };
        let (top, bot) = elevation_border(t);
        let mut e = el()
            .role("button")
            .label(self.label.clone())
            .min_h(sizes::CONTROL_HEIGHT)
            .min_w(40.0)
            .radius(4.0)
            .px(12.0)
            .center_y()
            .fill(fill)
            .child(
                row().items_center().gap(8.0).child(
                    text(self.label.clone()).font(body()).color(text_c).nowrap(),
                ).child(glyph(Symbol::ChevronDown, 12.0, text_c)),
            );
        if self.enabled {
            e = e.border_gradient(1.0, top, bot)
                .hover_fill(rc(t.control_fill_color_secondary))
                .press_fill(rc(t.control_fill_color_tertiary))
                .cursor(rune::Cursor::Pointer);
        } else {
            e = e.border(1.0, rc(t.control_strong_stroke_color_disabled));
        }
        e
    }
}
