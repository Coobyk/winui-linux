//! The gallery app: NavigationView-style shell over Mica, a page per control
//! family, styles pages, and a Settings page with light/dark + accent picker.

pub use rune;

use rune::{AppSpec, Cx, El, ScrollSpec, TextEdit, col, el, row, text};
use winui_controls as ctl;
use winui_icons::Symbol;
use winui_materials as mat;
use winui_theme::{Color, Theme};

/// The bundled text face: Selawik, Microsoft's open metric-compatible stand-in
/// for Segoe UI (embedded so snaps render identically on every machine).
const FONTS: &[&[u8]] = &[
    include_bytes!("../assets/Selawik-Light.ttf"),
    include_bytes!("../assets/Selawik-Regular.ttf"),
    include_bytes!("../assets/Selawik-SemiBold.ttf"),
    include_bytes!("../assets/Selawik-Bold.ttf"),
];

/// Windows personalization accents for the Settings picker.
pub const ACCENTS: &[(&str, Color)] = &[
    ("Blue", Color::rgb(0x00, 0x78, 0xD4)),
    ("Purple", Color::rgb(0x87, 0x64, 0xB8)),
    ("Teal", Color::rgb(0x03, 0x83, 0x87)),
    ("Green", Color::rgb(0x10, 0x7C, 0x10)),
    ("Orange", Color::rgb(0xCA, 0x50, 0x10)),
    ("Magenta", Color::rgb(0xE3, 0x00, 0x8C)),
];

pub struct State {
    pub page: &'static str,
    /// None = follow system; Some(dark) = user override from Settings.
    pub dark: Option<bool>,
    pub accent: usize,
    pub nav_expanded: bool,
    pub search: TextEdit,
    // Demo control state:
    pub toggle_btn: bool,
    pub checks: [ctl::CheckState; 3],
    pub radio: usize,
    pub switches: [bool; 2],
    pub slider: f32,
    pub rating: f32,
    pub text1: TextEdit,
    pub pass: TextEdit,
    pub num: TextEdit,
    pub suggest: TextEdit,
    pub combo_sel: usize,
    pub combo_open: bool,
    pub expander: bool,
    pub list_sel: usize,
    pub tab_sel: usize,
    pub swatch_sel: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            page: "home",
            dark: None,
            accent: 0,
            nav_expanded: true,
            search: TextEdit::new(""),
            toggle_btn: false,
            checks: [
                ctl::CheckState::Checked,
                ctl::CheckState::Unchecked,
                ctl::CheckState::Indeterminate,
            ],
            radio: 0,
            switches: [true, false],
            slider: 0.62,
            rating: 3.5,
            text1: TextEdit::new(""),
            pass: TextEdit::new("hunter2"),
            num: TextEdit::new("42"),
            suggest: TextEdit::new("but"),
            combo_sel: 1,
            combo_open: true,
            expander: true,
            list_sel: 1,
            tab_sel: 1,
            swatch_sel: 0,
        }
    }
}

/// Page ids — these are the `--scene` names.
pub const PAGES: &[&str] = &[
    "home",
    "buttons",
    "input",
    "collections",
    "menus",
    "navigation",
    "status",
    "styles-icons",
    "styles-colors",
    "styles-type",
    "settings",
];

pub const SCENES: &[(&str, fn(&mut State))] = &[
    ("home", |s| s.page = "home"),
    ("buttons", |s| s.page = "buttons"),
    ("input", |s| s.page = "input"),
    ("collections", |s| s.page = "collections"),
    ("menus", |s| s.page = "menus"),
    ("navigation", |s| s.page = "navigation"),
    ("status", |s| s.page = "status"),
    ("styles-icons", |s| s.page = "styles-icons"),
    ("styles-colors", |s| s.page = "styles-colors"),
    ("styles-type", |s| s.page = "styles-type"),
    ("settings", |s| s.page = "settings"),
];

pub fn spec() -> AppSpec<State> {
    AppSpec {
        scenes: SCENES,
        min_size: Some((640.0, 480.0)),
        fonts: FONTS,
        families: ("Selawik, Segoe UI, Inter, sans-serif", "Inter, sans-serif"),
        ..AppSpec::new("WinUI 3 Gallery", (1200.0, 760.0), State::default, view)
    }
}

