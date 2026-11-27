//! Local fallbacks for the winui-controls must-ship set, drawn with WinUI 3
//! theme-resource values (static visuals + Rest/PointerOver/Pressed/Disabled fills).
//! The integrator replaces these with `winui_controls` components.

use crate::fluent::{c, elevation_border, font, icon, shadow};
use rune::{El, Shape, col, el, row, text};
use winui_icons::Symbol;
use winui_theme::{Theme, radii, sizes};

fn body(t: &Theme, s: &str, color: winui_theme::Color) -> El {
    text(s).font(font(t.type_ramp().body)).color(c(color))
}

/// Standard / accent `Button` (Button_themeresources.xaml).
pub fn button(t: &Theme, label: &str, accent: bool, enabled: bool) -> El {
    let k = t.tokens();
    let (fill, hover, press, fg) = match (accent, enabled) {
        (false, true) => (
            k.control_fill_color_default,
            k.control_fill_color_secondary,
            k.control_fill_color_tertiary,
            k.text_fill_color_primary,
        ),
        (true, true) => (
            k.accent_fill_color_default,
            k.accent_fill_color_secondary,
            k.accent_fill_color_tertiary,
            k.text_on_accent_fill_color_primary,
        ),
        (false, false) => (
            k.control_fill_color_disabled,
            k.control_fill_color_disabled,
            k.control_fill_color_disabled,
            k.text_fill_color_disabled,
        ),
        (true, false) => (
            k.accent_fill_color_disabled,
            k.accent_fill_color_disabled,
            k.accent_fill_color_disabled,
            k.text_on_accent_fill_color_disabled,
        ),
    };
    let p = sizes::BUTTON_PADDING;
    let b = row()
        .h(sizes::CONTROL_HEIGHT)
        .items_center()
        .pl(p.left)
        .pr(p.right)
        .radius(radii::CONTROL)
        .fill(c(fill))
        .hover(c(hover))
        .press(c(press))
        .focusable(enabled)
        .role("button")
        .label(label.to_string())
        .child(body(t, label, fg));
    let b = if !enabled {
        if accent {
            b
        } else {
            b.border(1.0, c(k.control_stroke_color_default))
        }
    } else if accent {
        elevation_border(b, &k.accent_control_elevation_border, sizes::CONTROL_HEIGHT)
    } else {
        elevation_border(b, &k.control_elevation_border, sizes::CONTROL_HEIGHT)
    };
    b.w_fit()
}

pub fn icon_button(t: &Theme, sym: Symbol, label: &str) -> El {
    let k = t.tokens();
    elevation_border(
        row()
            .h(32.0)
            .items_center()
            .gap(8.0)
            .px(11.0)
            .radius(radii::CONTROL)
            .fill(c(k.control_fill_color_default))
            .hover(c(k.control_fill_color_secondary))
            .press(c(k.control_fill_color_tertiary))
            .child(icon(sym, 16.0, k.text_fill_color_primary))
            .child(body(t, label, k.text_fill_color_primary)),
        &k.control_elevation_border,
        32.0,
    )
    .w_fit()
}

pub fn hyperlink(t: &Theme, label: &str) -> El {
    let k = t.tokens();
    row()
        .h(32.0)
        .items_center()
        .px(11.0)
        .radius(radii::CONTROL)
        .hover(c(k.subtle_fill_color_secondary))
        .press(c(k.subtle_fill_color_tertiary))
        .child(body(t, label, k.accent_text_fill_color_primary))
        .w_fit()
}

pub fn toggle_button(t: &Theme, label: &str, on: bool) -> El {
    button(t, label, on, true)
}

fn chevron_box(_t: &Theme, fg: winui_theme::Color) -> El {
    el().size(12.0, 12.0)
        .child(icon(Symbol::ChevronDown, 12.0, fg))
}

pub fn dropdown_button(t: &Theme, label: &str) -> El {
    let k = t.tokens();
    elevation_border(
        row()
            .h(32.0)
            .items_center()
            .gap(8.0)
            .pl(11.0)
            .pr(9.0)
            .radius(radii::CONTROL)
            .fill(c(k.control_fill_color_default))
            .hover(c(k.control_fill_color_secondary))
            .child(body(t, label, k.text_fill_color_primary))
            .child(chevron_box(t, k.text_fill_color_secondary)),
        &k.control_elevation_border,
        32.0,
    )
    .w_fit()
}

