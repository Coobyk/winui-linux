//! Selection controls: CheckBox, RadioButton, ToggleSwitch, RatingControl,
//! Slider, and a swatch-grid stand-in for ColorPicker.
//! Matches "Basic input" Gallery samples.

use rune::{Color, El, col, el, row, text};
use winui_icons::Symbol;
use winui_materials::rc;
use winui_theme::{Theme, sizes};

use crate::{body, glyph};

/// Checkbox states, including indeterminate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckState {
    Unchecked,
    Checked,
    Indeterminate,
}

/// A WinUI CheckBox: 20px box, accent fill + check glyph when checked,
/// accent fill + white bar when indeterminate, strong border when unchecked.
#[derive(Clone, Debug)]
pub struct CheckBox {
    pub label: String,
    pub state: CheckState,
    pub enabled: bool,
}

impl CheckBox {
    pub fn new(label: impl Into<String>, state: CheckState) -> Self {
        Self { label: label.into(), state, enabled: true }
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let on = self.state != CheckState::Unchecked;
        let (fill, border_c) = if !self.enabled {
            (rc(t.control_fill_color_disabled), rc(t.control_strong_stroke_color_disabled))
        } else if on {
            (rc(t.accent_fill_color_default), rc(t.accent_fill_color_default))
        } else {
            (rc(t.control_fill_color_transparent), rc(t.control_strong_stroke_color_default))
        };
        let mark = match self.state {
            CheckState::Checked => glyph(Symbol::CheckMark, sizes::CHECKBOX_GLYPH_SIZE, rc(t.text_on_accent_fill_color_primary)),
            CheckState::Indeterminate => el().size(10.0, 2.0).radius(1.0).fill(rc(t.text_on_accent_fill_color_primary)),
            CheckState::Unchecked => el().size(0.0, 0.0),
        };
        row().role("checkbox")
            .label(self.label.clone())
            .items_center()
            .min_h(sizes::CONTROL_HEIGHT)
            .child(
                el().size(sizes::CHECKBOX_SIZE, sizes::CHECKBOX_SIZE)
                    .radius(4.0)
                    .fill(fill)
                    .border(1.0, border_c)
                    .center()
                    .child(mark),
            )
            .child(
                text(self.label.clone())
                    .font(body())
                    .color(rc(if self.enabled { t.text_fill_color_primary } else { t.text_fill_color_disabled }))
                    .ml(8.0)
                    .nowrap(),
            )
    }
}

/// A WinUI RadioButton: 20px ring; checked = accent ring + white core +
/// accent dot, unchecked = strong ring.
#[derive(Clone, Debug)]
pub struct RadioButton {
    pub label: String,
    pub checked: bool,
    pub enabled: bool,
}

impl RadioButton {
    pub fn new(label: impl Into<String>, checked: bool) -> Self {
        Self { label: label.into(), checked, enabled: true }
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let ring = if !self.enabled {
            rc(t.control_strong_stroke_color_disabled)
        } else if self.checked {
            rc(t.accent_fill_color_default)
        } else {
            rc(t.control_strong_stroke_color_default)
        };
        let inner = if self.checked {
            // Accent ring, white centre, accent dot — WinUI's checked glyph.
            el().w_full()
                .h_full()
                .radius(999.0)
                .border(1.0, ring)
                .fill(rc(t.accent_fill_color_default))
                .center()
                .child(el().size(10.0, 10.0).radius(999.0).fill(rc(t.text_on_accent_fill_color_primary)))
        } else {
            el().w_full()
                .h_full()
                .radius(999.0)
                .border(1.0, ring)
        };
        row().role("radio")
            .label(self.label.clone())
            .items_center()
            .min_h(sizes::CONTROL_HEIGHT)
            .child(el().size(20.0, 20.0).child(inner))
            .child(
                text(self.label.clone())
                    .font(body())
                    .color(rc(if self.enabled { t.text_fill_color_primary } else { t.text_fill_color_disabled }))
                    .ml(8.0)
                    .nowrap(),
            )
    }
}

/// A WinUI ToggleSwitch: 40×20 pill; off = ring + small knob, on = accent
/// fill + larger light knob. Optional "On"/"Off" content label.
#[derive(Clone, Debug)]
pub struct ToggleSwitch {
    pub on: bool,
    pub enabled: bool,
    /// Header shown above/left in the Gallery (None hides it).
    pub header: Option<String>,
}

impl ToggleSwitch {
    pub fn new(on: bool) -> Self {
        Self { on, enabled: true, header: None }
    }
    pub fn header(mut self, h: impl Into<String>) -> Self {
        self.header = Some(h.into());
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let (track_fill, track_border, knob_fill, knob_d) = if !self.enabled {
            (rc(t.control_fill_color_disabled), rc(t.control_strong_stroke_color_disabled), rc(t.text_fill_color_disabled), 12.0)
        } else if self.on {
            (rc(t.accent_fill_color_default), rc(t.accent_fill_color_default), rc(t.text_on_accent_fill_color_primary), 14.0)
        } else {
            (rc(t.control_alt_fill_color_transparent), rc(t.control_strong_stroke_color_default), rc(t.text_fill_color_secondary), 12.0)
        };
        let pill = el().size(sizes::TOGGLE_SWITCH_WIDTH, sizes::TOGGLE_SWITCH_HEIGHT)
            .radius(999.0)
            .fill(track_fill)
            .border(1.0, track_border)
            .child(
                el().size(knob_d, knob_d)
                    .radius(999.0)
                    .fill(knob_fill)
                    .self_center()
                    .ml(if self.on { sizes::TOGGLE_SWITCH_WIDTH - knob_d - 4.0 } else { 4.0 }),
            );
        let mut r = row().items_center().gap(10.0).child(pill);
        if let Some(h) = &self.header {
            r = r.child(text(h.clone()).font(body()).color(rc(t.text_fill_color_primary)).nowrap());
        }
        r.role("switch").label(if self.on { "On" } else { "Off" })
    }
}

