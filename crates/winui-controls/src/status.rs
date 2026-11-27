//! Status & info: InfoBar, ProgressBar, ProgressRing, TeachingTip, ToolTip.
//! Matches "Status & info" Gallery samples.

use rune::{El, LineCap, Path, Shape, canvas, col, el, row, text};
use winui_icons::Symbol;
use winui_materials::rc;
use winui_theme::{Theme, radii, sizes};

use crate::{body, caption, glyph};

/// InfoBar severity → icon + the system fills.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    Info,
    Success,
    Warning,
    Error,
}

impl Severity {
    fn symbol(self) -> Symbol {
        match self {
            Severity::Info => Symbol::Info,
            Severity::Success => Symbol::CheckmarkCircle,
            Severity::Warning => Symbol::Warning,
            Severity::Error => Symbol::ErrorCircle,
        }
    }
}

/// An InfoBar: severity icon + title + message + close, on the tinted
/// `SystemFillColor*Background` surface.
#[derive(Clone, Debug)]
pub struct InfoBar {
    pub severity: Severity,
    pub title: String,
    pub message: String,
    pub closable: bool,
}

impl InfoBar {
    pub fn new(severity: Severity, title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity,
            title: title.into(),
            message: message.into(),
            closable: true,
        }
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let (bg, fg) = match self.severity {
            Severity::Info => (t.system_fill_color_attention_background, t.system_fill_color_attention),
            Severity::Success => (t.system_fill_color_success_background, t.system_fill_color_success),
            Severity::Warning => (t.system_fill_color_caution_background, t.system_fill_color_caution),
            Severity::Error => (t.system_fill_color_critical_background, t.system_fill_color_critical),
        };
        let mut content = row().items_start().px(16.0).py(12.0).gap(12.0)
            .child(
                el().pt(1.0).child(glyph(self.severity.symbol(), sizes::INFO_BAR_ICON_SIZE, rc(fg))),
            )
            .child(
                col().gap(2.0).grow(1.0)
                    .child(
                        text(self.title.clone())
                            .font(rune::TextStyle::new(14.0, 600.0))
                            .color(rc(t.text_fill_color_primary)),
                    )
                    .child(text(self.message.clone()).font(body()).color(rc(t.text_fill_color_primary))),
            );
        if self.closable {
            content = content.child(
                el().size(24.0, 24.0)
                    .center()
                    .radius(4.0)
                    .role("button")
                    .label("Close")
                    .hover_fill(rc(t.subtle_fill_color_secondary))
                    .cursor(rune::Cursor::Pointer)
                    .child(glyph(Symbol::Dismiss, 10.0, rc(t.text_fill_color_secondary))),
            );
        }
        el().w_full()
            .min_h(sizes::INFO_BAR_MIN_HEIGHT)
            .radius(radii::OVERLAY)
            .fill(rc(bg))
            .border(1.0, rc(t.card_stroke_color_default))
            .child(content)
    }
}

/// A ProgressBar, determinate or indeterminate (indeterminate shows the
/// three-dot stream frozen mid-flight — the static look of the animation).
#[derive(Clone, Debug)]
pub struct ProgressBar {
    /// 0..=1 for determinate; `None` = indeterminate.
    pub value: Option<f32>,
}

impl ProgressBar {
    pub fn determinate(value: f32) -> Self {
        Self { value: Some(value.clamp(0.0, 1.0)) }
    }
    pub fn indeterminate() -> Self {
        Self { value: None }
    }
    pub fn view(&self, theme: &Theme, width: f32) -> El {
        let t = theme.tokens();
        let track = rc(t.control_fill_color_transparent).alpha(0.0);
        let rest = rc(t.control_strong_fill_color_default).alpha(0.25);
        let _ = track;
        let accent = rc(t.accent_fill_color_default);
        let mut e = el().w(width).h(4.0).radius(2.0).fill(rest);
        match self.value {
            Some(v) => {
                e = e.child(
                    el().size((width * v).max(4.0), 4.0).radius(2.0).fill(accent),
                );
            }
            None => {
                // WinUI's indeterminate dots: five accent dots evenly spaced.
                let mut dots = row().w_full().h_full().justify_between();
                for _ in 0..5 {
                    dots = dots.child(el().size(4.0, 4.0).radius(999.0).fill(accent));
                }
                e = e.fill(rune::Color::TRANSPARENT).child(dots);
            }
        }
        e.role("progressbar").label("progress")
    }
}

/// A ProgressRing: strong-fill track circle + accent arc. `value` = 0..=1 for
/// determinate; `None` draws the indeterminate 3/4 sweep.
#[derive(Clone, Debug)]
pub struct ProgressRing {
    pub value: Option<f32>,
    pub size: f32,
}

