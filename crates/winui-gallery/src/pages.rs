//! One page per control family, laid out like WinUI 3 Gallery's ControlExample cards.

use crate::controls::*;
use crate::fluent::{c, font, icon};
use crate::{Page, State};
use rune::{El, col, el, row, text};
use winui_icons::Symbol;
use winui_theme::{Theme, radii};

/// Gallery `ControlExample`: BodyStrong header, then a bordered card (radius 8)
/// with the sample on `CardBackgroundFillColorDefault`.
pub fn example(t: &Theme, header: &str, sample: El) -> El {
    let k = t.tokens();
    col()
        .w_full()
        .gap(12.0)
        .mb(24.0)
        .child(
            text(header)
                .font(font(t.type_ramp().body_strong))
                .color(c(k.text_fill_color_primary)),
        )
        .child(
            row()
                .w_full()
                .min_h(96.0)
                .items_center()
                .padding(16.0)
                .radius(radii::OVERLAY)
                .fill(c(k.card_background_fill_color_default))
                .border(1.0, c(k.card_stroke_color_default))
                .child(sample),
        )
}

fn header(t: &Theme, title: &str, desc: &str) -> El {
    let k = t.tokens();
    col()
        .w_full()
        .gap(8.0)
        .mb(24.0)
        .child(
            text(title)
                .font(font(t.type_ramp().title))
                .color(c(k.text_fill_color_primary)),
        )
        .child(
            text(desc)
                .font(font(t.type_ramp().body))
                .color(c(k.text_fill_color_secondary))
                .wrap(),
        )
}

fn hrow() -> El {
    row().gap(8.0).items_center().flex_wrap()
}

