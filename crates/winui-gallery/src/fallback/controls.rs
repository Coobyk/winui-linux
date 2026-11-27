//! Fallback for the `winui-controls` contract: WinUI 3 controls as Rune element builders.
//! Each takes the `Theme` explicitly; sizes and brushes follow microsoft-ui-xaml
//! `*_themeresources.xaml` (Button, CheckBox, RadioButton, ToggleSwitch, Slider, TextBox, ...).

use super::icons::Symbol;
use super::materials;
use super::theme::{Color, TYPE_RAMP, Theme, radii, sizes};
use rune::shape::Shape;
use rune::{El, TextEdit, canvas, col, el, row, text, text_input};

fn c(x: Color) -> rune::Color {
    x.into()
}

/// A Fluent System Icon at `size` points.
pub fn icon(sym: Symbol, size: f32, color: Color) -> El {
    let rule = if sym.evenodd() { rune::FillRule::EvenOdd } else { rune::FillRule::NonZero };
    canvas(Shape::new().view_box(20.0, 20.0).fill_svg_with(sym.path(), rule, c(color))).size(size, size)
}

pub fn body(s: impl Into<String>, color: Color) -> El {
    text(s).font(TYPE_RAMP.body.rune()).color(c(color))
}

pub fn caption(s: impl Into<String>, color: Color) -> El {
    text(s).font(TYPE_RAMP.caption.rune()).color(c(color))
}

pub fn strong(s: impl Into<String>, color: Color) -> El {
    text(s).font(TYPE_RAMP.body_strong.rune()).color(c(color))
}

fn focus(e: El, theme: &Theme, radius: f32) -> El {
    e.focusable(true).focus_ring(2.0, 1.0, c(theme.tokens().focus_stroke_color_outer)).focus_ring_radius(radius + 2.0)
}

/// Button styles: `DefaultButtonStyle`, `AccentButtonStyle`, subtle (`AppBarButton`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonStyle {
    Standard,
    Accent,
    Subtle,
}

/// Button frame (no content): states from ButtonBackground{,PointerOver,Pressed,Disabled}.
pub fn button_frame<S: 'static>(theme: &Theme, key: &str, style: ButtonStyle, enabled: bool, on: impl Fn(&mut S) + 'static) -> El {
    let t = theme.tokens();
    let (rest, over, press, dis, stroke) = match style {
        ButtonStyle::Standard => (
            t.control_fill_color_default,
            t.control_fill_color_secondary,
            t.control_fill_color_tertiary,
            t.control_fill_color_disabled,
            t.control_stroke_color_default,
        ),
        ButtonStyle::Accent => (
            t.accent_fill_color_default,
            t.accent_fill_color_secondary,
            t.accent_fill_color_tertiary,
            t.accent_fill_color_disabled,
            t.control_stroke_color_on_accent_default,
        ),
        ButtonStyle::Subtle => (
            t.subtle_fill_color_transparent,
            t.subtle_fill_color_secondary,
            t.subtle_fill_color_tertiary,
            t.subtle_fill_color_transparent,
            t.subtle_fill_color_transparent,
        ),
    };
    let bottom = match style {
        ButtonStyle::Accent => t.control_stroke_color_on_accent_secondary,
        ButtonStyle::Standard => t.control_stroke_color_secondary,
        ButtonStyle::Subtle => t.subtle_fill_color_transparent,
    };
    let mut e = row().key(key).role("button").h(sizes::CONTROL_HEIGHT).radius(radii::CONTROL).center().gap(8.0);
    if enabled {
        e = e.fill(c(rest)).border(1.0, c(stroke)).shadow(1.0, 0.0, c(bottom)).hover_fill(c(over)).press_fill(c(press)).on_press(on);
        e = focus(e, theme, radii::CONTROL);
    } else {
        e = e.fill(c(dis)).border(1.0, c(t.control_stroke_color_default)).disabled(true);
    }
    e
}

