//! `ref-store`: Microsoft Store "Updates & downloads" (docs/reference/store-updates.png).

use rune::{El, col, el, row};
use winui_controls::style::rc;
use winui_controls::{Button, selection_pill};
use winui_icons::Symbol;
use winui_theme::{Color, TextStyle, Theme, radii};

use super::{app_tile, icon, label, ms_logo, title_bar};

const ROWS: [(Symbol, u32, &str, &str, &str, &str); 10] = [
    (
        Symbol::Mail,
        0xFF0F6CBD,
        "Outlook for Windows",
        "Microsoft Corporation",
        "Apps",
        "Open",
    ),
    (
        Symbol::Camera,
        0xFF0078D4,
        "Raw Image Extension",
        "Microsoft Windows",
        "System Components",
        "More info",
    ),
    (
        Symbol::Toolbox,
        0xFF0078D4,
        "App Installer",
        "Microsoft Windows",
        "System Components",
        "More info",
    ),
    (
        Symbol::Play,
        0xFF0078D4,
        "HEIF Image Extension",
        "Microsoft Windows",
        "System Components",
        "More info",
    ),
    (
        Symbol::Play,
        0xFF0078D4,
        "VP9 Video Extensions",
        "Microsoft Windows",
        "System Components",
        "More info",
    ),
    (
        Symbol::Send,
        0xFF2AA3E0,
        "Telegram for Windows (Unigram)",
        "Unigram, Inc.",
        "Apps",
        "Open",
    ),
    (
        Symbol::Play,
        0xFF0078D4,
        "WebP Image Extension",
        "Microsoft Windows",
        "System Components",
        "More info",
    ),
    (
        Symbol::PaintBrush,
        0xFF1F86D6,
        "Paint",
        "Microsoft Corporation",
        "Apps",
        "Open",
    ),
    (
        Symbol::Games,
        0xFF107C10,
        "Game Bar",
        "Microsoft Windows",
        "System Components",
        "More info",
    ),
    (
        Symbol::Cut,
        0xFFC42B1C,
        "Snipping Tool",
        "Microsoft Corporation",
        "Apps",
        "Open",
    ),
];

const NAV_TOP: [(Symbol, &str); 5] = [
    (Symbol::Home, "Home"),
    (Symbol::Apps, "Apps"),
    (Symbol::Games, "Gaming"),
    (Symbol::Sparkle, "AI Hub"),
    (Symbol::PaintBrush, "Themes"),
];

