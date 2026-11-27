//! Spacing, corner radii, control sizes and motion. Values in device-independent
//! pixels (epx) / milliseconds, from the microsoft-ui-xaml resource named in each doc.

/// Edge thickness / padding / margin (XAML `Thickness`: left, top, right, bottom).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Thickness {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Thickness {
    pub const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
    pub const fn uniform(v: f32) -> Self {
        Self::new(v, v, v, v)
    }
    pub const fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self::new(horizontal, vertical, horizontal, vertical)
    }
    pub const fn horizontal(&self) -> f32 {
        self.left + self.right
    }
    pub const fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

/// Fluent spacing ramp (WinUI Gallery / Fluent layout guidance, 4px grid).
pub mod spacing {
    pub const XXS: f32 = 2.0;
    pub const XS: f32 = 4.0;
    pub const S: f32 = 8.0;
    pub const M: f32 = 12.0;
    pub const L: f32 = 16.0;
    pub const XL: f32 = 20.0;
    pub const XXL: f32 = 28.0;
}

/// Corner radii (`CornerRadius_themeresources.xaml`).
pub mod radii {
    /// `ControlCornerRadius`.
    pub const CONTROL: f32 = 4.0;
    /// `OverlayCornerRadius` (flyouts, dialogs, tooltips' outer frame, cards).
    pub const OVERLAY: f32 = 8.0;
    /// Pill / circle: clamp to half the shorter side.
    pub const CIRCLE: f32 = 999.0;
    /// `NavigationViewSelectionIndicatorRadius`.
    pub const SELECTION_INDICATOR: f32 = 2.0;
}

/// Control metrics.
pub mod sizes {
    use super::Thickness;

    /// Standard control height (`TextControlThemeMinHeight`, `ComboBoxMinHeight`,
    /// `CheckBoxHeight`, `SliderHorizontalHeight`, `MenuFlyoutThemeMinHeight`).
    pub const CONTROL_HEIGHT: f32 = 32.0;
    /// `ButtonBorderThemeThickness` and most control borders.
    pub const CONTROL_BORDER: f32 = 1.0;
    /// UIElement `FocusVisualPrimaryThickness` / `FocusVisualSecondaryThickness`.
    pub const FOCUS_STROKE_OUTER: f32 = 2.0;
    pub const FOCUS_STROKE_INNER: f32 = 1.0;
    /// Absolute end of the `ControlElevationBorderBrush` gradient band.
    pub const ELEVATION_BORDER_BAND: f32 = 3.0;

    // Button
    pub const BUTTON_PADDING: Thickness = Thickness::new(11.0, 5.0, 11.0, 6.0);

    // Text input
    pub const TEXT_CONTROL_PADDING: Thickness = Thickness::new(10.0, 5.0, 6.0, 6.0);
    pub const TEXT_CONTROL_MIN_WIDTH: f32 = 64.0;
    /// Bottom border thickness of a focused TextBox (`TextControlBorderThemeThicknessFocused` 1,1,1,2).
    pub const TEXT_CONTROL_FOCUSED_BOTTOM_BORDER: f32 = 2.0;
    pub const TEXT_BOX_ICON_FONT_SIZE: f32 = 12.0;
    pub const TEXT_BOX_INNER_BUTTON_MARGIN: Thickness = Thickness::new(0.0, 4.0, 4.0, 4.0);
    pub const HEADER_MARGIN: Thickness = Thickness::new(0.0, 0.0, 0.0, 8.0);

    // CheckBox / RadioButton
    pub const CHECKBOX_SIZE: f32 = 20.0;
    pub const CHECKBOX_GLYPH_SIZE: f32 = 12.0;
    pub const CHECKBOX_MIN_WIDTH: f32 = 120.0;
    pub const CHECKBOX_PADDING: Thickness = Thickness::new(8.0, 5.0, 0.0, 0.0);

    // ToggleSwitch (ToggleSwitch_themeresources.xaml template)
    pub const TOGGLE_SWITCH_WIDTH: f32 = 40.0;
    pub const TOGGLE_SWITCH_HEIGHT: f32 = 20.0;
    pub const TOGGLE_SWITCH_KNOB: f32 = 12.0;
    pub const TOGGLE_SWITCH_MIN_WIDTH: f32 = 154.0;
    pub const TOGGLE_SWITCH_CONTENT_MARGIN: f32 = 10.0;

    // Slider
    pub const SLIDER_TRACK_HEIGHT: f32 = 4.0;
    pub const SLIDER_THUMB_SIZE: f32 = 18.0;
    pub const SLIDER_INNER_THUMB_SIZE: f32 = 12.0;
    pub const SLIDER_TICK_HEIGHT: f32 = 4.0;
    pub const SLIDER_CONTENT_MARGIN: f32 = 14.0;

    // ComboBox
    pub const COMBO_BOX_MIN_WIDTH: f32 = 64.0;
    pub const COMBO_BOX_PADDING: Thickness = Thickness::new(12.0, 5.0, 0.0, 7.0);
    pub const COMBO_BOX_ITEM_PADDING: Thickness = Thickness::new(11.0, 5.0, 11.0, 7.0);
    pub const COMBO_BOX_ITEM_PILL_WIDTH: f32 = 3.0;
    pub const COMBO_BOX_ITEM_PILL_HEIGHT: f32 = 16.0;
    pub const COMBO_BOX_POPUP_MIN_WIDTH: f32 = 80.0;