fn theme(s: &State, cx: &mut Cx<State>) -> Theme {
    let dark = s.dark.unwrap_or_else(|| cx.system_dark());
    let th = if dark { Theme::dark() } else { Theme::light() };
    th.with_accent(ACCENTS[s.accent % ACCENTS.len()].1)
}

/// A Gallery sample block: caption label + control.
fn sample(label: &str, th: &Theme, control: El) -> El {
    col().gap(8.0)
        .w_fit()
        .child(
            text(label.to_string())
                .font(ctl::caption())
                .color(mat::rc(th.tokens().text_fill_color_secondary)),
        )
        .child(control)
}

/// A page section card like the Gallery's sample frames.
fn section(th: &Theme, title: &str, body: El) -> El {
    el().w_full()
        .radius(8.0)
        .fill(mat::rc(th.tokens().card_background_fill_color_default))
        .border(1.0, mat::rc(th.tokens().card_stroke_color_default))
        .child(
            col().w_full()
                .padding(20.0)
                .gap(16.0)
                .child(
                    text(title.to_string())
                        .font(rune::TextStyle::new(16.0, 600.0))
                        .color(mat::rc(th.tokens().text_fill_color_primary)),
                )
                .child(body),
        )
}

fn page_header(th: &Theme, title: &str) -> El {
    text(title.to_string())
        .font(rune::TextStyle::new(28.0, 600.0).lh(36.0))
        .color(mat::rc(th.tokens().text_fill_color_primary))
}

fn page_body(th: &Theme, title: &str, sections: Vec<El>) -> El {
    let mut c = col().w_full().px(44.0).pb(44.0).gap(24.0);
    for s in sections {
        c = c.child(s);
    }
    col().key("page")
        .w_full()
        .grow(1.0)
        .min_h(0.0)
        .scroll(ScrollSpec::default())
        .child(col().w_full().pt(36.0).pb(8.0).px(44.0).child(page_header(th, title)))
        .child(c)
}

/// One nav row; page == None renders a group header.
fn nav_row(s: &State, th: &Theme, key: usize, label: &'static str, sym: Symbol, page: Option<&'static str>) -> El {
    match page {
        None => ctl::NavItem::header(label).view(th, s.nav_expanded).key(format!("nav-{key}")),
        Some(p) => ctl::NavItem::new(label, sym)
            .selected(s.page == p)
            .view(th, s.nav_expanded)
            .key(format!("nav-{key}"))
            .on_press(move |s: &mut State| s.page = p),
    }
}

