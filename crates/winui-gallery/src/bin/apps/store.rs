//! Microsoft Store — Updates & downloads page replica.

use crate::chrome::{self, hex};
use crate::fluent::{c, font, icon};
use rune::{canvas, col, el, row, text, El, ScrollSpec, Shape, TextStyle};
use winui_icons::Symbol;
use winui_theme::{Color, Theme, radii, weights};

fn store_logo() -> El {
    // Shopping bag with the four-color Windows mark.
    let s = Shape::new()
        .fill(
            [[1.5, 4.0], [10.5, 4.0], [12.0, 14.0], [0.0, 14.0]],
            c(hex(0xFFFFFF)),
        )
        .fill([[2.2, 5.5], [5.5, 5.5], [5.5, 9.0], [2.2, 9.0]], c(hex(0xF25022)))
        .fill([[6.5, 5.5], [9.8, 5.5], [10.2, 9.0], [6.5, 9.0]], c(hex(0x7FBA00)))
        .fill([[2.2, 10.0], [5.5, 10.0], [5.5, 13.5], [1.8, 13.5]], c(hex(0x00A4EF)))
        .fill([[6.5, 10.0], [10.2, 10.0], [10.6, 13.5], [6.5, 13.5]], c(hex(0xFFB900)));
    canvas(s).size(12.0, 14.0)
}

fn search_box(t: &Theme) -> El {
    let k = t.tokens();
    row()
        .w(440.0)
        .h(30.0)
        .items_center()
        .px(14.0)
        .radius(6.0)
        .fill(c(hex(0x3A3A3A)))
        .border(1.0, c(k.control_stroke_color_default))
        .child(
            text("Search apps, games, and more")
                .font(font(t.type_ramp().body))
                .color(c(k.text_fill_color_secondary))
                .grow(1.0)
                .min_w(0.0),
        )
        .child(icon(Symbol::Find, 14.0, k.text_fill_color_secondary))
        .window_drag()
}

fn rail_item(t: &Theme, sym: Symbol, label: &str, selected: bool) -> El {
    let k = t.tokens();
    let mut r = col()
        .w_full()
        .h(56.0)
        .center()
        .gap(4.0)
        .radius(radii::CONTROL)
        .hover(c(k.subtle_fill_color_secondary));
    if selected {
        r = r.fill(c(k.subtle_fill_color_secondary)).child(
            el().size(3.0, 16.0)
                .radius(2.0)
                .fill(c(k.accent_fill_color_default))
                .absolute(0.0, 20.0),
        );
    }
    r.child(icon(sym, 18.0, k.text_fill_color_primary)).child(
        text(label)
            .font(TextStyle::new(10.5, weights::REGULAR as f32))
            .w_full().truncate()
            .color(c(if selected {
                k.text_fill_color_primary
            } else {
                k.text_fill_color_secondary
            })),
    )
}

fn app_icon(sym: Symbol, bg: u32, fg: Color) -> El {
    el().size(40.0, 40.0)
        .radius(8.0)
        .fill(c(hex(bg)))
        .center()
        .child(icon(sym, 22.0, fg))
}

fn update_row(
    t: &Theme,
    icon_el: El,
    name: &str,
    publisher: &str,
    category: &str,
    action: &str,
) -> El {
    let k = t.tokens();
    chrome::card(t).h(56.0).clip_rounded(radii::OVERLAY).child(
        row().w_full().h_full().items_center().px(12.0)
            .child(icon_el)
            .child(
                col().w(250.0).pl(10.0).gap(2.0)
                    .child(
                        text(name)
                            .font(font(t.type_ramp().body))
                            .color(c(k.text_fill_color_primary)),
                    )
                    .child(
                        text(publisher)
                            .font(font(t.type_ramp().caption))
                            .color(c(k.text_fill_color_secondary)),
                    ),
            )
            .child(
                el().w(200.0).child(
                    text(category)
                        .font(font(t.type_ramp().body))
                        .color(c(k.text_fill_color_primary)),
                ),
            )
            .child(
                el().grow(1.0).min_w(0.0).child(
                    text("Modified yesterday")
                        .font(font(t.type_ramp().body))
                        .color(c(k.text_fill_color_primary)),
                ),
            )
            .child(
                row().h(30.0).items_center().px(14.0).radius(radii::CONTROL)
                    .fill(c(k.control_fill_color_default))
                    .border(1.0, c(k.control_stroke_color_default))
                    .hover(c(k.control_fill_color_secondary))
                    .child(text(action).font(font(t.type_ramp().body)).color(c(k.text_fill_color_primary))),
            )
            .child(
                el().size(36.0, 36.0)
                    .center()
                    .ml(8.0)
                    .child(icon(Symbol::MoreHorizontal, 16.0, k.text_fill_color_primary)),
            ),
    )
}

