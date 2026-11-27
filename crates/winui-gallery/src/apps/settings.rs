//! Settings > System (`settings-system.png`).

use super::{ACCENT, WHITE, art, caption_buttons, icons, label, path_icon, rgb, rgba};
use crate::State;
use rune::{El, col, el, row};

const BG: u32 = 0x202020;
const CARD: u32 = 0x2B2B2B;
const STROKE: u32 = 0x1D1D1D;
const SUB: u32 = 0xCFCFCF;
const LINK: u32 = 0x99EBFF;

fn nav_icon(i: usize) -> El {
    let s = [20.0, 20.0];
    let l: &[(&str, u32, f32)] = match i {
        // Home
        0 => &[
            (
                "M4 9.5 L10 4.5 L16 9.5 V17 a1 1 0 0 1 -1 1 H5 a1 1 0 0 1 -1 -1Z",
                0xBDBDBD,
                0.0,
            ),
            ("M8 12.5 h4 v5.5 h-4z", 0x7E7E7E, 0.0),
            (
                "M10 1.6 L0.9 9.4 L2.4 11 L10 4.6 L17.6 11 L19.1 9.4Z",
                0xE2650C,
                0.0,
            ),
        ],
        // System
        1 => &[
            (
                "M1.5 4.5 a1.5 1.5 0 0 1 1.5 -1.5 h14 a1.5 1.5 0 0 1 1.5 1.5 V14 h-17Z",
                0x29B5D8,
                0.0,
            ),
            (
                "M1.5 13.5 h17 v1.5 a1.5 1.5 0 0 1 -1.5 1.5 h-14 a1.5 1.5 0 0 1 -1.5 -1.5Z",
                0x0A4563,
                0.0,
            ),
            ("M15 15 h1.2 M13 15 h1.2", 0x8FD8EA, 0.6),
        ],
        // Bluetooth
        2 => &[
            (
                "M10 0.5 C15.5 0.5 16 4 16 10 C16 16 15.5 19.5 10 19.5 C4.5 19.5 4 16 4 10 C4 4 4.5 0.5 10 0.5Z",
                0x0A84FC,
                0.0,
            ),
            ("M7 6.8 L13 12.6 L10 15.2 V4.8 L13 7.4 L7 13.2", WHITE, 1.3),
        ],
        // Network & internet
        3 => &[
            ("M10 16.5 L0.8 7.6 C6 2.6 14 2.6 19.2 7.6Z", 0x3CCBF4, 0.0),
            (
                "M10 16.5 L4.4 11.1 C7.6 8.1 12.4 8.1 15.6 11.1Z",
                0x1290DB,
                0.0,
            ),
            (
                "M10 16.5 L7.4 14 C8.9 12.6 11.1 12.6 12.6 14Z",
                0x0C59A3,
                0.0,
            ),
        ],
        // Personalization
        4 => &[
            ("M17.5 2.5 L9 11", 0x9C9C9C, 2.6),
            (
                "M8.2 10.2 L9.8 11.8 C9.6 15.6 6.2 18.2 1.8 18.2 C2.4 14.6 4.4 10.8 8.2 10.2Z",
                0xF4901E,
                0.0,
            ),
        ],
        // Apps
        5 => &[
            ("M2 2 h7 v7 h-7z", 0x9A9A9A, 0.0),
            ("M2 10.5 h7 v7 h-7z", 0x5E5E5E, 0.0),
            ("M10.5 10.5 h7 v7 h-7z", 0x3E3E3E, 0.0),
            ("M14.5 0.6 L19 5.1 L14.5 9.6 L10 5.1Z", 0x2FBAEE, 0.0),
        ],
        // Accounts
        6 => &[
            (
                "M10 1.5 a4.2 4.2 0 1 1 0 8.4 a4.2 4.2 0 1 1 0 -8.4Z",
                0x20C6B3,
                0.0,
            ),
            (
                "M3 14 a2.6 2.6 0 0 1 2.6 -2.6 h8.8 a2.6 2.6 0 0 1 2.6 2.6 v0.8 C17 17.4 14 18.6 10 18.6 C6 18.6 3 17.4 3 14.8Z",
                0x16B2A0,
                0.0,
            ),
        ],
        // Time & language
        7 => &[
            (
                "M12 1.5 a6.8 6.8 0 1 1 0 13.6 a6.8 6.8 0 1 1 0 -13.6Z",
                0x1692C7,
                0.0,
            ),
            (
                "M12 1.5 C16.5 1.5 18.8 5 18.8 8.3 H12Z M5.2 8.3 C5.2 5 7.5 1.5 12 1.5 V8.3Z",
                0x31C8DC,
                0.0,
            ),
            ("M6 9.3 a5 5 0 1 1 0 10 a5 5 0 1 1 0 -10Z", 0xE6E6E6, 0.0),
            ("M6 11.6 V14.3 H8.2", 0x404040, 1.1),
        ],
        // Gaming
        8 => &[
            (
                "M5.5 5 h9 a4.5 4.5 0 0 1 4.5 4.5 v2.5 a3.2 3.2 0 0 1 -5.6 2.1 l-1.1 -1.1 h-4.6 l-1.1 1.1 A3.2 3.2 0 0 1 1 12 V9.5 A4.5 4.5 0 0 1 5.5 5Z",
                0xC4C4C4,
                0.0,
            ),
            ("M4.3 9.6 h3.4 M6 7.9 v3.4", 0x4A4A4A, 1.1),
            (
                "M13.8 8.4 a0.9 0.9 0 1 1 0 1.8 a0.9 0.9 0 1 1 0 -1.8Z M15.6 10.2 a0.9 0.9 0 1 1 0 1.8 a0.9 0.9 0 1 1 0 -1.8Z",
                0x0075CF,
                0.0,
            ),
        ],
        // Accessibility
        9 => &[
            ("M10 1 a2 2 0 1 1 0 4 a2 2 0 1 1 0 -4Z", 0x1292DE, 0.0),
            (
                "M3.5 6.5 L10 8.3 L16.5 6.5 M10 8.3 V12 M10 12 L6.8 18.5 M10 12 L13.2 18.5",
                0x1292DE,
                2.2,
            ),
        ],
        // Privacy & security
        10 => &[
            (
                "M10 1.2 C12.6 3 15 3.8 17.6 3.8 V9.4 C17.6 14 14.6 17.2 10 18.8 C5.4 17.2 2.4 14 2.4 9.4 V3.8 C5 3.8 7.4 3 10 1.2Z",
                0x8E8E8E,
                0.0,
            ),
            (
                "M10 1.2 C7.4 3 5 3.8 2.4 3.8 V9.4 C2.4 14 5.4 17.2 10 18.8Z",
                0xC9C9C9,
                0.0,
            ),
        ],
        // Windows Update
        _ => &[
            ("M10 0 a10 10 0 1 1 0 20 a10 10 0 1 1 0 -20Z", 0x108FDD, 0.0),
            (
                "M5.4 9.2 A4.7 4.7 0 0 1 13.6 6.6 M14.6 10.8 A4.7 4.7 0 0 1 6.4 13.4",
                WHITE,
                1.3,
            ),
            ("M14.6 4.6 V7.4 H11.8 M5.4 15.4 V12.6 H8.2", WHITE, 1.3),
        ],
    };
    art(20.0, 20.0, s, l)
}