pub fn page(s: &State, t: &Theme) -> El {
    let p = s.page;
    let body = match p {
        Page::Home => return home(t),
        Page::Buttons => col().w_full()
            .child(header(t, "Buttons", "Buttons give the user a way to trigger an immediate action. Some buttons are specialized for particular tasks, such as navigation, repeated actions, or presenting menus."))
            .child(example(t, "A simple Button with text content.", hrow().child(button(t, "Standard XAML button", false, true)).child(button(t, "Disabled", false, false))))
            .child(example(t, "Accent style applied to Button.", hrow().child(button(t, "Accent style button", true, true)).child(button(t, "Disabled", true, false))))
            .child(example(t, "A HyperlinkButton, ToggleButton and RepeatButton.", hrow().child(hyperlink(t, "Microsoft home page")).child(toggle_button(t, "ToggleButton", s.toggle)).child(button(t, "Click and hold", false, true))))
            .child(example(t, "DropDownButton and SplitButton.", hrow().child(dropdown_button(t, "Email")).child(split_button(t, "Choose color"))))
            .child(example(t, "Buttons with icons.", hrow().child(icon_button(t, Symbol::Save, "Save")).child(icon_button(t, Symbol::Share, "Share")).child(icon_button(t, Symbol::Delete, "Delete")))),
        Page::Input => col().w_full()
            .child(header(t, "Basic input", "Check boxes, radio buttons, toggle switches, sliders, rating and text input."))
            .child(example(t, "A 3-state CheckBox.", hrow().gap(24.0).child(checkbox(t, "Unchecked", Some(false), true)).child(checkbox(t, "Checked", Some(true), true)).child(checkbox(t, "Indeterminate", None, true)).child(checkbox(t, "Disabled", Some(true), false))))
            .child(example(t, "A group of RadioButton controls.", col().child(text("Options").font(font(t.type_ramp().body)).color(c(t.tokens().text_fill_color_primary))).child(radio(t, "Option 1", true)).child(radio(t, "Option 2", false)).child(radio(t, "Option 3", false))))
            .child(example(t, "A simple ToggleSwitch.", hrow().gap(48.0).child(toggle_switch(t, s.toggle, Some("Toggle work"))).child(toggle_switch(t, !s.toggle, Some("Notifications")))))
            .child(example(t, "A simple Slider and RatingControl.", hrow().gap(48.0).child(slider(t, s.slider, 240.0)).child(rating(t, 3))))
            .child(example(t, "TextBox, PasswordBox, NumberBox and AutoSuggestBox.", col().gap(12.0)
                .child(hrow().child(text_box(t, "", "Enter your name", 240.0, true, false)).child(text_box(t, "Focused text", "", 240.0, true, true)).child(text_box(t, "", "Disabled", 200.0, false, false)))
                .child(hrow().child(password_box(t, 240.0)).child(number_box(t, "42", 160.0)).child(auto_suggest(t, "Search", 240.0)))))
            .child(example(t, "ColorPicker (swatch grid).", swatch_grid(t, &crate::ACCENTS.iter().map(|a| a.1).collect::<Vec<_>>(), s.accent))),
        Page::Collections => col().w_full()
            .child(header(t, "Collections", "ListView, item templates and expanders present collections of items."))
            .child(example(t, "A ListView with selection.", col().w(360.0)
                .child(list_row(t, "Kendall Collins", Some("Program manager"), true))
                .child(list_row(t, "Henry Ross", Some("Developer"), false))
                .child(list_row(t, "Vance DeLeon", Some("Designer"), false))))
            .child(example(t, "An Expander with content.", hrow().items_start().gap(24.0)
                .child(expander(t, "This text is in the header", true, text("This is in the content").font(font(t.type_ramp().body)).color(c(t.tokens().text_fill_color_primary))))
                .child(expander(t, "Collapsed expander", false, el())))),
        Page::Menus => col().w_full()
            .child(header(t, "Menus & toolbars", "ComboBox, MenuFlyout and CommandBar present commands and options."))
            .child(example(t, "A ComboBox with items.", hrow().child(combo_box(t, "Blue", 200.0)).child(combo_box(t, "Pick a font", 200.0))))
            .child(example(t, "A MenuFlyout (open).", menu_flyout(t, &[(Some(Symbol::Cut), "Cut", "Ctrl+X"), (Some(Symbol::Copy), "Copy", "Ctrl+C"), (Some(Symbol::Paste), "Paste", "Ctrl+V"), (None, "", ""), (Some(Symbol::Rename), "Rename", "F2"), (Some(Symbol::Delete), "Delete", "Del")])))
            .child(example(t, "A CommandBar.", command_bar(t, &[(Symbol::Add, "Add"), (Symbol::Edit, "Edit"), (Symbol::Share, "Share")]))),
        Page::Navigation => col().w_full()
            .child(header(t, "Navigation", "NavigationView, TabView and BreadcrumbBar move users between pages and documents."))
            .child(example(t, "A TabView with close buttons and an add button.", tab_view(t, &["Document 0", "Document 1", "Document 2"], 0)))
            .child(example(t, "A BreadcrumbBar.", breadcrumb(t, &["Home", "Documents", "Design", "Northwind", "Images"])))
            .child(example(t, "This app's shell is a NavigationView in Left mode.", text("See the pane on the left.").font(font(t.type_ramp().body)).color(c(t.tokens().text_fill_color_secondary)))),
        Page::Status => col().w_full()
            .child(header(t, "Status & info", "InfoBar, ProgressBar, ProgressRing, ToolTip, TeachingTip and InfoBadge communicate status."))
            .child(example(t, "InfoBar severities.", col().w_full().gap(8.0)
                .child(info_bar(t, Severity::Info, "Title", "Essential app message for your users to be informed of, acknowledge, or take action on."))
                .child(info_bar(t, Severity::Success, "Success", "The operation completed successfully."))
                .child(info_bar(t, Severity::Warning, "Warning", "Your trial expires in 3 days."))
                .child(info_bar(t, Severity::Error, "Error", "The file could not be saved."))))
            .child(example(t, "Determinate and indeterminate ProgressBar and ProgressRing.", hrow().gap(32.0)
                .child(progress_bar(t, Some(0.6), 200.0)).child(progress_bar(t, None, 200.0)).child(progress_ring(t, Some(0.75), 32.0)).child(progress_ring(t, None, 32.0))))
            .child(example(t, "ToolTip, TeachingTip and InfoBadge.", hrow().gap(32.0).items_start()
                .child(tooltip(t, "Simple ToolTip")).child(teaching_tip(t, "Saving automatically", "We save your changes as you go - so you never have to."))
                .child(row().gap(12.0).items_center().child(info_badge(t, Some(8))).child(info_badge(t, None)).child(person(t, "Jane Doe", 64.0))))),
        Page::Media => col().w_full()
            .child(header(t, "Media", "Image, PersonPicture and media transport controls."))
            .child(example(t, "PersonPicture with initials.", hrow().gap(16.0).child(person(t, "Jane Doe", 96.0)).child(person(t, "Kendall Collins", 64.0)).child(person(t, "Henry Ross", 32.0))))
            .child(example(t, "MediaTransportControls (visual).", media_controls(t))),
        Page::Icons => icons_page(s, t),
        Page::Colors => colors_page(t),
        Page::Type => type_page(t),
        Page::Settings => settings_page(s, t),
    };
    body
}

