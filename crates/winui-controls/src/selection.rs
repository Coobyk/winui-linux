//! Selection: `CheckBox`, `RadioButton`, `ToggleSwitch`, `RatingControl` (WinUI Gallery ›
//! Basic input). Slider and ColorPicker live in `range`.
use rune::{col, row, El};

use crate::impl_control;
use crate::shim::icons::Symbol;
use crate::shim::theme;
use crate::shim::theme::{sizes, Theme};
use crate::style::{self, focus_rect, glyph, layers, rc, Hook, R};

/// Tri-state value (`IsThreeState`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CheckState {
    #[default]
    Unchecked,
    Checked,
    Indeterminate,
}

/// `CheckBox` — Gallery "CheckBox": 20px box, 4px corners, 8px label gap, 32px row.
#[derive(Clone, Default)]
pub struct CheckBox {
    label: String,
    state: CheckState,
    enabled: bool,
    three_state: bool,
    key: Option<String>,
    toggle: Hook<CheckState>,
}

impl CheckBox {
    pub fn new(label: impl Into<String>) -> Self {
        CheckBox { label: label.into(), enabled: true, ..Default::default() }
    }
    pub fn checked(mut self, on: bool) -> Self {
        self.state = if on { CheckState::Checked } else { CheckState::Unchecked };
        self
    }
    pub fn state(mut self, s: CheckState) -> Self {
        self.state = s;
        self
    }
    pub fn three_state(mut self, on: bool) -> Self {
        self.three_state = on;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }
    /// `f(state, next)`: Unchecked → Checked → (Indeterminate →) Unchecked.
    pub fn on_toggle<S: 'static>(mut self, f: impl Fn(&mut S, CheckState) + 'static) -> Self {
        self.toggle.press(f);
        self
    }
    fn next(&self) -> CheckState {
        match self.state {
            CheckState::Unchecked => CheckState::Checked,
            CheckState::Checked if self.three_state => CheckState::Indeterminate,
            _ => CheckState::Unchecked,
        }
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let on = self.state != CheckState::Unchecked;
        let en = self.enabled;
        let (rest, hover) = match (on, en) {
            (false, true) => (
                (k.control_alt_fill_color_secondary, Some(k.control_strong_stroke_color_default)),
                (k.control_alt_fill_color_tertiary, Some(k.control_strong_stroke_color_default)),
            ),
            (false, false) => {
                let d = (k.control_alt_fill_color_disabled, Some(k.control_strong_stroke_color_disabled));
                (d, d)
            }
            (true, true) => ((k.accent_fill_color_default, None), (k.accent_fill_color_secondary, None)),
            (true, false) => ((k.accent_fill_color_disabled, None), (k.accent_fill_color_disabled, None)),
        };
        let mark_c = if en { k.text_on_accent_fill_color_primary } else { k.text_on_accent_fill_color_disabled };
        let mut bx = layers((20.0, 20.0), R, rest, hover).center();
        bx = match self.state {
            CheckState::Checked => bx.child(checkmark(mark_c)),
            CheckState::Indeterminate => bx.child(rune::el().size(8.0, 1.5).radius(1.0).fill(rc(mark_c))),
            CheckState::Unchecked => bx,
        };
        let fg = if en { k.text_fill_color_primary } else { k.text_fill_color_disabled };
        let mut e = row()
            .min_h(sizes::CONTROL_HEIGHT)
            .gap(8.0)
            .center_y()
            .radius(R)
            .hover_group()
            .child(bx)
            .child(rune::text(self.label.clone()).font(style::body(t)).color(rc(fg)))
            .role("checkbox")
            .label(self.label.clone())
            .disabled(!en);
        e = match self.state {
            CheckState::Indeterminate => e.checked_mixed(),
            s => e.checked(s == CheckState::Checked),
        };
        if en {
            e = focus_rect(e.press_color(rc(k.text_fill_color_secondary)), t, R);
            e = self.toggle.attach(e, self.next());
        }
        match &self.key {
            Some(key) => e.key(key.clone()),
            None => e,
        }
    }
}

/// `RadioButton` — Gallery "RadioButton": 20px ring, accent fill with a 12px dot when checked.
#[derive(Clone, Default)]
pub struct RadioButton {
    label: String,
    checked: bool,
    enabled: bool,
    key: Option<String>,
    select: Hook<()>,
}

