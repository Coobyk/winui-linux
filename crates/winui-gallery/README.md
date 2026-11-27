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
`radii`, `TypeRamp`, `AccentRamp`); every icon from `winui-icons::Symbol`. Controls
come from `winui-controls` and the Mica/Acrylic/shadow layers from `winui-materials`
(`src/controls.rs` and `src/fluent.rs` are thin adapters over them).

## Apps: Windows 11 app clones

The **Apps** nav group holds static clones of real Windows 11 apps, rebuilt from the
screenshots in [docs/reference](../../docs/reference). Each is a `--scene` that renders
full-window at the reference size (pass `--size` to override):

| Scene | Reference | Size |
| --- | --- | --- |
| `app-settings` | `settings-system.png`, Settings > System | 1201×934 |
| `app-calculator` | `calculator.png`, Calculator Standard | 328×532 |
| `app-clock` | `clock-timer.png`, Clock > Timer | 1201×933 |
| `app-clock-settings` | `clock-settings.png`, Clock > Settings | 1201×933 |
| `app-store` | `store-updates.png`, Microsoft Store > Updates & downloads | 1201×928 |
| `app-snipping` | `snipping-tool.png`, Snipping Tool | 502×232 |
| `app-terminal` | `terminal.png`, Windows Terminal / PowerShell | 1153×616 |

```sh
cargo run -p winui-gallery -- --scene app-settings --theme dark --snap settings.png
```

Icons missing from `winui_icons::Symbol` are drawn from Fluent System Icons path data
in `src/apps/glyphs.rs` (microsoft/fluentui-system-icons, MIT).

## Remaining gaps

- No Segoe UI Variable on Linux: the family list falls back to Inter, so text runs a
  little wider than the references.
- No wallpaper sampling: Mica is the solid base + noise, so the references' tinted
  backdrops (and the Settings hero's Bloom wallpaper) are approximated with theme
  accent gradients.
- `winui-icons` lacks many Fluent System Icons (Settings nav, Store rail, Snipping
  toolbar); `src/apps/glyphs.rs` carries them until the icon crate grows a
  `FluentSystemIcon` set. App tiles (Outlook, Paint, Spotify…) are accent tiles with a
  glyph, not the real artwork.
- Windows Terminal's surface uses its own Campbell colour scheme
  (microsoft/terminal `defaults.json`), not XAML theme tokens.
- The gallery pane's search box is visual only and the pane does not collapse to
  the 48 px compact mode yet.
