//! Range controls: `Slider` and `ColorPicker`-lite (WinUI Gallery › Basic input › Slider,
//! ColorPicker).
use std::rc::Rc;

use rune::{col, row, El, Phase};

use crate::impl_control;
use crate::selection::header;
use crate::shim::theme::{self, sizes, Theme};
use crate::style::{self, focus_rect, rc, Hook, R};

/// The slider geometry (`SliderTrackThemeHeight` 4, thumb 20 with a 12px accent core).
/// `SliderHorizontalThumbWidth` / `Height`.
const THUMB: f32 = 18.0;
const TRACK: f32 = 4.0;

/// `Slider` — Gallery "Slider": 4px track, accent value fill, 20px thumb whose accent core
/// grows on hover (14px) and shrinks on press (10px).
#[derive(Clone)]
pub struct Slider {
    header: Option<String>,
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    width: f32,
    enabled: bool,
    key: String,
    change: Hook<(f64, f64, f64, f32)>,
}

impl Default for Slider {
    fn default() -> Self {
        Slider { header: None, value: 0.0, min: 0.0, max: 100.0, step: 1.0, width: 300.0, enabled: true, key: "slider".into(), change: Hook::default() }
    }
}

impl Slider {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn header(mut self, h: impl Into<String>) -> Self {
        self.header = Some(h.into());
        self
    }
    pub fn value(mut self, v: f64) -> Self {
        self.value = v;
        self
    }
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    pub fn step(mut self, s: f64) -> Self {
        self.step = s;
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = k.into();
        self
    }
    /// `f(state, value)` while dragging or clicking the track, snapped to `step`.
    pub fn on_change<S: 'static>(mut self, f: impl Fn(&mut S, f64) + 'static) -> Self {
        let f = Rc::new(f);
        self.change.raw(move |e, (min, max, step, w)| {
            let f = f.clone();
            e.on_drag(move |s: &mut S, d: &mut rune::Drag, _tx: &mut rune::Tx| {
                if d.phase == Phase::Up && d.dx == 0.0 && d.dy == 0.0 {
                    return;
                }
                f(s, value_at(d.x, w, min, max, step));
            })
        });
        self
    }
    fn frac(&self) -> f32 {
        if self.max > self.min { ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0) as f32 } else { 0.0 }
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let en = self.enabled;
        let w = self.width;
        let travel = w - THUMB;
        let x = travel * self.frac();
        // SliderTrackFill / SliderTrackValueFill / SliderThumbBackground per state.
        let (fill, hover_fill, rest) = if en {
            (style::by_state(k.accent_fill_color_default, k.accent_fill_color_secondary, k.accent_fill_color_tertiary), k.accent_fill_color_secondary, k.control_strong_fill_color_default)
        } else {
            (k.accent_fill_color_disabled, k.accent_fill_color_disabled, k.control_strong_fill_color_disabled)
        };
        let ty = (sizes::CONTROL_HEIGHT - TRACK) / 2.0;
        let track = rune::el().absolute(THUMB / 2.0, ty).size(travel, TRACK).radius(TRACK / 2.0).fill(rc(rest));
        let value = |c: theme::Color| rune::el().absolute(THUMB / 2.0, ty).size(x.max(0.0), TRACK).radius(TRACK / 2.0).fill(rc(c));
        let thumb = thumb(t, en, x, (sizes::CONTROL_HEIGHT - THUMB) / 2.0, fill, hover_fill);
        let mut s = rune::el()
            .size(w, sizes::CONTROL_HEIGHT)
            .hover_group()
            .child(track)
            .child(style::hover_layer(value(fill), 1.0, 0.0))
            .child(style::hover_layer(value(hover_fill), 0.0, 1.0))
            .child(thumb)
            .role("slider")
            .label(self.header.clone().unwrap_or_else(|| "Slider".into()))
            .range_value(self.value, self.min, self.max)
            .disabled(!en)
            .key(self.key.clone());
        if en {
            s = focus_rect(s, t, R);
            s = self.change.attach(s, (self.min, self.max, self.step, w));
        }
        match &self.header {
            Some(h) => col().child(header(h, t, en)).child(s),
            None => s,
        }
    }
}

