//! Small image-encoding helpers.

use std::io::Cursor;

use image::{DynamicImage, ImageFormat, RgbaImage};
use yomika_core::Region;

pub trait RegionExt {
    fn clamp(&self, width: u32, height: u32) -> Option<(u32, u32, u32, u32)>;
}

impl RegionExt for Region {
    /// Intersect the region with an image of `width` × `height`.
    ///
    /// Returns `None` when the image is degenerate, when the region is
    /// degenerate, or when the region lies entirely outside the image.
    /// Callers must never receive a spurious 1-pixel strip at the image
    /// boundary for an out-of-bounds region.
    fn clamp(&self, width: u32, height: u32) -> Option<(u32, u32, u32, u32)> {
        if width == 0 || height == 0 || self.x >= width || self.y >= height {
            return None;
        }
        let x0 = self.x;
        let y0 = self.y;
        let x1 = self.x.saturating_add(self.width).min(width);
        let y1 = self.y.saturating_add(self.height).min(height);
        let w = x1.saturating_sub(x0);
        let h = y1.saturating_sub(y0);
        if w == 0 || h == 0 {
            return None;
        }
        Some((x0, y0, w, h))
    }
}

pub fn encode_image(image: &DynamicImage, ext: &str) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    let format = ImageFormat::from_extension(ext).unwrap_or(ImageFormat::Jpeg);
    image.write_to(&mut cursor, format)?;
    Ok(buf)
}

pub fn mime_from_ext(ext: &str) -> &'static str {
    match ext {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        _ => "application/octet-stream",
    }
}

pub fn blank_rgba(width: u32, height: u32, color: image::Rgba<u8>) -> DynamicImage {
    DynamicImage::ImageRgba8(RgbaImage::from_pixel(width, height, color))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn region(x: u32, y: u32, width: u32, height: u32) -> Region {
        Region {
            x,
            y,
            width,
            height,
        }
    }

    #[test]
    fn clamp_fully_inside_returns_region_unchanged() {
        let r = region(10, 20, 30, 40);
        assert_eq!(r.clamp(100, 100), Some((10, 20, 30, 40)));
    }

    #[test]
    fn clamp_exact_image_bounds_returns_full_image() {
        let r = region(0, 0, 100, 50);
        assert_eq!(r.clamp(100, 50), Some((0, 0, 100, 50)));
    }

    #[test]
    fn clamp_partially_overlapping_right_edge_trims_width() {
        let r = region(80, 10, 50, 20);
        assert_eq!(r.clamp(100, 100), Some((80, 10, 20, 20)));
    }

    #[test]
    fn clamp_partially_overlapping_bottom_edge_trims_height() {
        let r = region(10, 90, 30, 50);
        assert_eq!(r.clamp(100, 100), Some((10, 90, 30, 10)));
    }

    #[test]
    fn clamp_region_entirely_right_of_image_returns_none() {
        // Regression: previously returned a spurious 1×h strip at x = width - 1.
        let r = region(100, 10, 50, 20);
        assert_eq!(r.clamp(100, 100), None);
    }

    #[test]
    fn clamp_region_entirely_below_image_returns_none() {
        let r = region(10, 100, 30, 40);
        assert_eq!(r.clamp(100, 100), None);
    }

    #[test]
    fn clamp_region_far_outside_image_returns_none() {
        let r = region(10_000, 10_000, 100, 100);
        assert_eq!(r.clamp(100, 100), None);
    }

    #[test]
    fn clamp_region_starting_on_last_pixel_is_valid() {
        // Not out of bounds: the region genuinely covers the last column.
        let r = region(99, 99, 10, 10);
        assert_eq!(r.clamp(100, 100), Some((99, 99, 1, 1)));
    }

    #[test]
    fn clamp_degenerate_region_returns_none() {
        assert_eq!(region(10, 10, 0, 20).clamp(100, 100), None);
        assert_eq!(region(10, 10, 20, 0).clamp(100, 100), None);
    }

    #[test]
    fn clamp_degenerate_image_returns_none() {
        let r = region(0, 0, 10, 10);
        assert_eq!(r.clamp(0, 100), None);
        assert_eq!(r.clamp(100, 0), None);
        assert_eq!(r.clamp(0, 0), None);
    }

    #[test]
    fn clamp_extents_saturating_at_u32_max() {
        // x + width overflows u32 — must saturate, not wrap or panic.
        let r = region(50, 50, u32::MAX, u32::MAX);
        assert_eq!(r.clamp(100, 100), Some((50, 50, 50, 50)));
    }
}
