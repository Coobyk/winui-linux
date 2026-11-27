//! Local fallbacks for the winui-controls must-ship set, drawn with WinUI 3
//! theme-resource values (static visuals + Rest/PointerOver/Pressed/Disabled fills).
//! The integrator replaces these with `winui_controls` components.

use crate::fluent::{c, elevation_border, font, icon, shadow};
use rune::{El, col, el, row, text};
use winui_icons::Symbol;
use winui_theme::{Theme, radii, sizes};

fn body(t: &Theme, s: &str, color: winui_theme::Color) -> El {
    text(s).font(font(t.type_ramp().body)).color(c(color))
}

/// Standard / accent `Button` (Button_themeresources.xaml).
pub fn button(t: &Theme, label: &str, accent: bool, enabled: bool) -> El {
    winui_controls::Button::new(label)
        .accent(accent)
        .enabled(enabled)
        .view(t)
}

pub fn icon_button(t: &Theme, sym: Symbol, label: &str) -> El {
    winui_controls::Button::new(label).icon(sym).view(t)
}

pub fn hyperlink(t: &Theme, label: &str) -> El {
    winui_controls::HyperlinkButton::new(label).view(t)
}

pub fn toggle_button(t: &Theme, label: &str, on: bool) -> El {
    winui_controls::ToggleButton::new(label).checked(on).view(t)
}

fn chevron_box(_t: &Theme, fg: winui_theme::Color) -> El {
    el().size(12.0, 12.0)
        .child(icon(Symbol::ChevronDown, 12.0, fg))
}

pub fn dropdown_button(t: &Theme, label: &str) -> El {
    winui_controls::DropDownButton::new(label).view(t)
}

pub fn split_button(t: &Theme, label: &str) -> El {
    winui_controls::SplitButton::new(label).view(t)
}

pub fn checkbox(t: &Theme, label: &str, state: Option<bool>, enabled: bool) -> El {
    use winui_controls::CheckState;
    let st = match state {
        Some(true) => CheckState::Checked,
        Some(false) => CheckState::Unchecked,
        None => CheckState::Indeterminate,
    };
    winui_controls::CheckBox::new(label)
        .three_state(state.is_none())
        .state(st)
        .enabled(enabled)
        .view(t)
}

pub fn radio(t: &Theme, label: &str, on: bool) -> El {
    winui_controls::RadioButton::new(label).checked(on).view(t)
}

/// `ToggleSwitch`, 40×20 track with 12px knob, On/Off content.
pub fn toggle_switch(t: &Theme, on: bool, header: Option<&str>) -> El {
    let mut s = winui_controls::ToggleSwitch::new().on(on);
    if let Some(h) = header {
        s = s.header(h);
    }
    s.view(t)
}

pub fn slider(t: &Theme, value: f32, width: f32) -> El {
    winui_controls::Slider::new()
        .value(f64::from(value) * 100.0)
        .width(width)
        .view(t)
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
    let b = winui_controls::ProgressBar::new().width(width);
    match value {
        Some(v) => b.value(v * 100.0),
        None => b,
    }
    .view(t)
}

/// ProgressRing: 4px accent arc over a 32px ring (`value` None = indeterminate pose).
pub fn progress_ring(t: &Theme, value: Option<f32>, size: f32) -> El {
    let r = winui_controls::ProgressRing::new().size(size);
    match value {
        Some(v) => r.value(v * 100.0),
        None => r,
    }
    .view(t)
}

pub fn rating(t: &Theme, value: usize) -> El {
    winui_controls::RatingControl::new()
        .value(value as f32)
        .view(t)
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
    winui_controls::PersonPicture::new()
        .display_name(name)
        .size(size)
        .view(t)
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
