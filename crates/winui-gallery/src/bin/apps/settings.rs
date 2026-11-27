//! Windows 11 Settings — System page replica.

use crate::chrome::{self, hex};
use crate::fluent::{c, font, icon};
use rune::{canvas, col, el, row, text, El, ScrollSpec, Shape};
use winui_icons::Symbol;
use winui_theme::{Color, Theme, radii};

const BLUE: u32 = 0x4CC2FF;

fn search_box(t: &Theme) -> El {
    let k = t.tokens();
    row()
        .w(390.0)
        .h(32.0)
        .items_center()
        .pl(14.0)
        .radius(16.0)
        .fill(c(k.control_fill_color_default))
        .border(1.0, c(k.control_stroke_color_default))
        .child(icon(Symbol::Find, 14.0, k.text_fill_color_secondary))
        .child(
            text("Find a setting")
                .font(font(t.type_ramp().body))
                .color(c(k.text_fill_color_secondary))
                .ml(10.0),
        )
        .window_drag()
}

fn avatar(t: &Theme) -> El {
    let k = t.tokens();
    row()
        .w_full()
        .h(96.0)
        .items_center()
        .px(16.0)
        .child(
            el().size(60.0, 60.0)
                .radius(30.0)
                .fill(c(hex(0x6E6E6E)))
                .center()
                .child(icon(Symbol::Person, 34.0, hex(0xBFBFBF))),
        )
        .child(
            col().pl(16.0).gap(2.0)
                .child(
                    text("Coobyk")
                        .font(font(t.type_ramp().body_strong))
                        .color(c(k.text_fill_color_primary)),
                )
                .child(chrome::caption(t, "Local Account", k.text_fill_color_secondary)),
        )
}

fn ms365() -> El {
    let sq = |s: Shape, x: f32, y: f32, col: u32| {
        s.fill(
            [[x, y], [x + 6.0, y], [x + 6.0, y + 6.0], [x, y + 6.0]],
            c(hex(col)),
        )
    };
    let s = sq(
        sq(
            sq(
                sq(Shape::new(), 0.0, 0.0, 0xF25022),
                8.0,
                0.0,
                0x7FBA00,
            ),
            0.0,
            8.0,
            0x00A4EF,
        ),
        8.0,
        8.0,
        0xFFB900,
    );
    canvas(s).size(14.0, 14.0)
}

fn hero(t: &Theme) -> El {
    let k = t.tokens();
    chrome::card(t).h(108.0).clip_rounded(radii::OVERLAY).child(
        row().w_full().h_full().items_center().px(16.0).child(
            // Windows "bloom" thumbnail approximation.
            el().size(98.0, 66.0)
                .radius(6.0)
                .linear_gradient(
                    std::f32::consts::PI * 0.8,
                    [(0.0, c(hex(0x0B3B8F))), (1.0, c(hex(0x154F9E)))],
                )
                .child(
                    el().size(44.0, 44.0)
                        .radius(22.0)
                        .fill(c(hex(0x3B78D6)))
                        .opacity(0.55)
                        .pin(Some(27.0), Some(11.0), None, None),
                ),
        )
        .child(
            col().pl(16.0).gap(3.0).grow(1.0).min_w(0.0)
                .child(
                    text("Plumbum")
                        .font(font(t.type_ramp().body_strong))
                        .color(c(k.text_fill_color_primary)),
                )
                .child(chrome::caption(t, "System Product Name", k.text_fill_color_secondary))
                .child(chrome::caption(t, "Rename", k.accent_text_fill_color_primary)),
        )
        .child(
            col().w(160.0).gap(4.0)
                .child(row().items_center().gap(8.0).child(ms365()).child(
                    text("Microsoft 365")
                        .font(font(t.type_ramp().body_strong))
                        .color(c(k.text_fill_color_primary)),
                ))
                .child(chrome::caption(t, "View benefits", k.text_fill_color_secondary).pl(22.0)),
        )
        .child(
            col().w(170.0).gap(4.0)
                .child(row().items_center().gap(8.0)
                    .child(icon(Symbol::ArrowSync, 16.0, hex(BLUE)))
                    .child(
                        text("Windows Update")
                            .font(font(t.type_ramp().body_strong))
                            .color(c(k.text_fill_color_primary)),
                    ))
                .child(
                    row().pl(24.0).items_center().gap(6.0)
                        .child(el().size(5.0, 5.0).radius(2.5).fill(c(hex(0xFCE100))))
                        .child(chrome::caption(t, "Attention needed", k.text_fill_color_secondary)),
                ),
        ),
    )
}

