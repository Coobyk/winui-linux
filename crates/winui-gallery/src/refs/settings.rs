//! `ref-settings`: Windows Settings > System (docs/reference/settings-system.png).

use rune::{El, col, el, row};
use winui_controls::selection_pill;
use winui_controls::style::rc;
use winui_icons::Symbol;
use winui_theme::{Color, Theme, radii};

use super::{icon, label, ms_logo, title_bar};

const NAV: [(Symbol, &str, u32); 12] = [
    (Symbol::Home, "Home", 0xFFE8A33D),
    (Symbol::Desktop, "System", 0xFF4CC2FF),
    (Symbol::Bluetooth, "Bluetooth & devices", 0xFF2F8FEA),
    (Symbol::WiFi, "Network & internet", 0xFF3AA0F0),
    (Symbol::PaintBrush, "Personalization", 0xFFE89A5C),
    (Symbol::Apps, "Apps", 0xFF8AA4C8),
    (Symbol::Person, "Accounts", 0xFF2BC88C),
    (Symbol::Globe, "Time & language", 0xFF3AA0F0),
    (Symbol::Games, "Gaming", 0xFFB4B4B4),
    (Symbol::Accessibility, "Accessibility", 0xFF3AA0F0),
    (Symbol::Shield, "Privacy & security", 0xFF9AA0A8),
    (Symbol::ArrowSync, "Windows Update", 0xFF2F8FEA),
];

const CARDS: [(Symbol, &str, &str); 10] = [
    (
        Symbol::Desktop,
        "Display",
        "Monitors, brightness, night light, display profile",
    ),
    (
        Symbol::Volume,
        "Sound",
        "Volume levels, output, input, sound devices",
    ),
    (
        Symbol::Alert,
        "Notifications",
        "Alerts from apps and system, do not disturb",
    ),
    (Symbol::Timer, "Focus", "Reduce distractions"),
    (
        Symbol::Power,
        "Power",
        "Screen and sleep, power mode, energy saver",
    ),
    (
        Symbol::Database,
        "Storage",
        "Storage space, drives, configuration rules",
    ),
    (
        Symbol::Share,
        "Share",
        "Share files, photos, links and more",
    ),
    (
        Symbol::SquareMultiple,
        "Multitasking",
        "Snap windows, desktops, task switching",
    ),
    (
        Symbol::Wrench,
        "Advanced",
        "Performance, optimization, and developer features",
    ),
    (
        Symbol::CheckmarkCircle,
        "Activation",
        "Activation state, subscriptions, product key",
    ),
];