pub fn split_button(t: &Theme, label: &str) -> El {
    let k = t.tokens();
    elevation_border(
        row()
            .h(32.0)
            .items_center()
            .radius(radii::CONTROL)
            .fill(c(k.control_fill_color_default))
            .child(
                row()
                    .h_full()
                    .items_center()
                    .px(11.0)
                    .hover(c(k.control_fill_color_secondary))
                    .child(body(t, label, k.text_fill_color_primary)),
            )
            .child(el().w(1.0).h_full().fill(c(k.control_stroke_color_default)))
            .child(
                row()
                    .h_full()
                    .w(36.0)
                    .center()
                    .hover(c(k.control_fill_color_secondary))
                    .child(chevron_box(t, k.text_fill_color_secondary)),
            ),
        &k.control_elevation_border,
        32.0,
    )
    .w_fit()
}

pub fn checkbox(t: &Theme, label: &str, state: Option<bool>, enabled: bool) -> El {
    let k = t.tokens();
    let checked = state != Some(false);
    let mut bx = el()
        .size(sizes::CHECKBOX_SIZE, sizes::CHECKBOX_SIZE)
        .radius(radii::CONTROL)
        .center();
    bx = if checked {
        bx.fill(c(if enabled {
            k.accent_fill_color_default
        } else {
            k.accent_fill_color_disabled
        }))
    } else {
        bx.fill(c(k.control_alt_fill_color_secondary))
            .hover(c(k.control_alt_fill_color_tertiary))
            .border(
                1.0,
                c(if enabled {
                    k.control_strong_stroke_color_default
                } else {
                    k.control_strong_stroke_color_disabled
                }),
            )
    };
    let on = if enabled {
        k.text_on_accent_fill_color_primary
    } else {
        k.text_on_accent_fill_color_disabled
    };
    bx = match state {
        Some(true) => bx.child(icon(Symbol::CheckMark, sizes::CHECKBOX_GLYPH_SIZE, on)),
        None => bx.child(el().size(8.0, 1.0).radius(0.5).fill(c(on))),
        Some(false) => bx,
    };
    let fg = if enabled {
        k.text_fill_color_primary
    } else {
        k.text_fill_color_disabled
    };
    row()
        .items_center()
        .min_h(32.0)
        .gap(8.0)
        .role("checkbox")
        .child(bx)
        .child(body(t, label, fg))
        .w_fit()
}

pub fn radio(t: &Theme, label: &str, on: bool) -> El {
    let k = t.tokens();
    let dot = if on {
        el().size(20.0, 20.0)
            .radius(10.0)
            .fill(c(k.accent_fill_color_default))
            .center()
            .child(
                el().size(12.0, 12.0)
                    .radius(6.0)
                    .fill(c(k.text_on_accent_fill_color_primary)),
            )
    } else {
        el().size(20.0, 20.0)
            .radius(10.0)
            .fill(c(k.control_alt_fill_color_secondary))
            .border(1.0, c(k.control_strong_stroke_color_default))
    };
    row()
        .items_center()
        .min_h(32.0)
        .gap(8.0)
        .role("radio")
        .child(dot)
        .child(body(t, label, k.text_fill_color_primary))
        .w_fit()
}

/// `ToggleSwitch`, 40×20 track with 12px knob, On/Off content.
pub fn toggle_switch(t: &Theme, on: bool, header: Option<&str>) -> El {
    let k = t.tokens();
    let track = if on {
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
            .hover(c(k.control_alt_fill_color_tertiary))
            .border(1.0, c(k.control_strong_stroke_color_default))
            .child(
                el().size(12.0, 12.0)
                    .radius(6.0)
                    .fill(c(k.text_fill_color_secondary))
                    .absolute(4.0, 4.0),
            )
    };
    let r = row()
        .items_center()
        .gap(12.0)
        .h(32.0)
        .role("switch")
        .checked(on)
        .child(track)
        .child(body(
            t,
            if on { "On" } else { "Off" },
            k.text_fill_color_primary,
        ));
    match header {
        Some(h) => col()
            .gap(4.0)
            .child(body(t, h, k.text_fill_color_primary))
            .child(r),
        None => r,
    }
}