/// The value under a pointer `x` (element points) on a slider `w` wide.
pub fn value_at(x: f32, w: f32, min: f64, max: f64, step: f64) -> f64 {
    let f = ((x - THUMB / 2.0) / (w - THUMB).max(1.0)).clamp(0.0, 1.0) as f64;
    let v = min + f * (max - min);
    let v = if step > 0.0 { min + ((v - min) / step).round() * step } else { v };
    v.clamp(min, max)
}

fn thumb(t: &Theme, en: bool, x: f32, y: f32, core: theme::Color, hover_core: theme::Color) -> El {
    let k = t.tokens();
    // SliderOuterThumbBackground + SliderThumbBorderBrush (ControlElevationBorderBrush).
    let g = &k.control_elevation_border;
    let ring = rune::el()
        .absolute(x, y)
        .size(THUMB, THUMB)
        .radius(THUMB / 2.0)
        .fill(rc(k.control_solid_fill_color_default))
        .border_gradient(1.0, rc(g.top(THUMB)), rc(g.bottom(THUMB)));
    let dot = |size: f32, c: theme::Color| {
        rune::el().absolute((THUMB - size) / 2.0, (THUMB - size) / 2.0).size(size, size).radius(size / 2.0).fill(rc(c))
    };
    // SliderInnerThumb (12px) scaled 0.86 at rest, 1.167 PointerOver, 0.71 Pressed.
    let (rest, hover, pressed) = (12.0 * 0.86, 12.0 * 1.167, 12.0 * 0.71);
    if !en {
        return ring.child(dot(rest, core));
    }
    match style::pinned() {
        Some(style::VisualState::Pressed) => ring.child(dot(pressed, core)),
        _ => ring
            .child(style::hover_layer(dot(rest, core), 1.0, 0.0))
            .child(style::hover_layer(dot(hover, hover_core), 0.0, 1.0).press_scale(pressed / hover)),
    }
}

/// `ColorPicker`-lite — Gallery "ColorPicker" with `ColorSpectrumShape` swapped for a swatch
/// grid: a preview, hex readout, RGB channel sliders and the Windows accent swatches.
#[derive(Clone)]
pub struct ColorPicker {
    color: theme::Color,
    enabled: bool,
    key: String,
    change: Hook<theme::Color>,
    channel: Hook<(usize, theme::Color)>,
}

/// The Windows "Personalization › Colors" accent palette (first rows).
pub const SWATCHES: [u32; 16] = [
    0xFFB900, 0xFF8C00, 0xF7630C, 0xCA5010, 0xDA3B01, 0xEF6950, 0xD13438, 0xFF4343, 0xE74856, 0xE81123, 0xEA005E,
    0xC30052, 0x0078D4, 0x0063B1, 0x8E8CD8, 0x00B294,
];