/// A read-only RatingControl: filled stars in the system text colour, the
/// rest as outline stars. (Interactive picking is a Gallery extra.)
#[derive(Clone, Debug)]
pub struct RatingControl {
    /// 0–5, halves allowed (e.g. 3.5).
    pub value: f32,
    pub enabled: bool,
}

impl RatingControl {
    pub fn new(value: f32) -> Self {
        Self { value: value.clamp(0.0, 5.0), enabled: true }
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let c = if self.enabled {
            rc(t.text_fill_color_primary)
        } else {
            rc(t.text_fill_color_disabled)
        };
        let mut stars = row().gap(4.0).items_center();
        for i in 0..5 {
            let v = self.value - i as f32;
            let sym = if v >= 0.75 {
                Symbol::StarFilled
            } else if v >= 0.25 {
                Symbol::StarHalf
            } else {
                Symbol::Star
            };
            stars = stars.child(glyph(sym, 16.0, c));
        }
        stars.role("slider").label(format!("{} stars", self.value))
    }
}

/// A horizontal Slider: 4px rail, accent fill on the live portion, 20px
/// ring thumb with a 12px core. `value` is 0..=1 of `min..=max`.
#[derive(Clone, Debug)]
pub struct Slider {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub enabled: bool,
    /// Optional header like the Gallery's "A Slider with a header".
    pub header: Option<String>,
}

impl Slider {
    pub fn new(value: f32) -> Self {
        Self { value, min: 0.0, max: 100.0, enabled: true, header: None }
    }
    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    pub fn header(mut self, h: impl Into<String>) -> Self {
        self.header = Some(h.into());
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    fn frac(&self) -> f32 {
        ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
    }
    pub fn view(&self, theme: &Theme, width: f32) -> El {
        let t = theme.tokens();
        let f = self.frac();
        let live = if self.enabled {
            rc(t.accent_fill_color_default)
        } else {
            rc(t.accent_fill_color_disabled)
        };
        let rest = rc(if self.enabled {
            t.control_strong_fill_color_default
        } else {
            t.control_strong_fill_color_disabled
        })
        .alpha(0.35);
        let thumb_d = 20.0;
        let thumb_x = (width - thumb_d) * f;
        let track = el().w(width)
            .h(32.0)
            .child(
                el().absolute(0.0, 14.0)
                    .size(width, sizes::SLIDER_TRACK_HEIGHT)
                    .radius(2.0)
                    .fill(rest),
            )
            .child(
                el().absolute(0.0, 14.0)
                    .size((width * f).max(4.0), sizes::SLIDER_TRACK_HEIGHT)
                    .radius(2.0)
                    .fill(live),
            )
            .child(
                el().absolute(thumb_x, 6.0)
                    .size(thumb_d, thumb_d)
                    .radius(999.0)
                    .fill(live)
                    .center()
                    .child(
                        el().size(12.0, 12.0)
                            .radius(999.0)
                            .fill(rc(t.control_fill_color_input_active))
                            .border(1.0, rc(t.control_stroke_color_default)),
                    ),
            );
        match &self.header {
            Some(h) => col().gap(6.0)
                .child(text(h.clone()).font(body()).color(rc(t.text_fill_color_primary)))
                .child(track)
                .role("slider")
                .label(format!("{:.0}", self.value)),
            None => track.role("slider").label(format!("{:.0}", self.value)),
        }
    }
}

/// The swatch grid half of a ColorPicker: rows of 24px rounded swatches with
/// an accent ring + check on the selected one.
#[derive(Clone, Debug)]
pub struct ColorSwatchGrid {
    pub swatches: Vec<Color>,
    /// Index of the selected swatch, if any.
    pub selected: Option<usize>,
    pub columns: usize,
}

impl ColorSwatchGrid {
    pub fn new(swatches: Vec<Color>) -> Self {
        Self { swatches, selected: None, columns: 8 }
    }
    pub fn selected(mut self, i: usize) -> Self {
        self.selected = Some(i);
        self
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let mut grid = col().gap(4.0);
        let mut i = 0;
        while i < self.swatches.len() {
            let mut r = row().gap(4.0);
            for j in 0..self.columns {
                let idx = i + j;
                if idx >= self.swatches.len() {
                    break;
                }
                let c = self.swatches[idx];
                let mut sw = el().size(24.0, 24.0).radius(4.0).fill(c);
                if self.selected == Some(idx) {
                    sw = sw
                        .border(2.0, rc(t.accent_fill_color_default))
                        .child(
                            el().w_full().h_full().center().child(
                                glyph(Symbol::CheckMark, 12.0, Color::WHITE),
                            ),
                        );
                } else {
                    sw = sw.border(1.0, rc(t.control_stroke_color_default));
                }
                r = r.child(sw.role("button").label(format!("swatch {idx}")));
            }
            grid = grid.child(r);
            i += self.columns;
        }
        grid
    }
}
