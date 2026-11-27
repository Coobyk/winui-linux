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

## Sibling API gaps (integrator: read this)

On this branch `winui-theme` and `winui-icons` are still stubs, so the crate uses local
shims that follow the contract: `src/shim/theme.rs` and `src/shim/icons.rs`. Controls only
reach them through `crate::shim::{theme, icons}`. To wire the real crates, replace
`src/shim/mod.rs` with:

```rust
pub use winui_icons as icons;
pub use winui_theme as theme;
```

and delete the two shim files. The real crates also need what the shims add on top of the
contract:

- **`Color`**: `Color::rgb(u8, u8, u8)`, `Color::rgba(..)`, `.with_alpha(f32)`, `Default`,
  sRGB-encoded `f32` channels. `style::rc` converts these to Rune's linear `rune::Color`.
  Change it if `winui_theme::Color` stores something else.
- **`Theme`**: `is_dark()`, `of(ThemeMode)`, and `accent() -> &AccentRamp` (`base`,
  `light_1..3`, `dark_1..3`). The ColorPicker and the tests use `accent().base`.
- **`TextStyle`**: `size`, `weight: u16` and `line_height`, as in the contract.
- **`sizes`**: `CONTROL_HEIGHT = 32`, `NAV_PANE_OPEN = 320`, `NAV_PANE_COMPACT = 48`.
- **`Tokens`** fields beyond the contract minimum: `control_strong_stroke_color_default/
  disabled`, `control_strong_fill_color_default/disabled`, `control_stroke_color_secondary`,
  `control_stroke_color_on_accent_default/secondary`, `control_fill_color_input_active`,
  `control_fill_color_transparent`, `control_solid_fill_color_default`,
  `control_alt_fill_color_quarternary`, `subtle_fill_color_transparent/secondary/tertiary/
  disabled`, `divider_stroke_color_default`, `surface_stroke_color_flyout`,
  `acrylic_background_fill_color_default`, `layer_on_mica_base_alt_fill_color_secondary`,
  `solid_background_fill_color_tertiary`, `accent_text_fill_color_primary/secondary/
  tertiary`, `system_fill_color_neutral`, and
  `system_fill_color_{attention,success,caution,critical}_background`.
- **`Symbol`** variants the controls draw: `Add ChevronDown ChevronRight ChevronUp Contact
  Dismiss ErrorFilled Favorite FavoriteFilled Find GlobalNavigation InfoFilled
  MoreHorizontal Settings SuccessFilled View WarningFilled`. The demo uses the rest of
  `Symbol::ALL`. `Symbol::view_box()` is shim-only and defaults to 20. The contract's
  `Cancel` has no shim variant yet; controls use `Dismiss`.
- **`Glyph::Font`** draws through Rune text with whatever font is loaded. `Glyph::Path`
  draws SVG path data on a `rune::canvas`.
- **`winui-materials`**: not used yet. Flyouts, ToolTips and TeachingTips fill with the
  `acrylic_background_fill_color_default` token plus a shadow (`style::flyout_surface`),
  and NavigationView content uses `layer_fill_color_default`. Switch `flyout_surface` to the
  materials crate's acrylic when it is available.

## Known gaps

- `TeachingTip` has no beak. Indeterminate `ProgressBar`/`ProgressRing` draw a still
  segment, not the animation.
- Popups (`ComboBox`, `MenuFlyout`, `AutoSuggestBox`, `ToolTip`, `TeachingTip`) use
  `El::anchored` + `El::raise`. Outside clicks don't dismiss them: the app closes them by
  setting `open(false)` (rune-kit's `scrim` works for this).
- Focus and keyboard: Rune handles Tab focus and Space/Enter to press. There is no arrow-key
  roving inside RadioButtons/ListView/NavigationView, and no type-ahead in ComboBox.
