use csfml_sys::{
    sfTransform, sfTransform_combine, sfTransform_getInverse, sfTransform_rotate,
    sfTransform_scale, sfTransform_transformPoint, sfTransform_transformRect,
    sfTransform_translate, sfVector2f,
};

use crate::system::Vector2f;

use super::rect::FloatRect;

/// A 3x3 transformation matrix (used for translation, rotation, scaling, etc.)
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub matrix: [f32; 9],
}

impl Transform {
    /// Creates a new `Transform` matrix from raw values (for testing or custom transformation)
    #[must_use]
    pub const fn new(matrix: [f32; 9]) -> Self {
        Self { matrix }
    }

    /// The identity matrix (doesn't do anything)
    pub const IDENTITY: Self = Self {
        matrix: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
    };

    /// Converts this transform to a csfml one
    /// For internal FFI interactions
    #[must_use]
    pub fn to_csfml(self) -> sfTransform {
        unsafe { std::mem::transmute(self) }
    }

    /// Converts a csfml transform to a Rust `Transform`
    #[must_use]
    pub fn from_csfml(transform: sfTransform) -> Self {
        unsafe { std::mem::transmute(transform) }
    }

    /// Transforms a point by this matrix
    #[must_use]
    pub fn transform_point(&self, point: Vector2f) -> Vector2f {
        unsafe {
            let ptr = &self.to_csfml() as *const sfTransform;
            let transformed = sfTransform_transformPoint(ptr, sfVector2f::from(point));
            Vector2f::from(transformed)
        }
    }

    /// Transforms a rectangle by this matrix
    #[must_use]
    pub fn transform_rect(&self, rect: FloatRect) -> FloatRect {
        unsafe {
            let ptr = &self.to_csfml() as *const sfTransform;
            let transformed = sfTransform_transformRect(ptr, rect.to_csfml());
            FloatRect::from_csfml(transformed)
        }
    }

    /// Gets the inverse of this transformation.
    /// Returns the identity matrix if it can't be calculated.
    #[must_use]
    pub fn get_inverse(&self) -> Self {
        unsafe {
            let ptr = &self.to_csfml() as *const sfTransform;
            let inverse = sfTransform_getInverse(ptr);
            Self::from_csfml(inverse)
        }
    }

    /// Combines two transformations.
    pub fn combine(&mut self, other: Self) {
        unsafe {
            let ptr_self = &mut self.to_csfml() as *mut sfTransform;
            let ptr_other = &other.to_csfml() as *const sfTransform;
            sfTransform_combine(ptr_self, ptr_other);
        }
    }

    /// Translates this transform by x and y
    pub fn translate(&mut self, translation: Vector2f) {
        unsafe {
            let ptr = &mut self.to_csfml() as *mut sfTransform;
            sfTransform_translate(ptr, translation.x, translation.y);
        }
    }

    /// Rotates this transform by a given angle (in degrees)
    pub fn rotate(&mut self, angle: f32) {
        unsafe {
            let ptr = &mut self.to_csfml() as *mut sfTransform;
            sfTransform_rotate(ptr, angle);
        }
    }

    /// Scales this transform by the given factor (x and y)
    pub fn scale(&mut self, factor: Vector2f) {
        unsafe {
            let ptr = &mut self.to_csfml() as *mut sfTransform;
            sfTransform_scale(ptr, factor.x, factor.y);
        }
    }
}
