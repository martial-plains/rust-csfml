use std::ffi::CString;
use std::ptr;

use csfml_sys::{
    sfText, sfText_create, sfText_destroy, sfText_getCharacterSize, sfText_getFillColor,
    sfText_getGlobalBounds, sfText_getLocalBounds, sfText_getOrigin, sfText_getOutlineColor,
    sfText_getOutlineThickness, sfText_getPosition, sfText_getRotation, sfText_getScale,
    sfText_move, sfText_rotate, sfText_scale, sfText_setCharacterSize, sfText_setFillColor,
    sfText_setFont, sfText_setOrigin, sfText_setOutlineColor, sfText_setOutlineThickness,
    sfText_setPosition, sfText_setRotation, sfText_setScale, sfText_setString,
    sfText_setUnicodeString, sfVector2f,
};

use crate::{system::Vector2f, types::Result};

use super::{color::Color, rect::FloatRect, Font};

#[repr(C)]
pub struct Text {
    pub(crate) ptr: *mut sfText,
}

impl Drop for Text {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Text {
    pub fn create() -> Result<Self> {
        unsafe {
            let text = sfText_create();
            if text.is_null() {
                Err("Failed to create text".into())
            } else {
                Ok(Self { ptr: text })
            }
        }
    }

    pub fn create_with_text(string: &str, font: &Font, character_size: usize) -> Result<Self> {
        unsafe {
            let text = sfText_create();
            if text.is_null() {
                return Err("Failed to create text".into());
            }
            sfText_setFont(text, font.ptr);
            sfText_setCharacterSize(text, character_size as u32);
            let cstr = CString::new(string).unwrap();
            sfText_setString(text, cstr.as_ptr());
            Ok(Self { ptr: text })
        }
    }

    pub fn create_with_unicode(string: &[u32], font: &Font, character_size: usize) -> Result<Self> {
        unsafe {
            let text = sfText_create();
            if text.is_null() {
                return Err("Failed to create text".into());
            }
            sfText_setFont(text, font.ptr);
            sfText_setCharacterSize(text, character_size as u32);
            sfText_setUnicodeString(text, string.as_ptr());
            Ok(Self { ptr: text })
        }
    }

    pub fn destroy(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                sfText_destroy(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }

    pub fn set_string(&mut self, string: &str) {
        let cstr = CString::new(string).unwrap();
        unsafe {
            sfText_setString(self.ptr, cstr.as_ptr());
        }
    }

    pub fn set_string_unicode(&mut self, string: &[u32]) {
        unsafe {
            sfText_setUnicodeString(self.ptr, string.as_ptr());
        }
    }

    pub fn set_font(&mut self, font: &Font) {
        unsafe {
            sfText_setFont(self.ptr, font.ptr);
        }
    }

    pub fn set_fill_color(&mut self, color: Color) {
        unsafe {
            sfText_setFillColor(self.ptr, color.to_csfml());
        }
    }

    pub fn set_outline_color(&mut self, color: Color) {
        unsafe {
            sfText_setOutlineColor(self.ptr, color.to_csfml());
        }
    }

    pub fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe {
            sfText_setOutlineThickness(self.ptr, thickness);
        }
    }

    pub fn set_character_size(&mut self, size: usize) {
        unsafe {
            sfText_setCharacterSize(self.ptr, size as u32);
        }
    }

    pub fn set_position(&mut self, position: Vector2f) {
        unsafe {
            sfText_setPosition(self.ptr, sfVector2f::from(position));
        }
    }

    pub fn set_origin(&mut self, origin: Vector2f) {
        unsafe {
            sfText_setOrigin(self.ptr, sfVector2f::from(origin));
        }
    }

    pub fn set_rotation(&mut self, angle: f32) {
        unsafe {
            sfText_setRotation(self.ptr, angle);
        }
    }

    pub fn set_scale(&mut self, scale: Vector2f) {
        unsafe {
            sfText_setScale(self.ptr, sfVector2f::from(scale));
        }
    }

    #[must_use]
    pub fn fill_color(&self) -> Color {
        unsafe { Color::from_csfml(sfText_getFillColor(self.ptr)) }
    }

    #[must_use]
    pub fn outline_color(&self) -> Color {
        unsafe { Color::from_csfml(sfText_getOutlineColor(self.ptr)) }
    }

    #[must_use]
    pub fn outline_thickness(&self) -> f32 {
        unsafe { sfText_getOutlineThickness(self.ptr) }
    }

    #[must_use]
    pub fn character_size(&self) -> usize {
        unsafe { sfText_getCharacterSize(self.ptr) as usize }
    }

    #[must_use]
    pub fn position(&self) -> Vector2f {
        unsafe { Vector2f::from(sfText_getPosition(self.ptr)) }
    }

    pub fn r#move(&self, offset: Vector2f) {
        unsafe {
            sfText_move(self.ptr, sfVector2f::from(offset));
        }
    }

    #[must_use]
    pub fn origin(&self) -> Vector2f {
        unsafe { Vector2f::from(sfText_getOrigin(self.ptr)) }
    }

    #[must_use]
    pub fn rotation(&self) -> f32 {
        unsafe { sfText_getRotation(self.ptr) }
    }

    /// Rotates this text by a given amount
    pub fn rotate(&self, angle: f32) {
        unsafe { sfText_rotate(self.ptr, angle) };
    }

    #[must_use]
    pub fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from(sfText_getScale(self.ptr)) }
    }

    pub fn scale(&self, factor: Vector2f) {
        unsafe {
            sfText_scale(self.ptr, sfVector2f::from(factor));
        }
    }

    #[must_use]
    pub fn local_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_csfml(sfText_getLocalBounds(self.ptr)) }
    }

    #[must_use]
    pub fn global_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_csfml(sfText_getGlobalBounds(self.ptr)) }
    }

    pub fn center_origin(&mut self) {
        let bounds = self.local_bounds();
        self.set_origin(Vector2f::new(bounds.width / 2.0, bounds.height / 2.9));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_sane_getters_and_setters() {
        let mut text = Text::create().expect("Failed to create text");

        text.set_string("hello");

        text.set_fill_color(Color::YELLOW);
        text.set_outline_color(Color::RED);
        text.set_outline_thickness(2.0);
        text.set_character_size(10);
        text.set_rotation(15.0);
        text.set_position(Vector2f { x: 1.0, y: 2.0 });
        text.set_origin(Vector2f { x: 20.0, y: 25.0 });
        text.set_scale(Vector2f { x: 2.0, y: 2.0 });

        text.rotate(5.0);
        text.r#move(Vector2f { x: -5.0, y: 5.0 });
        text.scale(Vector2f { x: 2.0, y: 3.0 });

        assert_eq!(text.fill_color(), Color::YELLOW);
        assert_eq!(text.outline_color(), Color::RED);
        assert_eq!(text.outline_thickness(), 2.0);
        assert_eq!(text.character_size(), 10);
        assert_eq!(text.rotation(), 20.0);
        assert_eq!(text.position(), Vector2f { x: -4.0, y: 7.0 });
        assert_eq!(text.origin(), Vector2f { x: 20.0, y: 25.0 });
        assert_eq!(text.get_scale(), Vector2f { x: 4.0, y: 6.0 });

        let local_bounds = text.local_bounds();
        let global_bounds = text.global_bounds();
    }
}