fn nav_pane(s: &State, th: &Theme) -> El {
    let t = th.tokens();
    let w = if s.nav_expanded {
        winui_theme::sizes::NAV_OPEN_PANE_LENGTH
    } else {
        winui_theme::sizes::NAV_COMPACT_PANE_LENGTH
    };
    let rows: &[(&str, Symbol, Option<&'static str>)] = &[
        ("Home", Symbol::Home, Some("home")),
        ("Basic input", Symbol::Circle, None),
        ("Buttons", Symbol::CursorClick, Some("buttons")),
        ("Input", Symbol::CheckboxChecked, Some("input")),
        ("Collections", Symbol::Collections, Some("collections")),
        ("Menus & toolbars", Symbol::Circle, None),
        ("Menus", Symbol::MoreVertical, Some("menus")),
        ("Navigation", Symbol::Circle, None),
        ("Navigation", Symbol::Navigation, Some("navigation")),
        ("Status & info", Symbol::Circle, None),
        ("Status", Symbol::Info, Some("status")),
        ("Styles", Symbol::Circle, None),
        ("Icons", Symbol::Icons, Some("styles-icons")),
        ("Colors", Symbol::Color, Some("styles-colors")),
        ("Typography", Symbol::TextFont, Some("styles-type")),
    ];
    let mut items = col().w_full().grow(1.0).min_h(0.0).scroll(ScrollSpec::default());
    for (i, (label, sym, page)) in rows.iter().enumerate() {
        items = items.child(nav_row(s, th, i, label, *sym, *page));
    }
    let pane = col().w(w).h_full().py(4.0)
        .child(
            el().size(40.0, 36.0)
                .center()
                .radius(4.0)
                .mx(4.0)
                .role("button")
                .label("Toggle navigation")
                .hover_fill(mat::reveal_fill(th))
                .cursor(rune::Cursor::Pointer)
                .on_press(|s: &mut State| s.nav_expanded = !s.nav_expanded)
                .child(ctl::icon(th, Symbol::GlobalNavigation, 16.0)),
        )
        .children(if s.nav_expanded {
            Some(
                el().w_full()
                    .h(32.0)
                    .mx(8.0)
                    .mb(4.0)
                    .radius(4.0)
                    .fill(mat::rc(t.control_fill_color_input_active))
                    .border(1.0, mat::rc(t.control_stroke_color_secondary))
                    .clip_rounded(4.0)
                    .child(
                        row().items_center().w_full().pl(10.0)
                            .child(
                                rune::text_input(&s.search, ctl::body())
                                    .grow(1.0)
                                    .min_w(0.0)
                                    .color(mat::rc(t.text_fill_color_primary))
                                    .placeholder("Search", mat::rc(t.text_fill_color_tertiary))
                                    .label("Search")
                                    .on_edit(|s: &mut State| &mut s.search),
                            )
                            .child(
                                el().w(30.0).h_full().center().child(
                                    ctl::glyph(Symbol::Find, 12.0, mat::rc(t.text_fill_color_secondary)),
                                ),
                            ),
                    )
                    .child(
                        el().absolute(0.0, 31.0).w_full().h(1.0).fill(mat::rc(t.control_strong_stroke_color_default)),
                    ),
            )
        } else {
            None
        })
        .child(items)
        .child(
            ctl::NavItem::new("Settings", Symbol::Settings)
                .selected(s.page == "settings")
                .view(th, s.nav_expanded)
                .key("nav-settings")
                .on_press(|s: &mut State| s.page = "settings"),
        );
    el().w(w + 8.0).h_full().child(mat::nav_acrylic(th)).child(
        el().w(w + 8.0).h_full().absolute(0.0, 0.0).child(pane),
    )
}

/// The app's tiles icon: four rounded squares in accent shades, like the
/// Gallery's own icon.
fn app_icon(th: &Theme) -> El {
    let a = th.accent();
    let tile = |c: winui_theme::Color| {
        el().size(7.0, 7.0).radius(1.5).fill(mat::rc(c))
    };
    col().gap(2.0)
        .child(row().gap(2.0).child(tile(a.light2)).child(tile(a.base)))
        .child(row().gap(2.0).child(tile(a.base)).child(tile(a.dark1)))
}

/// A system caption button: 46×32 hit area, centred glyph, subtle hover.
fn caption_button(th: &Theme, glyph_el: El, close: bool) -> El {
    let t = th.tokens();
    let e = el().size(46.0, 32.0)
        .center()
        .cursor(rune::Cursor::Pointer)
        .child(glyph_el);
    if close {
        e.hover_fill(mat::rc(winui_theme::Color::rgb(0xC4, 0x2B, 0x1C)))
            .role("button")
            .label("Close")
    } else {
        e.hover_fill(mat::rc(t.subtle_fill_color_secondary))
            .role("button")
            .label("Window caption")
    }
}

/// The custom title bar (TitleBarHeightOption::Tall = 48px): back button, app
/// icon + title on the left, system caption buttons on the right, Mica
/// showing through everywhere.
fn title_bar(s: &State, th: &Theme) -> El {
    let t = th.tokens();
    let _ = s;
    let glyph_c = mat::rc(t.text_fill_color_secondary);
    row().w_full()
        .h(48.0)
        .items_center()
        .child(
            el().size(40.0, 32.0)
                .ml(6.0)
                .center()
                .radius(4.0)
                .role("button")
                .label("Back")
                .hover_fill(mat::rc(t.subtle_fill_color_secondary))
                .cursor(rune::Cursor::Pointer)
                .on_press(|s: &mut State| s.page = "home")
                .child(ctl::glyph(Symbol::Back, 12.0, glyph_c)),
        )
        .child(
            row().items_center().gap(8.0).ml(4.0)
                .child(app_icon(th))
                .child(
                    text("WinUI 3 Gallery".to_string())
                        .font(rune::TextStyle::new(12.0, 400.0))
                        .color(mat::rc(t.text_fill_color_primary)),
                ),
        )
        .child(el().grow(1.0))
        .child(caption_button(
            th,
            el().size(10.0, 1.0).fill(glyph_c),
            false,
        ))
        .child(caption_button(
            th,
            el().size(10.0, 10.0).border(1.0, glyph_c),
            false,
        ))
        .child(caption_button(
            th,
            ctl::glyph(Symbol::Dismiss, 10.0, glyph_c),
            true,
        ))
        .child(el().w(8.0))
}

pub fn view(s: &State, cx: &mut Cx<State>) -> El {
    let th = theme(s, cx);
    let t = th.tokens();
    let content = page_content(s, &th);
    mat::mica(&th).key("root").child(
        col().absolute(0.0, 0.0)
            .w_full()
            .h_full()
            .child(title_bar(s, &th))
            .child(
                row().grow(1.0)
                    .min_h(0.0)
                    .w_full()
                    .child(nav_pane(s, &th))
                    .child(
                        el().grow(1.0)
                            .min_w(0.0)
                            .h_full()
                            .fill(mat::rc(t.layer_on_mica_base_alt_fill_color_default))
                            .corners([8.0, 0.0, 0.0, 0.0])
                            .border_sides([1.0, 1.0, 0.0, 0.0], mat::rc(t.card_stroke_color_default))
                            .clip_corners([8.0, 0.0, 0.0, 0.0])
                            .child(content),
                    ),
            ),
    )
}

fn page_content(s: &State, th: &Theme) -> El {
    match s.page {
        "buttons" => buttons_page(s, th),
        "input" => input_page(s, th),
        "collections" => collections_page(s, th),
        "menus" => menus_page(s, th),
        "navigation" => navigation_page(s, th),
        "status" => status_page(s, th),
        "styles-icons" => icons_page(th),
        "styles-colors" => colors_page(th),
        "styles-type" => type_page(th),
        "settings" => settings_page(s, th),
        _ => home_page(s, th),
    }
}

fn home_page(s: &State, th: &Theme) -> El {
    let _ = s;
    let hero = el().w_full()
        .radius(8.0)
        .linear_gradient(0.35, [
            (0.0, mat::rc(th.accent().light1)),
            (1.0, mat::rc(th.accent().dark1)),
        ])
        .box_shadow(mat::elevation(th, 8.0))
        .child(
            col().padding(32.0).gap(8.0)
                .child(
                    text("WinUI 3 Gallery".to_string())
                        .font(rune::TextStyle::new(28.0, 600.0))
                        .color(mat::rc(th.tokens().text_on_accent_fill_color_primary)),
                )
                .child(
                    text("Fluent controls for Linux, in Rust + Rune.".to_string())
                        .font(ctl::body())
                        .color(mat::rc(th.tokens().text_on_accent_fill_color_secondary)),
                ),
        );
    let card = |icon: Symbol, title: &'static str, desc: &'static str, page: &'static str| {
        el().w(240.0)
            .radius(8.0)
            .fill(mat::rc(th.tokens().card_background_fill_color_default))
            .border(1.0, mat::rc(th.tokens().card_stroke_color_default))
            .hover_fill(mat::rc(th.tokens().card_background_fill_color_secondary))
            .cursor(rune::Cursor::Pointer)
            .on_press(move |s: &mut State| s.page = page)
            .child(
                col().padding(16.0).gap(8.0)
                    .child(ctl::glyph(icon, 24.0, mat::rc(th.tokens().accent_fill_color_default)))
                    .child(text(title.to_string()).font(rune::TextStyle::new(14.0, 600.0)).color(mat::rc(th.tokens().text_fill_color_primary)))
                    .child(text(desc.to_string()).font(ctl::caption()).color(mat::rc(th.tokens().text_fill_color_secondary))),
            )
    };
    page_body(th, "Home", vec![
        hero,
        row().gap(16.0)
            .child(card(Symbol::CursorClick, "Controls", "Buttons, inputs, pickers", "buttons"))
            .child(card(Symbol::Icons, "Styles", "Icons, colours, typography", "styles-icons"))
            .child(card(Symbol::Settings, "Settings", "Theme and accent", "settings")),
    ])
}