fn media_controls(t: &Theme) -> El {
    let k = t.tokens();
    let b = |s| {
        el().size(40.0, 40.0)
            .center()
            .radius(radii::CONTROL)
            .hover(c(k.subtle_fill_color_secondary))
            .child(icon(s, 20.0, k.text_fill_color_primary))
    };
    col()
        .w(480.0)
        .gap(8.0)
        .child(progress_bar(t, Some(0.35), 480.0))
        .child(
            row()
                .w_full()
                .justify_center()
                .gap(8.0)
                .child(b(Symbol::Previous))
                .child(b(Symbol::Rewind))
                .child(b(Symbol::Play))
                .child(b(Symbol::FastForward))
                .child(b(Symbol::Next))
                .child(b(Symbol::Volume)),
        )
}

fn home(t: &Theme) -> El {
    let k = t.tokens();
    let hero = col()
        .w_full()
        .h(280.0)
        .px(36.0)
        .pt(36.0)
        .gap(4.0)
        .radius(radii::OVERLAY)
        .linear_gradient(
            std::f32::consts::PI * 0.75,
            [
                (0.0, c(t.accent().light2.with_alpha(0.35))),
                (1.0, c(t.accent().dark1.with_alpha(0.15))),
            ],
        )
        .child(
            text("WinUI 3 Gallery")
                .font(font(t.type_ramp().title_large))
                .color(c(k.text_fill_color_primary)),
        )
        .child(
            text("Windows App SDK on Linux, rendered by Rune")
                .font(font(t.type_ramp().subtitle.with_weight(400)))
                .color(c(k.text_fill_color_secondary)),
        );
    let tile = |sym: Symbol, title: &str, desc: &str| {
        col()
            .w(198.0)
            .h(180.0)
            .padding(24.0)
            .gap(8.0)
            .radius(radii::OVERLAY)
            .fill(c(k.card_background_fill_color_default))
            .hover(c(k.card_background_fill_color_secondary))
            .border(1.0, c(k.card_stroke_color_default))
            .child(icon(sym, 32.0, k.accent_text_fill_color_primary).mb(12.0))
            .child(
                text(title)
                    .font(font(t.type_ramp().body_strong))
                    .color(c(k.text_fill_color_primary)),
            )
            .child(
                text(desc)
                    .font(font(t.type_ramp().caption))
                    .color(c(k.text_fill_color_secondary))
                    .wrap(),
            )
    };
    col()
        .w_full()
        .child(hero)
        .child(
            text("Recently added samples")
                .font(font(t.type_ramp().subtitle))
                .color(c(k.text_fill_color_primary))
                .mt(24.0)
                .mb(12.0),
        )
        .child(
            row()
                .w_full()
                .gap(12.0)
                .flex_wrap()
                .child(tile(
                    Symbol::CursorClick,
                    "Button",
                    "A control that responds to user input and raises a Click event.",
                ))
                .child(tile(
                    Symbol::ToggleRight,
                    "ToggleSwitch",
                    "A switch that can be toggled between 2 states.",
                ))
                .child(tile(
                    Symbol::Info,
                    "InfoBar",
                    "An inline message for essential app-wide messages.",
                ))
                .child(tile(
                    Symbol::Tab,
                    "TabView",
                    "A control that displays a collection of tabs.",
                ))
                .child(tile(
                    Symbol::Navigation,
                    "NavigationView",
                    "Top-level navigation for your app.",
                ))
                .child(tile(
                    Symbol::Color,
                    "Color",
                    "Fluent colour tokens for light and dark themes.",
                ))
                .child(tile(
                    Symbol::TextFont,
                    "Typography",
                    "The Windows type ramp.",
                ))
                .child(tile(Symbol::Icons, "Iconography", "Fluent System Icons.")),
        )
}