const NAV: [&str; 12] = [
    "Home",
    "System",
    "Bluetooth & devices",
    "Network & internet",
    "Personalization",
    "Apps",
    "Accounts",
    "Time & language",
    "Gaming",
    "Accessibility",
    "Privacy & security",
    "Windows Update",
];

fn nav() -> El {
    let mut c = col().absolute(17.0, 144.0).w(280.0).gap(4.0);
    for (i, t) in NAV.iter().enumerate() {
        let sel = i == 1;
        let mut r = row()
            .w_full()
            .h(36.0)
            .items_center()
            .radius(4.0)
            .hover(rgba(WHITE, 0.0605))
            .press(rgba(WHITE, 0.0419))
            .role("button")
            .label(*t)
            .child(nav_icon(i).ml(10.0))
            .child(label(*t, 14.0, 400.0, WHITE).ml(14.0));
        if sel {
            r = r.fill(rgba(WHITE, 0.0512)).child(
                el().size(3.0, 16.0)
                    .radius(1.5)
                    .fill(rgb(ACCENT))
                    .absolute(0.0, 10.0),
            );
        }
        c = c.child(r);
    }
    c
}

fn avatar() -> El {
    el().size(60.0, 60.0)
        .radius(30.0)
        .fill(rgb(0xFAFAFA))
        .clip_corners([30.0; 4])
        .absolute(17.0, 58.0)
        .child(
            art(
                60.0,
                60.0,
                [60.0, 60.0],
                &[
                    ("M30 15 a8.5 8.5 0 1 1 0 17 a8.5 8.5 0 1 1 0 -17Z", 0x8F8F8F, 0.0),
                    ("M17 42.5 a5 5 0 0 1 5 -5 h16 a5 5 0 0 1 5 5 v1.5 C43 46 38 48 30 48 C22 48 17 46 17 44Z", 0x8F8F8F, 0.0),
                ],
            )
            .absolute(0.0, 0.0),
        )
}

