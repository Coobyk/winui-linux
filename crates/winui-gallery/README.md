# winui-gallery

WinUI 3 Gallery-style showcase on Rune. See [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md).

NavigationView shell over a Mica window (tint + noise), acrylic nav pane with category
groups (Controls, Styles) and a search box, one page per control family with labelled
samples, Styles pages (icons grid, colour swatches, type ramp) and Settings (dark-mode
toggle, Light/Dark/System radios, accent picker).

## Run

```sh
cargo run -p winui-gallery                                   # window
cargo run -p winui-gallery -- --scene buttons --snap buttons.png
cargo run -p winui-gallery -- --scene settings --theme dark --at 0.5 --snap s.png
cargo run -p winui-gallery -- --accent '#E81123' -- --size 1000x700
```

- `--scene <page-id>`: `home, buttons, input, collections, menus, navigation, status, media,
  styles-icons, styles-colors, styles-type, settings` (plus `search`: results for "button").
- `--theme light|dark`, `--snap`, `--at`: Rune's own tooling flags (system appearance; the
  Settings page can override it at runtime).
- `--accent #RRGGBB`: initial accent (gallery-only, read from `rune::extra_args()`).
- `-- <rune args>`: everything after `--` is passed to Rune unchanged.

Headless tests: `cargo test -p winui-gallery` (scenes, shell, search, nav, settings, samples).

## Layout

- `src/lib.rs`: `spec()`, `SCENES`; `src/main.rs`: binary.
- `src/state.rs`: pages, nav groups, accents, state.
- `src/shell.rs`: title bar, nav pane, content frame.
- `src/pages/`: sample registry (`Sample { id, page, title, keywords, view }`) and pages.
- `src/fallback/`: local stand-ins for sibling crates (below).

## API gaps (local fallbacks)

When this crate was written every sibling crate was an empty stub, so the gallery ships
`src/fallback/*`, each mirroring its contract in docs/ARCHITECTURE.md. The integrator
replaces `crate::fallback::X` imports with the real crate and deletes the module:

| fallback | replaces | notes |
| --- | --- | --- |
| `fallback::theme` | `winui_theme` | `Theme`, `Tokens` (WinUI resource names lower-cased, values from `Common_themeresources_any.xaml`), `TypeRamp`, `spacing/radii/sizes`, accent ramp. Extras the contract doesn't name: `Theme::new(mode, accent)`, `Theme::accent()`, `TextStyle::rune()`, `Color::{argb,parse,to_hex}`, `body_large`. |
| `fallback::icons` | `winui_icons` | `Symbol`/`Glyph::Path`, 87 Fluent System Icons (MIT, 20px regular + some filled). Symbol names follow the icon file names (`Dismiss`, `ChevronDown`, ...), not the contract's examples (`Accept`, `Back`, `Find`). |
| `fallback::materials` | `winui_materials` | `mica`, `acrylic`, `shadow(depth)`, `reveal`, 64×64 noise tile. |
| `fallback::controls` | `winui_controls` | Free functions (`button`, `checkbox`, `toggle_switch`, `slider`, `info_bar`, ...) taking `&Theme`, generic over app state; not the contract's builder structs. |

Known simplifications: ProgressBar/Ring indeterminate states are static; ComboBox/flyouts
render inline rather than as Rune overlays; slider sets the value from 21 click targets.