pub fn slider(t: &Theme, value: f32, width: f32) -> El {
    let k = t.tokens();
    let x = (width - 20.0) * value.clamp(0.0, 1.0);
    let thumb = el()
        .size(20.0, 20.0)
        .radius(10.0)
        .fill(c(k.control_solid_fill_color_default))
        .center()
        .border_gradient(
            1.0,
            c(k.circle_elevation_border.top(20.0)),
            c(k.circle_elevation_border.bottom(20.0)),
        )
        .child(
            el().size(12.0, 12.0)
                .radius(6.0)
                .fill(c(k.accent_fill_color_default)),
        );
    el().w(width)
        .h(32.0)
        .child(
            el().w(width)
                .h(4.0)
                .radius(2.0)
                .fill(c(k.control_strong_fill_color_default))
                .absolute(0.0, 14.0),
        )
        .child(
            el().w(x + 10.0)
                .h(4.0)
                .radius(2.0)
                .fill(c(k.accent_fill_color_default))
                .absolute(0.0, 14.0),
        )
        .child(thumb.absolute(x, 6.0))
}

/// `TextBox`: placeholder in TextFillColorSecondary, bottom stroke ControlStrongStrokeColorDefault.
pub fn text_box(
    t: &Theme,
    value: &str,
    placeholder: &str,
    width: f32,
    enabled: bool,
    focused: bool,
) -> El {
    let k = t.tokens();
    let fill = if !enabled {
        k.control_fill_color_disabled
    } else if focused {
        k.control_fill_color_input_active
    } else {
        k.control_fill_color_default
    };
    let (txt, fg) = if value.is_empty() {
        (
            placeholder,
            if enabled {
                k.text_fill_color_secondary
            } else {
                k.text_fill_color_disabled
            },
        )
    } else {
        (
            value,
            if enabled {
                k.text_fill_color_primary
            } else {
                k.text_fill_color_disabled
            },
        )
    };
    let bottom = if focused {
        (2.0, k.accent_fill_color_default)
    } else {
        (1.0, k.control_strong_stroke_color_default)
    };
    let mut e = row()
        .w(width)
        .h(32.0)
        .items_center()
        .pl(10.0)
        .pr(6.0)
        .radius(radii::CONTROL)
        .fill(c(fill))
        .border(1.0, c(k.control_stroke_color_default))
        .hover(c(if enabled && !focused {
            k.control_fill_color_secondary
        } else {
            fill
        }))
        .clip_rounded(radii::CONTROL)
        .child(body(t, txt, fg).grow(1.0).truncate());
    if enabled {
        e = e.child(
            el().pin(Some(0.0), None, Some(0.0), Some(0.0))
                .h(bottom.0)
                .fill(c(bottom.1)),
        );
    }
    e
}

pub fn password_box(t: &Theme, width: f32) -> El {
    let k = t.tokens();
    text_box(t, "●●●●●●●●", "", width, true, false)
        .child(icon(Symbol::Eye, 16.0, k.text_fill_color_secondary).mr(6.0))
}

pub fn number_box(t: &Theme, value: &str, width: f32) -> El {
    let k = t.tokens();
    let spin = |s| {
        el().size(28.0, 24.0)
            .radius(radii::CONTROL)
            .center()
            .hover(c(k.subtle_fill_color_secondary))
            .child(icon(s, 12.0, k.text_fill_color_secondary))
    };
    text_box(t, value, "", width, true, false).child(
        row()
            .child(spin(Symbol::ChevronUp))
            .child(spin(Symbol::ChevronDown)),
    )
}

pub fn auto_suggest(t: &Theme, placeholder: &str, width: f32) -> El {
    let k = t.tokens();
    text_box(t, "", placeholder, width, true, false)
        .child(icon(Symbol::Find, 12.0, k.text_fill_color_secondary).mr(6.0))
}