fn sys_card(t: &Theme, sym: Symbol, tint: Color, title: &str, desc: &str) -> El {
    let k = t.tokens();
    chrome::card(t).child(
        row()
            .w_full()
            .h(70.0)
            .items_center()
            .px(16.0)
            .child(el().size(40.0, 40.0).center().child(icon(sym, 22.0, tint)))
            .child(
                col().grow(1.0).min_w(0.0).pl(8.0).gap(3.0)
                    .child(
                        text(title)
                            .font(font(t.type_ramp().body))
                            .color(c(k.text_fill_color_primary)),
                    )
                    .child(
                        text(desc)
                            .font(font(t.type_ramp().caption))
                            .color(c(k.text_fill_color_secondary)),
                    ),
            )
            .child(chrome::chevron(t)),
    )
}

pub fn view(t: &Theme) -> El {
    let k = t.tokens();
    let nav_items = [
        (Symbol::Home, "Home", None, false),
        (Symbol::Desktop, "System", Some(hex(BLUE)), true),
        (Symbol::Bluetooth, "Bluetooth & devices", Some(hex(0x0078D7)), false),
        (Symbol::WiFi, "Network & internet", Some(hex(0x0078D7)), false),
        (Symbol::PaintBrush, "Personalization", Some(hex(0xE3008C)), false),
        (Symbol::Grid, "Apps", Some(hex(0x0078D7)), false),
        (Symbol::Person, "Accounts", Some(hex(0x00B7C3)), false),
        (Symbol::Clock, "Time & language", Some(hex(0x0078D7)), false),
        (Symbol::Games, "Gaming", Some(hex(0xBAD80A)), false),
        (Symbol::Accessibility, "Accessibility", Some(hex(0x0078D7)), false),
        (Symbol::Shield, "Privacy & security", Some(hex(0x0078D7)), false),
        (Symbol::ArrowSync, "Windows Update", Some(hex(0x0078D7)), false),
    ];

    let mut navlist = col().w_full().px(4.0).gap(2.0);
    for (s, l, tint, sel) in nav_items {
        navlist = navlist.child(chrome::nav_row(t, s, l, tint, sel));
    }
    let pane = col()
        .w(288.0)
        .h_full()
        .min_h(0.0)
        .child(avatar(t))
        .child(
            col().w_full().grow(1.0).min_h(0.0).scroll(ScrollSpec::default()).child(navlist),
        );

    let cards: &[(Symbol, u32, &str, &str)] = &[
        (Symbol::Desktop, BLUE, "Display", "Monitors, brightness, night light, display profile"),
        (Symbol::Volume, BLUE, "Sound", "Volume levels, output, input, sound devices"),
        (Symbol::Alert, BLUE, "Notifications", "Alerts from apps and system, do not disturb"),
        (Symbol::Clock, BLUE, "Focus", "Reduce distractions"),
        (Symbol::Power, BLUE, "Power", "Screen and sleep, power mode, energy saver"),
        (Symbol::Database, BLUE, "Storage", "Storage space, drives, configuration rules"),
        (Symbol::Share, BLUE, "Share", "Share files, photos, links and more"),
        (Symbol::SquareMultiple, BLUE, "Multitasking", "Snap windows, desktops, task switching"),
        (Symbol::Options, BLUE, "Advanced", "Performance, optimization, and developer features"),
        (Symbol::CheckmarkCircle, BLUE, "Activation", "Activation state, subscriptions, product key"),
    ];
    let mut list = col().w_full().gap(4.0);
    for (s, tint, title, desc) in cards {
        list = list.child(sys_card(t, *s, hex(*tint), title, desc));
    }

    let page = col()
        .w_full()
        .grow(1.0)
        .min_h(0.0)
        .scroll(ScrollSpec::default())
        .child(
            col()
                .w_full()
                .max_w(760.0)
                .px(40.0)
                .pb(32.0)
                .child(
                    col().h(88.0).w_full().justify_end().pb(12.0).child(
                        text("System")
                            .font(font(t.type_ramp().title))
                            .color(c(k.text_fill_color_primary)),
                    ),
                )
                .child(hero(t))
                .child(el().h(12.0))
                .child(list),
        );

    let title = row()
        .w_full()
        .h(44.0)
        .items_center()
        .pl(12.0)
        .window_drag()
        .child(icon(Symbol::ChevronLeft, 12.0, k.text_fill_color_primary))
        .child(
            chrome::caption(t, "Settings", k.text_fill_color_primary).ml(10.0),
        )
        .child(el().grow(1.0).min_w(0.0))
        .child(search_box(t))
        .child(el().grow(1.0).min_w(0.0))
        .child(chrome::captions(t));

    let content = row()
        .w_full()
        .h_full()
        .child(pane)
        .child(el().grow(1.0).min_w(0.0).h_full().child(page));

    chrome::shell(t, title, content)
}
