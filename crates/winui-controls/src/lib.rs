//! The WinUI 3 control set as Rune components. See docs/ARCHITECTURE.md for the contract.
//!
//! Every control is a builder (`Button::new("Save").accent(true)`) whose `view(&self, &Theme)`
//! returns a Rune [`El`]. Colours for every visual state come from [`Theme::tokens`]; app
//! handlers are generic over the app's state and attached with `on_*` builder methods.
pub mod buttons;
pub mod content;
pub mod menus;
pub mod navigation;
pub mod range;
pub mod selection;
pub mod shim;
pub mod status;
pub mod style;
pub mod text;

pub use buttons::*;
pub use content::*;
pub use menus::*;
pub use navigation::*;
pub use range::*;
pub use rune;
pub use rune::El;
pub use selection::*;
pub use shim::icons::{Glyph, Symbol};
pub use shim::theme::{Theme, ThemeMode, Tokens};
pub use status::*;
pub use text::*;

/// Anything that renders against an explicit theme.
pub trait Control {
    fn view(&self, theme: &Theme) -> El;
}

macro_rules! impl_control {
    ($($t:ty),* $(,)?) => {$(
        impl $crate::Control for $t {
            fn view(&self, theme: &$crate::Theme) -> $crate::El {
                <$t>::view(self, theme)
            }
        }
    )*};
}
pub(crate) use impl_control;