pub fn combo_box(t: &Theme, value: &str, width: f32) -> El {
    let k = t.tokens();
    elevation_border(
        row()
            .w(width)
            .h(32.0)
            .items_center()
            .pl(12.0)
            .pr(11.0)
            .radius(radii::CONTROL)
            .fill(c(k.control_fill_color_default))
            .hover(c(k.control_fill_color_secondary))
            .press(c(k.control_fill_color_tertiary))
            .role("combobox")
            .child(body(t, value, k.text_fill_color_primary).grow(1.0))
            .child(chevron_box(t, k.text_fill_color_secondary)),
        &k.control_elevation_border,
        32.0,
    )
}

/// An open MenuFlyout surface (acrylic, overlay radius 8, depth-16 shadow).
pub fn menu_flyout(t: &Theme, items: &[(Option<Symbol>, &str, &str)]) -> El {
    let k = t.tokens();
    let mut m = col()
        .w(220.0)
        .py(2.0)
        .radius(radii::OVERLAY)
        .border(1.0, c(k.surface_stroke_color_flyout));
    m = shadow(
        m.child(
            crate::fluent::acrylic_default(t)
                .pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                .radius(radii::OVERLAY),
        ),
        t,
        16.0,
    );
    for (i, (sym, label, accel)) in items.iter().enumerate() {
        if label.is_empty() {
            m = m.child(
                el().h(1.0)
                    .mx(0.0)
                    .my(2.0)
                    .fill(c(k.divider_stroke_color_default)),
            );
            continue;
        }
        let mut r = row()
            .h(32.0)
            .mx(4.0)
            .my(2.0)
            .px(11.0)
            .gap(12.0)
            .items_center()
            .radius(radii::CONTROL)
            .hover(c(k.subtle_fill_color_secondary))
            .key(format!("menu-{i}"));
        if i == 0 {
            r = r.fill(c(k.subtle_fill_color_secondary));
        }
        r = r
            .child(match sym {
                Some(s) => icon(*s, 16.0, k.text_fill_color_primary),
                None => el().size(16.0, 16.0),
            })
            .child(body(t, label, k.text_fill_color_primary).grow(1.0))
            .child(
                text(*accel)
                    .font(font(t.type_ramp().caption))
                    .color(c(k.text_fill_color_secondary)),
            );
        m = m.child(r);
    }
    m
}

pub fn command_bar(t: &Theme, items: &[(Symbol, &str)]) -> El {
    let k = t.tokens();
    let mut r = row()
        .h(48.0)
        .items_center()
        .px(4.0)
        .gap(2.0)
        .radius(radii::OVERLAY)
        .fill(c(k.layer_fill_color_alt))
        .border(1.0, c(k.card_stroke_color_default));
    for (s, l) in items {
        r = r.child(
            row()
                .h(40.0)
                .px(12.0)
                .gap(8.0)
                .items_center()
                .radius(radii::CONTROL)
                .hover(c(k.subtle_fill_color_secondary))
                .child(icon(*s, 16.0, k.text_fill_color_primary))
                .child(body(t, l, k.text_fill_color_primary)),
        );
    }
    r.child(
        el().w(1.0)
            .h(24.0)
            .mx(4.0)
            .fill(c(k.divider_stroke_color_default)),
    )
    .child(
        el().size(40.0, 40.0)
            .center()
            .radius(radii::CONTROL)
            .child(icon(
                Symbol::MoreHorizontal,
                16.0,
                k.text_fill_color_primary,
            )),
    )
    .w_fit()
}

#[derive(Clone, Copy)]
pub enum Severity {
    Info,
    Success,
    Warning,
    Error,
}

