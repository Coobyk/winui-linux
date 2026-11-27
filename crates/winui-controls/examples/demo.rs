//! Every control in its Gallery arrangement. Headless:
//! `cargo run -p winui-controls --example demo -- --scene buttons --snap out.png [--theme dark]`.
use rune::{col, row, AppSpec, Cx, El};
use winui_controls::style::{self, rc};
use winui_controls::*;

#[derive(Default)]
struct State {
    scene: &'static str,
    toggled: bool,
    check: selection::CheckState,
    radio: Option<usize>,
    switch: bool,
    rating: u32,
    open: bool,
    slider: f64,
    color: winui_controls::shim::theme::Color,
    name: rune::TextEdit,
    password: rune::TextEdit,
    number: rune::TextEdit,
    query: rune::TextEdit,
    combo: Option<usize>,
    nav: usize,
    pane: bool,
    tab: usize,
    expanded: bool,
    list: Option<usize>,
}

const FONTS: &[&[u8]] = &[
    include_bytes!("../assets/Inter-Regular.ttf"),
    include_bytes!("../assets/Inter-Medium.ttf"),
    include_bytes!("../assets/Inter-SemiBold.ttf"),
];

macro_rules! scenes {
    ($($name:literal),* $(,)?) => {
        &[$(($name, |s: &mut State| s.scene = $name)),*]
    };
}

const SCENES: &[(&str, fn(&mut State))] = scenes![
    "buttons", "selection", "slider", "colorpicker", "text", "menus", "navigation", "tabview", "infobar", "progress",
    "tooltip", "expander", "listview", "people",
];

fn init() -> State {
    State {
        scene: "buttons",
        check: selection::CheckState::Checked,
        radio: Some(0),
        switch: true,
        rating: 3,
        open: true,
        slider: 40.0,
        color: winui_controls::shim::theme::Color::rgb(0x00, 0x78, 0xD4),
        name: rune::TextEdit::new("Ada Lovelace"),
        password: rune::TextEdit::new("hunter2"),
        number: rune::TextEdit::new("42"),
        query: rune::TextEdit::new("ca"),
        combo: Some(1),
        nav: 1,
        pane: true,
        tab: 0,
        expanded: true,
        list: Some(1),
        ..Default::default()
    }
}

fn section(t: &Theme, title: &str, body: El) -> El {
    let k = t.tokens();
    col()
        .gap(12.0)
        .child(rune::text(title.to_owned()).font(style::body_strong(t)).color(rc(k.text_fill_color_primary)))
        .child(style::card(t).padding(16.0).gap(12.0).child(body))
}

fn buttons(s: &State, t: &Theme) -> El {
    col()
        .gap(24.0)
        .child(section(
            t,
            "Button",
            row().gap(8.0).center_y()
                .child(Button::new("Standard XAML button").on_press(|_: &mut State| {}).view(t))
                .child(Button::new("Accent").accent(true).view(t))
                .child(Button::new("Disabled").enabled(false).view(t))
                .child(Button::new("Save").icon(Symbol::Save).view(t)),
        ))
        .child(section(
            t,
            "ToggleButton, HyperlinkButton, RepeatButton",
            row().gap(8.0).center_y()
                .child(ToggleButton::new("ToggleButton").checked(s.toggled).on_toggle(|s: &mut State, v| s.toggled = v).view(t))
                .child(ToggleButton::new("Checked").checked(true).view(t))
                .child(HyperlinkButton::new("Microsoft home page").view(t))
                .child(RepeatButton::new("Click and hold").view(t)),
        ))
        .child(section(
            t,
            "DropDownButton, SplitButton",
            row().gap(8.0).center_y()
                .child(DropDownButton::new("Email").icon(Symbol::Mail).view(t))
                .child(SplitButton::new("Choose color").view(t))
                .child(SplitButton::new("Accent split").accent(true).view(t))
                .child(SplitButton::new("Disabled").enabled(false).view(t)),
        ))
}

