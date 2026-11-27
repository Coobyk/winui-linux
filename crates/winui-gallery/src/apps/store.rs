//! Microsoft Store > Updates & downloads (`store-updates.png`).

use super::{ACCENT, WHITE, art, caption_buttons, icons, label, path_icon, rgb, rgba};
use crate::State;
use rune::{El, col, el, row};

const BG: u32 = 0x202020;
const PANE: u32 = 0x272727;
const ROW: u32 = 0x323232;
const RAIL_FG: u32 = 0x9A9A9A;
const RAIL_TEXT: u32 = 0xCCCCCC;

fn store_logo() -> El {
    art(
        16.0,
        16.0,
        [17.0, 17.0],
        &[
            ("M5.5 3 V2.5 a2.5 2.5 0 0 1 5 0 V3", 0xEDEDED, 1.2),
            (
                "M1 3.5 h14 v10 a2 2 0 0 1 -2 2 h-10 a2 2 0 0 1 -2 -2Z",
                WHITE,
                0.0,
            ),
            ("M4.5 6.5 h3.2 v3.2 h-3.2z", 0xF25022, 0.0),
            ("M8.3 6.5 h3.2 v3.2 h-3.2z", 0x7FBA00, 0.0),
            ("M4.5 10.3 h3.2 v3.2 h-3.2z", 0x00A4EF, 0.0),
            ("M8.3 10.3 h3.2 v3.2 h-3.2z", 0xFFB900, 0.0),
        ],
    )
}

fn rail_item(icon: &'static str, t: &'static str, y: f32) -> El {
    col()
        .absolute(5.0, y)
        .size(64.0, 56.0)
        .items_center()
        .radius(4.0)
        .hover(rgba(WHITE, 0.0605))
        .role("button")
        .label(t)
        .child(path_icon(icon, 24.0, RAIL_FG).mt(5.5))
        .child(label(t, 10.0, 600.0, RAIL_TEXT).mt(4.0))
}

fn rail() -> El {
    let sel = el()
        .absolute(5.0, 803.0)
        .size(64.0, 58.0)
        .radius(4.0)
        .fill(rgb(0x454545))
        .child(
            el().absolute(0.0, 17.0)
                .size(3.0, 24.0)
                .radius(1.5)
                .fill(rgb(ACCENT)),
        )
        .child(path_icon(icons::CLOUD_ARROW_DOWN, 26.0, ACCENT).absolute(19.0, 15.0))
        .child(
            el().absolute(49.0, 9.0)
                .size(7.0, 7.0)
                .radius(3.5)
                .fill(rgb(ACCENT)),
        );
    el().absolute(0.0, 0.0)
        .size(73.0, 928.0)
        .child(rail_item(icons::HOME, "Home", 53.0))
        .child(rail_item(icons::APPS, "Apps", 115.0))
        .child(rail_item(icons::XBOX_CONTROLLER, "Gaming", 177.0))
        .child(rail_item(icons::SPARKLE, "AI Hub", 239.0))
        .child(rail_item(icons::EDIT, "Themes", 301.0))
        .child(rail_item(icons::MEGAPHONE, "What\u{2019}s New", 745.0))
        .child(sel)
        .child(rail_item(icons::LIBRARY, "Library", 869.0))
}

fn title_bar() -> El {
    let search = row()
        .absolute(301.0, 9.0)
        .size(600.0, 33.0)
        .radius(4.0)
        .fill(rgb(0x2D2D2D))
        .border_sides([0.0, 1.0, 0.0, 1.0], rgb(0x333333))
        .items_center()
        .child(
            label("Search apps, games, and more", 14.0, 400.0, 0xCFCFCF)
                .ml(11.0)
                .mt(-1.0),
        )
        .child(rune::spacer())
        .child(path_icon(icons::SEARCH, 14.0, 0xEFEFEF).mr(12.0));
    let avatar = el()
        .absolute(979.0, 9.0)
        .size(32.0, 32.0)
        .radius(16.0)
        .fill(rgb(0x1C1C1C))
        .border(1.0, rgb(0x353535))
        .center()
        .child(path_icon(icons::PERSON, 16.0, WHITE));
    el().absolute(0.0, 0.0)
        .w_full()
        .h(48.0)
        .window_drag()
        .child(store_logo().absolute(19.5, 16.5))
        .child(label("Microsoft Store", 12.0, 400.0, 0x8F8F8F).absolute(56.0, 18.0))
        .child(search)
        .child(avatar)
        .child(caption_buttons(33.0, 0x737373).absolute(1063.0, 0.0))
}