pub fn info_bar(t: &Theme, sev: Severity, title: &str, message: &str) -> El {
    let k = t.tokens();
    let (bg, fg, sym) = match sev {
        Severity::Info => (
            k.system_fill_color_attention_background,
            k.system_fill_color_attention,
            Symbol::Info,
        ),
        Severity::Success => (
            k.system_fill_color_success_background,
            k.system_fill_color_success,
            Symbol::CheckMark,
        ),
        Severity::Warning => (
            k.system_fill_color_caution_background,
            k.system_fill_color_caution,
            Symbol::Important,
        ),
        Severity::Error => (
            k.system_fill_color_critical_background,
            k.system_fill_color_critical,
            Symbol::Dismiss,
        ),
    };
    let badge = el()
        .size(16.0, 16.0)
        .radius(8.0)
        .fill(c(fg))
        .center()
        .child(icon(sym, 10.0, k.text_fill_color_inverse));
    row()
        .w_full()
        .min_h(sizes::INFO_BAR_MIN_HEIGHT)
        .items_center()
        .pl(15.0)
        .radius(radii::CONTROL)
        .fill(c(bg))
        .border(1.0, c(k.card_stroke_color_default))
        .child(badge)
        .child(
            row()
                .grow(1.0)
                .gap(12.0)
                .pl(14.0)
                .items_center()
                .child(
                    text(title)
                        .font(font(t.type_ramp().body_strong))
                        .color(c(k.text_fill_color_primary)),
                )
                .child(body(t, message, k.text_fill_color_primary)),
        )
        .child(
            el().size(38.0, 38.0)
                .mr(5.0)
                .center()
                .radius(radii::CONTROL)
                .hover(c(k.subtle_fill_color_secondary))
                .child(icon(Symbol::Dismiss, 16.0, k.text_fill_color_primary)),
        )
}

/// Determinate ProgressBar: 1px track (ControlStrongStrokeColorDefault) and 3px accent indicator.
pub fn progress_bar(t: &Theme, value: Option<f32>, width: f32) -> El {
    let k = t.tokens();
    let (x, w) = match value {
        Some(v) => (0.0, width * v),
        None => (width * 0.3, width * 0.4),
    };
    el().w(width)
        .h(4.0)
        .child(
            el().w(width)
                .h(1.0)
                .fill(c(k.control_strong_stroke_color_default))
                .absolute(0.0, 1.5),
        )
        .child(
            el().w(w)
                .h(3.0)
                .radius(1.5)
                .fill(c(k.accent_fill_color_default))
                .absolute(x, 0.5),
        )
}

/// ProgressRing: 4px accent arc over a 32px ring (`value` None = indeterminate pose).
pub fn progress_ring(t: &Theme, value: Option<f32>, size: f32) -> El {
    let k = t.tokens();
    let (a0, sweep) = match value {
        Some(v) => (-90f32, 360.0 * v.min(0.999)),
        None => (-30.0, 110.0),
    };
    let r = (size - 4.0) / 2.0;
    let pt = |deg: f32| {
        let a = deg.to_radians();
        (size / 2.0 + r * a.cos(), size / 2.0 + r * a.sin())
    };
    let (x0, y0) = pt(a0);
    let (x1, y1) = pt(a0 + sweep);
    let large = if sweep > 180.0 { 1 } else { 0 };
    let d = format!("M{x0} {y0} A{r} {r} 0 {large} 1 {x1} {y1}");
    let mut shape = Shape::new();
    if value.is_some() {
        shape = shape.stroke_svg(
            &format!(
                "M{} {} A{r} {r} 0 1 1 {} {} A{r} {r} 0 1 1 {} {}",
                size / 2.0,
                2.0,
                size / 2.0,
                size - 2.0,
                size / 2.0,
                2.0
            ),
            4.0,
            rune::LineCap::Butt,
            c(k.control_strong_stroke_color_default.with_opacity(0.0)),
        );
    }
    canvas_el(
        shape.stroke_svg(
            &d,
            4.0,
            rune::LineCap::Round,
            c(k.accent_fill_color_default),
        ),
        size,
    )
}

fn canvas_el(shape: Shape, size: f32) -> El {
    rune::canvas(shape).size(size, size)
}

pub fn rating(t: &Theme, value: usize) -> El {
    let k = t.tokens();
    let mut r = row().gap(4.0).h(32.0).items_center();
    for i in 0..5 {
        r = r.child(if i < value {
            icon(Symbol::StarFilled, 20.0, k.accent_fill_color_default)
        } else {
            icon(Symbol::Star, 20.0, k.text_fill_color_secondary)
        });
    }
    r
}

