use crate::color::ColorId;

/// Parses text containing ANSI escape sequences (SGR colors, 256 colors, RGB, and cursor codes)
/// into a 2D grid of `(char, ColorId)` cells.
/// If no ANSI codes are present, plain text is parsed with `ColorId::Default`.
pub fn parse_ansi_text(text: &str) -> Vec<Vec<(char, ColorId)>> {
    let mut rows: Vec<Vec<(char, ColorId)>> = Vec::new();
    let mut current_row: Vec<(char, ColorId)> = Vec::new();
    let mut current_color = ColorId::Default;

    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let ch = chars[i];

        if ch == '\r' {
            if i + 1 < len && chars[i + 1] == '\n' {
                i += 1;
                rows.push(std::mem::take(&mut current_row));
            }
            i += 1;
            continue;
        } else if ch == '\n' {
            rows.push(std::mem::take(&mut current_row));
            i += 1;
            continue;
        } else if ch == '\t' {
            for _ in 0..4 {
                current_row.push((' ', current_color));
            }
            i += 1;
            continue;
        } else if ch == '\x1b' {
            i += 1;
            if i < len && chars[i] == '[' {
                // CSI sequence: parse until terminator (0x40..=0x7E)
                i += 1;
                let mut param_str = String::new();
                while i < len {
                    let c = chars[i];
                    if ('@'..='~').contains(&c) {
                        if c == 'm' {
                            current_color = parse_sgr_params(&param_str, current_color);
                        }
                        i += 1;
                        break;
                    } else {
                        param_str.push(c);
                        i += 1;
                    }
                }
            } else if i < len && chars[i] == ']' {
                // OSC sequence: skip until BEL (\x07) or ST (\x1b\)
                i += 1;
                while i < len {
                    if chars[i] == '\x07' {
                        i += 1;
                        break;
                    } else if chars[i] == '\x1b' && i + 1 < len && chars[i + 1] == '\\' {
                        i += 2;
                        break;
                    }
                    i += 1;
                }
            } else if i < len && !chars[i].is_ascii_alphanumeric() {
                i += 1;
            }
            continue;
        }

        if !ch.is_control() || ch == ' ' {
            current_row.push((ch, current_color));
        }
        i += 1;
    }

    if !current_row.is_empty() {
        rows.push(current_row);
    }

    rows
}

fn parse_sgr_params(params: &str, current: ColorId) -> ColorId {
    if params.is_empty() {
        return ColorId::Default;
    }

    let nums: Vec<u32> = params
        .split(';')
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect();

    if nums.is_empty() {
        return ColorId::Default;
    }

    let mut color = current;
    let mut bold = false;
    let mut idx = 0;

    while idx < nums.len() {
        match nums[idx] {
            0 => {
                color = ColorId::Default;
                bold = false;
            }
            1 => {
                bold = true;
                color = match color {
                    ColorId::Black => ColorId::BrightBlack,
                    ColorId::Red => ColorId::BrightRed,
                    ColorId::Green => ColorId::BrightGreen,
                    ColorId::Yellow => ColorId::BrightYellow,
                    ColorId::Blue => ColorId::BrightBlue,
                    ColorId::Magenta => ColorId::BrightMagenta,
                    ColorId::Cyan => ColorId::BrightCyan,
                    ColorId::White => ColorId::BrightWhite,
                    other => other,
                };
            }
            22 => {
                bold = false;
            }
            30 => color = if bold { ColorId::BrightBlack } else { ColorId::Black },
            31 => color = if bold { ColorId::BrightRed } else { ColorId::Red },
            32 => color = if bold { ColorId::BrightGreen } else { ColorId::Green },
            33 => color = if bold { ColorId::BrightYellow } else { ColorId::Yellow },
            34 => color = if bold { ColorId::BrightBlue } else { ColorId::Blue },
            35 => color = if bold { ColorId::BrightMagenta } else { ColorId::Magenta },
            36 => color = if bold { ColorId::BrightCyan } else { ColorId::Cyan },
            37 => color = if bold { ColorId::BrightWhite } else { ColorId::White },
            39 => color = ColorId::Default,
            90 => color = ColorId::BrightBlack,
            91 => color = ColorId::BrightRed,
            92 => color = ColorId::BrightGreen,
            93 => color = ColorId::BrightYellow,
            94 => color = ColorId::BrightBlue,
            95 => color = ColorId::BrightMagenta,
            96 => color = ColorId::BrightCyan,
            97 => color = ColorId::BrightWhite,
            38 => {
                if idx + 2 < nums.len() && nums[idx + 1] == 5 {
                    color = color_from_256(nums[idx + 2] as u8);
                    idx += 2;
                } else if idx + 4 < nums.len() && nums[idx + 1] == 2 {
                    let r = nums[idx + 2] as u8;
                    let g = nums[idx + 3] as u8;
                    let b = nums[idx + 4] as u8;
                    color = closest_ansi_color(r, g, b);
                    idx += 4;
                }
            }
            _ => {}
        }
        idx += 1;
    }

    color
}