fn buttons_page(s: &State, th: &Theme) -> El {
    let row1 = row().gap(12.0).items_center()
        .child(ctl::Button::new("Standard").view(th).on_press(|_: &mut State| {}))
        .child(ctl::Button::new("Accent").accent(true).view(th).on_press(|_: &mut State| {}))
        .child(ctl::Button::new("Disabled").enabled(false).view(th));
    let row2 = row().gap(12.0).items_center()
        .child(ctl::ToggleButton::new("Toggle", s.toggle_btn).view(th)
            .on_press(|s: &mut State| s.toggle_btn = !s.toggle_btn))
        .child(ctl::RepeatButton::new("Repeat").view(th))
        .child(ctl::HyperlinkButton::new("Hyperlink").view(th));
    let row3 = row().gap(12.0).items_center()
        .child(ctl::SplitButton::new("Split").view(th))
        .child(ctl::DropDownButton::new("Drop down").view(th))
        .child(ctl::Button::new("With icon").symbol(Symbol::Save).view(th));
    page_body(th, "Buttons", vec![
        section(th, "Button", sample("Standard, accent, disabled", th, row1)),
        section(th, "More buttons", sample("Toggle, repeat, hyperlink", th, row2)),
        section(th, "Split & drop-down", sample("SplitButton, DropDownButton", th, row3)),
    ])
}