pub fn expander(t: &Theme, title: &str, open: bool, content: El) -> El {
    let k = t.tokens();
    let header = row()
        .w_full()
        .min_h(sizes::EXPANDER_MIN_HEIGHT)
        .items_center()
        .pl(16.0)
        .pr(8.0)
        .fill(c(k.card_background_fill_color_default))
        .border(1.0, c(k.card_stroke_color_default))
        .radius(radii::CONTROL)
        .child(body(t, title, k.text_fill_color_primary).grow(1.0))
        .child(
            el().size(32.0, 32.0)
                .center()
                .radius(radii::CONTROL)
                .hover(c(k.subtle_fill_color_secondary))
                .child(icon(
                    if open {
                        Symbol::ChevronUp
                    } else {
                        Symbol::ChevronDown
                    },
                    12.0,
                    k.text_fill_color_primary,
                )),
        );
    let mut e = col().w(360.0).child(header);
    if open {
        e = e.child(
            col()
                .w_full()
                .padding(sizes::EXPANDER_CONTENT_PADDING)
                .fill(c(k.card_background_fill_color_secondary))
                .border(1.0, c(k.card_stroke_color_default))
                .radius(radii::CONTROL)
                .child(content),
        );
    }
    e
}

pub fn list_row(t: &Theme, label: &str, sub: Option<&str>, selected: bool) -> El {
    let k = t.tokens();
    let mut r = row()
        .w_full()
        .h(sizes::LIST_VIEW_ITEM_MIN_HEIGHT.max(if sub.is_some() { 56.0 } else { 40.0 }))
        .items_center()
        .mx(4.0)
        .my(2.0)
        .px(12.0)
        .gap(12.0)
        .radius(radii::CONTROL)
        .hover(c(k.subtle_fill_color_secondary))
        .press(c(k.subtle_fill_color_tertiary));
    if selected {
        r = r.fill(c(k.subtle_fill_color_secondary)).child(
            el().size(3.0, 16.0)
                .radius(1.5)
                .fill(c(k.accent_fill_color_default))
                .absolute(0.0, if sub.is_some() { 20.0 } else { 12.0 }),
        );
    }
    r = r.child(person(t, label, 32.0));
    let mut tx = col().child(body(t, label, k.text_fill_color_primary));
    if let Some(s) = sub {
        tx = tx.child(
            text(s)
                .font(font(t.type_ramp().caption))
                .color(c(k.text_fill_color_secondary)),
        );
    }
    r.child(tx)
}

pub fn person(t: &Theme, name: &str, size: f32) -> El {
    let k = t.tokens();
    let initials: String = name
        .split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect();
    el().size(size, size)
        .radius(size / 2.0)
        .fill(c(k.control_alt_fill_color_quarternary))
        .center()
        .border(1.0, c(k.card_stroke_color_default))
        .child(
            text(initials)
                .font(font(if size >= 64.0 {
                    t.type_ramp().subtitle
                } else {
                    t.type_ramp().caption.with_weight(600)
                }))
                .color(c(k.text_fill_color_primary)),
        )
}

pub fn info_badge(t: &Theme, n: Option<u32>) -> El {
    let k = t.tokens();
    match n {
        Some(n) => row()
            .h(16.0)
            .min_w(16.0)
            .px(4.0)
            .radius(8.0)
            .center()
            .fill(c(k.system_fill_color_attention))
            .child(
                text(n.to_string())
                    .font(rune::TextStyle::new(11.0, 400.0))
                    .color(c(k.text_on_accent_fill_color_primary)),
            ),
        None => el()
            .size(6.0, 6.0)
            .radius(3.0)
            .fill(c(k.system_fill_color_attention)),
    }
}

pub fn tooltip(t: &Theme, s: &str) -> El {
    let k = t.tokens();
    let p = sizes::TOOLTIP_PADDING;
    shadow(
        row()
            .pl(p.left)
            .pt(p.top)
            .pr(p.right)
            .pb(p.bottom)
            .radius(radii::CONTROL)
            .fill(c(k.solid_background_fill_color_quarternary))
            .border(1.0, c(k.surface_stroke_color_flyout))
            .child(
                text(s)
                    .font(font(t.type_ramp().caption))
                    .color(c(k.text_fill_color_primary)),
            ),
        t,
        8.0,
    )
    .w_fit()
}

