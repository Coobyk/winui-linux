//! Print the accent ramp derived for an accent colour:
//! `cargo run -p winui-theme --example ramp -- '#881798'`.
fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "#0078D4".into());
    let accent = winui_theme::Color::from_hex(&arg).expect("hex colour");
    let ramp = winui_theme::AccentRamp::derive(accent);
    let names = [
        "Dark3", "Dark2", "Dark1", "Base", "Light1", "Light2", "Light3",
    ];
    for (name, c) in names.iter().zip(ramp.steps()) {
        println!("{name:7} {c}");
    }
}
