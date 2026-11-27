//! Buttons: `Button` (+ `AccentButtonStyle`), `ToggleButton`, `HyperlinkButton`, `RepeatButton`,
//! `DropDownButton`, `SplitButton`. Brushes from `Button_themeresources.xaml`,
//! `ToggleButton_themeresources.xaml`, `HyperlinkButton_themeresources.xaml`,
//! `DropDownButton_themeresources.xaml`, `SplitButton_themeresources.xaml`.
//! Gallery samples: ButtonPage, ToggleButtonPage, HyperlinkButtonPage, RepeatButtonPage,
//! DropDownButtonPage, SplitButtonPage.

use rune::{El, Tx, el, row, text};
use winui_icons::Symbol;
use winui_theme::{Color, LinearGradient, Theme, Tokens, radii, sizes};

use crate::icon::glyph;
use crate::style::{Acts, Brushes, Common, VisualState, body, common_setters, elevation_border, fill, focus_visual, press_ink, rc};

/// Which button style resources to draw with.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonStyle {
    /// `DefaultButtonStyle`.
    #[default]
    Standard,
    /// `AccentButtonStyle`.
    Accent,
    /// Transparent at rest (`SubtleFillColor*`), as AppBarButton / title-bar style buttons.
    Subtle,
}

/// The four brush sets a button style draws with.
pub(crate) struct ButtonBrushes {
    pub bg: Brushes,
    pub fg: Brushes,
    pub border: Option<(LinearGradient, Color, Color)>,
}

pub(crate) fn button_brushes(t: &Tokens, style: ButtonStyle) -> ButtonBrushes {
    match style {
        ButtonStyle::Standard => ButtonBrushes {
            bg: Brushes::new(
                t.control_fill_color_default,
                t.control_fill_color_secondary,
                t.control_fill_color_tertiary,
                t.control_fill_color_disabled,
            ),
            fg: Brushes::new(
                t.text_fill_color_primary,
                t.text_fill_color_primary,
                t.text_fill_color_secondary,
                t.text_fill_color_disabled,
            ),
            border: Some((t.control_elevation_border, t.control_stroke_color_default, t.control_stroke_color_default)),
        },
        ButtonStyle::Accent => ButtonBrushes {
            bg: Brushes::new(
                t.accent_fill_color_default,
                t.accent_fill_color_secondary,
                t.accent_fill_color_tertiary,
                t.accent_fill_color_disabled,
            ),
            fg: Brushes::new(
                t.text_on_accent_fill_color_primary,
                t.text_on_accent_fill_color_primary,
                t.text_on_accent_fill_color_secondary,
                t.text_on_accent_fill_color_disabled,
            ),
            border: Some((t.accent_control_elevation_border, t.control_fill_color_transparent, t.control_fill_color_transparent)),
        },
        ButtonStyle::Subtle => ButtonBrushes {
            bg: Brushes::new(
                t.subtle_fill_color_transparent,
                t.subtle_fill_color_secondary,
                t.subtle_fill_color_tertiary,
                t.subtle_fill_color_disabled,
            ),
            fg: Brushes::new(
                t.text_fill_color_primary,
                t.text_fill_color_primary,
                t.text_fill_color_secondary,
                t.text_fill_color_disabled,
            ),
            border: None,
        },
    }
}

/// The shared button chrome: 32px tall, 4px corners, `ButtonPadding` 11,5,11,6 inside a 1px
/// border (content 12px from the edge), elevation border, focus visual.
pub(crate) fn chrome(theme: &Theme, b: &ButtonBrushes, s: VisualState, focused: bool, content: El) -> El {
    let t = theme.tokens();
    let p = sizes::BUTTON_PADDING;
    let bw = sizes::CONTROL_BORDER;
    let h = sizes::CONTROL_HEIGHT;
    let mut e = row()
        .h(h)
        .min_w(h)
        .pl(p.left + bw)
        .pr(p.right + bw)
        .center()
        .radius(radii::CONTROL)
        .child(content);
    e = fill(e, &b.bg, s);
    e = match &b.border {
        Some((g, pressed, disabled)) => elevation_border(e, g, *pressed, *disabled, h, s),
        None => e,
    };
    e = press_ink(e, &b.fg, s);
    focus_visual(e, t, radii::CONTROL, focused)
}

pub(crate) fn label_el(theme: &Theme, label: &str, ink: Color) -> El {
    text(label.to_string()).font(body(theme)).color(rc(ink)).single_line()
}

fn content(theme: &Theme, label: &str, icon: Option<Symbol>, ink: Color) -> El {
    let mut r = row().gap(8.0).center_y();
    if let Some(i) = icon {
        r = r.child(glyph(i, 16.0, ink));
    }
    if !label.is_empty() {
        r = r.child(label_el(theme, label, ink));
    }
    r
}