fn input_page(s: &State, th: &Theme) -> El {
    let checks = col().gap(4.0)
        .child(ctl::CheckBox::new("Checked", s.checks[0]).view(th)
            .on_press(|s: &mut State| s.checks[0] = if s.checks[0] == ctl::CheckState::Checked { ctl::CheckState::Unchecked } else { ctl::CheckState::Checked }))
        .child(ctl::CheckBox::new("Unchecked", s.checks[1]).view(th)
            .on_press(|s: &mut State| s.checks[1] = if s.checks[1] == ctl::CheckState::Unchecked { ctl::CheckState::Checked } else { ctl::CheckState::Unchecked }))
        .child(ctl::CheckBox::new("Indeterminate", s.checks[2]).view(th)
            .on_press(|s: &mut State| s.checks[2] = if s.checks[2] == ctl::CheckState::Indeterminate { ctl::CheckState::Checked } else { ctl::CheckState::Indeterminate }))
        .child(ctl::CheckBox::new("Disabled", ctl::CheckState::Checked).enabled(false).view(th));
    let radios = col().gap(4.0)
        .child(ctl::RadioButton::new("Option A", s.radio == 0).view(th).on_press(|s: &mut State| s.radio = 0))
        .child(ctl::RadioButton::new("Option B", s.radio == 1).view(th).on_press(|s: &mut State| s.radio = 1))
        .child(ctl::RadioButton::new("Option C", s.radio == 2).view(th).on_press(|s: &mut State| s.radio = 2))
        .child(ctl::RadioButton::new("Disabled", false).enabled(false).view(th));
    let switches = col().gap(8.0)
        .child(ctl::ToggleSwitch::new(s.switches[0]).header("Wi-Fi").view(th)
            .on_press(|s: &mut State| s.switches[0] = !s.switches[0]))
        .child(ctl::ToggleSwitch::new(s.switches[1]).header("Bluetooth").view(th)
            .on_press(|s: &mut State| s.switches[1] = !s.switches[1]))
        .child(ctl::ToggleSwitch::new(false).header("Disabled").view(th));
    let slider = col().gap(8.0)
        .child(ctl::Slider::new(s.slider * 100.0).header("Volume").view(th, 360.0))
        .child(ctl::Slider::new(40.0).enabled(false).header("Disabled").view(th, 360.0));
    let rating = row().gap(16.0).items_center()
        .child(ctl::RatingControl::new(s.rating).view(th))
        .child(ctl::RatingControl::new(2.0).view(th));
    let texts = col().gap(16.0)
        .child(ctl::TextBox::new("Type something…").header("TextBox").view(th, &s.text1).w(360.0))
        .child(ctl::TextBox::new("Disabled").enabled(false).view(th, &s.text1).w(360.0))
        .child(ctl::PasswordBox::new("Password").view(th, &s.pass).w(360.0))
        .child(ctl::NumberBox::new().view(th, &s.num).w(360.0))
        .child(
            ctl::AutoSuggestBox::new("Search controls")
                .suggestions(["Button", "BreadcrumbBar", "Badge"])
                .view(th, &s.suggest)
                .w(360.0),
        );
    page_body(th, "Input", vec![
        section(th, "Text controls", texts),
        section(th, "CheckBox", checks),
        section(th, "RadioButton", radios),
        section(th, "ToggleSwitch", switches),
        section(th, "Slider", slider),
        section(th, "RatingControl", rating),
    ])
}