pub fn view(t: &Theme) -> El {
    let k = t.tokens();

    let rail = col()
        .w(68.0)
        .h_full()
        .min_h(0.0)
        .pt(8.0)
        .px(4.0)
        .child(rail_item(t, Symbol::Home, "Home", false))
        .child(rail_item(t, Symbol::Grid, "Apps", false))
        .child(rail_item(t, Symbol::Games, "Gaming", false))
        .child(rail_item(t, Symbol::Sparkle, "AI Hub", false))
        .child(rail_item(t, Symbol::PaintBrush, "Themes", false))
        .child(el().grow(1.0).min_h(0.0))
        .child(rail_item(t, Symbol::Send, "What's New", false))
        .child(rail_item(t, Symbol::Library, "Library", true))
        .fill(c(hex(0x181818)));

    let rows: &[(&str, &str, &str, &str, Symbol, u32)] = &[
        ("Outlook for Windows", "Microsoft Corporation", "Apps", "Open", Symbol::Mail, 0x0F6CBD),
        ("Raw Image Extension", "Microsoft Windows", "System Components", "More info", Symbol::Camera, 0x0078D4),
        ("App Installer", "Microsoft Windows", "System Components", "More info", Symbol::Grid, 0x0078D4),
        ("HEIF Image Extension", "Microsoft Windows", "System Components", "More info", Symbol::Image, 0x0078D4),
        ("VP9 Video Extensions", "Microsoft Windows", "System Components", "More info", Symbol::Image, 0x0078D4),
        ("Telegram for Windows (Unigram)", "Unigram, Inc.", "Apps", "Open", Symbol::Send, 0x0098FF),
        ("WebP Image Extension", "Microsoft Windows", "System Components", "More info", Symbol::Image, 0x0078D4),
        ("Paint", "Microsoft Corporation", "Apps", "Open", Symbol::Color, 0x313131),
        ("Game Bar", "Microsoft Windows", "System Components", "More info", Symbol::Games, 0x107C10),
        ("Snipping Tool", "Microsoft Corporation", "Apps", "Open", Symbol::Cut, 0x0078D4),
    ];
    let mut list = col().w_full().gap(4.0);
    for (name, publisher, cat, action, sym, bg) in rows {
        list = list.child(update_row(
            t,
            app_icon(*sym, *bg, hex(0xFFFFFF)),
            name,
            publisher,
            cat,
            action,
        ));
    }

    let page = col()
        .w_full()
        .grow(1.0)
        .min_h(0.0)
        .scroll(ScrollSpec::default())
        .child(
            col()
                .w_full()
                .max_w(900.0)
                .px(36.0)
                .pb(32.0)
                .child(
                    row().w_full().h(96.0).items_end().pb(16.0)
                        .child(
                            text("Updates & downloads")
                                .font(TextStyle::new(24.0, weights::SEMIBOLD as f32))
                                .color(c(k.text_fill_color_primary))
                                .grow(1.0)
                                .min_w(0.0),
                        )
                        .child(
                            row().h(32.0).items_center().px(14.0).radius(radii::CONTROL)
                                .fill(c(k.accent_fill_color_default))
                                .hover(c(k.accent_fill_color_secondary))
                                .child(
                                    text("Check for updates")
                                        .font(font(t.type_ramp().body))
                                        .color(c(k.text_on_accent_fill_color_primary)),
                                ),
                        ),
                )
                .child(
                    text("Recent updates")
                        .font(TextStyle::new(16.0, weights::SEMIBOLD as f32))
                        .color(c(k.text_fill_color_primary))
                        .pb(8.0),
                )
                .child(list),
        );

    let title = row()
        .w_full()
        .h(44.0)
        .items_center()
        .pl(10.0)
        .window_drag()
        .child(store_logo())
        .child(chrome::caption(t, "Microsoft Store", k.text_fill_color_primary).ml(8.0))
        .child(el().grow(1.0).min_w(0.0))
        .child(search_box(t))
        .child(el().grow(1.0).min_w(0.0))
        .child(icon(Symbol::Person, 16.0, k.text_fill_color_primary))
        .child(chrome::captions(t));

    let content = row()
        .w_full()
        .h_full()
        .child(rail)
        .child(el().grow(1.0).min_w(0.0).h_full().child(page));

    chrome::shell(t, title, content)
}