fn icons_page(s: &State, t: &Theme) -> El {
    let k = t.tokens();
    let q = s.search.to_lowercase();
    let mut g = row().w_full().flex_wrap().gap(8.0);
    for sym in winui_icons::Symbol::ALL
        .iter()
        .filter(|x| q.is_empty() || x.name().to_lowercase().contains(&q))
    {
        g = g.child(
            col()
                .w(96.0)
                .h(96.0)
                .items_center()
                .justify_center()
                .gap(12.0)
                .radius(radii::OVERLAY)
                .fill(c(k.card_background_fill_color_default))
                .hover(c(k.card_background_fill_color_secondary))
                .border(1.0, c(k.card_stroke_color_default))
                .child(icon(*sym, 24.0, k.text_fill_color_primary))
                .child(
                    text(sym.name())
                        .font(font(t.type_ramp().caption))
                        .color(c(k.text_fill_color_secondary))
                        .truncate()
                        .max_w(88.0),
                ),
        );
    }
    col()
        .w_full()
        .child(header(
            t,
            "Iconography",
            &format!(
                "{} Fluent System Icons (MIT, microsoft/fluentui-system-icons).",
                winui_icons::Symbol::ALL.len()
            ),
        ))
        .child(g)
}

fn colors_page(t: &Theme) -> El {
    let k = t.tokens();
    let mut g = row().w_full().flex_wrap().gap(12.0);
    for (name, col_) in k.named() {
        g = g.child(
            col()
                .w(212.0)
                .radius(radii::OVERLAY)
                .border(1.0, c(k.card_stroke_color_default))
                .clip_rounded(radii::OVERLAY)
                .child(
                    el().w_full()
                        .h(64.0)
                        .fill(c(k.solid_background_fill_color_base))
                        .child(el().w_full().h_full().fill(c(col_))),
                )
                .child(
                    col()
                        .w_full()
                        .padding(8.0)
                        .fill(c(k.card_background_fill_color_default))
                        .child(
                            text(name)
                                .font(font(t.type_ramp().caption.with_weight(600)))
                                .color(c(k.text_fill_color_primary))
                                .truncate(),
                        )
                        .child(
                            text(format!("#{:08X}", col_.to_argb()))
                                .font(font(t.type_ramp().caption))
                                .color(c(k.text_fill_color_secondary)),
                        ),
                ),
        );
    }
    let mut ramp = row()
        .w_full()
        .h(64.0)
        .radius(radii::OVERLAY)
        .clip_rounded(radii::OVERLAY)
        .mb(24.0);
    for (i, a) in t.accent().steps().iter().enumerate() {
        let names = [
            "Dark 3", "Dark 2", "Dark 1", "Accent", "Light 1", "Light 2", "Light 3",
        ];
        let fg = if i < 4 {
            winui_theme::Color::WHITE
        } else {
            winui_theme::Color::BLACK
        };
        ramp = ramp.child(
            col()
                .grow(1.0)
                .h_full()
                .fill(c(*a))
                .padding(8.0)
                .justify_end()
                .child(
                    text(names[i])
                        .font(font(t.type_ramp().caption))
                        .color(c(fg)),
                ),
        );
    }
    col().w_full().child(header(t, "Color", "Theme brushes from microsoft-ui-xaml Common_themeresources_any.xaml, resolved for the current theme.")).child(ramp).child(g)
}