fn selection(s: &State, t: &Theme) -> El {
    col()
        .gap(24.0)
        .child(section(
            t,
            "CheckBox",
            row().gap(24.0)
                .child(CheckBox::new("Two-state CheckBox").state(s.check).on_toggle(|s: &mut State, v| s.check = v).view(t))
                .child(CheckBox::new("Unchecked").view(t))
                .child(CheckBox::new("Indeterminate").state(selection::CheckState::Indeterminate).view(t))
                .child(CheckBox::new("Disabled").checked(true).enabled(false).view(t)),
        ))
        .child(section(
            t,
            "RadioButtons, ToggleSwitch, RatingControl",
            row().gap(48.0)
                .child(RadioButtons::new(["Option 1", "Option 2", "Option 3"]).header("Options").selected(s.radio).on_select(|s: &mut State, i| s.radio = Some(i)).view(t))
                .child(col().gap(8.0)
                    .child(ToggleSwitch::new().header("Toggle work").on(s.switch).on_toggle(|s: &mut State, v| s.switch = v).view(t))
                    .child(ToggleSwitch::new().view(t))
                    .child(ToggleSwitch::new().on(true).enabled(false).view(t)))
                .child(RatingControl::new().value(s.rating as f32).caption("(12 ratings)").on_rate(|s: &mut State, v| s.rating = v).view(t)),
        ))
}

fn slider(s: &State, t: &Theme) -> El {
    section(
        t,
        "Slider",
        row().gap(48.0)
            .child(Slider::new().header("Volume").value(s.slider).on_change(|s: &mut State, v| s.slider = v).view(t))
            .child(Slider::new().header("Disabled").value(70.0).enabled(false).view(t)),
    )
}

fn text(s: &State, t: &Theme) -> El {
    col().gap(24.0)
        .child(section(t, "TextBox", row().gap(24.0)
            .child(TextBox::new(&s.name).header("Name").placeholder("Enter your name").on_edit(|s: &mut State| &mut s.name).view(t))
            .child(TextBox::new(&rune::TextEdit::new("")).header("Focused").placeholder("Type here").focused(true).key("tb2").view(t))
            .child(TextBox::new(&rune::TextEdit::new("Read only")).header("Disabled").enabled(false).key("tb3").view(t))))
        .child(section(t, "PasswordBox, NumberBox", row().gap(24.0)
            .child(PasswordBox::new(&s.password).header("Password").on_edit(|s: &mut State| &mut s.password).view(t))
            .child(NumberBox::new(42.0, &s.number).header("Quantity").range(0.0, 100.0).on_edit(|s: &mut State| &mut s.number).view(t))))
        .child(section(t, "AutoSuggestBox", AutoSuggestBox::new(&s.query).placeholder("Search controls").focused(true)
            .suggestions(AutoSuggestBox::filter(&["Calendar", "Canvas", "Card", "CheckBox", "ComboBox"], &s.query.text))
            .open(s.open).on_edit(|s: &mut State| &mut s.query).view(t)))
}

fn menus(s: &State, t: &Theme) -> El {
    let menu = MenuFlyout::new([
        MenuItem::item("Cut").icon(Symbol::Cut).accelerator("Ctrl+X"),
        MenuItem::item("Copy").icon(Symbol::Copy).accelerator("Ctrl+C"),
        MenuItem::item("Paste").icon(Symbol::Paste).accelerator("Ctrl+V").enabled(false),
        MenuItem::Separator,
        MenuItem::toggle("Word wrap", true),
    ]);
    col().gap(24.0)
        .child(section(t, "CommandBar", CommandBar::new([
            AppBarItem::button(Symbol::Add, "Add"),
            AppBarItem::button(Symbol::Edit, "Edit"),
            AppBarItem::button(Symbol::Share, "Share"),
            AppBarItem::Separator,
            AppBarItem::toggle(Symbol::Favorite, "Favorite", true),
        ]).view(t)))
        .child(section(t, "ComboBox, DropDownButton + MenuFlyout", row().gap(48.0).items_start().h(260.0)
            .child(ComboBox::new(["Blue", "Green", "Red", "Yellow"]).header("Colors").selected(s.combo).open(s.open)
                .on_select(|s: &mut State, i| s.combo = Some(i)).on_toggle(|s: &mut State, v| s.open = v).view(t))
            .child(ComboBox::new(["A"]).header("Placeholder").placeholder("Pick a font").key("combo2").view(t))
            .child(DropDownButton::new("Edit").open(s.open).flyout(menu).view(t))))
}

