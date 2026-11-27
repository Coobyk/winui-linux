//! WinUI 3 Gallery-style showcase app. See docs/ARCHITECTURE.md.
//!
//! `cargo run -p winui-gallery -- --scene buttons --theme dark --snap out.png`
//! renders a page headless; `--scene`, `--theme`, `--snap`, `--at` and `--size`
//! are Rune's headless CLI (scenes are the page ids below).

#![forbid(unsafe_code)]

mod apps;
mod controls;
mod fluent;
mod pages;

use fluent::{c, font, icon};
use rune::{AppSpec, Cx, El, ScrollSpec, col, el, row, text};
use winui_icons::Symbol;
use winui_theme::{AccentRamp, Color, Theme, ThemeMode, radii, sizes};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Home,
    Buttons,
    Input,
    Collections,
    Menus,
    Navigation,
    Status,
    Media,
    Icons,
    Colors,
    Type,
    Settings,
    App(apps::App),
}

impl Page {
    pub const ALL: [Page; 19] = [
        Page::Home,
        Page::Buttons,
        Page::Input,
        Page::Collections,
        Page::Menus,
        Page::Navigation,
        Page::Status,
        Page::Media,
        Page::Icons,
        Page::Colors,
        Page::Type,
        Page::Settings,
        Page::App(apps::App::Settings),
        Page::App(apps::App::Calculator),
        Page::App(apps::App::Clock),
        Page::App(apps::App::ClockSettings),
        Page::App(apps::App::Store),
        Page::App(apps::App::Snipping),
        Page::App(apps::App::Terminal),
    ];
    pub fn id(self) -> &'static str {
        match self {
            Page::Home => "home",
            Page::Buttons => "buttons",
            Page::Input => "input",
            Page::Collections => "collections",
            Page::Menus => "menus",
            Page::Navigation => "navigation",
            Page::Status => "status",
            Page::Media => "media",
            Page::Icons => "styles-icons",
            Page::Colors => "styles-colors",
            Page::Type => "styles-type",
            Page::Settings => "settings",
            Page::App(a) => a.id(),
        }
    }
    fn title(self) -> &'static str {
        match self {
            Page::Home => "Home",
            Page::Buttons => "Basic input: Buttons",
            Page::Input => "Basic input",
            Page::Collections => "Collections",
            Page::Menus => "Menus & toolbars",
            Page::Navigation => "Navigation",
            Page::Status => "Status & info",
            Page::Media => "Media",
            Page::Icons => "Iconography",
            Page::Colors => "Color",
            Page::Type => "Typography",
            Page::Settings => "Settings",
            Page::App(a) => a.title(),
        }
    }
    fn nav_label(self) -> &'static str {
        match self {
            Page::Buttons => "Buttons",
            Page::Input => "Basic input",
            p => p.title(),
        }
    }
    fn symbol(self) -> Symbol {
        match self {
            Page::Home => Symbol::Home,
            Page::Buttons => Symbol::CursorClick,
            Page::Input => Symbol::CheckboxChecked,
            Page::Collections => Symbol::Grid,
            Page::Menus => Symbol::Board,
            Page::Navigation => Symbol::Navigation,
            Page::Status => Symbol::Info,
            Page::Media => Symbol::Image,
            Page::Icons => Symbol::Icons,
            Page::Colors => Symbol::Color,
            Page::Type => Symbol::TextFont,
            Page::Settings => Symbol::Settings,
            Page::App(a) => a.icon(),
        }
    }
    pub fn from_id(id: &str) -> Option<Page> {
        Page::ALL.into_iter().find(|p| p.id() == id)
    }
}

/// Windows 11 Settings > Personalization > Colors accents (first row + a few).
pub const ACCENTS: &[(&str, Color)] = &[
    ("Default blue", Color::from_argb(0xFF0078D4)),
    ("Navy blue", Color::from_argb(0xFF0063B1)),
    ("Purple", Color::from_argb(0xFF8764B8)),
    ("Orchid", Color::from_argb(0xFF881798)),
    ("Rose", Color::from_argb(0xFFE3008C)),
    ("Red", Color::from_argb(0xFFE81123)),
    ("Orange", Color::from_argb(0xFFCA5010)),
    ("Green", Color::from_argb(0xFF107C10)),
    ("Teal", Color::from_argb(0xFF038387)),
    ("Steel", Color::from_argb(0xFF68768A)),
];

pub struct State {
    pub page: Page,
    /// None follows the system (Rune's `--theme` sets that headless).
    pub theme: Option<ThemeMode>,
    pub dark_system: bool,
    pub accent: usize,
    pub toggle: bool,
    pub slider: f32,
    pub search: String,
    /// App-clone scenes render the clone as the whole window, without the gallery shell.
    pub full_app: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            page: Page::Home,
            theme: None,
            dark_system: false,
            accent: 0,
            toggle: true,
            slider: 0.5,
            search: String::new(),
            full_app: false,
        }
    }
}