fn collections_page(s: &State, th: &Theme) -> El {
    let names = [
        ("Documents", Symbol::Folder, "24 items"),
        ("Pictures", Symbol::Image, "1,204 items"),
        ("Music", Symbol::MusicNote, "317 items"),
        ("Downloads", Symbol::Download, "58 items"),
    ];
    let mut list = col().w_full();
    for (i, (name, sym, sub)) in names.iter().enumerate() {
        list = list.child(
            ctl::ListRow::new(*name)
                .symbol(*sym)
                .subtitle(*sub)
                .selected(s.list_sel == i)
                .view(th)
                .on_press(move |s: &mut State| s.list_sel = i),
        );
    }
    let people = row().gap(16.0).items_center()
        .child(ctl::PersonPicture::new("JD").view(th))
        .child(ctl::PersonPicture::new("SK").view(th))
        .child(ctl::PersonPicture::new("").view(th))
        .child(ctl::PersonPicture::new("AB").size(56.0).view(th));
    let badges = row().gap(24.0).items_center()
        .child(el().size(24.0, 24.0).center().child(ctl::icon(th, Symbol::Mail, 20.0))
            .child(ctl::InfoBadge::count(3).view(th).absolute(14.0, -4.0)))
        .child(el().size(24.0, 24.0).center().child(ctl::icon(th, Symbol::Chat, 20.0))
            .child(ctl::InfoBadge::dot().view(th).absolute(16.0, 0.0)));
    let expanders = col().w_full().gap(8.0)
        .child(ctl::Expander::new("What's new").expanded(s.expander)
            .content("Reveal highlights, accent switching and the full type ramp landed in this build.")
            .view(th)
            .on_press(|s: &mut State| s.expander = !s.expander))
        .child(ctl::Expander::new("Advanced settings").view(th));
    page_body(th, "Collections", vec![
        section(th, "ListView", list),
        section(th, "PersonPicture", people),
        section(th, "InfoBadge", badges),
        section(th, "Expander", expanders),
    ])
}

fn menus_page(s: &State, th: &Theme) -> El {
    let combo_items = ["Arial", "Segoe UI", "Cascadia Mono", "Inter"]
        .iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let combo = col().gap(8.0)
        .child(ctl::ComboBox::new("Segoe UI").view(th).w(280.0)
            .on_press(|s: &mut State| s.combo_open = !s.combo_open))
        .children(if s.combo_open {
            Some(ctl::ComboBox { selected: s.combo_sel, ..ctl::ComboBox::new("") }
                .popup(th, &combo_items, 280.0))
        } else {
            None
        });
    let menu = ctl::MenuFlyout::new(vec![
        ctl::MenuItem::new("Copy").symbol(Symbol::Copy).shortcut("Ctrl+C"),
        ctl::MenuItem::new("Cut").symbol(Symbol::Cut).shortcut("Ctrl+X"),
        ctl::MenuItem::new("Paste").symbol(Symbol::Paste).shortcut("Ctrl+V"),
        ctl::MenuItem::new("---"),
        ctl::MenuItem::new("Share").symbol(Symbol::Share).submenu(true),
        ctl::MenuItem { selected: true, ..ctl::MenuItem::new("Delete").symbol(Symbol::Delete) },
    ])
    .view(th, 240.0);
    let bar = ctl::CommandBar::new(vec![
        (Symbol::Add, "New".into()),
        (Symbol::Edit, "Edit".into()),
        (Symbol::Share, "Share".into()),
        (Symbol::Delete, "Delete".into()),
    ])
    .view(th);
    let flyout = ctl::Flyout::new("Flyouts float over content with acrylic + elevation.")
        .title("A Flyout")
        .view(th, 300.0);
    page_body(th, "Menus & toolbars", vec![
        section(th, "ComboBox", sample("Closed + open popup", th, combo)),
        section(th, "MenuFlyout", menu),
        section(th, "CommandBar", bar),
        section(th, "Flyout", flyout),
    ])
}

