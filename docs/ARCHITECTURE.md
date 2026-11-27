# Architecture

winui-linux is a cargo workspace. Five crates, strict dependency direction:

```
winui-theme    ──> (no deps)          Fluent tokens: colours, type, spacing, radii
winui-icons    ──> (no deps)          Fluent System Icons glyph data
winui-materials──> winui-theme, rune  Mica / Acrylic / reveal / elevation
winui-controls ──> theme, icons, materials, rune
winui-gallery  ──> everything         showcase app, binary
```

Crates lower in the table must not depend on crates above them.
`winui-theme` and `winui-icons` must not depend on `rune` — they stay pure
data and unit-testable anywhere.

## The reference

The look targets **WinUI 3 as shipped in Windows 11's WinUI 3 Gallery**
(microsoft/WinUI-Gallery on GitHub; design tokens live in
microsoft/microsoft-ui-xaml under `dev/CommonStyles` and
`controls/dev*/**/*themeresources*.xaml`). Match real WinUI values 1:1 —
do not invent a "similar" palette.

Key facts every owner should respect:

- Control corner radius **4px**, overlay/flyout radius **8px**.
- Font: Segoe UI Variable Text; fall back to a metric-compatible sans
  (Inter is acceptable). Type ramp: Caption 12, Body 14, Subtitle 20,
  Title 28, TitleLarge 40, Display 68.
- Default accent `#0078D4` (light) / `#4CC2FF` (dark) — take the real
  values from the theme resources, plus the full accent ramp.
- Control states: Rest / PointerOver / Pressed / Disabled (+ Focus visuals
  as a 2px accent or ThemeSettings-dependent focus rect).
- Light and dark themes must both exist and switch at runtime.

## Contracts

The owner of each crate owns its public API, but these cross-crate
contracts are fixed:

### winui-theme

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThemeMode { Light, Dark }

/// One sRGBA colour, serialisable and trivially convertible to rune::Color.
#[derive(Clone, Copy, Debug)]
pub struct Color { pub r: f32, pub g: f32, pub b: f32, pub a: f32 }
impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self;
    pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self;
}

/// The Fluent token surface other crates consume. Field names follow the
/// WinUI resource names lower-cased: text_fill_color_primary, etc.
pub struct Tokens { /* fills, strokes, accents, backgrounds, card/layer */ }

pub struct Theme { /* mode + accent + resolved tokens */ }
impl Theme {
    pub fn light() -> Self;
    pub fn dark() -> Self;
    pub fn mode(&self) -> ThemeMode;
    pub fn tokens(&self) -> &Tokens;
    /// Re-resolve with a different accent colour.
    pub fn with_accent(&self, accent: Color) -> Self;
}

/// Fluent type ramp.
#[derive(Clone, Copy, Debug)]
pub struct TextStyle { pub size: f32, pub weight: u16, pub line_height: f32 }
pub struct TypeRamp { pub caption: TextStyle, pub body: TextStyle,
                      pub body_strong: TextStyle, pub subtitle: TextStyle,
                      pub title: TextStyle, pub title_large: TextStyle,
                      pub display: TextStyle }
impl Theme { pub fn type_ramp(&self) -> &TypeRamp; }

pub mod spacing { /* XXS=2 XS=4 S=8 M=12 L=16 XL=20 XXL=28 */ }
pub mod radii { /* CONTROL=4 OVERLAY=8 CIRCLE=999 */ }
pub mod sizes { /* CONTROL_HEIGHT=32, nav widths, etc */ }
```

Tokens must cover at minimum: TextFillColorPrimary/Secondary/Tertiary/
Disabled, AccentTextFillColorDisabled, TextOnAccentFillColor*,
AccentFillColorDefault/Secondary/Tertiary/Disabled, ControlFillColor*/
ControlAltFillColor*, ControlStrokeColor*, ControlElevationBorder,
CardBackgroundFillColor*, LayerFillColor*, SolidBackgroundFillColorBase,
SubtleFillColor*, SystemFillColorAttention/Success/Caution/Critical.

### winui-icons

```rust
/// A Fluent System Icons glyph.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Symbol { /* Accept, Add, Back, Cancel, CheckMark, ChevronDown,
                     ChevronRight, Delete, Dismiss, Edit, Find, Home,
                     MoreHorizontal, Settings, Share, ... */ }