fn button_text_color(theme: &Theme, style: ButtonStyle, enabled: bool) -> Color {
    let t = theme.tokens();
    match (style, enabled) {
        (_, false) if style == ButtonStyle::Accent => t.text_on_accent_fill_color_disabled,
        (_, false) => t.text_fill_color_disabled,
        (ButtonStyle::Accent, true) => t.text_on_accent_fill_color_primary,
        _ => t.text_fill_color_primary,
    }
}

/// `Button` with a text label (padding 11,5,11,6).
pub fn button<S: 'static>(theme: &Theme, key: &str, label: &str, style: ButtonStyle, enabled: bool, on: impl Fn(&mut S) + 'static) -> El {
    let fg = button_text_color(theme, style, enabled);
    button_frame(theme, key, style, enabled, on).px(11.0).label(label).child(body(label, fg))
}

/// `Button` with a leading icon.
pub fn icon_button<S: 'static>(theme: &Theme, key: &str, sym: Symbol, label: Option<&str>, style: ButtonStyle, on: impl Fn(&mut S) + 'static) -> El {
    let fg = button_text_color(theme, style, true);
    let mut b = button_frame(theme, key, style, true, on).px(if label.is_some() { 11.0 } else { 8.0 }).child(icon(sym, 16.0, fg));
    if let Some(l) = label {
        b = b.label(l).child(body(l, fg));
    } else {
        b = b.label(sym.name());
    }
    b
}

/// `HyperlinkButton`: accent text on a subtle frame.
pub fn hyperlink_button<S: 'static>(theme: &Theme, key: &str, label: &str, on: impl Fn(&mut S) + 'static) -> El {
    let fg = theme.tokens().accent_text_fill_color_primary;
    button_frame(theme, key, ButtonStyle::Subtle, true, on).px(11.0).role("link").label(label).child(body(label, fg))
}

/// `ToggleButton`: standard at rest, accent when checked.
pub fn toggle_button<S: 'static>(theme: &Theme, key: &str, label: &str, checked: bool, on: impl Fn(&mut S) + 'static) -> El {
    let style = if checked { ButtonStyle::Accent } else { ButtonStyle::Standard };
    button(theme, key, label, style, true, on).checked(checked)
}

/// `DropDownButton`: label + ChevronDown (12px glyph, 8px gap).
pub fn dropdown_button<S: 'static>(theme: &Theme, key: &str, label: &str, on: impl Fn(&mut S) + 'static) -> El {
    let fg = theme.tokens().text_fill_color_primary;
    button_frame(theme, key, ButtonStyle::Standard, true, on)
        .px(11.0)
        .label(label)
        .child(body(label, fg))
        .child(icon(Symbol::ChevronDown, 12.0, theme.tokens().text_fill_color_secondary))
}

/// `SplitButton`: primary part + secondary chevron part, 1px divider.
pub fn split_button<S: 'static>(theme: &Theme, key: &str, content: El, on_main: impl Fn(&mut S) + 'static, on_more: impl Fn(&mut S) + 'static) -> El {
    let t = theme.tokens();
    let main = button_frame(theme, &format!("{key}-main"), ButtonStyle::Standard, true, on_main).px(11.0).corners([radii::CONTROL, 0.0, 0.0, radii::CONTROL]).label("Apply").child(content);
    let more = button_frame(theme, &format!("{key}-more"), ButtonStyle::Standard, true, on_more)
        .w(36.0)
        .corners([0.0, radii::CONTROL, radii::CONTROL, 0.0])
        .label("More options")
        .child(icon(Symbol::ChevronDown, 12.0, t.text_fill_color_secondary));
    row().key(key).child(main).child(el().w(1.0).h(sizes::CONTROL_HEIGHT).fill(c(t.control_stroke_color_secondary))).child(more)
}