fn navigation_page(s: &State, th: &Theme) -> El {
    let tabs = ctl::TabView::new(vec![
        ctl::Tab::new("Home").symbol(Symbol::Home).selected(s.tab_sel == 0),
        ctl::Tab::new("Documents").symbol(Symbol::Document).selected(s.tab_sel == 1),
        ctl::Tab::new("Settings").symbol(Symbol::Settings).selected(s.tab_sel == 2),
    ])
    .view(th);
    let crumbs = ctl::BreadcrumbBar::new(vec![
        "Home".into(),
        "Documents".into(),
        "Reports".into(),
        "Q4 review".into(),
    ])
    .view(th);
    let mini_nav = el().h(280.0).w(300.0).clip_rounded(8.0)
        .border(1.0, mat::rc(th.tokens().card_stroke_color_default))
        .child(
            ctl::NavigationView { search: false, ..ctl::NavigationView::new("App") }
                .expanded(true)
                .items(vec![
                    ctl::NavItem::new("Dashboard", Symbol::Grid).selected(true),
                    ctl::NavItem::new("Projects", Symbol::Folder),
                    ctl::NavItem::new("Team", Symbol::People),
                ])
                .footer(vec![ctl::NavItem::new("Settings", Symbol::Settings)])
                .view(th),
        );
    page_body(th, "Navigation", vec![
        section(th, "TabView", tabs),
        section(th, "BreadcrumbBar", crumbs),
        section(th, "NavigationView", mini_nav),
    ])
}

fn status_page(_s: &State, th: &Theme) -> El {
    let bars = col().w_full().gap(8.0)
        .child(ctl::InfoBar::new(ctl::Severity::Info, "FYI", "This update brings minor improvements.").view(th))
        .child(ctl::InfoBar::new(ctl::Severity::Success, "Done", "Your changes were saved.").view(th))
        .child(ctl::InfoBar::new(ctl::Severity::Warning, "Careful", "Disk space is running low.").view(th))
        .child(ctl::InfoBar::new(ctl::Severity::Error, "Failed", "The operation couldn't complete.").view(th));
    let progress = col().gap(20.0)
        .child(sample("Determinate 62%", th, ctl::ProgressBar::determinate(0.62).view(th, 400.0)))
        .child(sample("Indeterminate", th, ctl::ProgressBar::indeterminate().view(th, 400.0)))
        .child(row().gap(24.0).items_center()
            .child(ctl::ProgressRing::indeterminate().view(th))
            .child(ctl::ProgressRing::determinate(0.62).view(th)));
    let extras = row().gap(24.0).items_start()
        .child(ctl::TeachingTip::new("Try this", "TeachingTips point at features and explain them inline.").view(th, 280.0))
        .child(col().gap(12.0)
            .child(ctl::ToolTip::new("A tooltip says hello").view(th))
            .child(ctl::Button::new("Hover me").view(th)));
    page_body(th, "Status & info", vec![
        section(th, "InfoBar", bars),
        section(th, "Progress", progress),
        section(th, "TeachingTip & ToolTip", extras),
    ])
}