impl State {
    pub fn mode(&self) -> ThemeMode {
        self.theme.unwrap_or(if self.dark_system {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        })
    }
    #[cfg(test)]
    pub fn toggle_theme(&mut self) {
        self.theme = Some(self.mode().toggled());
    }
    pub fn theme(&self) -> Theme {
        let ramp = if self.accent == 0 {
            AccentRamp::DEFAULT
        } else {
            AccentRamp::derive(ACCENTS[self.accent].1)
        };
        Theme::new(self.mode(), ramp)
    }
}

const NAV_GROUPS: &[(Option<&str>, &[Page])] = &[
    (None, &[Page::Home]),
    (
        Some("Controls"),
        &[
            Page::Buttons,
            Page::Input,
            Page::Collections,
            Page::Menus,
            Page::Navigation,
            Page::Status,
            Page::Media,
        ],
    ),
    (Some("Design"), &[Page::Icons, Page::Colors, Page::Type]),
    (
        Some("Apps"),
        &[
            Page::App(apps::App::Settings),
            Page::App(apps::App::Calculator),
            Page::App(apps::App::Clock),
            Page::App(apps::App::ClockSettings),
            Page::App(apps::App::Store),
            Page::App(apps::App::Snipping),
            Page::App(apps::App::Terminal),
        ],
    ),
];

fn nav_item(s: &State, t: &Theme, p: Page) -> El {
    let k = t.tokens();
    let sel = s.page == p;
    let mut r = row()
        .key(format!("nav-{}", p.id()))
        .role("tab")
        .label(p.nav_label())
        .h(sizes::NAV_ITEM_HEIGHT)
        .mx(sizes::NAV_ITEM_MARGIN.left)
        .my(sizes::NAV_ITEM_MARGIN.top)
        .items_center()
        .radius(radii::CONTROL)
        .hover(c(k.subtle_fill_color_secondary))
        .press(c(k.subtle_fill_color_tertiary))
        .focusable(true)
        .on_press(move |s: &mut State| s.page = p);
    if sel {
        r = r.fill(c(k.subtle_fill_color_secondary)).child(
            el().size(
                sizes::NAV_SELECTION_INDICATOR_WIDTH,
                sizes::NAV_SELECTION_INDICATOR_HEIGHT,
            )
            .radius(radii::SELECTION_INDICATOR)
            .fill(c(k.accent_fill_color_default))
            .absolute(
                0.0,
                (sizes::NAV_ITEM_HEIGHT - sizes::NAV_SELECTION_INDICATOR_HEIGHT) / 2.0,
            ),
        );
    }
    r.child(
        row()
            .w(sizes::NAV_ICON_BOX_WIDTH)
            .h_full()
            .center()
            .child(icon(
                p.symbol(),
                sizes::NAV_ICON_SIZE,
                k.text_fill_color_primary,
            )),
    )
    .child(
        text(p.nav_label())
            .font(font(t.type_ramp().body))
            .color(c(k.text_fill_color_primary))
            .truncate(),
    )
}

fn pane(s: &State, t: &Theme) -> El {
    let k = t.tokens();
    let mut items = col()
        .w_full()
        .min_h(0.0)
        .grow(1.0)
        .scroll(ScrollSpec::default());
    let mut list = col().w_full();
    for (title, pages) in NAV_GROUPS {
        if let Some(title) = title {
            list = list
                .child(
                    el().h(1.0)
                        .mx(16.0)
                        .my(4.0)
                        .fill(c(k.divider_stroke_color_default)),
                )
                .child(
                    row().h(40.0).pl(16.0).items_center().child(
                        text(*title)
                            .font(font(t.type_ramp().body_strong))
                            .color(c(k.text_fill_color_primary)),
                    ),
                );
        }
        for p in *pages {
            list = list.child(nav_item(s, t, *p));
        }
    }
    items = items.child(list);
    let search = controls::auto_suggest(
        t,
        "Search controls and samples...",
        sizes::NAV_OPEN_PANE_LENGTH - 32.0,
    )
    .mx(16.0)
    .my(4.0);
    let toggle = row()
        .h(sizes::PANE_TOGGLE_BUTTON_HEIGHT)
        .w(sizes::PANE_TOGGLE_BUTTON_WIDTH)
        .mx(4.0)
        .center()
        .radius(radii::CONTROL)
        .hover(c(k.subtle_fill_color_secondary))
        .child(icon(
            Symbol::GlobalNavigation,
            16.0,
            k.text_fill_color_primary,
        ));
    let acrylic = fluent::acrylic_default(t).pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0));
    col()
        .w(sizes::NAV_OPEN_PANE_LENGTH)
        .h_full()
        .min_h(0.0)
        .child(acrylic)
        .child(toggle)
        .child(search)
        .child(items)
        .child(
            el().h(1.0)
                .mx(16.0)
                .my(4.0)
                .fill(c(k.divider_stroke_color_default)),
        )
        .child(nav_item(s, t, Page::Settings).mb(4.0))
}