fn navigation(s: &State, t: &Theme) -> El {
    let k = t.tokens();
    let nav = NavigationView::new([
        NavItem::new(Symbol::Home, "Home"),
        NavItem::new(Symbol::Apps, "All controls"),
        NavItem::new(Symbol::Mail, "Inbox").badge(5),
        NavItem::new(Symbol::Document, "Documents"),
    ])
    .selected(s.nav)
    .pane_open(s.pane)
    .header("All controls")
    .on_select(|s: &mut State, i| s.nav = i)
    .on_toggle_pane(|s: &mut State, v| s.pane = v)
    .content(|t| col().gap(16.0)
        .child(BreadcrumbBar::new(["Home", "Controls", "Navigation"]).view(t))
        .child(InfoBar::new("NavigationView", "Left pane, 320px open / 48px compact.").view(t)));
    col().w_full().h_full().fill(rc(k.solid_background_fill_color_base)).child(nav.view(t))
}

fn tabview(s: &State, t: &Theme) -> El {
    TabView::new([Tab::new("Document 0").icon(Symbol::Document), Tab::new("Document 1").icon(Symbol::Document), Tab::new("Document 2").icon(Symbol::Document)])
        .selected(s.tab)
        .on_select(|s: &mut State, i| s.tab = i)
        .content(|t, i| rune::text(format!("Document {i}")).font(style::body(t)).color(rc(t.tokens().text_fill_color_primary)))
        .view(t)
        .h(240.0)
}

fn infobar(t: &Theme) -> El {
    col().gap(12.0)
        .child(InfoBar::new("Title", "Essential app message for your users to be informed of, acknowledge, or take action on.").view(t))
        .child(InfoBar::new("Success", "Your changes were saved.").severity(status::Severity::Success).view(t))
        .child(InfoBar::new("Warning", "Your trial expires in 3 days.").severity(status::Severity::Warning).action(Button::new("Upgrade")).view(t))
        .child(InfoBar::new("Error", "The file couldn't be opened.").severity(status::Severity::Error).closable(false).view(t))
}

fn progress(t: &Theme) -> El {
    col().gap(24.0)
        .child(section(t, "ProgressBar", col().gap(24.0)
            .child(ProgressBar::new().width(300.0).view(t))
            .child(ProgressBar::new().value(60.0).width(300.0).view(t))
            .child(ProgressBar::new().value(40.0).state(status::ProgressState::Paused).width(300.0).view(t))
            .child(ProgressBar::new().value(80.0).state(status::ProgressState::Error).width(300.0).view(t))))
        .child(section(t, "ProgressRing", row().gap(32.0).center_y()
            .child(ProgressRing::new().view(t))
            .child(ProgressRing::new().value(70.0).size(48.0).view(t))
            .child(ProgressRing::new().value(30.0).size(64.0).state(status::ProgressState::Paused).view(t))))
}

fn tooltip(t: &Theme) -> El {
    section(t, "ToolTip, TeachingTip", row().gap(160.0).items_start().h(320.0).pt(48.0)
        .child(col().child(Button::new("Hover for tooltip").key("tipbtn").view(t)).child(ToolTip::new("Simple ToolTip").anchor("tipbtn").view(t)))
        .child(col().child(Button::new("Show TeachingTip").key("ttbtn").view(t))
            .child(TeachingTip::new("Change themes without hassle", "It's easier than ever to see control samples in both light and dark theme!")
                .icon(Symbol::Info).action(Button::new("Got it")).close_text("Close").anchor("ttbtn").view(t))))
}