/// A circle arc as cubic segments (κ = 0.5523), centred in a `2r` box.
/// `start`/`sweep` in degrees, 0° at 12 o'clock, clockwise.
fn arc(r: f32, start: f32, sweep: f32) -> Path {
    let k = 0.552_284_75_f32;
    let mut path = Path::new();
    let steps = (sweep.abs() / 90.0).ceil().max(1.0) as usize;
    for i in 0..steps {
        let a0 = (start + sweep * i as f32 / steps as f32).to_radians();
        let a1 = (start + sweep * (i + 1) as f32 / steps as f32).to_radians();
        let p = |a: f32, rr: f32| [r + rr * a.sin(), r - rr * a.cos()];
        let [x0, y0] = p(a0, r);
        let [x1, y1] = p(a1, r);
        let seg = (a1 - a0).abs();
        let d = r * k * (seg / (std::f32::consts::PI / 2.0));
        let t0 = [a0.cos(), a0.sin()];
        let t1 = [a1.cos(), a1.sin()];
        let c1 = [x0 + d * t0[0], y0 + d * t0[1]];
        let c2 = [x1 - d * t1[0], y1 - d * t1[1]];
        if i == 0 {
            path = path.move_to([x0, y0]);
        }
        path = path.cubic_to(c1, c2, [x1, y1]);
    }
    path
}

impl ProgressRing {
    pub fn determinate(value: f32) -> Self {
        Self { value: Some(value.clamp(0.0, 1.0)), size: 32.0 }
    }
    pub fn indeterminate() -> Self {
        Self { value: None, size: 32.0 }
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let r = self.size / 2.0;
        let stroke = (self.size / 16.0).clamp(1.5, 3.0);
        let track = Shape::new().stroke_path(&arc(r, 0.0, 360.0), stroke, LineCap::Round, rc(t.control_strong_fill_color_default).alpha(0.25));
        let (start, sweep) = match self.value {
            Some(v) => (0.0, 360.0 * v),
            None => (0.0, 270.0),
        };
        let ring = track.stroke_path(&arc(r, start, sweep), stroke, LineCap::Round, rc(t.accent_fill_color_default));
        canvas(ring).size(self.size, self.size)
    }
}

/// TeachingTip-lite: elevated card with title, body and a dismiss button.
#[derive(Clone, Debug)]
pub struct TeachingTip {
    pub title: String,
    pub body: String,
}

impl TeachingTip {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self { title: title.into(), body: body.into() }
    }
    pub fn view(&self, theme: &Theme, width: f32) -> El {
        let t = theme.tokens();
        el().w(width)
            .radius(radii::OVERLAY)
            .fill(rc(t.layer_on_acrylic_fill_color_default))
            .border(1.0, rc(t.surface_stroke_color_flyout))
            .box_shadow(winui_materials::elevation(theme, 32.0))
            .child(
                col().padding(16.0).gap(6.0)
                    .child(
                        row().items_center()
                            .child(
                                text(self.title.clone())
                                    .font(rune::TextStyle::new(14.0, 600.0))
                                    .color(rc(t.text_fill_color_primary))
                                    .grow(1.0),
                            )
                            .child(
                                el().size(24.0, 24.0)
                                    .center()
                                    .radius(4.0)
                                    .role("button")
                                    .label("Dismiss")
                                    .hover_fill(rc(t.subtle_fill_color_secondary))
                                    .child(glyph(Symbol::Dismiss, 10.0, rc(t.text_fill_color_secondary))),
                            ),
                    )
                    .child(text(self.body.clone()).font(body()).color(rc(t.text_fill_color_secondary))),
            )
    }
}

/// A ToolTip: small dark caption on an elevated overlay surface.
#[derive(Clone, Debug)]
pub struct ToolTip {
    pub text: String,
}

impl ToolTip {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
    pub fn view(&self, theme: &Theme) -> El {
        let t = theme.tokens();
        let pad = sizes::TOOLTIP_PADDING;
        el().radius(4.0)
            .fill(rc(t.layer_on_acrylic_fill_color_default))
            .border(1.0, rc(t.surface_stroke_color_flyout))
            .box_shadow(winui_materials::elevation(theme, 8.0))
            .pl(pad.left)
            .pr(pad.right)
            .pt(pad.top)
            .pb(pad.bottom)
            .child(
                text(self.text.clone())
                    .font(caption())
                    .color(rc(t.text_fill_color_primary))
                    .nowrap(),
            )
    }
}