fn icons_page(th: &Theme) -> El {
    let mut grid = col().w_full().gap(4.0);
    let per_row = 10;
    let syms = winui_icons::Symbol::ALL;
    let mut i = 0;
    while i < syms.len() {
        let mut r = row().gap(4.0);
        for sym in syms.iter().skip(i).take(per_row) {
            r = r.child(
                el().size(64.0, 72.0)
                    .radius(4.0)
                    .center()
                    .hover_fill(mat::rc(th.tokens().subtle_fill_color_secondary))
                    .child(
                        col().items_center().gap(6.0)
                            .child(ctl::glyph(*sym, 20.0, mat::rc(th.tokens().text_fill_color_primary)))
                            .child(
                                text(sym.name().to_string())
                                    .font(rune::TextStyle::new(9.0, 400.0))
                                    .color(mat::rc(th.tokens().text_fill_color_tertiary))
                                    .w(60.0)
                                    .max_lines(1)
                                    .elide_middle(),
                            ),
                    ),
            );
        }
        grid = grid.child(r);
        i += per_row;
    }
    page_body(th, "Icons", vec![section(th, "Fluent System Icons", grid)])
}

fn colors_page(th: &Theme) -> El {
    let t = th.tokens();
    let names = t.named();
    let mut grid = col().w_full().gap(4.0);
    let per_row = 6;
    let mut i = 0;
    while i < names.len() {
        let mut r = row().gap(8.0);
        for (name, c) in names.iter().skip(i).take(per_row) {
            let show = if c.a < 0.05 {
                mat::rc(t.card_stroke_color_default)
            } else {
                mat::rc(*c)
            };
            r = r.child(
                col().w(150.0).gap(4.0)
                    .child(
                        el().w_full()
                            .h(48.0)
                            .radius(4.0)
                            .fill(show)
                            .border(1.0, mat::rc(t.control_stroke_color_default)),
                    )
                    .child(
                        text(name.to_string())
                            .font(rune::TextStyle::new(10.0, 400.0))
                            .color(mat::rc(t.text_fill_color_secondary))
                            .max_lines(1)
                            .elide_middle(),
                    ),
            );
        }
        grid = grid.child(r);
        i += per_row;
    }
    page_body(th, "Colors", vec![section(th, "Theme resources", grid)])
}

fn type_page(th: &Theme) -> El {
    let ramp = th.type_ramp();
    let mut list = col().w_full().gap(12.0);
    for (name, style) in ramp.named() {
        list = list.child(
            row().w_full().items_end().gap(24.0)
                .child(el().w(140.0).child(
                    text(name.to_string())
                        .font(ctl::caption())
                        .color(mat::rc(th.tokens().text_fill_color_tertiary)),
                ))
                .child(
                    text("The quick brown fox jumps over the lazy dog".to_string())
                        .font(ctl::ts(&style))
                        .color(mat::rc(th.tokens().text_fill_color_primary)),
                ),
        );
    }
    page_body(th, "Typography", vec![section(th, "Type ramp", list)])
}

fn settings_page(s: &State, th: &Theme) -> El {
    let dark = s.dark.unwrap_or(false);
    let mode = col().gap(8.0)
        .child(
            ctl::ToggleSwitch::new(dark)
                .header("Dark mode")
                .view(th)
                .on_press(|s: &mut State| {
                    let d = !s.dark.unwrap_or(false);
                    s.dark = Some(d);
                }),
        )
        .child(ctl::caption_txt(th, "Off follows the app's light theme for this snap."));
    let swatches = ACCENTS.iter().map(|(_, c)| mat::rc(*c)).collect::<Vec<_>>();
    let accent = col().gap(8.0)
        .child(
            ctl::ColorSwatchGrid::new(swatches)
                .selected(s.accent)
                .view(th),
        )
        .child(ctl::caption_txt(th, format!("Accent: {}", ACCENTS[s.accent].0)));
    let about = col().gap(4.0)
        .child(ctl::txt(th, ctl::body(), "winui-linux"))
        .child(ctl::caption_txt(th, "Fluent controls in Rust + Rune. github.com/Coobyk/winui-linux"));
    page_body(th, "Settings", vec![
        section(th, "Personalization", col().gap(20.0).child(mode).child(accent)),
        section(th, "About", about),
    ])
}