fn tile(i: usize) -> El {
    let blue = |glyph: El| {
        el().size(48.0, 48.0)
            .radius(4.0)
            .fill(rgb(0x0078D7))
            .border(1.0, rgb(0x1282DA))
            .center()
            .child(glyph)
    };
    let puzzle_play = || {
        art(
            32.0,
            32.0,
            [40.0, 40.0],
            &[
                (
                    "M5 9 h6 a4 4 0 0 1 8 0 h4 v6 a4 4 0 0 1 0 8 v4 h-18Z",
                    WHITE,
                    0.0,
                ),
                ("M10 13 L18 18 L10 23Z", 0x0078D7, 0.0),
            ],
        )
    };
    match i {
        0 => el()
            .size(48.0, 48.0)
            .radius(4.0)
            .fill(rgb(0x323232))
            .border(1.0, rgb(0x404040))
            .center()
            .child(art(
                32.0,
                32.0,
                [42.0, 42.0],
                &[
                    ("M10 10 L24 4 L30 12 V24 a2 2 0 0 1 -2 2 H12Z", 0x1490DF, 0.0),
                    ("M12 14 L30 12 V24 a2 2 0 0 1 -2 2 H12Z", 0x28A8EA, 0.0),
                    ("M12 14 L22 20 L30 12", 0x50D9FF, 1.2),
                    ("M2 13 a2 2 0 0 1 2 -2 h10 a2 2 0 0 1 2 2 v10 a2 2 0 0 1 -2 2 h-10 a2 2 0 0 1 -2 -2Z", 0x0A5BC4, 0.0),
                    ("M9 14.5 a3.2 3.5 0 1 1 0 7 a3.2 3.5 0 1 1 0 -7Z", WHITE, 1.6),
                ],
            )),
        1 => blue(path_icon(icons::CAMERA_F, 40.0, WHITE)),
        2 => blue(path_icon(icons::BOX_F, 40.0, WHITE)),
        3 | 4 | 6 => blue(puzzle_play()),
        5 => el()
            .size(48.0, 48.0)
            .radius(4.0)
            .linear_gradient(
                std::f32::consts::PI * 0.75,
                [(0.0, rgb(0x37AEE2)), (1.0, rgb(0x1E96C8))],
            )
            .center()
            .child(art(
                24.0,
                24.0,
                [26.0, 26.0],
                &[("M2 11.5 L21 4 L18 20 L12.5 16 L9.5 19 L9 14.5 L17 7.5 L7.5 13.5Z", WHITE, 0.0)],
            )),
        7 => blue(art(
            32.0,
            32.0,
            [34.0, 34.0],
            &[
                ("M16 3 C24 3 30 8.5 30 15 C30 20 26 21 23 20 C20 19 19 22 20 25 C21 28 19 30 15 30 C8 30 2 24 2 16 C2 8.5 8 3 16 3Z", 0x1CA0E8, 0.0),
                ("M9 11 a2 2 0 1 1 0 4 a2 2 0 1 1 0 -4Z", 0xF25022, 0.0),
                ("M14 6.5 a2 2 0 1 1 0 4 a2 2 0 1 1 0 -4Z", 0xFFB900, 0.0),
                ("M20 7.5 a2 2 0 1 1 0 4 a2 2 0 1 1 0 -4Z", 0x7FBA00, 0.0),
                ("M8 18 a2 2 0 1 1 0 4 a2 2 0 1 1 0 -4Z", 0xE3008C, 0.0),
                ("M27 6 L21 24", 0xC8A26A, 2.2),
            ],
        )),
        8 => el()
            .size(48.0, 48.0)
            .radius(4.0)
            .fill(rgb(0x107C10))
            .border(1.0, rgb(0x248024))
            .center()
            .child(art(
                32.0,
                32.0,
                [32.0, 32.0],
                &[
                    ("M6 11 h16 v12 h-16z", 0xFFFFFF, 1.8),
                    ("M11 16 h14 v9 h-14z", 0x107C10, 0.0),
                    ("M11 16 h14 v9 h-14z", WHITE, 1.8),
                    ("M14 22 L22 18", WHITE, 1.6),
                ],
            )),
        _ => el()
            .size(48.0, 48.0)
            .radius(4.0)
            .fill(rgb(0x0078D7))
            .border(1.0, rgb(0x1282DA))
            .child(el().absolute(6.0, 6.0).size(28.0, 28.0).radius(2.0).fill(rgb(0xF2F2F2)))
            .child(art(
                28.0,
                28.0,
                [28.0, 28.0],
                &[("M4 4 h4 v4 h-4z M12 4 h4 v4 h-4z M4 12 h4 v4 h-4z", 0xE81123, 0.0)],
            )
            .absolute(6.0, 6.0)),
    }
}