pub fn view(t: &Theme) -> El {
    let k = t.tokens();
    let ramp = t.type_ramp();
    let search = row()
        .absolute(343.0, 8.0)
        .size(515.0, 32.0)
        .radius(radii::CONTROL)
        .fill(rc(k.control_fill_color_default))
        .border(1.0, rc(k.control_stroke_color_default))
        .pl(16.0)
        .gap(12.0)
        .center_y()
        .child(icon(Symbol::Find, 14.0, k.text_fill_color_secondary))
        .child(label(
            "Find a setting",
            ramp.body,
            k.text_fill_color_secondary,
        ));
    let bar = el()
        .child(
            el().absolute(4.0, 6.0)
                .size(40.0, 36.0)
                .radius(radii::CONTROL)
                .center()
                .hover_fill(rc(k.subtle_fill_color_secondary))
                .child(icon(Symbol::Back, 16.0, k.text_fill_color_primary)),
        )
        .child(label("Settings", ramp.caption, k.text_fill_color_primary).absolute(51.0, 16.0))
        .child(search);

    let user = el()
        .absolute(17.0, 8.0)
        .size(60.0, 60.0)
        .radius(30.0)
        .fill(rc(Color::from_argb(0xFFFAFAFA)))
        .center()
        .child(icon(Symbol::Person, 34.0, Color::from_argb(0xFF8F8F8F)));
    let mut pane = el()
        .w(312.0)
        .h_full()
        .child(user)
        .child(label("Coobyk", ramp.body_strong, k.text_fill_color_primary).absolute(89.0, 22.0))
        .child(
            label("Local Account", ramp.caption, k.text_fill_color_primary).absolute(89.0, 42.0),
        );
    for (i, (sym, name, tint)) in NAV.into_iter().enumerate() {
        let sel = i == 1;
        let mut item = row()
            .absolute(16.0, 96.0 + 40.0 * i as f32)
            .size(280.0, 36.0)
            .radius(radii::CONTROL)
            .pl(16.0)
            .gap(15.0)
            .center_y()
            .fill(rc(if sel {
                k.subtle_fill_color_secondary
            } else {
                k.subtle_fill_color_transparent
            }))
            .hover_fill(rc(k.subtle_fill_color_secondary))
            .child(icon(sym, 20.0, Color::from_argb(tint)))
            .child(label(name, ramp.body, k.text_fill_color_primary));
        if sel {
            item = item.child(selection_pill(t, 0.0, 10.0));
        }
        pane = pane.child(item);
    }

    let mut content = el()
        .grow(1.0)
        .h_full()
        .child(label("System", ramp.title, k.text_fill_color_primary).absolute(13.0, 10.0))
        .child(
            el().absolute(13.0, 76.0)
                .size(120.0, 68.0)
                .radius(radii::CONTROL)
                .border(1.0, rc(Color::from_argb(0xFF3A3A3A)))
                .linear_gradient(
                    std::f32::consts::PI * 0.25,
                    [
                        (0.0, rc(Color::from_argb(0xFF06123A))),
                        (0.55, rc(Color::from_argb(0xFF1F5FE0))),
                        (1.0, rc(Color::from_argb(0xFF0A1C5C))),
                    ],
                ),
        )
        .child(label("Plumbum", ramp.subtitle, k.text_fill_color_primary).absolute(149.0, 79.0))
        .child(
            label("System Product Name", ramp.body, k.text_fill_color_primary)
                .absolute(149.0, 105.0),
        )
        .child(label("Rename", ramp.body, k.accent_text_fill_color_primary).absolute(149.0, 125.0))
        .child(ms_logo(20.0).absolute(552.0, 99.0))
        .child(
            label("Microsoft 365", ramp.body_strong, k.text_fill_color_primary)
                .absolute(598.0, 90.0),
        )
        .child(
            label("View benefits", ramp.caption, k.text_fill_color_secondary)
                .absolute(598.0, 113.0),
        )
        .child(
            el().absolute(722.0, 99.0)
                .size(20.0, 20.0)
                .radius(10.0)
                .fill(rc(Color::from_argb(0xFF1A86D9)))
                .center()
                .child(icon(Symbol::ArrowSync, 14.0, Color::from_argb(0xFFFFFFFF))),
        )
        .child(
            label(
                "Windows Update",
                ramp.body_strong,
                k.text_fill_color_primary,
            )
            .absolute(747.0, 90.0),
        )
        .child(
            row()
                .absolute(747.0, 113.0)
                .gap(6.0)
                .center_y()
                .child(
                    el().size(4.0, 4.0)
                        .radius(2.0)
                        .fill(rc(k.accent_fill_color_default)),
                )
                .child(label(
                    "Attention needed",
                    ramp.caption,
                    k.text_fill_color_secondary,
                )),
        );
    for (i, (sym, title, sub)) in CARDS.into_iter().enumerate() {
        content = content.child(card(t, sym, title, sub).absolute(13.0, 175.0 + 72.0 * i as f32));
    }

    let root = col()
        .w_full()
        .h_full()
        .child(title_bar(t, 48.0, bar))
        .child(
            row()
                .w_full()
                .grow(1.0)
                .min_h(0.0)
                .child(pane)
                .child(content),
        );
    root.clip_rounded(0.0)
}

/// A `SettingsCard`: 20px icon, title + description, trailing chevron.
fn card(t: &Theme, sym: Symbol, title: &str, sub: &str) -> El {
    let k = t.tokens();
    let ramp = t.type_ramp();
    row()
        .size(852.0, 69.0)
        .radius(radii::CONTROL)
        .fill(rc(k.card_background_fill_color_default))
        .border(1.0, rc(k.card_stroke_color_default))
        .hover_fill(rc(k.card_background_fill_color_secondary))
        .pl(18.0)
        .pr(18.0)
        .gap(21.0)
        .center_y()
        .child(icon(sym, 20.0, k.text_fill_color_primary))
        .child(
            col()
                .grow(1.0)
                .child(label(title, ramp.body, k.text_fill_color_primary))
                .child(label(sub, ramp.caption, k.text_fill_color_secondary)),
        )
        .child(icon(Symbol::ChevronRight, 16.0, k.text_fill_color_primary))
}
