# Agent brief template

Repo: https://github.com/Coobyk/winui-linux (public)
Branch: agent/winui-<crate>, from main
Contracts: docs/ARCHITECTURE.md — read first, it is the law.
Rules: touch only your crate dir + additive [workspace.dependencies] entries.
Verify: cargo build -p winui-<crate> && cargo test -p winui-<crate> on Linux, headless.
Done: commit, git push -u origin agent/winui-<crate>, open PR to main with gh if authed.

Per-crate missions:

## winui-theme
Implement the Fluent token surface: Color (sRGBA), Tokens for light AND dark
with REAL values from microsoft/microsoft-ui-xaml theme resources (fetch the
repo or docs if needed — do not invent), Theme{light,dark,with_accent},
TypeRamp (Segoe UI Variable ramp), spacing/radii/sizes. with_accent derives a
plausible accent ramp (lighten/darken steps like Windows). Unit tests: every
token populated both modes, accent switch changes accent fields.

## winui-icons
Vendor real Fluent System Icons data (fluentui-system-icons is MIT). Symbol
enum (~120+ glyphs covering the gallery + control needs), Glyph::Path with
real SVG path data (regular 24px variants), Symbol::ALL, glyph(). Include a
build script or generator if you like, but commit the generated data — no
network at build time. Tests: all glyphs non-empty, ALL unique.

## winui-materials
Rune elements: mica(), acrylic(), reveal(), shadow(depth), deterministic
noise tile helper. Read colours from winui_theme::Tokens so they track
theme/accent. Ship examples/demo.rs with --scene mica|acrylic|reveal|shadow
--snap out.png for headless verification (copy the demo-harness idiom from
wt-app if you can see it — otherwise --snap must render a PNG without a
display).

## winui-controls
The contract's must-ship set, as Rune components with builder APIs and real
Rest/Hover/Pressed/Disabled visuals pulled from Theme::tokens(). Ship
examples/demo.rs --scene <control> --snap. Prioritise visual fidelity to
WinUI 3: 4px radius, 32px control height, correct paddings and focus rects.

## winui-gallery
The app + binary: NavigationView shell over Mica, acrylic nav pane, a page
per control family, styles pages (icons grid, colour swatches, type ramp),
settings page with light/dark toggle + accent picker, --scene/--theme/--snap
flags per contract. Sibling crates may still be stubs — implement against
the documented contracts; where an API is missing, provide a local fallback
that keeps `cargo build -p winui-gallery` green and note the gap in the
crate README. The integrator will wire real APIs.