pub fn view(t: &Theme) -> El {
    let k = t.tokens();
    let ramp = t.type_ramp();
    let search = row()
        .absolute(302.0, 9.0)
        .size(598.0, 32.0)
        .radius(radii::CONTROL)
        .fill(rc(k.control_fill_color_default))
        .border(1.0, rc(k.control_stroke_color_default))
        .pl(10.0)
        .pr(12.0)
        .center_y()
        .justify_between()
        .child(label(
            "Search apps, games, and more",
            ramp.body,
            k.text_fill_color_secondary,
        ))
        .child(icon(Symbol::Find, 14.0, k.text_fill_color_primary));
    let bag = el()
        .size(16.0, 16.0)
        .radius(3.0)
        .fill(rc(Color::from_argb(0xFFEDEDED)))
        .center()
        .child(ms_logo(8.0));
    let bar = el()
        .child(bag.absolute(20.0, 17.0))
        .child(
            label("Microsoft Store", ramp.caption, k.text_fill_color_primary).absolute(55.0, 17.0),
        )
        .child(search)
        .child(
            el().absolute(979.0, 9.0)
                .size(32.0, 32.0)
                .radius(16.0)
                .fill(rc(k.control_fill_color_default))
                .border(1.0, rc(k.control_stroke_color_default))
                .center()
                .child(icon(Symbol::Person, 16.0, k.text_fill_color_primary)),
        );

    let nav_label = TextStyle::new(11.0, 400, 14.0);
    let item = |sym: Symbol, name: &str, top: f32| {
        col()
            .absolute(5.0, top)
            .size(64.0, 56.0)
            .radius(radii::CONTROL)
            .hover_fill(rc(k.subtle_fill_color_secondary))
            .items_center()
            .pt(12.0)
            .gap(6.0)
            .child(icon(sym, 20.0, k.text_fill_color_primary))
            .child(label(name, nav_label, k.text_fill_color_primary))
    };
    let mut pane = el().w(74.0).h_full();
    for (i, (sym, name)) in NAV_TOP.into_iter().enumerate() {
        pane = pane.child(item(sym, name, -2.0 + 63.0 * i as f32));
    }
    let downloads = el()
        .absolute(5.0, 753.0)
        .size(64.0, 56.0)
        .radius(radii::CONTROL)
        .fill(rc(k.subtle_fill_color_secondary))
        .center()
        .child(icon(
            Symbol::Download,
            20.0,
            k.accent_text_fill_color_primary,
        ))
        .child(
            el().absolute(40.0, 14.0)
                .size(6.0, 6.0)
                .radius(3.0)
                .fill(rc(k.accent_fill_color_default)),
        )
        .child(selection_pill(t, 0.0, 20.0));
    pane = pane
        .child(item(Symbol::Gift, "What's New", 690.0))
        .child(downloads)
        .child(item(Symbol::Library, "Library", 816.0));

    let mut layer = el()
        .grow(1.0)
        .h_full()
        .corners([radii::OVERLAY, 0.0, 0.0, 0.0])
        .fill(rc(k.layer_fill_color_default))
        .border_sides([1.0, 1.0, 0.0, 0.0], rc(k.card_stroke_color_default))
        .clip_corners([radii::OVERLAY, 0.0, 0.0, 0.0])
        .child(
            label("Updates & downloads", ramp.title, k.text_fill_color_primary)
                .absolute(36.0, 35.0),
        )
        .child(
            Button::new("Check for updates")
                .accent(true)
                .width(135.0)
                .view(t)
                .absolute(954.0, 37.0),
        )
        .child(
            label("Recent updates", ramp.subtitle, k.text_fill_color_primary).absolute(35.0, 93.0),
        );
    for (i, r) in ROWS.into_iter().enumerate() {
        layer = layer.child(update_row(t, r).absolute(36.0, 132.0 + 78.0 * i as f32));
    }

    col()
        .w_full()
        .h_full()
        .child(title_bar(t, 50.0, bar))
        .child(row().w_full().grow(1.0).min_h(0.0).child(pane).child(layer))
        .clip_rounded(0.0)
}

fn update_row(
    t: &Theme,
    (sym, tint, name, publisher, category, action): (Symbol, u32, &str, &str, &str, &str),
) -> El {
    let k = t.tokens();
    let ramp = t.type_ramp();
    el().size(1054.0, 72.0)
        .radius(radii::CONTROL)
        .fill(rc(k.card_background_fill_color_default))
        .border(1.0, rc(k.card_stroke_color_default))
        .child(app_tile(sym, 48.0, tint).absolute(11.0, 12.0))
        .child(
            col()
                .absolute(71.0, 18.0)
                .child(label(name, ramp.body, k.text_fill_color_primary))
                .child(label(publisher, ramp.caption, k.text_fill_color_secondary)),
        )
        .child(label(category, ramp.body, k.text_fill_color_primary).absolute(338.0, 26.0))
        .child(
            label("Modified yesterday", ramp.body, k.text_fill_color_primary).absolute(556.0, 26.0),
        )
        .child(
            Button::new(action)
                .width(117.0)
                .view(t)
                .absolute(874.0, 20.0),
        )
        .child(
            el().absolute(1004.0, 20.0)
                .size(40.0, 32.0)
                .radius(radii::CONTROL)
                .hover_fill(rc(k.subtle_fill_color_secondary))
                .center()
                .child(icon(
                    Symbol::MoreHorizontal,
                    16.0,
                    k.text_fill_color_primary,
                )),
        )
}