impl RadioButton {
    pub fn new(label: impl Into<String>) -> Self {
        RadioButton { label: label.into(), enabled: true, ..Default::default() }
    }
    pub fn checked(mut self, on: bool) -> Self {
        self.checked = on;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }
    pub fn on_select<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.select.press(move |s: &mut S, ()| f(s));
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let en = self.enabled;
        let c = 10.0;
        let (rest, hover) = match (self.checked, en) {
            (false, true) => (
                (k.control_alt_fill_color_secondary, Some(k.control_strong_stroke_color_default)),
                (k.control_alt_fill_color_tertiary, Some(k.control_strong_stroke_color_default)),
            ),
            (false, false) => {
                let d = (k.control_alt_fill_color_disabled, Some(k.control_strong_stroke_color_disabled));
                (d, d)
            }
            (true, true) => ((k.accent_fill_color_default, None), (k.accent_fill_color_secondary, None)),
            (true, false) => ((k.accent_fill_color_disabled, None), (k.accent_fill_color_disabled, None)),
        };
        let mut ring = layers((20.0, 20.0), c, rest, hover).center();
        if self.checked {
            let dot = if en { k.text_on_accent_fill_color_primary } else { k.text_on_accent_fill_color_disabled };
            let d = |size: f32| rune::el().absolute((20.0 - size) / 2.0, (20.0 - size) / 2.0).size(size, size).radius(size / 2.0).fill(rc(dot));
            ring = ring.child(d(12.0).group_opacity(1.0, 0.0)).child(d(14.0).group_opacity(0.0, 1.0));
        }
        let fg = if en { k.text_fill_color_primary } else { k.text_fill_color_disabled };
        let mut e = row()
            .min_h(sizes::CONTROL_HEIGHT)
            .gap(8.0)
            .center_y()
            .radius(R)
            .hover_group()
            .child(ring)
            .child(rune::text(self.label.clone()).font(style::body(t)).color(rc(fg)))
            .role("radio")
            .label(self.label.clone())
            .checked(self.checked)
            .disabled(!en);
        if en {
            e = focus_rect(e.press_color(rc(k.text_fill_color_secondary)), t, R);
            e = self.select.attach(e, ());
        }
        match &self.key {
            Some(key) => e.key(key.clone()),
            None => e,
        }
    }
}

/// A vertical group of `RadioButton`s with a header — Gallery "RadioButtons".
#[derive(Clone, Default)]
pub struct RadioButtons {
    header: Option<String>,
    items: Vec<String>,
    selected: Option<usize>,
    enabled: bool,
    select: Hook<usize>,
}

impl RadioButtons {
    pub fn new<I: Into<String>>(items: impl IntoIterator<Item = I>) -> Self {
        RadioButtons { items: items.into_iter().map(Into::into).collect(), enabled: true, ..Default::default() }
    }
    pub fn header(mut self, h: impl Into<String>) -> Self {
        self.header = Some(h.into());
        self
    }
    pub fn selected(mut self, i: Option<usize>) -> Self {
        self.selected = i;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn on_select<S: 'static>(mut self, f: impl Fn(&mut S, usize) + 'static) -> Self {
        self.select.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let mut c = col().role("radiogroup");
        if let Some(h) = &self.header {
            c = c.child(header(h, t, self.enabled));
        }
        for (i, item) in self.items.iter().enumerate() {
            let r = RadioButton::new(item.clone()).checked(self.selected == Some(i)).enabled(self.enabled).key(format!("radio:{item}"));
            c = c.child(if self.enabled { self.select.attach(r.view(t), i) } else { r.view(t) });
        }
        c
    }
}

/// A control header (`ControlHeaderThemeStyle`): body text, 8px above the control.
pub fn header(s: &str, t: &Theme, enabled: bool) -> El {
    let k = t.tokens();
    rune::text(s.to_owned()).font(style::body(t)).color(rc(if enabled { k.text_fill_color_primary } else { k.text_fill_color_disabled })).pb(8.0)
}

/// `ToggleSwitch` — Gallery "ToggleSwitch": 40×20 pill track, 12px knob (14px on hover),
/// On/Off content 12px to the right.
#[derive(Clone)]
pub struct ToggleSwitch {
    header: Option<String>,
    on: bool,
    enabled: bool,
    on_content: String,
    off_content: String,
    key: Option<String>,
    toggle: Hook<bool>,
}

impl Default for ToggleSwitch {
    fn default() -> Self {
        ToggleSwitch {
            header: None,
            on: false,
            enabled: true,
            on_content: "On".into(),
            off_content: "Off".into(),
            key: None,
            toggle: Hook::default(),
        }
    }
}

