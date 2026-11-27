# winui-gallery

The WinUI 3 Gallery-style showcase app for winui-linux: a left-mode NavigationView
shell (320 px pane, 36 px items, 3×16 accent selection pill) over a Mica base, with
a page per control family, the styles pages and a settings page. See
[docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md).

```sh
cargo run -p winui-gallery                                  # window
cargo run -p winui-gallery -- --scene buttons --theme dark --snap out.png   # headless PNG
cargo run -p winui-gallery -- --list-scenes
```

Scenes (page ids): `home, buttons, input, collections, menus, navigation, status,
media, styles-icons, styles-colors, styles-type, settings`. `--scene`, `--theme`,
`--snap`, `--at` and `--size` are Rune's headless CLI (the `tooling` feature keeps
them in release builds); `--theme` sets the system appearance the app follows until
the settings page's theme switch overrides it. Snapshots need no display.

Every colour, size and type style comes from `winui-theme` (`Tokens`, `sizes`,
`radii`, `TypeRamp`, `AccentRamp`); every icon from `winui-icons::Symbol`.

## Local fallbacks (sibling-crate gaps)

`winui-materials` and `winui-controls` are still stubs in `main`, so the gallery
draws their contracts locally. Replace these modules with the real crates once
they land:

| Gap | Local fallback |
| --- | --- |
| `winui_materials::{mica, acrylic, shadow}` | `src/fluent.rs`: Mica base = `SolidBackgroundFillColorBase` + 2% noise tile (no wallpaper tint); acrylic = `AcrylicBackgroundFillColorDefault` tint/luminosity composite + noise (no backdrop blur); shadow = one key shadow at y = depth/2. |
| `winui_controls` must-ship set | `src/controls.rs`: static visuals with the WinUI rest/hover/pressed/disabled fills for Button (standard/accent), Hyperlink/Toggle/DropDown/SplitButton, CheckBox, RadioButton, ToggleSwitch, Slider, TextBox/PasswordBox/NumberBox/AutoSuggestBox, ComboBox, MenuFlyout, CommandBar, InfoBar, ProgressBar/Ring, RatingControl, Expander, ListView rows, PersonPicture, InfoBadge, ToolTip, TeachingTip, TabView, BreadcrumbBar. Only the nav items, settings theme switch and accent swatches are interactive. |

Other known gaps: no Segoe UI Variable on Linux (family list falls back to Inter,
Rune's bundled sans); the pane's search box is visual only; the pane is not
collapsible to the 48 px compact mode yet.