/// `CheckBox`: 20×20 box, radius 4; `None` is indeterminate.
pub fn checkbox<S: 'static>(theme: &Theme, key: &str, label: &str, state: Option<bool>, enabled: bool, on: impl Fn(&mut S) + 'static) -> El {
    let t = theme.tokens();
    let filled = state != Some(false);
    let mut glyph_box = el().size(20.0, 20.0).radius(radii::CONTROL).center();
    glyph_box = if filled {
        let fill = if enabled { t.accent_fill_color_default } else { t.accent_fill_color_disabled };
        glyph_box.fill(c(fill)).child(match state {
            Some(true) => icon(Symbol::Checkmark, 14.0, t.text_on_accent_fill_color_primary),
            _ => el().size(8.0, 1.5).radius(0.5).fill(c(t.text_on_accent_fill_color_primary)),
        })
    } else {
        let stroke = if enabled { t.control_strong_stroke_color_default } else { t.control_strong_stroke_color_disabled };
        glyph_box.fill(c(t.control_alt_fill_color_secondary)).border(1.0, c(stroke)).hover_fill(c(t.control_alt_fill_color_tertiary))
    };
    let fg = if enabled { t.text_fill_color_primary } else { t.text_fill_color_disabled };
    let mut e = row().key(key).role("checkbox").label(label).min_h(32.0).gap(8.0).items_center().child(glyph_box).child(body(label, fg));
    e = match state {
        Some(v) => e.checked(v),
        None => e.checked_mixed(),
    };
    if enabled { focus(e.on_press(on), theme, radii::CONTROL) } else { e.disabled(true) }
}

/// `RadioButton`: 20px ring; checked = accent ring with 12px inner dot.
pub fn radio<S: 'static>(theme: &Theme, key: &str, label: &str, checked: bool, on: impl Fn(&mut S) + 'static) -> El {
    let t = theme.tokens();
    let ring = if checked {
        el().size(20.0, 20.0).radius(10.0).center().fill(c(t.accent_fill_color_default)).child(el().size(12.0, 12.0).radius(6.0).fill(c(t.text_on_accent_fill_color_primary)))
    } else {
        el().size(20.0, 20.0).radius(10.0).fill(c(t.control_alt_fill_color_secondary)).border(1.0, c(t.control_strong_stroke_color_default))
    };
    let e = row().key(key).role("radio").label(label).checked(checked).min_h(32.0).gap(8.0).items_center().child(ring).child(body(label, t.text_fill_color_primary)).on_press(on);
    focus(e, theme, 10.0)
}

/// `ToggleSwitch`: 40×20 track, 12px knob (14 on hover in WinUI), On/Off content.
pub fn toggle_switch<S: 'static>(theme: &Theme, key: &str, header: Option<&str>, on_state: bool, on: impl Fn(&mut S) + 'static) -> El {
    let t = theme.tokens();
    let knob_fill = if on_state { t.text_on_accent_fill_color_primary } else { t.text_fill_color_secondary };
    let knob = el().size(12.0, 12.0).radius(6.0).fill(c(knob_fill)).absolute(if on_state { 24.0 } else { 4.0 }, 4.0).motion(rune::tok::motion::SNAPPY);
    let mut track = el().size(40.0, 20.0).radius(10.0).child(knob);
    track = if on_state {
        track.fill(c(t.accent_fill_color_default)).hover_fill(c(t.accent_fill_color_secondary))
    } else {
        track.fill(c(t.control_alt_fill_color_secondary)).border(1.0, c(t.control_strong_stroke_color_default)).hover_fill(c(t.control_alt_fill_color_tertiary))
    };
    let sw = row()
        .key(key)
        .role("switch")
        .label(header.unwrap_or("Toggle"))
        .checked(on_state)
        .min_h(32.0)
        .gap(12.0)
        .items_center()
        .child(track)
        .child(body(if on_state { "On" } else { "Off" }, t.text_fill_color_primary))
        .on_press(on);
    let sw = focus(sw, theme, 10.0);
    match header {
        Some(h) => col().gap(4.0).child(body(h, t.text_fill_color_primary)).child(sw),
        None => sw,
    }
}

