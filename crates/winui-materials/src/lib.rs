//! Mica, Acrylic, reveal, and elevation effects in Rune.
//! See docs/ARCHITECTURE.md for the contract.

pub mod acrylic;
pub mod color;
pub mod mica;
pub mod noise;
pub mod reveal;
pub mod shadow;
pub mod theme;

pub use acrylic::Acrylic;
pub use acrylic::acrylic;
pub use color::to_rune;
pub use mica::MicaKind;
pub use mica::mica;
pub use mica::mica_alt;
pub use noise::noise_layer;
pub use noise::noise_pixels;
pub use noise::noise_tile;
pub use reveal::RevealLight;
pub use reveal::RevealStyle;
pub use reveal::reveal;
pub use reveal::reveal_with;
pub use shadow::Elevation;
pub use shadow::ElevationExt;
pub use shadow::shadow;
pub use shadow::shadow_for;

#[cfg(test)]
mod tests;
