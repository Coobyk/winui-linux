//! Buttons, Input, Collections and Menus samples.

use super::{Sample, c};
use crate::fallback::controls::{self as k, ButtonStyle, body, caption, icon};
use crate::fallback::icons::Symbol;
use crate::fallback::theme::{Theme, radii};
use crate::state::{Page, State};
use rune::{El, col, el, row};

fn output(theme: &Theme, key: &str, s: String) -> El {
    caption(s, theme.tokens().text_fill_color_secondary).key(key)
}

pub static SAMPLES: &[Sample] = &[
    Sample { id: "button", page: Page::Buttons, title: "A simple Button with text content", keywords: "Button standard click", view: |s, t| {
        row().gap(16.0).items_center()
            .child(k::button(t, "btn-standard", "Standard XAML button", ButtonStyle::Standard, true, |s: &mut State| s.clicks += 1))
            .child(k::button(t, "btn-disabled", "Disabled button", ButtonStyle::Standard, false, |_: &mut State| {}))
            .child(output(t, "btn-output", format!("You clicked: {} times", s.clicks)))
    }},
    Sample { id: "accent-button", page: Page::Buttons, title: "Accent style applied to Button", keywords: "Button accent primary", view: |_, t| {
        row().gap(16.0)
            .child(k::button(t, "btn-accent", "Accent style button", ButtonStyle::Accent, true, |s: &mut State| s.clicks += 1))
            .child(k::button(t, "btn-accent-disabled", "Disabled", ButtonStyle::Accent, false, |_: &mut State| {}))
            .child(k::icon_button(t, "btn-icon", Symbol::Save, Some("Save"), ButtonStyle::Standard, |_: &mut State| {}))
    }},
    Sample { id: "hyperlink-button", page: Page::Buttons, title: "A HyperlinkButton", keywords: "HyperlinkButton link", view: |_, t| {
        k::hyperlink_button(t, "btn-link", "Microsoft home page", |_: &mut State| {})
    }},
    Sample { id: "toggle-button", page: Page::Buttons, title: "A simple ToggleButton with text content", keywords: "ToggleButton toggle", view: |s, t| {
        row().gap(16.0).items_center()
            .child(k::toggle_button(t, "btn-toggle", "ToggleButton", s.toggled, |s: &mut State| s.toggled = !s.toggled))
            .child(output(t, "toggle-output", format!("Output: {}", if s.toggled { "On" } else { "Off" })))
    }},
    Sample { id: "repeat-button", page: Page::Buttons, title: "A RepeatButton", keywords: "RepeatButton repeat", view: |s, t| {
        row().gap(16.0).items_center()
            .child(k::button(t, "btn-repeat", "Click and hold", ButtonStyle::Standard, true, |s: &mut State| s.repeats += 1))
            .child(output(t, "repeat-output", format!("Number of clicks: {}", s.repeats)))
    }},
    Sample { id: "split-button", page: Page::Buttons, title: "A SplitButton", keywords: "SplitButton split color", view: |s, t| {
        const COLORS: [(u8, u8, u8); 4] = [(0x00, 0x78, 0xD4), (0xE8, 0x11, 0x23), (0x10, 0x7C, 0x10), (0xFF, 0xB9, 0x00)];
        let (r, g, b) = COLORS[s.split_color % COLORS.len()];
        let swatch = el().size(32.0, 16.0).radius(2.0).fill(c(crate::fallback::theme::Color::rgb(r, g, b)));
        k::split_button(t, "btn-split", swatch, |_: &mut State| {}, |s: &mut State| s.split_color += 1)
    }},
    Sample { id: "dropdown-button", page: Page::Buttons, title: "A DropDownButton", keywords: "DropDownButton dropdown flyout", view: |s, t| {
        row().gap(16.0)
            .child(col().gap(4.0).child(k::dropdown_button(t, "btn-dropdown", "Email", |s: &mut State| s.dropdown_open = !s.dropdown_open)).child(if s.dropdown_open {
                k::flyout_surface(t).key("dropdown-menu").role("menu").w(160.0).py(2.0)
                    .child(k::menu_item(t, "dropdown-send", Some(Symbol::Send), "Send", None, |s: &mut State| s.dropdown_open = false))
                    .child(k::menu_item(t, "dropdown-reply", Some(Symbol::Mail), "Reply", None, |s: &mut State| s.dropdown_open = false))
            } else { el() }))
            .child(k::button_frame(t, "btn-dropdown-icon", ButtonStyle::Standard, true, |s: &mut State| s.dropdown_open = !s.dropdown_open).px(11.0).label("Mail")
                .child(icon(Symbol::Mail, 16.0, t.tokens().text_fill_color_primary))
                .child(icon(Symbol::ChevronDown, 12.0, t.tokens().text_fill_color_secondary)))
    }},
    // Input
    Sample { id: "checkbox", page: Page::Input, title: "CheckBox: two-state and three-state", keywords: "CheckBox check indeterminate", view: |s, t| {
        row().gap(32.0)
            .child(k::checkbox(t, "cb-two", "Two-state CheckBox", Some(s.check), true, |s: &mut State| s.check = !s.check))
            .child(k::checkbox(t, "cb-three", "Three-state CheckBox", s.tri, true, |s: &mut State| s.tri = match s.tri { Some(false) => Some(true), Some(true) => None, None => Some(false) }))
            .child(k::checkbox(t, "cb-disabled", "Disabled CheckBox", Some(true), false, |_: &mut State| {}))
    }},
    Sample { id: "radio", page: Page::Input, title: "A group of RadioButton controls", keywords: "RadioButton RadioButtons option", view: |s, t| {
        let mut g = col().key("radios").gap(0.0).child(body("Options", t.tokens().text_fill_color_primary));
        for (i, name) in ["Option 1", "Option 2", "Option 3"].into_iter().enumerate() {
            g = g.child(k::radio(t, &format!("radio-{i}"), name, s.radio == i, move |s: &mut State| s.radio = i));
        }
        g
    }},
    Sample { id: "toggle-switch", page: Page::Input, title: "A simple ToggleSwitch", keywords: "ToggleSwitch switch on off", view: |s, t| {
        k::toggle_switch(t, "switch", Some("Toggle work"), s.switch_on, |s: &mut State| s.switch_on = !s.switch_on)
    }},
    Sample { id: "slider", page: Page::Input, title: "A simple Slider", keywords: "Slider range value", view: |s, t| {
        row().gap(24.0).items_center()
            .child(k::slider(t, "slider", 280.0, s.slider, |s: &mut State, v| s.slider = v))
            .child(output(t, "slider-output", format!("Value: {:.0}", s.slider * 100.0)))
    }},
    Sample { id: "rating", page: Page::Input, title: "A RatingControl", keywords: "RatingControl rating stars", view: |s, t| {
        k::rating(t, "rating", s.stars, |s: &mut State, v| s.stars = v)
    }},
    Sample { id: "textbox", page: Page::Input, title: "TextBox with a header and placeholder text", keywords: "TextBox text input edit placeholder", view: |s, t| {
        row().gap(24.0)
            .child(k::header(t, "Enter your name:", k::text_box(t, "textbox", &s.text, "Name", 280.0, true, |s: &mut State| &mut s.text)))
            .child(k::header(t, "Disabled:", k::text_box(t, "textbox-disabled", &rune::TextEdit::new(""), "Can't type here", 200.0, false, |s: &mut State| &mut s.text)))
    }},
    Sample { id: "passwordbox", page: Page::Input, title: "A PasswordBox", keywords: "PasswordBox password secret", view: |s, t| {
        k::header(t, "Password", k::password_box(t, "passwordbox", &s.password, 280.0, |s: &mut State| &mut s.password))
    }},
    Sample { id: "numberbox", page: Page::Input, title: "A NumberBox with spin buttons", keywords: "NumberBox number spin", view: |s, t| {
        k::header(t, "Enter an integer:", k::number_box(t, "numberbox", s.number, |s: &mut State, v| s.number = v))
    }},
    // Collections
    Sample { id: "listview", page: Page::Collections, title: "Basic ListView with simple data", keywords: "ListView list rows selection", view: |s, t| {
        let tk = t.tokens();
        let people = [("Kendall Collins", "KC", "Adatum Corporation"), ("Henry Ross", "HR", "Fabrikam"), ("Vance DeLeon", "VD", "Contoso Ltd."), ("Victoria Burke", "VB", "Northwind")];
        let mut list = col().key("listview").role("listbox").w(360.0).py(4.0).radius(radii::OVERLAY).fill(c(tk.solid_background_fill_color_tertiary)).border(1.0, c(tk.card_stroke_color_default));
        for (i, (name, ini, company)) in people.into_iter().enumerate() {
            let content = row().gap(12.0).items_center().child(k::person_picture(t, ini, 32.0))
                .child(col().child(body(name, tk.text_fill_color_primary)).child(caption(company, tk.text_fill_color_secondary)));
            list = list.child(k::list_row(t, &format!("list-{i}"), 52.0, s.list_sel == i, content, move |s: &mut State| s.list_sel = i).label(name));
        }
        list
    }},
    Sample { id: "gridview", page: Page::Collections, title: "GridView with selection", keywords: "GridView grid tiles", view: |s, t| {
        let tk = t.tokens();
        let mut grid = row().key("gridview").role("listbox").gap(8.0).flex_wrap();
        let syms = [Symbol::Camera, Symbol::MusicNote2, Symbol::Video, Symbol::Folder, Symbol::Document, Symbol::Globe, Symbol::Calendar, Symbol::Mail];
        for (i, sym) in syms.into_iter().enumerate() {
            let sel = s.grid_sel == i;
            let mut tile = col().key(format!("grid-{i}")).role("option").label(sym.name()).checked(sel).size(96.0, 96.0).radius(radii::CONTROL).center().gap(8.0)
                .fill(c(tk.control_fill_color_secondary)).hover_fill(c(tk.control_fill_color_tertiary))
                .child(icon(sym, 32.0, tk.text_fill_color_primary)).child(caption(sym.name(), tk.text_fill_color_secondary))
                .on_press(move |s: &mut State| s.grid_sel = i);
            tile = if sel { tile.border(2.0, c(tk.accent_fill_color_default)) } else { tile.border(1.0, c(tk.card_stroke_color_default)) };
            grid = grid.child(tile);
        }
        grid
    }},
    // Menus
    Sample { id: "combobox", page: Page::Menus, title: "A ComboBox with items defined inline", keywords: "ComboBox combo dropdown select", view: |s, t| {
        const ITEMS: [&str; 4] = ["Blue", "Green", "Red", "Yellow"];
        let mut col_el = col().gap(4.0).child(k::header(t, "Colors", k::combo_box(t, "combobox", ITEMS[s.combo_sel], 200.0, |s: &mut State| s.combo_open = !s.combo_open)));
        if s.combo_open {
            let mut pop = k::flyout_surface(t).key("combobox-popup").role("listbox").w(200.0).py(2.0);
            for (i, name) in ITEMS.into_iter().enumerate() {
                pop = pop.child(k::list_row(t, &format!("combo-{i}"), 32.0, s.combo_sel == i, body(name, t.tokens().text_fill_color_primary), move |s: &mut State| { s.combo_sel = i; s.combo_open = false; }).label(name));
            }
            col_el = col_el.child(pop);
        }
        col_el
    }},
    Sample { id: "menuflyout", page: Page::Menus, title: "MenuFlyout with icons and keyboard accelerators", keywords: "MenuFlyout menu context flyout", view: |s, t| {
        let menu = k::flyout_surface(t).key("menuflyout").role("menu").w(240.0).py(2.0)
            .child(k::menu_item(t, "menu-cut", Some(Symbol::Cut), "Cut", Some("Ctrl+X"), |s: &mut State| s.menu_last = "Cut"))
            .child(k::menu_item(t, "menu-copy", Some(Symbol::Copy), "Copy", Some("Ctrl+C"), |s: &mut State| s.menu_last = "Copy"))
            .child(k::menu_item(t, "menu-paste", Some(Symbol::ClipboardPaste), "Paste", Some("Ctrl+V"), |s: &mut State| s.menu_last = "Paste"))
            .child(k::menu_separator(t))
            .child(k::menu_item(t, "menu-bold", Some(Symbol::TextBold), if s.bold { "Bold \u{2713}" } else { "Bold" }, Some("Ctrl+B"), |s: &mut State| s.bold = !s.bold))
            .child(k::menu_item(t, "menu-share", Some(Symbol::Share), "Share", None, |s: &mut State| s.menu_last = "Share"));
        row().gap(24.0).child(menu).child(output(t, "menu-output", if s.menu_last.is_empty() { "Pick an item".into() } else { format!("You clicked: {}", s.menu_last) }))
    }},
    Sample { id: "commandbar", page: Page::Menus, title: "A CommandBar with labels", keywords: "CommandBar AppBarButton toolbar", view: |_, t| {
        let tk = t.tokens();
        let mut bar = row().key("commandbar").role("toolbar").gap(4.0).padding(4.0).radius(radii::OVERLAY).fill(c(tk.solid_background_fill_color_quarternary)).border(1.0, c(tk.card_stroke_color_default));
        for (i, (sym, label)) in [(Symbol::Add, "Add"), (Symbol::Edit, "Edit"), (Symbol::Share, "Share"), (Symbol::Delete, "Delete")].into_iter().enumerate() {
            bar = bar.child(k::icon_button(t, &format!("cmd-{i}"), sym, Some(label), ButtonStyle::Subtle, |_: &mut State| {}));
        }
        bar.child(el().w(1.0).h(20.0).mx(4.0).fill(c(tk.divider_stroke_color_default)))
            .child(k::icon_button(t, "cmd-more", Symbol::MoreHorizontal, None, ButtonStyle::Subtle, |_: &mut State| {}))
    }},
    Sample { id: "flyout", page: Page::Menus, title: "A Button with a Flyout", keywords: "Flyout popup confirm", view: |s, t| {
        let tk = t.tokens();
        let fly = k::flyout_surface(t).key("flyout").role("dialog").padding(16.0).gap(12.0).w(300.0)
            .child(body("All items will be removed. Do you want to continue?", tk.text_fill_color_primary))
            .child(k::button(t, "flyout-confirm", "Yes, empty my cart", ButtonStyle::Standard, true, |s: &mut State| s.cart_open = false));
        col().gap(8.0).child(k::button(t, "flyout-open", "Empty cart", ButtonStyle::Standard, true, |s: &mut State| s.cart_open = !s.cart_open)).child(if s.cart_open { fly } else { el() })
    }},
];
