//! Gallery state: current page, search, theme override, accent, and per-sample state.

use crate::fallback::icons::Symbol;
use crate::fallback::theme::{AccentRamp, Color, Theme, ThemeMode};
use rune::TextEdit;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page {
    Home,
    Buttons,
    Input,
    Collections,
    Menus,
    Navigation,
    Status,
    Media,
    StylesIcons,
    StylesColors,
    StylesType,
    Settings,
}

impl Page {
    pub const ALL: [Page; 12] = [
        Page::Home,
        Page::Buttons,
        Page::Input,
        Page::Collections,
        Page::Menus,
        Page::Navigation,
        Page::Status,
        Page::Media,
        Page::StylesIcons,
        Page::StylesColors,
        Page::StylesType,
        Page::Settings,
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
            Page::StylesIcons => "styles-icons",
            Page::StylesColors => "styles-colors",
            Page::StylesType => "styles-type",
            Page::Settings => "settings",
        }
    }

    pub fn from_id(id: &str) -> Option<Page> {
        Page::ALL.into_iter().find(|p| p.id() == id)
    }

    pub fn title(self) -> &'static str {
        match self {
            Page::Home => "Home",
            Page::Buttons => "Basic input: Buttons",
            Page::Input => "Basic input: Selection & text",
            Page::Collections => "Collections",
            Page::Menus => "Menus & toolbars",
            Page::Navigation => "Navigation",
            Page::Status => "Status & info",
            Page::Media => "Media & content",
            Page::StylesIcons => "Iconography",
            Page::StylesColors => "Color",
            Page::StylesType => "Typography",
            Page::Settings => "Settings",
        }
    }

    /// Nav pane label.
    pub fn nav_label(self) -> &'static str {
        match self {
            Page::Buttons => "Buttons",
            Page::Input => "Input",
            Page::Menus => "Menus & toolbars",
            Page::Status => "Status & info",
            Page::Media => "Media",
            Page::StylesIcons => "Icons",
            Page::StylesColors => "Colors",
            Page::StylesType => "Typography",
            p => p.title(),
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Page::Home => "Explore WinUI 3 controls and Fluent styles, rendered by Rune on Linux.",
            Page::Buttons => "Button, HyperlinkButton, ToggleButton, RepeatButton, SplitButton and DropDownButton.",
            Page::Input => "CheckBox, RadioButtons, ToggleSwitch, Slider, RatingControl and text inputs.",
            Page::Collections => "ListView and GridView with selection.",
            Page::Menus => "ComboBox, MenuFlyout, Flyout and CommandBar.",
            Page::Navigation => "NavigationView, TabView, BreadcrumbBar and SelectorBar.",
            Page::Status => "InfoBar, ProgressBar, ProgressRing, ToolTip, TeachingTip and InfoBadge.",
            Page::Media => "Image, PersonPicture, media transport and Expander.",
            Page::StylesIcons => "Fluent System Icons used across the controls.",
            Page::StylesColors => "Theme brushes: text, fills, strokes, backgrounds, signal and accent.",
            Page::StylesType => "The Fluent type ramp, Caption to Display.",
            Page::Settings => "Theme and accent colour.",
        }
    }

    pub fn icon(self) -> Symbol {
        match self {
            Page::Home => Symbol::Home,
            Page::Buttons => Symbol::CursorClick,
            Page::Input => Symbol::CheckboxChecked,
            Page::Collections => Symbol::List,
            Page::Menus => Symbol::Window,
            Page::Navigation => Symbol::Navigation,
            Page::Status => Symbol::Info,
            Page::Media => Symbol::Image,
            Page::StylesIcons => Symbol::Icons,
            Page::StylesColors => Symbol::Color,
            Page::StylesType => Symbol::TextFont,
            Page::Settings => Symbol::Settings,
        }
    }
}

/// Nav pane groups: (header, pages). `None` header = top-level items.
pub const NAV_GROUPS: &[(Option<&str>, &[Page])] = &[
    (None, &[Page::Home]),
    (Some("Controls"), &[Page::Buttons, Page::Input, Page::Collections, Page::Menus, Page::Navigation, Page::Status, Page::Media]),
    (Some("Styles"), &[Page::StylesIcons, Page::StylesColors, Page::StylesType]),
];

