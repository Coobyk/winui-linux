//! Deterministic grain tile shared by Mica and Acrylic.

use rune::{El, Pixels, el};

/// Edge of the square noise tile, in texels (and points when tiled).
pub const NOISE_SIZE: usize = 64;

/// Seed of the shared tile.
pub const NOISE_SEED: u32 = 0x57C1_0A11;

/// Opacity WinUI composites its noise texture at (`AcrylicBrush::sc_noiseOpacity`).
pub const NOISE_OPACITY: f32 = 0.02;

/// A `NOISE_SIZE`² single-channel tile of uniform white noise, row-major. Same seed, same bytes,
/// on every platform: a PCG-style integer hash of the texel index, no global RNG state.
pub fn noise_tile(seed: u32) -> [u8; NOISE_SIZE * NOISE_SIZE] {
    let mut out = [0u8; NOISE_SIZE * NOISE_SIZE];
    for (i, px) in out.iter_mut().enumerate() {
        *px = (hash(i as u32 ^ seed.wrapping_mul(0x9E37_79B9)) >> 24) as u8;
    }
    out
}

fn hash(v: u32) -> u32 {
    let state = v.wrapping_mul(747_796_405).wrapping_add(2_891_336_453);
    let word = ((state >> ((state >> 28) + 4)) ^ state).wrapping_mul(277_803_737);
    (word >> 22) ^ word
}

/// `tile` as opaque grey RGBA bytes, ready for [`Pixels::set`].
pub fn noise_rgba(tile: &[u8]) -> Vec<u8> {
    tile.iter().flat_map(|&v| [v, v, v, 255]).collect()
}

thread_local! {
    static NOISE: Pixels = {
        let p = Pixels::new();
        p.set(NOISE_SIZE as u32, NOISE_SIZE as u32, noise_rgba(&noise_tile(NOISE_SEED)));
        p
    };
}

/// The shared noise tile as a Rune pixel buffer (one upload per thread).
pub fn noise_pixels() -> Pixels {
    NOISE.with(Pixels::clone)
}

/// A layer that fills its parent with the tiled noise at `opacity`. Draws above the parent's
/// fill and below children added after it; never a hit target.
pub fn noise_layer(opacity: f32) -> El {
    el().pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
        .pixels_tile(&noise_pixels(), [NOISE_SIZE as f32; 2])
        .opacity(opacity.clamp(0.0, 1.0))
}
