//! The WinUI 3 control set as Rune components.
//!
//! Every control is a builder that ends in `view(self, theme: &Theme) -> El`: the theme is
//! passed down explicitly (no globals), and every brush comes from [`Theme::tokens`] with the
//! state mapping of the control's `*_themeresources.xaml` in microsoft/microsoft-ui-xaml.
//! Every control has `.enabled(bool)`, `.state(VisualState)` (force a common visual state for
//! scenes and snapshots; `Rest` is live), `.focused(bool)` and `.key(..)`.
//!
//! See docs/ARCHITECTURE.md for the contract.

mod button;
mod icon;
mod style;

pub use button::{Button, ButtonStyle, DropDownButton, HyperlinkButton, RepeatButton, SplitButton, ToggleButton};
pub use icon::{IconElement, glyph, icon_of};
pub use style::{Brushes, VisualState, acrylic_fallback, body, font, rc};
pub use winui_icons::Symbol;
pub use winui_theme::Theme;
