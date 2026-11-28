# winui-controls

WinUI 3 Fluent controls as Rune components. The fixed contract is
[docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md).

Every control is a builder that renders against an explicit theme:

```rust
use winui_controls::*;

let t = Theme::dark();
let el = Button::new("Save").accent(true).icon(Symbol::Save).on_press(|s: &mut App| s.save()).view(&t);
```

All controls also implement `Control { fn view(&self, theme: &Theme) -> El }`. Colours come
from `Theme::tokens()` (WinUI resource names, lower-cased). States use Rune's
`hover_*`/`press_*`/`disabled`. Controls are 32px tall with 4px corners, flyouts have 8px
corners, and focusable controls get a 2px focus ring. App state stays with the app. Each
control takes its value (`checked`, `value`, `selected`, `open`, …) and reports changes with
`on_*(|state, value| …)`.

## Controls and Gallery samples

| Module | Control | WinUI 3 Gallery sample |
|---|---|---|
| `buttons` | `Button`, `HyperlinkButton`, `ToggleButton`, `RepeatButton`, `DropDownButton`, `SplitButton` | Basic input › Button, HyperlinkButton, ToggleButton, RepeatButton, DropDownButton, SplitButton |
| `selection` | `CheckBox` (three-state), `RadioButton`, `RadioButtons`, `ToggleSwitch`, `RatingControl` | Basic input › CheckBox, RadioButton, ToggleSwitch, RatingControl |
| `range` | `Slider`, `ColorPicker` (lite: RGB sliders + accent swatches + hex) | Basic input › Slider, ColorPicker |
| `text` | `TextBox`, `PasswordBox` (reveal), `NumberBox` (inline spin), `AutoSuggestBox` (lite) | Text › TextBox, PasswordBox, NumberBox, AutoSuggestBox |
| `menus` | `ComboBox`, `Flyout`, `MenuFlyout` (`MenuItem`), `CommandBar` (lite, `AppBarItem`) | Basic input › ComboBox; Dialogs & flyouts › Flyout; Menus & toolbars › MenuFlyout, CommandBar |
| `navigation` | `NavigationView` (Left, open/compact), `TabView`, `BreadcrumbBar` (lite) | Navigation › NavigationView, TabView, BreadcrumbBar |
| `status` | `InfoBar`, `ProgressBar`, `ProgressRing`, `ToolTip`, `TeachingTip` (lite) | Status & info › InfoBar, ProgressBar, ProgressRing, ToolTip; Dialogs & flyouts › TeachingTip |
| `content` | `Expander`, `ListView` (lite), `PersonPicture`, `IconElement`, `InfoBadge` | Layout › Expander; Collections › ListView; People › PersonPicture; Iconography; Status & info › InfoBadge |

Not built (they are optional in the contract): `RichTextBlock`, `CalendarDatePicker`.

## Demo and tests

```sh
cargo run -p winui-controls --example demo -- --scene buttons --snap out.png [--theme light]
cargo test -p winui-controls
```

Scenes: `buttons selection slider colorpicker text menus navigation tabview infobar progress
tooltip expander listview people`. The headless tests press controls through Rune's
headless runtime and check sizes, state changes and token helpers.

The demo bundles Inter (SIL OFL 1.1, `assets/OFL.txt`) because Segoe UI Variable can't be
redistributed.

## Sibling crates and gaps (integrator: read this)

Controls use `winui-theme` and `winui-icons` from `main`, through the re-exports in
`src/shim/mod.rs` (`crate::shim::{theme, icons}`). If either API moves, fix it there and in
`src/style.rs`. The following are still missing:

- **`Tokens::acrylic_background_fill_color_default`** isn't in `winui-theme`.
  `style::acrylic` returns WinUI's solid fallback (`#F9F9F9` light, `#2C2C2C` dark).
- **`winui-materials`** isn't used yet. Flyouts, ToolTips and TeachingTips draw
  `style::flyout_surface` (the acrylic fallback, the flyout stroke and a shadow), and
  NavigationView content uses `layer_fill_color_default`. Point `flyout_surface` at the
  materials crate's acrylic once it can draw behind an anchored overlay.
- **Icons**: paths come from `Symbol::glyph()` in the `winui_icons::VIEWBOX` square, filled
  with `Symbol::fill_rule()`. Severity glyphs use `Info`, `CheckmarkCircle`, `Warning` and
  `ErrorCircle`. Filled `*Filled` variants would match WinUI's InfoBar more closely.

## Known gaps

- `TeachingTip` has no beak. Indeterminate `ProgressBar`/`ProgressRing` draw a still
  segment, not the animation.
- Popups (`ComboBox`, `MenuFlyout`, `AutoSuggestBox`, `ToolTip`, `TeachingTip`) use
  `El::anchored` + `El::raise`. Outside clicks don't dismiss them: the app closes them by
  setting `open(false)` (rune-kit's `scrim` works for this).
- Focus and keyboard: Rune handles Tab focus and Space/Enter to press. There is no arrow-key
  roving inside RadioButtons/ListView/NavigationView, and no type-ahead in ComboBox.

## Visual fidelity notes

Values come from `microsoft/microsoft-ui-xaml` theme resources (`Common_themeresources_any.xaml`,
`Button_themeresources.xaml`, `TextBox_themeresources.xaml`, `ToggleSwitch_themeresources.xaml`,
`Expander_themeresources.xaml`) and the Windows Community Toolkit `SettingsCard`, checked against
`docs/reference/*.png` with `--snap`.

- Bordered faces (buttons, ComboBox, TextBox, cards, Expander headers, `SettingsCard`) fill
  *inside* a 1px ring (`BackgroundSizing="InnerBorderEdge"`), so translucent strokes composite
  over the page: a dark card on `#202020` reads 29 / 43, as in the Settings screenshot.
- `ControlElevationBorderBrush` and `TextControlElevationBorderBrush` are absolute 3px / 2px
  gradients, so only one device row differs: the top row in dark (light: the bottom), the bottom
  row on accent; TextBox gets a 1px `ControlStrongStrokeColorDefault` bottom, 2px accent focused.

Deltas not closed:

- Pressed rings: WinUI swaps the border to `ControlStrokeColorDefault` (accent: transparent) while
  pressed. The ring is a child layer and Rune's press states only restyle the pressed element's
  own fill/border/text, so pressed faces keep the rest ring (a one-row, ~2% difference).
- `SettingsCard` pressed background (`ControlFillColorTertiary`) is not drawn: its fill is a
  layer inside the ring, and Rune exposes hover (not press) to group children.
- The focus visual draws the 2px outer stroke only; WinUI's 1px `FocusStrokeColorInner` between
  the stroke and the control has no second-ring API in Rune.
- Text uses Inter (Segoe UI Variable can't be redistributed): same ramp sizes, weights and line
  heights, but slightly wider glyphs and no optical-size axis.
- Card and window backgrounds are solid; Mica/Acrylic come from `winui-materials`, which this
  crate does not use yet.