fn title_bar() -> El {
    let search = row()
        .absolute(343.0, 9.0)
        .size(516.0, 32.0)
        .radius(16.0)
        .fill(rgb(0x2D2D2D))
        .border(1.0, rgb(0x303030))
        .items_center()
        .child(path_icon(icons::SEARCH, 16.0, 0xEFEFEF).ml(12.0))
        .child(label("Find a setting", 14.0, 400.0, SUB).ml(13.0).mt(-2.0));
    el().w_full()
        .h(48.0)
        .window_drag()
        .child(
            row()
                .absolute(5.0, 9.0)
                .size(40.0, 32.0)
                .center()
                .radius(4.0)
                .hover(rgba(WHITE, 0.0605))
                .child(path_icon(icons::ARROW_LEFT, 16.0, 0xCBCBCB)),
        )
        .child(label("Settings", 12.0, 400.0, WHITE).absolute(52.0, 18.0))
        .child(search)
        .child(caption_buttons(33.0, 0x737373).absolute(1063.0, 0.0))
}

/// Windows 11 "Bloom" wallpaper thumbnail inside the device frame.
fn bloom() -> El {
    art(
        112.0,
        60.0,
        [112.0, 60.0],
        &[
            (
                "M20 60 C16 46 28 38 38 36 C36 24 48 12 60 10 C72 6 84 12 86 22 C96 28 102 44 96 60Z",
                0x0A2BB8,
                0.0,
            ),
            (
                "M24 60 C22 48 32 42 42 41 C42 31 52 21 62 19 C71 17 79 23 80 31 C88 35 91 48 87 60Z",
                0x1747F0,
                0.0,
            ),
            (
                "M26 58 C30 50 40 47 50 50 C46 42 52 34 60 32 C56 26 62 20 70 22",
                0x3C74FF,
                1.6,
            ),
            (
                "M40 60 C42 54 50 52 58 56 M54 42 C62 38 70 40 74 46",
                0x2A5CFF,
                1.4,
            ),
            ("M58 14 C66 12 76 16 80 24", 0x6A98FF, 1.2),
            ("M66 38 C76 36 86 44 86 58", 0x0A1D80, 2.2),
            ("M30 46 C36 40 44 39 50 42", 0x7BA4FF, 1.0),
        ],
    )
}