/// Accent picker swatches (Windows personalization palette subset).
pub const ACCENTS: &[(&str, Color)] = &[
    ("Default blue", AccentRamp::DEFAULT_ACCENT),
    ("Gold", Color::rgb(0xFF, 0xB9, 0x00)),
    ("Orange", Color::rgb(0xF7, 0x63, 0x0C)),
    ("Rust", Color::rgb(0xDA, 0x3B, 0x01)),
    ("Red", Color::rgb(0xE8, 0x11, 0x23)),
    ("Rose", Color::rgb(0xEA, 0x00, 0x5E)),
    ("Plum", Color::rgb(0xC3, 0x00, 0x52)),
    ("Orchid", Color::rgb(0xB1, 0x46, 0xC2)),
    ("Violet", Color::rgb(0x88, 0x17, 0x98)),
    ("Iris", Color::rgb(0x6B, 0x69, 0xD6)),
    ("Navy", Color::rgb(0x00, 0x63, 0xB1)),
    ("Teal", Color::rgb(0x00, 0x99, 0xBC)),
    ("Sea", Color::rgb(0x03, 0x83, 0x87)),
    ("Mint", Color::rgb(0x00, 0xB2, 0x94)),
    ("Green", Color::rgb(0x10, 0x7C, 0x10)),
    ("Slate", Color::rgb(0x68, 0x76, 0x8A)),
];

pub struct State {
    pub page: Page,
    pub search: TextEdit,
    /// `None` follows the system / `--theme`.
    pub theme_override: Option<ThemeMode>,
    pub accent: Color,
    pub pane_open: bool,
    // Buttons
    pub clicks: u32,
    pub toggled: bool,
    pub repeats: u32,
    pub split_color: usize,
    // Input
    pub check: bool,
    pub tri: Option<bool>,
    pub radio: usize,
    pub switch_on: bool,
    pub slider: f32,
    pub stars: u8,
    pub text: TextEdit,
    pub password: TextEdit,
    pub number: i32,
    // Collections
    pub list_sel: usize,
    pub grid_sel: usize,
    // Menus
    pub combo_open: bool,
    pub combo_sel: usize,
    pub menu_last: &'static str,
    pub bold: bool,
    // Navigation
    pub nav_sel: usize,
    pub tabs: Vec<String>,
    pub tab_sel: usize,
    pub next_tab: u32,
    pub crumbs: usize,
    pub selector: usize,
    // Status
    pub infobars: [bool; 4],
    pub tip_open: bool,
    // Media
    pub expander_open: bool,
    pub playing: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            page: Page::Home,
            search: TextEdit::new(""),
            theme_override: None,
            accent: AccentRamp::DEFAULT_ACCENT,
            pane_open: true,
            clicks: 0,
            toggled: false,
            repeats: 0,
            split_color: 0,
            check: true,
            tri: None,
            radio: 0,
            switch_on: true,
            slider: 0.4,
            stars: 3,
            text: TextEdit::new(""),
            password: TextEdit::new("hunter2"),
            number: 42,
            list_sel: 1,
            grid_sel: 0,
            combo_open: false,
            combo_sel: 0,
            menu_last: "",
            bold: false,
            nav_sel: 0,
            tabs: vec!["Document 0".into(), "Document 1".into(), "Document 2".into()],
            tab_sel: 0,
            next_tab: 3,
            crumbs: 4,
            selector: 0,
            infobars: [true; 4],
            tip_open: true,
            expander_open: true,
            playing: false,
        }
    }
}

impl State {
    /// Initial state from gallery-only args (`--accent #RRGGBB`) left over by Rune's parser.
    pub fn from_args() -> State {
        let mut s = State::default();
        let extra = rune::extra_args();
        let mut it = extra.iter();
        while let Some(a) = it.next() {
            if a == "--accent" {
                if let Some(c) = it.next().and_then(|v| Color::parse(v)) {
                    s.accent = c;
                }
            }
        }
        s
    }

    pub fn query(&self) -> &str {
        self.search.text.trim()
    }

    pub fn set_search(&mut self, q: &str) {
        self.search = TextEdit::new(q);
    }

    pub fn navigate(&mut self, page: Page) {
        self.page = page;
        self.search = TextEdit::new("");
        self.combo_open = false;
    }

    pub fn mode(&self, system_dark: bool) -> ThemeMode {
        self.theme_override.unwrap_or(if system_dark { ThemeMode::Dark } else { ThemeMode::Light })
    }

    pub fn theme(&self, system_dark: bool) -> Theme {
        Theme::new(self.mode(system_dark), self.accent)
    }
}