/// `Slider`: 4px track, 20px thumb with accent inner (12px); `steps` clickable segments set the value.
pub fn slider<S: 'static>(theme: &Theme, key: &str, width: f32, value: f32, set: fn(&mut S, f32)) -> El {
    let t = theme.tokens();
    let v = value.clamp(0.0, 1.0);
    let thumb_x = (width - 20.0) * v;
    let track = el().absolute(0.0, 14.0).size(width, 4.0).radius(2.0).fill(c(t.control_strong_fill_color_default));
    let fill = el().absolute(0.0, 14.0).size((thumb_x + 10.0).max(2.0), 4.0).radius(2.0).fill(c(t.accent_fill_color_default));
    let thumb = el()
        .absolute(thumb_x, 2.0)
        .size(20.0, 20.0)
        .radius(10.0)
        .center()
        .fill(c(t.control_solid_fill_color_default))
        .border(1.0, c(t.control_stroke_color_default))
        .shadow(1.0, 0.0, c(t.control_stroke_color_secondary))
        .child(el().size(12.0, 12.0).radius(6.0).fill(c(t.accent_fill_color_default)).hover_fill(c(t.accent_fill_color_secondary)));
    const STEPS: usize = 21;
    let seg_w = width / STEPS as f32;
    let mut e = el().key(key).role("slider").label(format!("{:.0}", v * 100.0)).size(width, 32.0).child(track).child(fill).child(thumb);
    for i in 0..STEPS {
        let target = i as f32 / (STEPS - 1) as f32;
        e = e.child(el().key(format!("{key}-step{i}")).absolute(seg_w * i as f32, 0.0).size(seg_w, 32.0).on_press(move |s: &mut S| set(s, target)));
    }
    focus(e, theme, 4.0)
}

/// `RatingControl` (visual): filled stars in accent, empty in strong fill.
pub fn rating<S: 'static>(theme: &Theme, key: &str, value: u8, set: fn(&mut S, u8)) -> El {
    let t = theme.tokens();
    let mut r = row().key(key).role("slider").label(format!("Rating {value} of 5")).gap(4.0);
    for i in 1..=5u8 {
        let (sym, color) = if i <= value { (Symbol::StarFilled, t.accent_fill_color_default) } else { (Symbol::Star, t.control_strong_fill_color_default) };
        r = r.child(el().key(format!("{key}-{i}")).size(32.0, 32.0).center().child(icon(sym, 24.0, color)).on_press(move |s: &mut S| set(s, i)));
    }
    r
}

/// `TextBox`: 32px, control fill, bottom elevation stroke; focused = input-active fill + accent underline.
pub fn text_box<S: 'static>(theme: &Theme, key: &str, edit: &TextEdit, placeholder: &str, width: f32, enabled: bool, bind: fn(&mut S) -> &mut TextEdit) -> El {
    let t = theme.tokens();
    let fg = if enabled { t.text_fill_color_primary } else { t.text_fill_color_disabled };
    let input = text_input(edit, TYPE_RAMP.body.rune())
        .single_line()
        .key(key)
        .label(placeholder)
        .placeholder(placeholder, c(if enabled { t.text_fill_color_secondary } else { t.text_fill_color_disabled }))
        .color(c(fg))
        .w(width)
        .h(sizes::CONTROL_HEIGHT)
        .px(11.0)
        .radius(radii::CONTROL)
        .border(1.0, c(t.control_stroke_color_default))
        .shadow(1.0, 0.0, c(t.control_strong_stroke_color_default));
    if enabled {
        input.fill(c(t.control_fill_color_default)).hover_fill(c(t.control_fill_color_secondary)).on_edit(bind)
    } else {
        input.fill(c(t.control_fill_color_disabled)).disabled(true)
    }
}

/// `PasswordBox`: a masked single-line field.
pub fn password_box<S: 'static>(theme: &Theme, key: &str, edit: &TextEdit, width: f32, bind: fn(&mut S) -> &mut TextEdit) -> El {
    text_box(theme, key, edit, "Password", width, true, bind).secure()
}