pub fn tab_view(t: &Theme, tabs: &[&str], selected: usize) -> El {
    let k = t.tokens();
    let mut strip = row().w_full().h(40.0).items_end().pl(8.0).gap(0.0);
    for (i, name) in tabs.iter().enumerate() {
        let sel = i == selected;
        let mut tab = row()
            .h(32.0)
            .w(180.0)
            .items_center()
            .pl(8.0)
            .pr(4.0)
            .gap(8.0)
            .corners([8.0, 8.0, 0.0, 0.0])
            .child(icon(Symbol::Document, 16.0, k.text_fill_color_primary))
            .child(
                text(*name)
                    .font(font(t.type_ramp().caption))
                    .color(c(if sel {
                        k.text_fill_color_primary
                    } else {
                        k.text_fill_color_secondary
                    }))
                    .grow(1.0)
                    .truncate(),
            )
            .child(
                el().size(24.0, 24.0)
                    .center()
                    .radius(radii::CONTROL)
                    .hover(c(k.subtle_fill_color_secondary))
                    .child(icon(Symbol::Dismiss, 12.0, k.text_fill_color_secondary)),
            );
        tab = if sel {
            tab.fill(c(k.solid_background_fill_color_tertiary))
                .border_sides([1.0, 1.0, 1.0, 0.0], c(k.card_stroke_color_default))
        } else {
            tab.hover(c(k.layer_on_mica_base_alt_fill_color_secondary))
        };
        strip = strip.child(tab);
    }
    strip = strip.child(
        el().size(32.0, 24.0)
            .mb(4.0)
            .ml(4.0)
            .center()
            .radius(radii::CONTROL)
            .hover(c(k.subtle_fill_color_secondary))
            .child(icon(Symbol::Add, 12.0, k.text_fill_color_primary)),
    );
    col()
        .w_full()
        .radius(radii::OVERLAY)
        .fill(c(k.layer_on_mica_base_alt_fill_color_transparent))
        .clip_rounded(radii::OVERLAY)
        .child(strip)
        .child(
            el().w_full()
                .h(120.0)
                .fill(c(k.solid_background_fill_color_tertiary))
                .padding(16.0)
                .child(body(
                    t,
                    &format!("Content of {}", tabs[selected]),
                    k.text_fill_color_primary,
                )),
        )
}

pub fn breadcrumb(t: &Theme, parts: &[&str]) -> El {
    let k = t.tokens();
    let mut r = row().items_center().gap(4.0);
    for (i, p) in parts.iter().enumerate() {
        let last = i + 1 == parts.len();
        r = r.child(text(*p).font(font(t.type_ramp().body)).color(c(if last {
            k.text_fill_color_primary
        } else {
            k.text_fill_color_secondary
        })));
        if !last {
            r = r.child(icon(
                Symbol::ChevronRight,
                12.0,
                k.text_fill_color_secondary,
            ));
        }
    }
    r
}

pub fn teaching_tip(t: &Theme, title: &str, msg: &str) -> El {
    let k = t.tokens();
    shadow(
        col()
            .w(320.0)
            .padding(12.0)
            .gap(4.0)
            .radius(radii::OVERLAY)
            .fill(c(k.solid_background_fill_color_quarternary))
            .border(1.0, c(k.surface_stroke_color_flyout))
            .child(
                row()
                    .child(
                        text(title)
                            .font(font(t.type_ramp().body_strong))
                            .color(c(k.text_fill_color_primary))
                            .grow(1.0),
                    )
                    .child(icon(Symbol::Dismiss, 12.0, k.text_fill_color_secondary)),
            )
            .child(body(t, msg, k.text_fill_color_secondary).wrap())
            .child(row().mt(12.0).child(button(t, "Got it", true, true))),
        t,
        32.0,
    )
}

pub fn swatch_grid(t: &Theme, colors: &[winui_theme::Color], selected: usize) -> El {
    let k = t.tokens();
    let mut g = row().flex_wrap().gap(4.0).w(8.0 * 44.0);
    for (i, col_) in colors.iter().enumerate() {
        let mut s = el()
            .size(40.0, 40.0)
            .radius(radii::CONTROL)
            .fill(c(*col_))
            .center();
        if i == selected {
            s = s.border(2.0, c(k.text_fill_color_primary)).child(icon(
                Symbol::CheckMark,
                16.0,
                winui_theme::Color::WHITE,
            ));
        }
        g = g.child(s);
    }
    g
}