fn hero() -> El {
    let img = el()
        .size(120.0, 68.0)
        .radius(6.0)
        .fill(rgb(0x000000))
        .absolute(325.0, 124.0)
        .child(
            el().absolute(4.0, 4.0)
                .size(112.0, 60.0)
                .radius(2.0)
                .clip_corners([2.0; 4])
                .radial_gradient(
                    [0.55, 0.35],
                    70.0,
                    [
                        (0.0, rgb(0x123A6A)),
                        (0.6, rgb(0x0A1A33)),
                        (1.0, rgb(0x050A14)),
                    ],
                )
                .child(bloom().absolute(0.0, 0.0)),
        );
    let promo = |x: f32, icon: El, t: &'static str, sub: El| {
        el().absolute(0.0, 0.0)
            .child(icon.absolute(x, 148.0))
            .child(label(t, 14.0, 600.0, WHITE).absolute(x + 36.0, 138.0))
            .child(sub.absolute(x + 36.0, 161.0))
    };
    let ms = art(
        20.0,
        20.0,
        [20.0, 20.0],
        &[
            ("M0 0 h9.5 v9.5 h-9.5z", 0xF25022, 0.0),
            ("M10.5 0 h9.5 v9.5 h-9.5z", 0x7FBA00, 0.0),
            ("M0 10.5 h9.5 v9.5 h-9.5z", 0x00A4EF, 0.0),
            ("M10.5 10.5 h9.5 v9.5 h-9.5z", 0xFFB900, 0.0),
        ],
    );
    let gray = 0xA6A6A6;
    el().absolute(0.0, 0.0)
        .w_full()
        .h(200.0)
        .child(img)
        .child(label("Plumbum", 18.0, 600.0, WHITE).absolute(461.0, 130.0))
        .child(label("System Product Name", 14.0, 400.0, WHITE).absolute(461.0, 154.0))
        .child(
            label("Rename", 14.0, 400.0, LINK)
                .absolute(462.0, 173.0)
                .role("link"),
        )
        .child(promo(
            874.0,
            ms,
            "Microsoft 365",
            label("View benefits", 12.0, 400.0, gray),
        ))
        .child(promo(
            1023.0,
            nav_icon(11),
            "Windows Update",
            row()
                .items_center()
                .child(el().size(3.0, 3.0).radius(1.5).fill(rgb(ACCENT)).ml(1.0))
                .child(label("Attention needed", 12.0, 400.0, gray).ml(6.0)),
        ))
}

const ROWS: [(&str, &str, &str); 10] = [
    (
        "Display",
        "Monitors, brightness, night light, display profile",
        icons::LAPTOP,
    ),
    (
        "Sound",
        "Volume levels, output, input, sound devices",
        icons::SPEAKER_2,
    ),
    (
        "Notifications",
        "Alerts from apps and system, do not disturb",
        icons::ALERT,
    ),
    ("Focus", "Reduce distractions", icons::TARGET),
    (
        "Power",
        "Screen and sleep, power mode, energy saver",
        icons::POWER,
    ),
    (
        "Storage",
        "Storage space, drives, configuration rules",
        icons::STORAGE,
    ),
    ("Share", "Share files, photos, links and more", icons::SHARE),
    (
        "Multitasking",
        "Snap windows, desktops, task switching",
        icons::SQUARE_MULTIPLE,
    ),
    (
        "Advanced",
        "Performance, optimization, and developer features",
        icons::WRENCH_SCREWDRIVER,
    ),
    (
        "Activation",
        "Activation state, subscriptions, product key",
        icons::CHECKMARK_CIRCLE,
    ),
];

fn card(t: &'static str, sub: &'static str, icon: &'static str) -> El {
    row()
        .w_full()
        .h(69.0)
        .radius(4.0)
        .fill(rgb(CARD))
        .border(1.0, rgb(STROKE))
        .hover(rgb(0x323232))
        .press(rgb(0x272727))
        .items_center()
        .role("button")
        .label(t)
        .child(path_icon(icon, 25.0, WHITE).ml(15.5))
        .child(
            col()
                .ml(18.5)
                .child(label(t, 14.0, 400.0, WHITE))
                .child(label(sub, 12.0, 400.0, SUB).mt(1.0)),
        )
        .child(rune::spacer())
        .child(path_icon(icons::CHEVRON_RIGHT, 18.0, 0xE8E8E8).mr(16.0))
}

pub fn view(_s: &State) -> El {
    let mut cards = col().absolute(325.0, 223.0).w(852.0).gap(3.0);
    for (t, sub, i) in ROWS {
        cards = cards.child(card(t, sub, i));
    }
    el().w_full()
        .h_full()
        .fill(rgb(BG))
        .child(title_bar())
        .child(avatar())
        .child(label("Coobyk", 14.0, 600.0, WHITE).absolute(89.0, 72.0))
        .child(label("Local Account", 12.0, 400.0, WHITE).absolute(89.0, 91.0))
        .child(nav())
        .child(label("System", 28.0, 600.0, WHITE).absolute(325.0, 56.0))
        .child(hero())
        .child(cards)
}
