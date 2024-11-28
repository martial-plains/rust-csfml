//! Utility class for manipulating RGBA colors.

use csfml_sys::sfColor;
use std::cmp::PartialEq;

/// A struct representing an RGBA color.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
    /// Alpha (opacity) component
    pub a: u8,
}

impl Color {
    /// Converts a csfml color object into a Rust `Color` struct.
    #[must_use]
    pub const fn from_csfml(sfColor { r, g, b, a }: sfColor) -> Self {
        Self { r, g, b, a }
    }

    /// Converts this `Color` struct into a csfml color object.
    #[must_use]
    pub const fn to_csfml(self) -> sfColor {
        sfColor {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }

    /// Initializes a color with RGB components.
    #[must_use]
    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
            a: 0xff,
        }
    }

    /// Initializes a color with RGBA components.
    #[must_use]
    pub const fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }

    /// Initializes a color from a 32-bit integer (RGBA format).
    #[must_use]
    pub const fn from_integer(int: u32) -> Self {
        Self {
            r: (int >> 24) as u8,
            g: ((int >> 16) & 0xFF) as u8,
            b: ((int >> 8) & 0xFF) as u8,
            a: (int & 0xFF) as u8,
        }
    }

    /// Converts the color to a 32-bit integer (RGBA format).
    #[must_use]
    pub const fn to_integer(self) -> u32 {
        (self.r as u32) << 24 | (self.g as u32) << 16 | (self.b as u32) << 8 | (self.a as u32)
    }

    /// Creates a color from float values between 0 and 1 for RGBA.
    #[must_use]
    pub fn from_floats(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            r: (red.clamp(0.0, 1.0) * 255.0) as u8,
            g: (green.clamp(0.0, 1.0) * 255.0) as u8,
            b: (blue.clamp(0.0, 1.0) * 255.0) as u8,
            a: (alpha.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    /// Creates a color from a hexadecimal string (e.g., "#RRGGBB").
    #[must_use]
    pub fn from_hex(hex: &str) -> Self {
        assert!(
            hex.len() == 7 && hex.starts_with('#'),
            "Invalid hexadecimal color format"
        );

        let int = u32::from_str_radix(&hex[1..], 16).expect("Invalid hex string");
        Self::from_integer((int << 8) | 0xFF)
    }

    /// Creates a color from HSV (hue in degrees, saturation and value in percentages).
    #[must_use]
    pub fn from_hsva(hue: f32, saturation: f32, value: f32, alpha: f32) -> Self {
        let (h, s, v, a) = (hue, saturation / 100.0, value / 100.0, alpha);

        let mut hh = h;

        if v <= 0.0 {
            return Self::from_floats(0.0, 0.0, 0.0, a);
        }

        if hh >= 360.0 {
            hh = 0.0;
        }

        hh /= 60.0;
        let ff = hh - hh.floor();

        let p = v * (1.0 - s);
        let q = v * (1.0 - s * ff);
        let t = v * (1.0 - s * (1.0 - ff));

        match hh as usize {
            0 => Self::from_floats(v, t, p, a),
            1 => Self::from_floats(q, v, p, a),
            2 => Self::from_floats(p, v, t, a),
            3 => Self::from_floats(p, q, v, a),
            4 => Self::from_floats(t, p, v, a),
            _ => Self::from_floats(v, p, q, a),
        }
    }

    /// Converts this color to a GLSL float vector (for shaders).
    #[must_use]
    pub fn to_fvec4(self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }

    /// Converts this color to a GLSL int vector (for shaders).
    #[must_use]
    pub const fn to_ivec4(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}

// Constant color values
impl Color {
    pub const BLACK: Self = Self::from_rgb(0, 0, 0);
    pub const WHITE: Self = Self::from_rgb(255, 255, 255);
    pub const RED: Self = Self::from_rgb(255, 0, 0);
    pub const GREEN: Self = Self::from_rgb(0, 255, 0);
    pub const BLUE: Self = Self::from_rgb(0, 0, 255);
    pub const YELLOW: Self = Self::from_rgb(255, 255, 0);
    pub const MAGENTA: Self = Self::from_rgb(255, 0, 255);
    pub const CYAN: Self = Self::from_rgb(0, 255, 255);
    pub const TRANSPARENT: Self = Self::from_rgba(0, 0, 0, 0);
}

impl From<sfColor> for Color {
    fn from(value: sfColor) -> Self {
        Self::from_csfml(value)
    }
}

impl From<Color> for sfColor {
    fn from(value: Color) -> Self {
        value.to_csfml()
    }
}

// Tests

#[cfg(test)]
mod tests {
    use csfml_sys::sfColor_fromInteger;

    use super::*;

    #[test]
    fn color_conversions() {
        let code: u32 = 0x4BDA_9CFF;
        let col = Color::from_integer(code);

        assert_eq!(Color::from_hex("#4BDA9C"), col);
        assert_eq!(Color::from_rgb(75, 218, 156), col);
        assert_eq!(code, col.to_integer());

        let csfml_col = unsafe { sfColor_fromInteger(code) };

        assert_eq!(Color::from_csfml(csfml_col), col);
    }

    #[test]
    fn color_hsv_to_rgb() {
        let col = Color::from_hsva(10.0, 20.0, 100.0, 255.0);
        assert_eq!(Color::from_rgb(255, 212, 204), col);
    }

    #[test]
    fn color_sane_from_to_csfml() {
        let col = Color::from_rgba(5, 12, 28, 127);
        let ccol = col.to_csfml();

        assert_eq!(col.r, ccol.r);
        assert_eq!(col.g, ccol.g);
        assert_eq!(col.b, ccol.b);
        assert_eq!(col.a, ccol.a);

        let col2 = Color::from_csfml(ccol);
        assert_eq!(col, col2);
    }
}