fn expander(s: &State, t: &Theme) -> El {
    col().gap(12.0).w(480.0)
        .child(Expander::new("This text is in the header", |t| rune::text("This is in the content").font(style::body(t)).color(rc(t.tokens().text_fill_color_primary))).expanded(s.expanded).on_toggle(|s: &mut State, v| s.expanded = v).view(t))
        .child(Expander::new("Collapsed with icon", |_| rune::el()).description("And a description").icon(Symbol::Settings).view(t))
}

fn listview(s: &State, t: &Theme) -> El {
    section(t, "ListView", ListView::new([
        ListItem::new("Kendall Collins").subtitle("Adatum Corporation").icon(Symbol::Contact),
        ListItem::new("Henry Ross").subtitle("Adventure Works Cycles").icon(Symbol::Contact),
        ListItem::new("Vance DeLeon").subtitle("Alpine Ski House").icon(Symbol::Contact),
        ListItem::new("Plain row"),
    ]).selected(s.list).width(360.0).on_select(|s: &mut State, i| s.list = Some(i)).view(t))
}

fn people(t: &Theme) -> El {
    let mut icons = row().gap(16.0).flex_wrap().w(640.0);
    for sym in Symbol::ALL {
        icons = icons.child(IconElement::new(*sym).size(20.0).view(t));
    }
    col().gap(24.0)
        .child(section(t, "PersonPicture", row().gap(24.0).center_y()
            .child(PersonPicture::new().display_name("Jane Doe").view(t))
            .child(PersonPicture::new().display_name("John").badge(3).size(64.0).view(t))
            .child(PersonPicture::new().size(48.0).view(t))))
        .child(section(t, "InfoBadge", row().gap(24.0).center_y()
            .child(InfoBadge::dot().view(t))
            .child(InfoBadge::value(8).view(t))
            .child(InfoBadge::value(128).severity(content::BadgeSeverity::Critical).view(t))
            .child(InfoBadge::icon(Symbol::Accept).severity(content::BadgeSeverity::Success).view(t))
            .child(InfoBadge::icon(Symbol::Dismiss).severity(content::BadgeSeverity::Caution).view(t))))
        .child(section(t, "IconElement", icons))
}

fn view(s: &State, cx: &mut Cx<State>) -> El {
    let t = if cx.system_dark() { Theme::dark() } else { Theme::light() };
    let k = t.tokens();
    let body = match s.scene {
        "selection" => selection(s, &t),
        "slider" => slider(s, &t),
        "colorpicker" => section(&t, "ColorPicker", ColorPicker::new(s.color).on_change(|s: &mut State, c| s.color = c).view(&t)),
        "text" => text(s, &t),
        "menus" => menus(s, &t),
        "navigation" => return navigation(s, &t),
        "tabview" => tabview(s, &t),
        "infobar" => infobar(&t),
        "progress" => progress(&t),
        "tooltip" => tooltip(&t),
        "expander" => expander(s, &t),
        "listview" => listview(s, &t),
        "people" => people(&t),
        _ => buttons(s, &t),
    };
    col()
        .w_full()
        .h_full()
        .fill(rc(k.solid_background_fill_color_base))
        .padding(36.0)
        .gap(24.0)
        .child(rune::text(s.scene.to_owned()).font(style::font(t.type_ramp().title)).color(rc(k.text_fill_color_primary)))
        .child(body)
}

fn main() {
    rune::run(AppSpec {
        scenes: SCENES,
        fonts: FONTS,
        families: ("Inter", "ui-monospace"),
        ..AppSpec::new("winui-controls demo", (1000.0, 720.0), init, view)
    });
}
