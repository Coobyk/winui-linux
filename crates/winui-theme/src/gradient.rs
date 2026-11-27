//! Linear gradients (XAML `LinearGradientBrush`), used for elevation borders.

use crate::color::Color;

/// How a gradient's end point is interpreted.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GradientMapping {
    /// `MappingMode="Absolute"`: start at (0,0), end at this offset in pixels.
    /// The control elevation borders end at (0,3): a 3px band at the top edge
    /// (or bottom edge when flipped) and the last stop beyond it.
    Absolute { end_x: f32, end_y: f32 },
    /// `MappingMode="RelativeToBoundingBox"` from (0,0) to (0,1).
    Relative,
}

/// A two-stop vertical linear gradient, stops sorted by offset.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinearGradient {
    pub mapping: GradientMapping,
    /// `RelativeTransform` `ScaleTransform ScaleY="-1" CenterY="0.5"`: the gradient
    /// runs from the bottom edge upwards.
    pub flip_y: bool,
    pub stops: [(f32, Color); 2],
}

impl LinearGradient {
    /// Colour at normalised position `t` along the gradient axis (`0.0` = start).
    pub fn sample(&self, t: f32) -> Color {
        let [(o0, c0), (o1, c1)] = self.stops;
        if t <= o0 {
            c0
        } else if t >= o1 {
            c1
        } else {
            c0.lerp(c1, (t - o0) / (o1 - o0))
        }
    }

    /// Colour at pixel row `y` of an element `height` px tall, honouring
    /// mapping mode and flip. Rows outside the element clamp to the edge stops.
    pub fn sample_at(&self, y: f32, height: f32) -> Color {
        let y = if self.flip_y { height - y } else { y };
        let t = match self.mapping {
            GradientMapping::Absolute { end_y, .. } => y / end_y,
            GradientMapping::Relative => y / height.max(f32::EPSILON),
        };
        self.sample(t)
    }

    /// Colour for the top edge of a border.
    pub fn top(&self, height: f32) -> Color {
        self.sample_at(0.0, height)
    }

    /// Colour for the bottom edge of a border.
    pub fn bottom(&self, height: f32) -> Color {
        self.sample_at(height, height)
    }
}
