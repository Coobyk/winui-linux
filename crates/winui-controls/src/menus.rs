use rune::El;
use crate::shim::theme::Theme;
#[derive(Clone, Default)]
pub struct MenuFlyout;
impl MenuFlyout { pub fn view(&self, _t: &Theme) -> El { rune::col() } }
