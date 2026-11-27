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

const SCENES: &[(&str, fn(&mut State))] = scenes!["buttons", "selection"];

fn init() -> State {
    State { scene: "buttons", check: selection::CheckState::Checked, radio: Some(0), switch: true, rating: 3, ..Default::default() }
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

fn view(s: &State, cx: &mut Cx<State>) -> El {
    let t = if cx.system_dark() { Theme::dark() } else { Theme::light() };
    let k = t.tokens();
    let body = match s.scene {
        "selection" => selection(s, &t),
        _ => buttons(s, &t),
    };
    let _ = s.open;
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