pub fn color_from_256(n: u8) -> ColorId {
    match n {
        0 => ColorId::Black,
        1 => ColorId::Red,
        2 => ColorId::Green,
        3 => ColorId::Yellow,
        4 => ColorId::Blue,
        5 => ColorId::Magenta,
        6 => ColorId::Cyan,
        7 => ColorId::White,
        8 => ColorId::BrightBlack,
        9 => ColorId::BrightRed,
        10 => ColorId::BrightGreen,
        11 => ColorId::BrightYellow,
        12 => ColorId::BrightBlue,
        13 => ColorId::BrightMagenta,
        14 => ColorId::BrightCyan,
        15 => ColorId::BrightWhite,
        16..=231 => {
            let n = n - 16;
            let r_idx = (n / 36) % 6;
            let g_idx = (n / 6) % 6;
            let b_idx = n % 6;
            let r = if r_idx == 0 { 0 } else { 55 + r_idx * 40 };
            let g = if g_idx == 0 { 0 } else { 55 + g_idx * 40 };
            let b = if b_idx == 0 { 0 } else { 55 + b_idx * 40 };
            closest_ansi_color(r, g, b)
        }
        232..=255 => {
            let gray = 8 + (n - 232) * 10;
            closest_ansi_color(gray, gray, gray)
        }
    }
}

pub fn closest_ansi_color(r: u8, g: u8, b: u8) -> ColorId {
    let all_colors = [
        ColorId::Black,
        ColorId::Red,
        ColorId::Green,
        ColorId::Yellow,
        ColorId::Blue,
        ColorId::Magenta,
        ColorId::Cyan,
        ColorId::White,
        ColorId::BrightBlack,
        ColorId::BrightRed,
        ColorId::BrightGreen,
        ColorId::BrightYellow,
        ColorId::BrightBlue,
        ColorId::BrightMagenta,
        ColorId::BrightCyan,
        ColorId::BrightWhite,
    ];

    let mut best_color = ColorId::White;
    let mut best_dist = u32::MAX;

    for c in all_colors {
        let (cr, cg, cb) = c.rgb();
        let dr = (r as i32) - (cr as i32);
        let dg = (g as i32) - (cg as i32);
        let db = (b as i32) - (cb as i32);
        let dist = (dr * dr + dg * dg + db * db) as u32;
        if dist < best_dist {
            best_dist = dist;
            best_color = c;
        }
    }

    best_color
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_text_parsing() {
        let input = "abc\ndef";
        let rows = parse_ansi_text(input);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], vec![('a', ColorId::Default), ('b', ColorId::Default), ('c', ColorId::Default)]);
        assert_eq!(rows[1], vec![('d', ColorId::Default), ('e', ColorId::Default), ('f', ColorId::Default)]);
    }

    #[test]
    fn test_ansi_color_parsing() {
        let input = "\x1b[32mOK\x1b[0m \x1b[1;31mERR\x1b[0m";
        let rows = parse_ansi_text(input);
        assert_eq!(rows.len(), 1);
        let row = &rows[0];
        assert_eq!(row[0], ('O', ColorId::Green));
        assert_eq!(row[1], ('K', ColorId::Green));
        assert_eq!(row[2], (' ', ColorId::Default));
        assert_eq!(row[3], ('E', ColorId::BrightRed));
        assert_eq!(row[4], ('R', ColorId::BrightRed));
        assert_eq!(row[5], ('R', ColorId::BrightRed));
    }

    #[test]
    fn test_bright_ansi_and_256_colors() {
        let input = "\x1b[94mBLUE\x1b[0m\x1b[38;5;10mGREEN\x1b[0m";
        let rows = parse_ansi_text(input);
        assert_eq!(rows.len(), 1);
        let row = &rows[0];
        assert_eq!(row[0], ('B', ColorId::BrightBlue));
        assert_eq!(row[4], ('G', ColorId::BrightGreen));
    }
}