/// A labelled field (`Header` above a control).
pub fn header(theme: &Theme, label: &str, control: El) -> El {
    col().gap(8.0).child(body(label, theme.tokens().text_fill_color_primary)).child(control)
}

/// `NumberBox` with inline spin buttons.
pub fn number_box<S: 'static>(theme: &Theme, key: &str, value: i32, set: fn(&mut S, i32)) -> El {
    let t = theme.tokens();
    let spin = |k: String, sym: Symbol, d: i32| {
        el().key(k).size(28.0, 24.0).radius(radii::CONTROL).center().hover_fill(c(t.subtle_fill_color_secondary)).press_fill(c(t.subtle_fill_color_tertiary)).child(icon(sym, 12.0, t.text_fill_color_secondary)).on_press(move |s: &mut S| set(s, value + d))
    };
    row()
        .key(key)
        .role("spinbutton")
        .label(value.to_string())
        .w(160.0)
        .h(sizes::CONTROL_HEIGHT)
        .pl(11.0)
        .pr(4.0)
        .items_center()
        .radius(radii::CONTROL)
        .fill(c(t.control_fill_color_default))
        .border(1.0, c(t.control_stroke_color_default))
        .shadow(1.0, 0.0, c(t.control_strong_stroke_color_default))
        .child(body(value.to_string(), t.text_fill_color_primary).grow(1.0))
        .child(spin(format!("{key}-up"), Symbol::ChevronUp, 1))
        .child(spin(format!("{key}-down"), Symbol::ChevronDown, -1))
}

/// `ComboBox` closed state: value + ChevronDown, 120px min width.
pub fn combo_box<S: 'static>(theme: &Theme, key: &str, value: &str, width: f32, on: impl Fn(&mut S) + 'static) -> El {
    let t = theme.tokens();
    button_frame(theme, key, ButtonStyle::Standard, true, on)
        .role("combobox")
        .label(value)
        .w(width)
        .pl(11.0)
        .pr(11.0)
        .justify_between()
        .child(body(value, t.text_fill_color_primary))
        .child(icon(Symbol::ChevronDown, 12.0, t.text_fill_color_secondary))
}

/// Flyout surface: acrylic, 8px radius, flyout stroke, depth-16 shadow.
pub fn flyout_surface(theme: &Theme) -> El {
    let t = theme.tokens();
    let e = materials::acrylic_default(theme).radius(radii::OVERLAY).clip_rounded(radii::OVERLAY).border(1.0, c(t.surface_stroke_color_flyout));
    materials::elevate(e, theme, 16.0)
}

/// One `MenuFlyoutItem` (icon, text, keyboard accelerator).
pub fn menu_item<S: 'static>(theme: &Theme, key: &str, sym: Option<Symbol>, label: &str, accel: Option<&str>, on: impl Fn(&mut S) + 'static) -> El {
    let t = theme.tokens();
    let mut r = row().key(key).role("menuitem").label(label).h(32.0).mx(4.0).my(2.0).px(12.0).gap(12.0).items_center().radius(radii::CONTROL);
    r = materials::reveal(r, theme).on_press(on);
    r = match sym {
        Some(s) => r.child(icon(s, 16.0, t.text_fill_color_primary)),
        None => r.child(el().size(16.0, 16.0)),
    };
    r = r.child(body(label, t.text_fill_color_primary).grow(1.0));
    if let Some(a) = accel {
        r = r.child(caption(a, t.text_fill_color_secondary));
    }
    r
}

/// `MenuFlyoutSeparator`.
pub fn menu_separator(theme: &Theme) -> El {
    el().h(1.0).w_full().my(2.0).fill(c(theme.tokens().divider_stroke_color_default))
}