    // NavigationView (NavigationView.idl defaults + NavigationView_themeresources.xaml)
    pub const NAV_OPEN_PANE_LENGTH: f32 = 320.0;
    pub const NAV_COMPACT_PANE_LENGTH: f32 = 48.0;
    pub const NAV_COMPACT_MODE_THRESHOLD: f32 = 641.0;
    pub const NAV_EXPANDED_MODE_THRESHOLD: f32 = 1008.0;
    pub const NAV_ITEM_HEIGHT: f32 = 36.0;
    pub const NAV_ITEM_MARGIN: Thickness = Thickness::symmetric(4.0, 2.0);
    pub const NAV_ICON_BOX_WIDTH: f32 = 40.0;
    pub const NAV_ICON_SIZE: f32 = 16.0;
    pub const NAV_TOP_PANE_HEIGHT: f32 = 48.0;
    pub const NAV_PANE_HEADER_MIN_HEIGHT: f32 = 40.0;
    pub const NAV_AUTO_SUGGEST_AREA_HEIGHT: f32 = 40.0;
    pub const NAV_SELECTION_INDICATOR_WIDTH: f32 = 3.0;
    pub const NAV_SELECTION_INDICATOR_HEIGHT: f32 = 16.0;
    pub const NAV_HEADER_MARGIN: Thickness = Thickness::new(56.0, 44.0, 0.0, 0.0);
    pub const PANE_TOGGLE_BUTTON_WIDTH: f32 = 40.0;
    pub const PANE_TOGGLE_BUTTON_HEIGHT: f32 = 36.0;

    // TabView
    pub const TAB_ITEM_MIN_HEIGHT: f32 = 32.0;
    pub const TAB_ITEM_MIN_WIDTH: f32 = 100.0;
    pub const TAB_ITEM_MAX_WIDTH: f32 = 240.0;
    pub const TAB_ITEM_PADDING: Thickness = Thickness::new(8.0, 3.0, 4.0, 3.0);
    pub const TAB_CLOSE_BUTTON_WIDTH: f32 = 32.0;
    pub const TAB_CLOSE_BUTTON_HEIGHT: f32 = 24.0;
    pub const TAB_ADD_BUTTON_WIDTH: f32 = 32.0;
    pub const TAB_ADD_BUTTON_HEIGHT: f32 = 24.0;
    pub const TAB_HEADER_FONT_SIZE: f32 = 12.0;

    // ScrollBar
    pub const SCROLL_BAR_SIZE: f32 = 12.0;
    pub const SCROLL_BAR_THUMB_MIN_LENGTH: f32 = 30.0;
    pub const SCROLL_BAR_THUMB_THICKNESS: f32 = 8.0;

    // Expander / InfoBar / ListView
    pub const EXPANDER_MIN_HEIGHT: f32 = 48.0;
    pub const EXPANDER_CONTENT_PADDING: f32 = 16.0;
    pub const EXPANDER_CHEVRON_BUTTON_SIZE: f32 = 32.0;
    pub const INFO_BAR_MIN_HEIGHT: f32 = 48.0;
    pub const INFO_BAR_ICON_SIZE: f32 = 16.0;
    pub const INFO_BAR_CLOSE_BUTTON_SIZE: f32 = 38.0;
    pub const LIST_VIEW_ITEM_MIN_HEIGHT: f32 = 40.0;

    // Overlays
    pub const FLYOUT_CONTENT_PADDING: Thickness = Thickness::new(16.0, 15.0, 16.0, 17.0);
    pub const FLYOUT_MIN_WIDTH: f32 = 96.0;
    pub const FLYOUT_MAX_WIDTH: f32 = 456.0;
    pub const TOOLTIP_PADDING: Thickness = Thickness::new(8.0, 5.0, 8.0, 7.0);
    pub const CONTENT_DIALOG_MIN_WIDTH: f32 = 320.0;
    pub const CONTENT_DIALOG_MAX_WIDTH: f32 = 548.0;
    pub const CONTENT_DIALOG_MIN_HEIGHT: f32 = 184.0;
    pub const CONTENT_DIALOG_MAX_HEIGHT: f32 = 756.0;
    pub const CONTENT_DIALOG_PADDING: Thickness = Thickness::new(24.0, 18.0, 24.0, 24.0);
    /// `PaneOverlayShadowDepth` / `TabViewShadowDepth`.
    pub const OVERLAY_SHADOW_DEPTH: f32 = 16.0;
}

/// Animation timings (`Common_themeresources_any.xaml`).
pub mod motion {
    use core::time::Duration;

    /// `ControlNormalAnimationDuration` 250ms.
    pub const NORMAL: Duration = Duration::from_millis(250);
    /// `ControlFastAnimationDuration` 167ms.
    pub const FAST: Duration = Duration::from_millis(167);
    /// `ControlFastAnimationAfterDuration` 168ms.
    pub const FAST_AFTER: Duration = Duration::from_millis(168);
    /// `ControlFasterAnimationDuration` 83ms.
    pub const FASTER: Duration = Duration::from_millis(83);
    /// `ControlFastOutSlowInKeySpline` cubic-bezier control points (x1, y1, x2, y2).
    pub const FAST_OUT_SLOW_IN: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    /// Evaluate a cubic-bezier easing `[x1, y1, x2, y2]` at progress `t` in `0..=1`.
    pub fn ease(spline: [f32; 4], t: f32) -> f32 {
        let [x1, y1, x2, y2] = spline;
        let t = t.clamp(0.0, 1.0);
        let bez = |a: f32, b: f32, s: f32| {
            let u = 1.0 - s;
            3.0 * u * u * s * a + 3.0 * u * s * s * b + s * s * s
        };
        let (mut lo, mut hi) = (0.0f32, 1.0f32);
        for _ in 0..32 {
            let mid = (lo + hi) / 2.0;
            if bez(x1, x2, mid) < t {
                lo = mid
            } else {
                hi = mid
            }
        }
        bez(y1, y2, (lo + hi) / 2.0)
    }
}