fn title_bar(t: &Theme) -> El {
    let k = t.tokens();
    row()
        .w_full()
        .h(48.0)
        .items_center()
        .pl(16.0)
        .gap(16.0)
        .window_drag()
        .child(el().size(16.0, 16.0).radius(3.0).linear_gradient(
            std::f32::consts::PI * 0.75,
            [(0.0, c(t.accent().light2)), (1.0, c(t.accent().dark1))],
        ))
        .child(
            text("WinUI 3 Gallery")
                .font(font(t.type_ramp().caption))
                .color(c(k.text_fill_color_primary)),
        )
        .child(
            text("Preview")
                .font(font(t.type_ramp().caption))
                .color(c(k.text_fill_color_secondary)),
        )
}

fn view(s: &State, cx: &mut Cx<State>) -> El {
    let sys = cx.system_dark();
    let mut s2;
    let s = if s.theme.is_none() && s.dark_system != sys {
        s2 = State {
            search: s.search.clone(),
            ..*s
        };
        s2.dark_system = sys;
        &s2
    } else {
        s
    };
    let t = s.theme();
    if let (Page::App(a), true) = (s.page, s.full_app) {
        return a.window(&t);
    }
    let k = t.tokens();
    let content = col()
        .w_full()
        .h_full()
        .min_h(0.0)
        .grow(1.0)
        .scroll(ScrollSpec::default())
        .key(format!("content-{}", s.page.id()))
        .child(
            col()
                .w_full()
                .max_w(1000.0 + 112.0)
                .px(56.0)
                .pt(if s.page == Page::Home { 0.0 } else { 36.0 })
                .pb(36.0)
                .child(pages::page(s, &t)),
        );
    let frame = col()
        .grow(1.0)
        .h_full()
        .min_h(0.0)
        .corners([radii::OVERLAY, 0.0, 0.0, 0.0])
        .fill(c(k.layer_fill_color_default))
        .border_sides([1.0, 1.0, 0.0, 0.0], c(k.card_stroke_color_default))
        .clip_corners([radii::OVERLAY, 0.0, 0.0, 0.0])
        .child(content);
    let body = row()
        .w_full()
        .grow(1.0)
        .min_h(0.0)
        .child(pane(s, &t))
        .child(frame);
    let root = col().w_full().h_full().child(title_bar(&t)).child(body);
    el().w_full()
        .h_full()
        .child(fluent::mica(&t).pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0)))
        .child(root.pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0)))
}

macro_rules! scenes {
    ($($id:literal => $p:expr),* $(,)?) => { &[$(($id, |s: &mut State| { s.page = $p; s.full_app = s.page.id() == $id && $id.starts_with("app-"); })),*] };
}

const fn app(a: apps::App) -> Page {
    Page::App(a)
}

pub fn spec() -> AppSpec<State> {
    AppSpec {
        scenes: scenes![
            "home" => Page::Home, "buttons" => Page::Buttons, "input" => Page::Input, "collections" => Page::Collections,
            "menus" => Page::Menus, "navigation" => Page::Navigation, "status" => Page::Status, "media" => Page::Media,
            "styles-icons" => Page::Icons, "styles-colors" => Page::Colors, "styles-type" => Page::Type, "settings" => Page::Settings,
            "app-settings" => app(apps::App::Settings), "app-calculator" => app(apps::App::Calculator),
            "app-clock" => app(apps::App::Clock), "app-clock-settings" => app(apps::App::ClockSettings),
            "app-store" => app(apps::App::Store), "app-snipping" => app(apps::App::Snipping),
            "app-terminal" => app(apps::App::Terminal),
        ],
        families: (fluent::families(), "ui-monospace"),
        min_size: Some((640.0, 480.0)),
        ..AppSpec::new("WinUI 3 Gallery", (1280.0, 800.0), State::default, view)
    }
}

/// App-clone scenes default to their reference screenshot's window size, so
/// `--scene app-calculator --snap out.png` frames like `docs/reference/calculator.png`.
fn argv() -> Vec<String> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let scene = args
        .iter()
        .position(|a| a == "--scene")
        .and_then(|i| args.get(i + 1));
    if let Some(a) = scene.and_then(|id| apps::App::from_id(id)) {
        if !args.iter().any(|a| a == "--size") {
            let (w, h) = a.size();
            args.extend(["--size".into(), format!("{w}x{h}")]);
        }
        if !args.iter().any(|a| a == "--scale") {
            args.extend(["--scale".into(), "1".into()]);
        }
    }
    args
}

fn main() {
    rune::App::new(spec()).args(argv()).run();
}

#[cfg(test)]
mod tests;