/// How a glyph is drawn: a font codepoint (Segoe Fluent Icons / vendor
/// subset) or raw SVG path data from fluentui-system-icons (MIT).
#[derive(Clone, Copy, Debug)]
pub enum Glyph {
    Font(char),
    Path(&'static str),   // SVG path data, fill-rule evenodd unless noted
}

impl Symbol {
    pub fn glyph(self) -> Glyph;
    pub const ALL: &'static [Symbol];
}
```

Aim for ~120+ symbols covering the controls' needs (chevrons, checks,
dismiss, arrows, nav icons for gallery pages). Vendor real Fluent icon
data — fluentui-system-icons is MIT; keep the licence note.

### winui-materials

Rune visuals other crates layer under/around content. Exact API is the
owner's, but these must exist as composable Rune elements/nodes:

- `mica(theme)`: opaque theme base colour + faint procedural noise
  (real Mica = desktop tint + noise; we do tint + noise, no wallpaper).
- `acrylic(theme, tint, tint_opacity, luminosity_opacity)`: translucent
  noise-blend panel for flyouts/nav pane.
- `reveal(...)`: the pointer-tracking border/hover highlight used on
  NavigationView items and list rows.
- `shadow(depth)`: Fluent elevation — ambient + key shadow params for the
  standard depths (2/4/8/16/32/64/128).
- A noise texture helper (deterministic, tiny — e.g. 64×64 u8 tile).

Expose colours that read from `winui_theme::Tokens` so materials track
theme/accent switches.

### winui-controls

Each control is a Rune component with a builder-style API and real state
visuals. Conventions:

- `pub struct Button { /* label, style, enabled, on_press */ }` with
  `Button::new("Label")`, `.accent(bool)`, `.enabled(bool)` — implementer
  picks exact setters, but every control has `.enabled(false)` support.
- States come from `Theme::tokens()`; disabled never below spec contrast.
- Controls take theme context, not globals: `view(&self, theme: &Theme, ...)`
  or a `Theme` passed down through app context — owner's choice, document it.

Must-ship set (WinUI 3 Gallery parity):

- Buttons: Button (standard + Accent), HyperlinkButton, ToggleButton,
  RepeatButton (visual), SplitButton, DropDownButton
- Selection: CheckBox (incl. indeterminate), RadioButton, ToggleSwitch,
  RatingControl (visual), Slider, ColorPicker-lite (swatch grid ok)
- Text: TextBox (incl. placeholder + disabled), PasswordBox, NumberBox
  (spin buttons), AutoSuggestBox-lite, RichTextBlock-lite optional
- Menus/flyouts: ComboBox, Flyout/MenuFlyout, CommandBar-lite
- Navigation: NavigationView (left minimal+expanded modes), TabView
  (close buttons, add button), BreadcrumbBar-lite
- Status: InfoBar (Info/Success/Warning/Error severities), ProgressBar
  (determinate + indeterminate), ProgressRing, TeachingTip-lite, ToolTip
- Content: Expander, ListView-lite (reveal rows), PersonPicture,
  IconElement, Badge/InfoBadge, CalendarDatePicker-lite optional

Every control documents which Gallery sample it matches.

### winui-gallery

The showcase app + binary (`src/main.rs`). Shape: WinUI 3 Gallery —
NavigationView shell (Mica window, acrylic nav pane), category groups,
a page per control family with labelled samples, Settings page with
light/dark toggle + accent colour picker, search box filtering samples.

```rust
fn main() { /* rune::run(spec()) */ }
// args: --scene <name> (page id), --theme light|dark, --snap out.png,
//       --at <secs> (settle delay), -- <rune args>
```

Scene names are page ids: `home, buttons, input, collections, menus,
navigation, status, media, styles-icons, styles-colors, styles-type,
settings`.

## Coordination rules for agents

- One crate per agent, one git branch per agent (`agent/<crate>`).
- Do not edit files outside your crate, `Cargo.toml` workspace root
  (only under `[workspace.dependencies]`, additive only), or other agents'
  branches.
- If you need an API from another crate that doesn't exist yet, implement
  against the contract above and note the gap in your crate's README.
- `cargo build -p <your-crate>` and `cargo test -p <your-crate>` must pass
  on Linux headless (no display required). Rune apps run headless —
  render to PNG / dump element tree for verification.
- This is a look port: screenshot-fidelity to real WinUI 3 matters more
  than API parity. Check microsoft/microsoft-ui-xaml resources for real
  values before inventing any colour, size, or duration.
- Push your branch when done; open a PR to `main` with `gh` if available.
