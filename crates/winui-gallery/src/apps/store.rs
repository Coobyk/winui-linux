//! Microsoft Store, Updates & downloads (`docs/reference/store-updates.png`).

use rune::{El, col, el, row};
use winui_controls::Button;
use winui_icons::Symbol;
use winui_theme::{AccentRamp, Color, Theme, radii};

use super::glyphs as g;
use super::{caption_buttons, path_icon, sym, txt};
use crate::fluent::c;

const RAIL_W: f32 = 73.0;

fn rail_item(t: &Theme, icon: El, label: &str, selected: bool) -> El {
    let k = t.tokens();
    let mut r = col()
        .w(62.0)
        .h(selected.then_some(57.0).unwrap_or(62.0))
        .radius(radii::CONTROL)
        .items_center()
        .justify_center()
        .gap(4.0)
        .child(icon);
    if !label.is_empty() {
        r = r.child(txt(
            label,
            super::px_style(11.0, 400, 14.0),
            k.text_fill_color_primary,
        ));
    }
    if selected {
        let pill = el()
            .size(3.0, 16.0)
            .radius(radii::SELECTION_INDICATOR)
            .fill(c(k.accent_fill_color_default))
            .absolute(0.0, 20.0);
        r = r.fill(c(k.subtle_fill_color_secondary)).child(pill);
    }
    r
}

fn rail(t: &Theme) -> El {
    let k = t.tokens();
    let ink = k.text_fill_color_primary;
    let i = |d: &'static str| path_icon(d, 20.0, ink);
    let downloads = el().child(i(g::CLOUD_ARROW_DOWN)).child(
        el().size(6.0, 6.0)
            .radius(3.0)
            .fill(c(k.accent_fill_color_default))
            .absolute(17.0, -4.0),
    );
    col()
        .w(RAIL_W)
        .h_full()
        .pl(6.0)
        .pt(2.0)
        .pb(6.0)
        .child(rail_item(t, i(g::HOME), "Home", false))
        .child(rail_item(t, i(g::APPS), "Apps", false))
        .child(rail_item(t, i(g::GAMES_REG), "Gaming", false))
        .child(rail_item(t, i(g::SPARKLE), "AI Hub", false))
        .child(rail_item(t, i(g::BRUSH), "Themes", false))
        .child(el().grow(1.0))
        .child(rail_item(t, i(g::MEGAPHONE), "What's New", false))
        .child(el().h(6.0))
        .child(rail_item(t, downloads, "", true))
        .child(el().h(6.0))
        .child(rail_item(t, i(g::LIBRARY), "Library", false))
}

fn title_bar(t: &Theme) -> El {
    let k = t.tokens();
    let ink = k.text_fill_color_primary;
    let logo = super::settings_logo(t, 7.0);
    let search = row()
        .w(600.0)
        .h(32.0)
        .radius(radii::CONTROL)
        .fill(c(k.control_fill_color_default))
        .border(1.0, c(k.control_stroke_color_default))
        .items_center()
        .pl(10.0)
        .pr(12.0)
        .child(txt(
            "Search apps, games, and more",
            t.type_ramp().body,
            k.text_fill_color_secondary,
        ))
        .child(el().grow(1.0))
        .child(sym(Symbol::Find, 14.0, ink));
    let profile = el()
        .size(30.0, 30.0)
        .radius(15.0)
        .fill(c(k.control_fill_color_default))
        .border(1.0, c(k.control_stroke_color_default))
        .center()
        .child(path_icon(g::PERSON_CIRCLE, 16.0, ink));
    row()
        .w_full()
        .h(50.0)
        .items_start()
        .child(
            row()
                .w(301.0)
                .h(50.0)
                .items_center()
                .pl(22.0)
                .child(logo)
                .child(
                    el().pl(10.0)
                        .child(txt("Microsoft Store", t.type_ramp().caption, ink)),
                ),
        )
        .child(el().h(50.0).center().child(search))
        .child(el().w(80.0).h(50.0).center().child(profile))
        .child(el().grow(1.0))
        .child(caption_buttons(t, 32.0))
}

fn tile(t: &Theme, fill: Color, glyph: &'static str) -> El {
    let k = t.tokens();
    el().size(48.0, 48.0)
        .radius(radii::CONTROL)
        .fill(c(fill))
        .center()
        .child(path_icon(
            glyph,
            28.0,
            k.text_on_accent_fill_color_primary.with_alpha(1.0),
        ))
}