impl ToggleSwitch {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn header(mut self, h: impl Into<String>) -> Self {
        self.header = Some(h.into());
        self
    }
    pub fn on(mut self, on: bool) -> Self {
        self.on = on;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn content(mut self, on: impl Into<String>, off: impl Into<String>) -> Self {
        self.on_content = on.into();
        self.off_content = off.into();
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }
    pub fn on_toggle<S: 'static>(mut self, f: impl Fn(&mut S, bool) + 'static) -> Self {
        self.toggle.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let en = self.enabled;
        let (rest, hover, knob) = match (self.on, en) {
            (false, true) => (
                (k.control_alt_fill_color_secondary, Some(k.control_strong_stroke_color_default)),
                (k.control_alt_fill_color_tertiary, Some(k.control_strong_stroke_color_default)),
                k.text_fill_color_secondary,
            ),
            (false, false) => {
                let d = (k.control_alt_fill_color_disabled, Some(k.control_strong_stroke_color_disabled));
                (d, d, k.text_fill_color_disabled)
            }
            (true, true) => ((k.accent_fill_color_default, None), (k.accent_fill_color_secondary, None), k.text_on_accent_fill_color_primary),
            (true, false) => ((k.accent_fill_color_disabled, None), (k.accent_fill_color_disabled, None), k.text_on_accent_fill_color_disabled),
        };
        let knob_at = |size: f32| {
            let x = if self.on { 40.0 - 4.0 - size + (size - 12.0) / 2.0 } else { 4.0 - (size - 12.0) / 2.0 };
            rune::el().absolute(x, (20.0 - size) / 2.0).size(size, size).radius(size / 2.0).fill(rc(knob))
        };
        let track = layers((40.0, 20.0), 10.0, rest, hover)
            .child(knob_at(12.0).group_opacity(1.0, 0.0))
            .child(knob_at(14.0).group_opacity(0.0, 1.0));
        let fg = if en { k.text_fill_color_primary } else { k.text_fill_color_disabled };
        let content = if self.on { &self.on_content } else { &self.off_content };
        let mut sw = row()
            .min_h(sizes::CONTROL_HEIGHT)
            .min_w(154.0)
            .gap(12.0)
            .center_y()
            .radius(R)
            .hover_group()
            .child(track)
            .child(rune::text(content.clone()).font(style::body(t)).color(rc(fg)))
            .role("switch")
            .label(self.header.clone().unwrap_or_else(|| content.clone()))
            .checked(self.on)
            .disabled(!en);
        if en {
            sw = focus_rect(sw, t, R);
            sw = self.toggle.attach(sw, !self.on);
        }
        if let Some(key) = &self.key {
            sw = sw.key(key.clone());
        }
        match &self.header {
            Some(h) => col().child(header(h, t, en)).child(sw),
            None => sw,
        }
    }
}

/// `RatingControl` — Gallery "RatingControl": five 20px stars, 8px apart, accent when rated.
#[derive(Clone)]
pub struct RatingControl {
    value: f32,
    max: u32,
    enabled: bool,
    caption: Option<String>,
    key: String,
    rate: Hook<u32>,
}

impl Default for RatingControl {
    fn default() -> Self {
        RatingControl { value: 0.0, max: 5, enabled: true, caption: None, key: "rating".into(), rate: Hook::default() }
    }
}

impl RatingControl {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn value(mut self, v: f32) -> Self {
        self.value = v;
        self
    }
    pub fn max(mut self, n: u32) -> Self {
        self.max = n;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn caption(mut self, s: impl Into<String>) -> Self {
        self.caption = Some(s.into());
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = k.into();
        self
    }
    pub fn on_rate<S: 'static>(mut self, f: impl Fn(&mut S, u32) + 'static) -> Self {
        self.rate.press(f);
        self
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let en = self.enabled;
        let filled = if en { k.accent_fill_color_default } else { k.accent_fill_color_disabled };
        let empty = if en { k.control_strong_stroke_color_default } else { k.control_strong_stroke_color_disabled };
        let mut r = row().gap(8.0).center_y().min_h(sizes::CONTROL_HEIGHT).role("slider").label("Rating").range_value(self.value as f64, 0.0, self.max as f64);
        for i in 0..self.max {
            let on = self.value >= i as f32 + 0.5;
            let star = if on { glyph(Symbol::StarFilled, 20.0, filled) } else { glyph(Symbol::Star, 20.0, empty) };
            let mut cell = rune::el().size(20.0, 20.0).radius(R).child(star).key(format!("{}:{}", self.key, i + 1));
            if en {
                cell = self.rate.attach(cell.hover(rc(k.subtle_fill_color_secondary)), i + 1);
            }
            r = r.child(cell);
        }
        if let Some(c) = &self.caption {
            r = r.child(rune::text(c.clone()).font(style::caption(t)).color(rc(k.text_fill_color_secondary)).pl(4.0));
        }
        if en { focus_rect(r.key(self.key.clone()), t, R) } else { r.key(self.key.clone()) }
    }
}


/// The CheckBox glyph (`CheckBoxCheckedGlyph`, E73E at 12px), stroked.
pub(crate) fn checkmark(c: theme::Color) -> El {
    rune::canvas(rune::Shape::new().stroke(vec![[1.5, 6.5], [4.5, 9.5], [10.5, 2.5]], 1.4, rc(c)).view_box(12.0, 12.0)).size(12.0, 12.0)
}

impl_control!(CheckBox, RadioButton, RadioButtons, ToggleSwitch, RatingControl);