/// List / nav row: 36–40px, 4px radius, accent selection pill (3×16).
pub fn list_row<S: 'static>(theme: &Theme, key: &str, height: f32, selected: bool, content: El, on: impl Fn(&mut S) + 'static) -> El {
    let t = theme.tokens();
    let pill = el().size(3.0, 16.0).radius(1.5).fill(c(if selected { t.accent_fill_color_default } else { t.subtle_fill_color_transparent }));
    let mut r = row().key(key).role("option").checked(selected).h(height).mx(4.0).my(2.0).pr(12.0).gap(0.0).items_center().radius(radii::CONTROL).focusable(true).on_press(on);
    if selected {
        r = r.fill(c(t.subtle_fill_color_secondary)).hover_fill(c(t.subtle_fill_color_tertiary));
    } else {
        r = materials::reveal(r, theme);
    }
    r.child(pill).child(content.ml(9.0).grow(1.0))
}

/// `InfoBar` severities.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Severity {
    Informational,
    Success,
    Warning,
    Error,
}

/// `InfoBar`: severity background, 16px icon badge, title + message, close button.
pub fn info_bar<S: 'static>(theme: &Theme, key: &str, sev: Severity, title: &str, message: &str, on_close: impl Fn(&mut S) + 'static) -> El {
    let t = theme.tokens();
    let (bg, badge, glyph) = match sev {
        Severity::Informational => (t.system_fill_color_attention_background, t.system_fill_color_attention, Symbol::InfoFilled),
        Severity::Success => (t.system_fill_color_success_background, t.system_fill_color_success, Symbol::CheckmarkCircleFilled),
        Severity::Warning => (t.system_fill_color_caution_background, t.system_fill_color_caution, Symbol::WarningFilled),
        Severity::Error => (t.system_fill_color_critical_background, t.system_fill_color_critical, Symbol::ErrorCircleFilled),
    };
    let close = el()
        .key(format!("{key}-close"))
        .role("button")
        .label("Close")
        .size(38.0, 38.0)
        .radius(radii::CONTROL)
        .center()
        .hover_fill(c(t.subtle_fill_color_secondary))
        .press_fill(c(t.subtle_fill_color_tertiary))
        .child(icon(Symbol::Dismiss, 16.0, t.text_fill_color_primary))
        .on_press(on_close);
    row()
        .key(key)
        .role("status")
        .label(title)
        .w_full()
        .min_h(48.0)
        .pl(16.0)
        .pr(5.0)
        .gap(12.0)
        .items_center()
        .radius(radii::CONTROL)
        .fill(c(bg))
        .border(1.0, c(t.card_stroke_color_default))
        .child(icon(glyph, 16.0, badge))
        .child(row().gap(12.0).grow(1.0).child(strong(title, t.text_fill_color_primary)).child(body(message, t.text_fill_color_primary)))
        .child(close)
}

/// Determinate (`Some`) or indeterminate (`None`, a fixed segment) `ProgressBar`.
pub fn progress_bar(theme: &Theme, key: &str, width: f32, value: Option<f32>) -> El {
    let t = theme.tokens();
    let track = el().absolute(0.0, 1.0).size(width, 1.0).fill(c(t.control_strong_stroke_color_default));
    let (x, w) = match value {
        Some(v) => (0.0, width * v.clamp(0.0, 1.0)),
        None => (width * 0.3, width * 0.4),
    };
    let bar = el().absolute(x, 0.0).size(w.max(3.0), 3.0).radius(1.5).fill(c(t.accent_fill_color_default));
    let mut e = el().key(key).role("progressbar").size(width, 3.0).child(track).child(bar);
    if let Some(v) = value {
        e = e.label(format!("{:.0}%", v * 100.0));
    }
    e
}

fn arc_points(cx: f32, cy: f32, r: f32, a0: f32, a1: f32) -> Vec<[f32; 2]> {
    let n = (((a1 - a0).abs() / 0.08).ceil() as usize).max(2);
    (0..=n).map(|i| {
        let a = a0 + (a1 - a0) * i as f32 / n as f32;
        [cx + r * a.cos(), cy + r * a.sin()]
    }).collect()
}

