//! Settings > System (`docs/reference/settings-system.png`).

use rune::{El, TextEdit, col, el, row};
use winui_controls::{HyperlinkButton, PersonPicture};
use winui_icons::Symbol;
use winui_theme::{Color, Theme, radii};

use super::glyphs as g;
use super::{caption_buttons, chevron_right, path_icon, sym, txt};
use crate::fluent::c;

const NAV_W: f32 = 320.0;
const TITLE_H: f32 = 48.0;

enum Ico {
    Sym(Symbol),
    Path(&'static str),
}

fn ico(i: &Ico, size: f32, color: Color) -> El {
    match i {
        Ico::Sym(s) => sym(*s, size, color),
        Ico::Path(d) => path_icon(d, size, color),
    }
}

fn search(t: &Theme) -> El {
    let k = t.tokens();
    let _edit = TextEdit::new("");
    row()
        .w(515.0)
        .h(32.0)
        .radius(16.0)
        .fill(c(k.control_fill_color_default))
        .border(1.0, c(k.control_stroke_color_default))
        .items_center()
        .pl(14.0)
        .gap(12.0)
        .child(sym(Symbol::Find, 14.0, k.text_fill_color_primary))
        .child(txt(
            "Find a setting",
            t.type_ramp().body,
            k.text_fill_color_secondary,
        ))
}

fn title_bar(t: &Theme) -> El {
    let k = t.tokens();
    let back = el()
        .size(40.0, 36.0)
        .radius(radii::CONTROL)
        .center()
        .child(sym(Symbol::Back, 16.0, k.text_fill_color_primary));
    row()
        .w_full()
        .h(TITLE_H)
        .items_start()
        .child(
            row()
                .w(NAV_W + 23.0)
                .h(TITLE_H)
                .items_center()
                .pl(4.0)
                .child(back)
                .child(el().pl(6.0).child(txt(
                    "Settings",
                    t.type_ramp().caption,
                    k.text_fill_color_primary,
                ))),
        )
        .child(el().h(TITLE_H).center().child(search(t)))
        .child(el().grow(1.0))
        .child(caption_buttons(t, 32.0))
}

fn nav(t: &Theme) -> El {
    let k = t.tokens();
    let a = t.accent();
    let blue = if t.is_dark() { a.light1 } else { a.base };
    let grey = k.text_fill_color_secondary;
    let warm = k.system_fill_color_caution;
    let green = k.system_fill_color_success;
    let items: [(Ico, Color, &str); 12] = [
        (Ico::Path(g::HOME_FILLED), warm, "Home"),
        (Ico::Path(g::DESKTOP_FILLED), blue, "System"),
        (Ico::Path(g::BLUETOOTH_FILLED), blue, "Bluetooth & devices"),
        (Ico::Sym(Symbol::WiFi), blue, "Network & internet"),
        (Ico::Path(g::BRUSH_FILLED), warm, "Personalization"),
        (Ico::Path(g::APPS_FILLED), blue, "Apps"),
        (Ico::Path(g::PERSON_FILLED), green, "Accounts"),
        (Ico::Path(g::GLOBE_CLOCK_FILLED), blue, "Time & language"),
        (Ico::Path(g::GAMES_FILLED), grey, "Gaming"),
        (Ico::Path(g::ACCESSIBILITY_FILLED), blue, "Accessibility"),
        (Ico::Path(g::SHIELD_FILLED), grey, "Privacy & security"),
        (Ico::Path(g::SYNC_CIRCLE_FILLED), blue, "Windows Update"),
    ];
    let account = row()
        .h(60.0)
        .items_center()
        .gap(12.0)
        .child(PersonPicture::new().size(60.0).view(t))
        .child(
            col()
                .gap(1.0)
                .child(txt(
                    "Coobyk",
                    t.type_ramp().body_strong,
                    k.text_fill_color_primary,
                ))
                .child(txt(
                    "Local Account",
                    t.type_ramp().caption,
                    k.text_fill_color_primary,
                )),
        );
    let mut list = col().gap(4.0).pt(27.0);
    for (i, (icon, color, label)) in items.iter().enumerate() {
        let selected = i == 1;
        let mut item = row()
            .w(280.0)
            .h(36.0)
            .radius(radii::CONTROL)
            .items_center()
            .child(el().w(44.0).center().child(ico(icon, 18.0, *color)))
            .child(txt(*label, t.type_ramp().body, k.text_fill_color_primary));
        if selected {
            let pill = el()
                .size(3.0, 16.0)
                .radius(radii::SELECTION_INDICATOR)
                .fill(c(k.accent_fill_color_default))
                .absolute(0.0, 10.0);
            item = item.fill(c(k.subtle_fill_color_secondary)).child(pill);
        }
        list = list.child(item);
    }
    col()
        .w(NAV_W)
        .h_full()
        .pl(17.0)
        .pt(10.0)
        .child(account)
        .child(list)
}

fn hero_image(t: &Theme) -> El {
    let a = t.accent();
    let k = t.tokens();
    // The Windows 11 "Bloom" wallpaper as a radial accent glow.
    let glow = el()
        .pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
        .radial_gradient(
            [0.52, 0.68],
            [40.0, 26.0],
            [
                (0.0, c(a.light2)),
                (0.3, c(a.light1)),
                (0.65, c(a.base.with_alpha(0.6))),
                (1.0, c(a.base.with_alpha(0.0))),
            ],
        );
    let wall = el()
        .w_full()
        .h_full()
        .radius(2.0)
        .linear_gradient(
            std::f32::consts::FRAC_PI_4 * 3.0,
            [(0.0, c(a.dark3)), (1.0, c(a.dark2))],
        )
        .child(glow);
    el().size(120.0, 68.0)
        .radius(radii::CONTROL)
        .padding(3.0)
        .fill(c(k.solid_background_fill_color_secondary))
        .border(1.0, c(k.card_stroke_color_default))
        .child(wall)
}

fn ms365(t: &Theme) -> El {
    let k = t.tokens();
    let q = |col: Color| el().size(9.0, 9.0).fill(c(col));
    col()
        .gap(1.0)
        .child(
            row()
                .gap(1.0)
                .child(q(k.system_fill_color_critical))
                .child(q(k.system_fill_color_success)),
        )
        .child(
            row()
                .gap(1.0)
                .child(q(t.accent().light1))
                .child(q(k.system_fill_color_caution)),
        )
}

fn quick_link(t: &Theme, icon: El, title: &str, sub: El) -> El {
    let k = t.tokens();
    let r = row()
        .items_start()
        .gap(10.0)
        .child(el().pt(6.0).child(icon))
        .child(
            col()
                .gap(2.0)
                .child(txt(
                    title,
                    t.type_ramp().body_strong,
                    k.text_fill_color_primary,
                ))
                .child(sub),
        );
    el().pt(8.0).child(r)
}

fn hero(t: &Theme) -> El {
    let k = t.tokens();
    let cap = t.type_ramp().caption;
    let rename = HyperlinkButton::new("Rename").view(t);
    let device = col()
        .gap(1.0)
        .child(txt(
            "Plumbum",
            super::px_style(18.0, 600, 24.0),
            k.text_fill_color_primary,
        ))
        .child(txt(
            "System Product Name",
            t.type_ramp().body,
            k.text_fill_color_primary,
        ))
        .child(el().ml(-11.0).my(-6.0).child(rename));
    let update_sub = row()
        .items_center()
        .gap(6.0)
        .child(
            el().size(4.0, 4.0)
                .radius(2.0)
                .fill(c(k.accent_text_fill_color_primary)),
        )
        .child(txt("Attention needed", cap, k.text_fill_color_secondary));
    row()
        .w_full()
        .h(68.0)
        .items_center()
        .child(hero_image(t))
        .child(el().pl(18.0).pt(10.0).child(device))
        .child(el().grow(1.0))
        .child(quick_link(
            t,
            ms365(t),
            "Microsoft 365",
            txt("View benefits", cap, k.text_fill_color_secondary),
        ))
        .child(el().w(34.0))
        .child(quick_link(
            t,
            path_icon(g::SYNC_CIRCLE_FILLED, 20.0, t.accent().light1),
            "Windows Update",
            update_sub,
        ))
        .child(el().w(8.0))
}

fn card(t: &Theme, icon: Ico, title: &str, sub: &str) -> El {
    let k = t.tokens();
    row()
        .w_full()
        .h(68.0)
        .radius(radii::CONTROL)
        .fill(c(k.card_background_fill_color_default))
        .border(1.0, c(k.card_stroke_color_default))
        .items_center()
        .pl(19.0)
        .pr(18.0)
        .child(ico(&icon, 20.0, k.text_fill_color_primary))
        .child(
            col()
                .pl(20.0)
                .grow(1.0)
                .gap(1.0)
                .child(txt(title, t.type_ramp().body, k.text_fill_color_primary))
                .child(txt(sub, t.type_ramp().caption, k.text_fill_color_secondary)),
        )
        .child(chevron_right(t))
}

fn content(t: &Theme) -> El {
    let k = t.tokens();
    let rows = [
        (
            Ico::Path(g::LAPTOP),
            "Display",
            "Monitors, brightness, night light, display profile",
        ),
        (
            Ico::Path(g::SPEAKER),
            "Sound",
            "Volume levels, output, input, sound devices",
        ),
        (
            Ico::Sym(Symbol::Alert),
            "Notifications",
            "Alerts from apps and system, do not disturb",
        ),
        (Ico::Path(g::CIRCLE_HALF), "Focus", "Reduce distractions"),
        (
            Ico::Sym(Symbol::Power),
            "Power",
            "Screen and sleep, power mode, energy saver",
        ),
        (
            Ico::Path(g::STORAGE),
            "Storage",
            "Storage space, drives, configuration rules",
        ),
        (
            Ico::Sym(Symbol::Share),
            "Share",
            "Share files, photos, links and more",
        ),
        (
            Ico::Path(g::WINDOW_MULTIPLE),
            "Multitasking",
            "Snap windows, desktops, task switching",
        ),
        (
            Ico::Path(g::WRENCH_SCREWDRIVER),
            "Advanced",
            "Performance, optimization, and developer features",
        ),
        (
            Ico::Sym(Symbol::CheckmarkCircle),
            "Activation",
            "Activation state, subscriptions, product key",
        ),
        (
            Ico::Sym(Symbol::Wrench),
            "Troubleshoot",
            "Recommended troubleshooters, preferences, history",
        ),
    ];
    let mut cards = col().w_full().gap(4.0).pt(32.0);
    for (i, title, sub) in rows {
        cards = cards.child(card(t, i, title, sub));
    }
    col()
        .grow(1.0)
        .h_full()
        .min_w(0.0)
        .pt(11.0)
        .pl(5.0)
        .pr(25.0)
        .child(txt(
            "System",
            t.type_ramp().title.with_weight(600),
            k.text_fill_color_primary,
        ))
        .child(el().h(23.0))
        .child(hero(t))
        .child(cards)
        .clip_rounded(0.0)
}

pub fn view(t: &Theme) -> El {
    let k = t.tokens();
    col()
        .fill(c(k.solid_background_fill_color_base))
        .child(title_bar(t))
        .child(
            row()
                .w_full()
                .grow(1.0)
                .min_h(0.0)
                .child(nav(t))
                .child(content(t)),
        )
}