fn type_page(t: &Theme) -> El {
    let k = t.tokens();
    let mut c0 = col()
        .w_full()
        .radius(radii::OVERLAY)
        .fill(c(k.card_background_fill_color_default))
        .border(1.0, c(k.card_stroke_color_default));
    for (i, (name, st)) in t.type_ramp().named().iter().enumerate().rev() {
        let mut r = row()
            .w_full()
            .items_center()
            .px(16.0)
            .py(12.0)
            .gap(16.0)
            .child(
                text(name.to_string())
                    .font(font(*st))
                    .color(c(k.text_fill_color_primary))
                    .grow(1.0)
                    .truncate(),
            )
            .child(
                text(format!(
                    "{}/{} epx, {}",
                    st.size,
                    st.line_height,
                    if st.weight >= 600 {
                        "Semibold"
                    } else {
                        "Regular"
                    }
                ))
                .font(font(t.type_ramp().caption))
                .color(c(k.text_fill_color_secondary))
                .w(160.0),
            );
        if i != 0 {
            r = r.border_sides([0.0, 0.0, 0.0, 1.0], c(k.divider_stroke_color_default));
        }
        c0 = c0.child(r);
    }
    col()
        .w_full()
        .child(header(
            t,
            "Typography",
            "The Windows type ramp: Segoe UI Variable sizes, weights and line heights.",
        ))
        .child(c0)
}

fn settings_page(s: &State, t: &Theme) -> El {
    let k = t.tokens();
    let card = |sym: Symbol, title: &str, desc: &str, ctl: El| {
        row()
            .w_full()
            .min_h(68.0)
            .items_center()
            .px(16.0)
            .gap(16.0)
            .mb(4.0)
            .radius(radii::CONTROL)
            .fill(c(k.card_background_fill_color_default))
            .border(1.0, c(k.card_stroke_color_default))
            .child(icon(sym, 20.0, k.text_fill_color_primary))
            .child(
                col()
                    .grow(1.0)
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
            .child(ctl)
    };
    let dark = t.is_dark();
    let next = t.mode().toggled();
    let theme_switch = toggle_switch(t, dark, None)
        .key("theme-toggle")
        .focusable(true)
        .on_press(move |s: &mut State| s.theme = Some(next));
    let mut sw = row()
        .gap(4.0)
        .flex_wrap()
        .justify_end()
        .max_w(5.0 * 36.0)
        .py(8.0);
    for (i, (name, col_)) in crate::ACCENTS.iter().enumerate() {
        let mut b = el()
            .size(32.0, 32.0)
            .radius(radii::CONTROL)
            .fill(c(*col_))
            .center()
            .key(format!("accent-{i}"))
            .role("button")
            .focusable(true)
            .label(name.to_string())
            .on_press(move |s: &mut State| s.accent = i);
        if i == s.accent {
            b = b.border(2.0, c(k.text_fill_color_primary)).child(icon(
                Symbol::CheckMark,
                14.0,
                winui_theme::Color::WHITE,
            ));
        }
        sw = sw.child(b);
    }
    col()
        .w_full()
        .child(header(t, "Settings", ""))
        .child(
            text("Appearance & behavior")
                .font(font(t.type_ramp().body_strong))
                .color(c(k.text_fill_color_primary))
                .mb(8.0),
        )
        .child(card(
            Symbol::DarkTheme,
            "App theme",
            if dark { "Dark" } else { "Light" },
            theme_switch,
        ))
        .child(card(
            Symbol::Color,
            "Accent color",
            crate::ACCENTS[s.accent].0,
            sw,
        ))
        .child(card(
            Symbol::Navigation,
            "Navigation style",
            "Left",
            combo_box(t, "Left", 160.0),
        ))
        .child(
            text("About")
                .font(font(t.type_ramp().body_strong))
                .color(c(k.text_fill_color_primary))
                .mt(24.0)
                .mb(8.0),
        )
        .child(card(
            Symbol::Info,
            "WinUI 3 Gallery for Linux",
            "winui-linux, rendered with Rune",
            text(env!("CARGO_PKG_VERSION"))
                .font(font(t.type_ramp().body))
                .color(c(k.text_fill_color_secondary)),
        ))
}