impl ColorPicker {
    pub fn new(color: theme::Color) -> Self {
        ColorPicker { color, enabled: true, key: "colorpicker".into(), change: Hook::default(), channel: Hook::default() }
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = k.into();
        self
    }
    pub fn on_change<S: 'static>(mut self, f: impl Fn(&mut S, theme::Color) + 'static) -> Self {
        let f = Rc::new(f);
        let g = f.clone();
        self.change.press(move |s: &mut S, c| f(s, c));
        self.channel.raw(move |e, (i, base): (usize, theme::Color)| {
            let g = g.clone();
            e.on_drag(move |s: &mut S, d: &mut rune::Drag, _tx: &mut rune::Tx| {
                let v = value_at(d.x, 256.0, 0.0, 255.0, 1.0) as f32 / 255.0;
                let mut c = base;
                match i {
                    0 => c.r = v,
                    1 => c.g = v,
                    _ => c.b = v,
                }
                g(s, c);
            })
        });
        self
    }
    pub fn hex(&self) -> String {
        let b = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        format!("#{:02X}{:02X}{:02X}", b(self.color.r), b(self.color.g), b(self.color.b))
    }
    pub fn view(&self, t: &Theme) -> El {
        let k = t.tokens();
        let c = self.color;
        let preview = rune::el().size(44.0, 44.0 * 3.0 + 16.0).radius(R).fill(rc(c)).border(1.0, rc(k.control_stroke_color_default));
        let mut channels = col().gap(8.0);
        for (i, (name, v)) in [("Red", c.r), ("Green", c.g), ("Blue", c.b)].into_iter().enumerate() {
            let (lo, hi) = match i {
                0 => (theme::Color { r: 0.0, ..c }, theme::Color { r: 1.0, ..c }),
                1 => (theme::Color { g: 0.0, ..c }, theme::Color { g: 1.0, ..c }),
                _ => (theme::Color { b: 0.0, ..c }, theme::Color { b: 1.0, ..c }),
            };
            let x = (256.0 - THUMB) * v;
            let bar = rune::el()
                .absolute(0.0, (sizes::CONTROL_HEIGHT - 12.0) / 2.0)
                .size(256.0, 12.0)
                .radius(6.0)
                .linear_gradient(90.0, [(0.0, rc(lo.with_alpha(1.0))), (1.0, rc(hi.with_alpha(1.0)))]);
            let mut track = rune::el()
                .size(256.0, sizes::CONTROL_HEIGHT)
                .hover_group()
                .child(bar)
                .child(thumb(t, self.enabled, x, (sizes::CONTROL_HEIGHT - THUMB) / 2.0, c.with_alpha(1.0), c.with_alpha(1.0)))
                .role("slider")
                .label(name)
                .range_value((v * 255.0).round() as f64, 0.0, 255.0)
                .key(format!("{}:{}", self.key, name.to_lowercase()));
            if self.enabled {
                track = self.channel.attach(focus_rect(track, t, R), (i, c));
            }
            channels = channels.child(
                row().gap(12.0).center_y()
                    .child(rune::text(name).font(style::body(t)).color(rc(k.text_fill_color_secondary)).w(48.0))
                    .child(track)
                    .child(rune::text(format!("{}", (v * 255.0).round() as u8)).font(style::body(t)).color(rc(k.text_fill_color_primary))),
            );
        }
        let mut swatches = row().gap(4.0).flex_wrap().w(8.0 * 28.0);
        for (i, hex) in SWATCHES.iter().enumerate() {
            let sc = theme::Color::rgb((hex >> 16) as u8, (hex >> 8) as u8, *hex as u8);
            let selected = self.hex() == format!("#{hex:06X}");
            let mut sw = rune::el().size(24.0, 24.0).radius(R).fill(rc(sc)).key(format!("{}:swatch{i}", self.key)).role("button").label(format!("#{hex:06X}"));
            if selected {
                sw = sw.outline(2.0, 1.0, rc(k.text_fill_color_primary));
            }
            if self.enabled {
                sw = self.change.attach(focus_rect(sw.hover_border(rc(k.text_fill_color_primary)).border(1.0, rc(k.control_stroke_color_default)), t, R), sc);
            }
            swatches = swatches.child(sw);
        }
        let hex = row().gap(8.0).center_y()
            .child(rune::text("Hex").font(style::body(t)).color(rc(k.text_fill_color_secondary)))
            .child(rune::text(self.hex()).font(style::body(t)).color(rc(k.text_fill_color_primary)));
        row()
            .gap(16.0)
            .key(self.key.clone())
            .child(preview)
            .child(col().gap(12.0).child(channels).child(hex))
            .child(swatches)
            .disabled(!self.enabled)
    }
}

impl_control!(Slider, ColorPicker);