fn update_row(t: &Theme, icon: El, name: &str, publisher: &str, category: &str, open: bool) -> El {
    let k = t.tokens();
    let ink = k.text_fill_color_primary;
    let action = Button::new(if open { "Open" } else { "More info" })
        .width(120.0)
        .view(t);
    row()
        .w_full()
        .h(70.0)
        .radius(radii::CONTROL)
        .fill(c(k.card_background_fill_color_default))
        .border(1.0, c(k.card_stroke_color_default))
        .items_center()
        .pl(11.0)
        .child(icon)
        .child(
            col()
                .w(279.0)
                .pl(11.0)
                .gap(1.0)
                .child(txt(name, t.type_ramp().body, ink))
                .child(txt(
                    publisher,
                    t.type_ramp().caption,
                    k.text_fill_color_secondary,
                )),
        )
        .child(el().w(218.0).child(txt(category, t.type_ramp().body, ink)))
        .child(
            el().grow(1.0)
                .child(txt("Modified yesterday", t.type_ramp().body, ink)),
        )
        .child(action)
        .child(
            el().w(64.0)
                .center()
                .child(sym(Symbol::MoreHorizontal, 16.0, ink)),
        )
}

pub fn view(t: &Theme) -> El {
    let k = t.tokens();
    let ink = k.text_fill_color_primary;
    let blue = t.accent().base;
    let white = Color::from_argb(0xFFFFFFFF);
    let green = AccentRamp::derive(crate::ACCENTS[7].1).base;
    let rows = [
        (
            tile(t, blue, g::MAIL_FILLED),
            "Outlook for Windows",
            "Microsoft Corporation",
            "Apps",
            true,
        ),
        (
            tile(t, blue, g::CAMERA_FILLED),
            "Raw Image Extension",
            "Microsoft Windows",
            "System Components",
            false,
        ),
        (
            tile(t, blue, g::BOX),
            "App Installer",
            "Microsoft Windows",
            "System Components",
            false,
        ),
        (
            tile(t, blue, g::PUZZLE),
            "HEIF Image Extension",
            "Microsoft Windows",
            "System Components",
            false,
        ),
        (
            tile(t, blue, g::PUZZLE),
            "VP9 Video Extensions",
            "Microsoft Windows",
            "System Components",
            false,
        ),
        (
            tile(t, t.accent().light1, g::SEND_FILLED),
            "Telegram for Windows (Unigram)",
            "Unigram, Inc.",
            "Apps",
            true,
        ),
        (
            tile(t, blue, g::PUZZLE),
            "WebP Image Extension",
            "Microsoft Windows",
            "System Components",
            false,
        ),
        (
            tile(t, t.accent().dark1, g::PAINT_FILLED),
            "Paint",
            "Microsoft Corporation",
            "Apps",
            true,
        ),
        (
            tile(t, green, g::GAMES_FILLED),
            "Game Bar",
            "Microsoft Windows",
            "System Components",
            false,
        ),
        (
            tile(t, white, g::CUT_FILLED).fill(c(white)),
            "Snipping Tool",
            "Microsoft Corporation",
            "Apps",
            true,
        ),
    ];
    let mut list = col().w_full().gap(8.0);
    for (icon, name, publisher, category, open) in rows {
        list = list.child(update_row(t, icon, name, publisher, category, open));
    }
    let header = row()
        .w_full()
        .h(62.0)
        .items_center()
        .child(txt(
            "Updates & downloads",
            t.type_ramp().title.with_weight(600),
            ink,
        ))
        .child(el().grow(1.0))
        .child(
            Button::new("Check for updates")
                .accent(true)
                .width(135.0)
                .view(t),
        );
    let body = col()
        .w_full()
        .pl(37.0)
        .pr(36.0)
        .pt(21.0)
        .child(header)
        .child(el().pt(9.0).pb(12.0).child(txt(
            "Recent updates",
            t.type_ramp().subtitle.with_weight(600),
            ink,
        )))
        .child(list);
    let frame = el()
        .grow(1.0)
        .h_full()
        .min_w(0.0)
        .corners([radii::OVERLAY, 0.0, 0.0, 0.0])
        .fill(c(k.layer_fill_color_default))
        .border_sides([1.0, 1.0, 0.0, 0.0], c(k.card_stroke_color_default))
        .clip_corners([radii::OVERLAY, 0.0, 0.0, 0.0])
        .child(body);
    let main = row()
        .w_full()
        .grow(1.0)
        .min_h(0.0)
        .child(rail(t).ml(0.0))
        .child(frame);
    el().fill(c(k.solid_background_fill_color_base))
        .child(col().w_full().h_full().child(el().h(50.0)).child(main).pin(
            Some(0.0),
            Some(0.0),
            Some(0.0),
            Some(0.0),
        ))
        .child(title_bar(t).pin(Some(0.0), Some(0.0), Some(0.0), None))
}