/// `Button`. `Button::new("Standard XAML button")`, `.accent()` for `AccentButtonStyle`.
pub struct Button {
    label: String,
    icon: Option<Symbol>,
    style: ButtonStyle,
    width: Option<f32>,
    common: Common,
    acts: Acts,
}
common_setters!(Button);

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), icon: None, style: ButtonStyle::Standard, width: None, common: Common::new(), acts: Acts::default() }
    }
    /// `Style="{StaticResource AccentButtonStyle}"`.
    pub fn accent(mut self) -> Self {
        self.style = ButtonStyle::Accent;
        self
    }
    pub fn style(mut self, s: ButtonStyle) -> Self {
        self.style = s;
        self
    }
    /// A leading 16px symbol (an icon-only button when the label is empty).
    pub fn icon(mut self, s: Symbol) -> Self {
        self.icon = Some(s);
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    /// `Click`.
    pub fn on_click<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.acts.push(move |e| e.on_press(f));
        self
    }
    pub fn on_click_tx<S: 'static>(mut self, f: impl Fn(&mut S, &mut Tx) + 'static) -> Self {
        self.acts.push(move |e| e.on_press_tx(f));
        self
    }
    pub fn view(self, theme: &Theme) -> El {
        let s = self.common.state();
        let b = button_brushes(theme.tokens(), self.style);
        let ink = b.fg.get(if s == VisualState::Pressed { s } else if s == VisualState::Disabled { s } else { VisualState::Rest });
        let mut e = chrome(theme, &b, s, self.common.focused, content(theme, &self.label, self.icon, ink));
        if let Some(w) = self.width {
            e = e.w(w);
        }
        let label = if self.label.is_empty() { self.icon.map(|i| i.name().to_string()).unwrap_or_default() } else { self.label.clone() };
        let e = e.role("button").label(label).focusable(self.common.enabled).disabled(!self.common.enabled);
        self.common.keyed(self.acts.apply(e, self.common.enabled))
    }
}

/// `RepeatButton`: Button visuals; `Click` repeats while held (`Delay` 250ms, `Interval`
/// 33ms upstream; here a press fires once, held repetition is the app's timer).
pub struct RepeatButton(Button);

impl RepeatButton {
    pub fn new(label: impl Into<String>) -> Self {
        Self(Button::new(label))
    }
    pub fn enabled(self, on: bool) -> Self {
        Self(self.0.enabled(on))
    }
    pub fn state(self, s: VisualState) -> Self {
        Self(self.0.state(s))
    }
    pub fn focused(self, on: bool) -> Self {
        Self(self.0.focused(on))
    }
    pub fn key(self, k: impl Into<String>) -> Self {
        Self(self.0.key(k))
    }
    pub fn on_click<S: 'static>(self, f: impl Fn(&mut S) + 'static) -> Self {
        Self(self.0.on_click(f))
    }
    pub fn view(self, theme: &Theme) -> El {
        self.0.view(theme)
    }
}

/// `ToggleButton`: standard Button brushes unchecked, accent brushes checked
/// (`ToggleButtonBackgroundChecked` = `AccentFillColorDefault`, ...).
pub struct ToggleButton {
    label: String,
    checked: bool,
    common: Common,
    acts: Acts,
}
common_setters!(ToggleButton);

impl ToggleButton {
    pub fn new(label: impl Into<String>, checked: bool) -> Self {
        Self { label: label.into(), checked, common: Common::new(), acts: Acts::default() }
    }
    pub fn on_toggle<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.acts.push(move |e| e.on_press(f));
        self
    }
    pub fn view(self, theme: &Theme) -> El {
        let s = self.common.state();
        let style = if self.checked { ButtonStyle::Accent } else { ButtonStyle::Standard };
        let b = button_brushes(theme.tokens(), style);
        let ink = b.fg.get(if matches!(s, VisualState::Pressed | VisualState::Disabled) { s } else { VisualState::Rest });
        let e = chrome(theme, &b, s, self.common.focused, label_el(theme, &self.label, ink))
            .role("togglebutton")
            .label(self.label.clone())
            .checked(self.checked)
            .focusable(self.common.enabled)
            .disabled(!self.common.enabled);
        self.common.keyed(self.acts.apply(e, self.common.enabled))
    }
}

/// `HyperlinkButton`: transparent `SubtleFillColor*` background, `AccentTextFillColor*` text.
pub struct HyperlinkButton {
    label: String,
    uri: Option<String>,
    common: Common,
    acts: Acts,
}
common_setters!(HyperlinkButton);

impl HyperlinkButton {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), uri: None, common: Common::new(), acts: Acts::default() }
    }
    /// `NavigateUri` (exposed to accessibility; opening it is the app's job in `on_click`).
    pub fn uri(mut self, u: impl Into<String>) -> Self {
        self.uri = Some(u.into());
        self
    }
    pub fn on_click<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.acts.push(move |e| e.on_press(f));
        self
    }
    pub fn view(self, theme: &Theme) -> El {
        let t = theme.tokens();
        let s = self.common.state();
        let b = ButtonBrushes {
            bg: Brushes::new(
                t.subtle_fill_color_transparent,
                t.subtle_fill_color_secondary,
                t.subtle_fill_color_tertiary,
                t.subtle_fill_color_disabled,
            ),
            fg: Brushes::new(
                t.accent_text_fill_color_primary,
                t.accent_text_fill_color_secondary,
                t.accent_text_fill_color_tertiary,
                t.accent_text_fill_color_disabled,
            ),
            border: None,
        };
        let ink = b.fg.get(s);
        let e = chrome(theme, &b, s, self.common.focused, label_el(theme, &self.label, ink));
        let e = e.role("link").label(self.uri.clone().map_or(self.label.clone(), |u| format!("{} ({u})", self.label)));
        let e = e.focusable(self.common.enabled).disabled(!self.common.enabled);
        self.common.keyed(self.acts.apply(e, self.common.enabled))
    }
}