const APPS: [(&str, &str, bool); 10] = [
    ("Outlook for Windows", "Microsoft Corporation", true),
    ("Raw Image Extension", "Microsoft Windows", false),
    ("App Installer", "Microsoft Windows", false),
    ("HEIF Image Extension", "Microsoft Windows", false),
    ("VP9 Video Extensions", "Microsoft Windows", false),
    ("Telegram for Windows (Unigram)", "Unigram, Inc.", true),
    ("WebP Image Extension", "Microsoft Windows", false),
    ("Paint", "Microsoft Corporation", true),
    ("Game Bar", "Microsoft Windows", false),
    ("Snipping Tool", "Microsoft Corporation", true),
];

fn button(t: &'static str, w: f32, h: f32) -> El {
    el().size(w, h)
        .radius(4.0)
        .fill(rgb(0x3E3E3E))
        .border_sides([0.0, 1.0, 0.0, 1.0], rgb(0x444444))
        .hover(rgb(0x454545))
        .press(rgb(0x393939))
        .center()
        .role("button")
        .label(t)
        .child(label(t, 14.0, 400.0, WHITE).mt(-1.0))
}

fn app_row(i: usize) -> El {
    let (name, publisher, app) = APPS[i];
    el().absolute(38.0, 134.0 + 78.0 * i as f32)
        .size(1053.0, 71.0)
        .radius(4.0)
        .fill(rgb(ROW))
        .border(1.0, rgb(0x232323))
        .child(tile(i).absolute(11.0, 11.0))
        .child(label(name, 14.0, 400.0, WHITE).absolute(72.0, 16.0))
        .child(label(publisher, 12.0, 400.0, 0xD0D0D0).absolute(72.0, 36.0))
        .child(
            label(
                if app { "Apps" } else { "System Components" },
                14.0,
                400.0,
                WHITE,
            )
            .absolute(338.0, 26.0),
        )
        .child(label("Modified yesterday", 14.0, 400.0, WHITE).absolute(556.0, 26.0))
        .child(button(if app { "Open" } else { "More info" }, 120.0, 32.0).absolute(873.0, 19.0))
        .child(
            el().absolute(1005.0, 19.0)
                .size(40.0, 32.0)
                .radius(4.0)
                .hover(rgba(WHITE, 0.0605))
                .center()
                .child(path_icon(icons::MORE_HORIZONTAL, 26.0, WHITE)),
        )
}

pub fn view(_s: &State) -> El {
    let mut pane = el()
        .absolute(73.0, 49.0)
        .size(1128.0, 879.0)
        .corners([8.0, 0.0, 0.0, 0.0])
        .clip_corners([8.0, 0.0, 0.0, 0.0])
        .fill(rgb(PANE))
        .border_sides([1.0, 1.0, 0.0, 0.0], rgb(0x1D1D1D))
        .child(label("Updates & downloads", 23.5, 600.0, WHITE).absolute(37.0, 38.0))
        .child(
            el().absolute(955.0, 37.0)
                .size(136.0, 33.0)
                .radius(4.0)
                .fill(rgb(ACCENT))
                .border_sides([0.0, 0.0, 0.0, 1.0], rgb(0x42A7DC))
                .hover(rgba(ACCENT, 0.9))
                .center()
                .role("button")
                .child(label("Check for updates", 14.0, 400.0, 0x000000).mt(-1.0)),
        )
        .child(label("Recent updates", 19.5, 600.0, WHITE).absolute(36.0, 95.0));
    for i in 0..APPS.len() {
        pane = pane.child(app_row(i));
    }
    el().w_full()
        .h_full()
        .fill(rgb(BG))
        .child(title_bar())
        .child(rail())
        .child(pane)
}