/// `ProgressRing`: 32px default, 4px stroke; `None` draws the indeterminate arc.
pub fn progress_ring(theme: &Theme, key: &str, size: f32, value: Option<f32>) -> El {
    let t = theme.tokens();
    let stroke = (size / 8.0).max(2.0);
    let r = (size - stroke) / 2.0;
    let start = -std::f32::consts::FRAC_PI_2;
    let sweep = match value {
        Some(v) => v.clamp(0.0, 1.0) * std::f32::consts::TAU,
        None => std::f32::consts::TAU * 0.3,
    };
    let mut shape = Shape::new().view_box(size, size);
    if value.is_some() {
        shape = shape.stroke(arc_points(size / 2.0, size / 2.0, r, 0.0, std::f32::consts::TAU), stroke, c(t.control_strong_stroke_color_disabled));
    }
    shape = shape.stroke(arc_points(size / 2.0, size / 2.0, r, start, start + sweep.max(0.01)), stroke, c(t.accent_fill_color_default));
    canvas(shape).key(key).role("progressbar").size(size, size)
}

/// `InfoBadge`: dot (None) or numeric pill, accent fill.
pub fn info_badge(theme: &Theme, value: Option<u32>) -> El {
    let t = theme.tokens();
    match value {
        None => el().size(6.0, 6.0).radius(3.0).fill(c(t.accent_fill_color_default)),
        Some(n) => el()
            .h(16.0)
            .min_w(16.0)
            .px(4.0)
            .radius(8.0)
            .center()
            .fill(c(t.accent_fill_color_default))
            .child(text(n.to_string()).font(rune::tok::TextStyle::new(11.0, 400.0)).color(c(t.text_on_accent_fill_color_primary))),
    }
}

/// `PersonPicture`: circle with initials.
pub fn person_picture(theme: &Theme, initials: &str, size: f32) -> El {
    let t = theme.tokens();
    el().size(size, size)
        .radius(size / 2.0)
        .center()
        .fill(c(t.control_alt_fill_color_quarternary))
        .border(1.0, c(t.control_stroke_color_default))
        .child(text(initials).font(rune::tok::TextStyle::new(size * 0.4, 600.0)).color(c(t.text_fill_color_primary)))
}

/// `ToolTip` surface: 4px radius? WinUI uses OverlayCornerRadius (8), padding 9,6,9,8, 12pt text.
pub fn tooltip(theme: &Theme, label: &str) -> El {
    let t = theme.tokens();
    let e = materials::acrylic_default(theme).radius(radii::OVERLAY).border(1.0, c(t.surface_stroke_color_flyout)).px(9.0).pt(6.0).pb(8.0).child(caption(label, t.text_fill_color_primary));
    materials::elevate(e, theme, 16.0)
}

/// `Expander` header (48px min) and optional content.
pub fn expander<S: 'static>(theme: &Theme, key: &str, title: &str, open: bool, content: El, on: impl Fn(&mut S) + 'static) -> El {
    let t = theme.tokens();
    let chevron = el().size(32.0, 32.0).radius(radii::CONTROL).center().hover_fill(c(t.subtle_fill_color_secondary)).child(icon(if open { Symbol::ChevronUp } else { Symbol::ChevronDown }, 12.0, t.text_fill_color_primary));
    let head = row()
        .key(key)
        .role("button")
        .label(title)
        .expanded(open)
        .w_full()
        .min_h(48.0)
        .pl(16.0)
        .pr(8.0)
        .items_center()
        .justify_between()
        .fill(c(t.card_background_fill_color_default))
        .border(1.0, c(t.card_stroke_color_default))
        .corners(if open { [radii::CONTROL, radii::CONTROL, 0.0, 0.0] } else { [radii::CONTROL; 4] })
        .child(body(title, t.text_fill_color_primary))
        .child(chevron)
        .on_press(on);
    let mut e = col().w_full().child(head);
    if open {
        e = e.child(
            col()
                .w_full()
                .padding(16.0)
                .fill(c(t.card_background_fill_color_secondary))
                .border(1.0, c(t.card_stroke_color_default))
                .corners([0.0, 0.0, radii::CONTROL, radii::CONTROL])
                .child(content),
        );
    }
    e
}
