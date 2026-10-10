use image::{Rgba, RgbaImage};
use crate::model::CanvasGrid;

pub fn adjust_pixel(r: u8, g: u8, b: u8, brightness: f32, contrast: f32) -> f32 {
    let mut lum = 0.299 * (r as f32) + 0.587 * (g as f32) + 0.114 * (b as f32);

    // Apply contrast
    let contrast_factor = (259.0 * (contrast + 255.0)) / (255.0 * (259.0 - contrast).max(1.0));
    lum = contrast_factor * (lum - 128.0) + 128.0;

    // Apply brightness
    lum += brightness * 2.55;

    (lum / 255.0).clamp(0.0, 1.0)
}

pub fn convert_image_to_ascii(
    img: &RgbaImage,
    target_w: usize,
    target_h: usize,
    brightness: f32,
    contrast: f32,
    palette_chars: &[char],
) -> CanvasGrid {
    let mut grid = CanvasGrid::new(target_w, target_h);
    if palette_chars.is_empty() {
        return grid;
    }

    let src_w = img.width() as f32;
    let src_h = img.height() as f32;

    for y in 0..target_h {
        let sample_y = ((y as f32 / target_h as f32) * src_h).min(src_h - 1.0) as u32;
        for x in 0..target_w {
            let sample_x = ((x as f32 / target_w as f32) * src_w).min(src_w - 1.0) as u32;
            let pixel = img.get_pixel(sample_x, sample_y);

            let intensity = adjust_pixel(pixel[0], pixel[1], pixel[2], brightness, contrast);
            let pal_idx = ((1.0 - intensity) * (palette_chars.len() - 1) as f32).round() as usize;
            let ch = palette_chars[pal_idx.min(palette_chars.len() - 1)];

            grid.set(x, y, ch);
        }
    }

    grid
}

pub fn convert_image_to_braille(
    img: &RgbaImage,
    target_w: usize,
    target_h: usize,
    brightness: f32,
    contrast: f32,
) -> CanvasGrid {
    let mut grid = CanvasGrid::new(target_w, target_h);

    let dots_w = target_w * 2;
    let dots_h = target_h * 4;

    let src_w = img.width() as f32;
    let src_h = img.height() as f32;

    for cell_y in 0..target_h {
        for cell_x in 0..target_w {
            let mut byte: u8 = 0;

            // Check 2x4 dots for this braille cell
            // Dot order for Unicode Braille:
            // 1 4
            // 2 5
            // 3 6
            // 7 8
            let dot_offsets = [
                (0, 0, 0x01), // dot 1
                (0, 1, 0x02), // dot 2
                (0, 2, 0x04), // dot 3
                (1, 0, 0x08), // dot 4
                (1, 1, 0x10), // dot 5
                (1, 2, 0x20), // dot 6
                (0, 3, 0x40), // dot 7
                (1, 3, 0x80), // dot 8
            ];

            for &(dx, dy, bit) in &dot_offsets {
                let dot_x = cell_x * 2 + dx;
                let dot_y = cell_y * 4 + dy;

                let sample_x = ((dot_x as f32 / dots_w as f32) * src_w).min(src_w - 1.0) as u32;
                let sample_y = ((dot_y as f32 / dots_h as f32) * src_h).min(src_h - 1.0) as u32;

                let pixel = img.get_pixel(sample_x, sample_y);
                let intensity = adjust_pixel(pixel[0], pixel[1], pixel[2], brightness, contrast);

                // If pixel is darker than 0.5 threshold, set dot
                if intensity < 0.5 {
                    byte |= bit;
                }
            }

            let braille_char = char::from_u32(0x2800 + byte as u32).unwrap_or(' ');
            grid.set(cell_x, cell_y, braille_char);
        }
    }

    grid
}

pub fn render_grid_to_png(grid: &CanvasGrid, width_px: u32, height_px: u32) -> RgbaImage {
    let mut image = RgbaImage::from_pixel(width_px, height_px, Rgba([255, 255, 255, 255]));

    let cell_w = (width_px as f32 / grid.width as f32).max(1.0);
    let cell_h = (height_px as f32 / grid.height as f32).max(1.0);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let ch = grid.get(x, y).unwrap_or(' ');
            if ch != ' ' {
                let px_start_x = (x as f32 * cell_w) as u32;
                let px_end_x = (((x + 1) as f32 * cell_w) as u32).min(width_px);
                let px_start_y = (y as f32 * cell_h) as u32;
                let px_end_y = (((y + 1) as f32 * cell_h) as u32).min(height_px);

                let color = match ch {
                    '█' => Rgba([20, 20, 20, 255]),
                    '▓' => Rgba([60, 60, 60, 255]),
                    '▒' => Rgba([120, 120, 120, 255]),
                    '░' => Rgba([180, 180, 180, 255]),
                    _ => Rgba([30, 30, 30, 255]),
                };

                for py in px_start_y..px_end_y {
                    for px in px_start_x..px_end_x {
                        image.put_pixel(px, py, color);
                    }
                }
            }
        }
    }

    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braille_conversion() {
        let img = RgbaImage::from_pixel(4, 8, Rgba([0, 0, 0, 255]));
        let grid = convert_image_to_braille(&img, 2, 2, 0.0, 0.0);
        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 2);
        // All dots on is 0x28FF
        assert_eq!(grid.get(0, 0), Some('⣿'));
    }

    #[test]
    fn test_ascii_conversion() {
        let img = RgbaImage::from_pixel(2, 2, Rgba([255, 255, 255, 255]));
        let palette = vec![' ', '#'];
        let grid = convert_image_to_ascii(&img, 2, 2, 0.0, 0.0, &palette);
        assert_eq!(grid.get(0, 0), Some(' '));
    }
}