/// The 12px chevron (`E70D`) DropDownButton and SplitButton draw.
fn chevron(theme: &Theme, s: VisualState) -> El {
    let t = theme.tokens();
    let c = if s == VisualState::Disabled { t.text_fill_color_disabled } else { t.text_fill_color_secondary };
    glyph(Symbol::ChevronDown, 12.0, c)
}

/// `DropDownButton`: a Button with a trailing chevron 8px after the content, opening a flyout.
pub struct DropDownButton {
    label: String,
    common: Common,
    acts: Acts,
}
common_setters!(DropDownButton);

impl DropDownButton {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), common: Common::new(), acts: Acts::default() }
    }
    pub fn on_click<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.acts.push(move |e| e.on_press(f));
        self
    }
    pub fn view(self, theme: &Theme) -> El {
        let s = self.common.state();
        let b = button_brushes(theme.tokens(), ButtonStyle::Standard);
        let ink = b.fg.get(if matches!(s, VisualState::Pressed | VisualState::Disabled) { s } else { VisualState::Rest });
        let inner = row().gap(8.0).center_y().child(label_el(theme, &self.label, ink)).child(chevron(theme, s));
        let e = chrome(theme, &b, s, self.common.focused, inner)
            .role("button")
            .label(self.label.clone())
            .focusable(self.common.enabled)
            .disabled(!self.common.enabled);
        self.common.keyed(self.acts.apply(e, self.common.enabled))
    }
}

/// `SplitButton`: a primary action and a 32px chevron part split by a 1px divider, each with
/// its own PointerOver/Pressed fill, one shared elevation border.
pub struct SplitButton {
    label: String,
    accent: bool,
    common: Common,
    primary: Acts,
    secondary: Acts,
}
common_setters!(SplitButton);

impl SplitButton {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), accent: false, common: Common::new(), primary: Acts::default(), secondary: Acts::default() }
    }
    pub fn accent(mut self) -> Self {
        self.accent = true;
        self
    }
    pub fn on_click<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.primary.push(move |e| e.on_press(f));
        self
    }
    /// The chevron part: opens the flyout.
    pub fn on_open<S: 'static>(mut self, f: impl Fn(&mut S) + 'static) -> Self {
        self.secondary.push(move |e| e.on_press(f));
        self
    }
    pub fn view(self, theme: &Theme) -> El {
        let t = theme.tokens();
        let s = self.common.state();
        let style = if self.accent { ButtonStyle::Accent } else { ButtonStyle::Standard };
        let b = button_brushes(t, style);
        let h = sizes::CONTROL_HEIGHT;
        let p = sizes::BUTTON_PADDING;
        let ink = b.fg.get(if matches!(s, VisualState::Pressed | VisualState::Disabled) { s } else { VisualState::Rest });
        let enabled = self.common.enabled;
        let primary = fill(row().h_full().pl(p.left + 1.0).pr(p.right).center(), &b.bg, s)
            .child(label_el(theme, &self.label, ink))
            .role("button")
            .label(self.label.clone())
            .focusable(enabled)
            .disabled(!enabled);
        let primary = press_ink(self.primary.apply(primary, enabled), &b.fg, s);
        // The secondary part stays at rest when the primary is forced, as in Gallery.
        let sec_state = if s == VisualState::Disabled { s } else { VisualState::Rest };
        let chev_ink = if self.accent { b.fg.get(sec_state) } else if s == VisualState::Disabled { t.text_fill_color_disabled } else { t.text_fill_color_secondary };
        let secondary = fill(el().w(32.0).h_full().center(), &b.bg, sec_state)
            .child(glyph(Symbol::ChevronDown, 12.0, chev_ink))
            .role("button")
            .label("More options")
            .focusable(enabled)
            .disabled(!enabled);
        let secondary = self.secondary.apply(secondary, enabled);
        let divider_c = if self.accent { t.control_stroke_color_on_accent_default } else { t.divider_stroke_color_default };
        let divider = el().w(1.0).h_full().fill(rc(divider_c));
        let mut e = row().h(h).radius(radii::CONTROL).clip_rounded(radii::CONTROL).child(primary).child(divider).child(secondary);
        e = match &b.border {
            Some((g, _, _)) if s != VisualState::Disabled => e.border_gradient(1.0, rc(g.top(h)), rc(g.bottom(h))),
            Some((_, _, disabled)) => e.border(1.0, rc(*disabled)),
            None => e,
        };
        self.common.keyed(focus_visual(e, t, radii::CONTROL, self.common.focused))
    }
}
