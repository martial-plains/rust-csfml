use std::ffi::c_int;

use csfml_sys::{sfFloatRect, sfIntRect};

use crate::system::Vector2;

pub type IntRect = Rect<c_int>;
pub type FloatRect = Rect<f32>;

/// Utility struct for manipulating 2D axis-aligned rectangles
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Rect<T> {
    pub left: T,
    pub top: T,
    pub width: T,
    pub height: T,
}

impl<T: PartialOrd + Copy> Rect<T> {
    /// Creates a new rectangle with the specified parameters
    pub const fn new(left: T, top: T, width: T, height: T) -> Self {
        Self {
            left,
            top,
            width,
            height,
        }
    }

    /// Checks if the given point is inside the rectangle
    pub fn contains(&self, point: Vector2<T>) -> bool
    where
        T: std::ops::Add<Output = T> + Ord + Eq,
    {
        let min_x = self.left.min(self.left + self.width);
        let max_x = self.left.max(self.left + self.width);
        let min_y = self.top.min(self.top + self.height);
        let max_y = self.top.max(self.top + self.height);

        point.x >= min_x && point.x < max_x && point.y >= min_y && point.y < max_y
    }

    /// Checks if two rectangles intersect, returns the intersection if it exists
    pub fn intersects(&self, other: &Self) -> Option<Self>
    where
        T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Ord + Eq,
    {
        let r1_min_x = self.left.min(self.left + self.width);
        let r1_max_x = self.left.max(self.left + self.width);
        let r1_min_y = self.top.min(self.top + self.height);
        let r1_max_y = self.top.max(self.top + self.height);

        let r2_min_x = other.left.min(other.left + other.width);
        let r2_max_x = other.left.max(other.left + other.width);
        let r2_min_y = other.top.min(other.top + other.height);
        let r2_max_y = other.top.max(other.top + other.height);

        let inter_left = r1_min_x.max(r2_min_x);
        let inter_top = r1_min_y.max(r2_min_y);
        let inter_right = r1_max_x.min(r2_max_x);
        let inter_bottom = r1_max_y.min(r2_max_y);

        if inter_left < inter_right && inter_top < inter_bottom {
            Some(Self::new(
                inter_left,
                inter_top,
                inter_right - inter_left,
                inter_bottom - inter_top,
            ))
        } else {
            None
        }
    }

    /// Checks if two rectangles are identical
    pub fn equals(&self, other: &Self) -> bool {
        self.left == other.left
            && self.top == other.top
            && self.width == other.width
            && self.height == other.height
    }

    /// Gets the top-left corner of the rectangle as a vector
    pub const fn get_corner(&self) -> Vector2<T> {
        Vector2 {
            x: self.left,
            y: self.top,
        }
    }

    /// Gets the bottom-right corner of the rectangle as a vector
    pub fn get_other_corner(&self) -> Vector2<T>
    where
        T: std::ops::Add<Output = T>,
    {
        Vector2 {
            x: self.left + self.width,
            y: self.top + self.height,
        }
    }

    /// Gets the size (width and height) as a vector
    pub const fn get_size(&self) -> Vector2<T> {
        Vector2 {
            x: self.width,
            y: self.height,
        }
    }
}

/// Conversion methods to and from CSFML equivalents (C-style structs)
impl IntRect {
    #[must_use]
    pub const fn to_csfml(&self) -> sfIntRect {
        sfIntRect {
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
        }
    }

    #[must_use]
    pub const fn from_csfml(rect: sfIntRect) -> Self {
        Self::new(rect.left, rect.top, rect.width, rect.height)
    }
}

impl From<sfIntRect> for IntRect {
    fn from(value: sfIntRect) -> Self {
        Self::from_csfml(value)
    }
}

impl From<IntRect> for sfIntRect {
    fn from(value: IntRect) -> Self {
        value.to_csfml()
    }
}

impl FloatRect {
    #[must_use]
    pub const fn to_csfml(&self) -> sfFloatRect {
        sfFloatRect {
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
        }
    }

    #[must_use]
    pub const fn from_csfml(rect: sfFloatRect) -> Self {
        Self::new(rect.left, rect.top, rect.width, rect.height)
    }
}

impl From<sfFloatRect> for FloatRect {
    fn from(value: sfFloatRect) -> Self {
        Self::from_csfml(value)
    }
}

impl From<FloatRect> for sfFloatRect {
    fn from(value: FloatRect) -> Self {
        value.to_csfml()
    }
}

#[cfg(test)]
mod tests {

    use std::mem;

    use csfml_sys::sfIntRect_intersects;

    use super::*;

    #[test]
    fn test_rect_intersects() {
        let r1 = Rect::new(0, 0, 10, 10);
        let r2 = Rect::new(6, 6, 20, 20);
        let r3 = Rect::new(-5, -5, 10, 10);

        assert!(r2.intersects(&r3).is_none());

        let mut inter1: sfIntRect = unsafe { mem::zeroed() };
        let mut inter2: sfIntRect = unsafe { mem::zeroed() };

        let r1_csfml = r1.to_csfml();
        let r2_csfml = r2.to_csfml();
        let r3_csfml = r3.to_csfml();
        assert_eq!(
            unsafe {
                sfIntRect_intersects(&raw const r1_csfml, &raw const r2_csfml, &raw mut inter1)
            },
            1
        );
        assert_eq!(
            unsafe {
                sfIntRect_intersects(&raw const r1_csfml, &raw const r3_csfml, &raw mut inter2)
            },
            1
        );

        let inter1_from_csfml = Rect::from(inter1);
        let inter2_from_csfml = Rect::from(inter2);

        assert_eq!(r1.intersects(&r2).unwrap(), inter1_from_csfml);
        assert_eq!(r1.intersects(&r3).unwrap(), inter2_from_csfml);
    }

    #[test]
    fn test_rect_contains() {
        let r1 = Rect::new(0, 0, 10, 10);

        assert!(r1.contains(Vector2 { x: 0, y: 0 }));
        assert!(r1.contains(Vector2 { x: 9, y: 9 }));
        assert!(!r1.contains(Vector2 { x: 5, y: -1 }));
        assert!(!r1.contains(Vector2 { x: 10, y: 5 }));
    }

    #[test]
    fn test_get_corner() {
        let r1 = Rect::new(1, 3, 10, 10);
        let pos = r1.get_corner();

        assert_eq!(pos.x, 1);
        assert_eq!(pos.y, 3);
    }

    #[test]
    fn test_get_size() {
        let r1 = Rect::new(1, 3, 10, 12);
        let size = r1.get_size();

        assert_eq!(size.x, 10);
        assert_eq!(size.y, 12);
    }

    #[test]
    fn test_rect_sanity_check() {
        let rect_int = Rect::new(1, 3, 5, 10);
        let csfml_rect = rect_int.to_csfml();
        let rect_from_csfml: Rect<i32> = Rect::from(csfml_rect);

        assert_eq!(rect_int, rect_from_csfml);
    }
}
