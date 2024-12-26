use std::ptr::{self};

use csfml_sys::{
    sfSprite, sfSprite_create, sfSprite_destroy, sfSprite_getColor, sfSprite_getGlobalBounds,
    sfSprite_getLocalBounds, sfSprite_getOrigin, sfSprite_getPosition, sfSprite_getRotation,
    sfSprite_getScale, sfSprite_getTexture, sfSprite_getTextureRect, sfSprite_move,
    sfSprite_rotate, sfSprite_scale, sfSprite_setColor, sfSprite_setOrigin, sfSprite_setPosition,
    sfSprite_setRotation, sfSprite_setScale, sfSprite_setTexture, sfSprite_setTextureRect,
    sfVector2f,
};

use crate::{system::Vector2f, types::Result};

use super::{
    color::Color,
    rect::{FloatRect, IntRect},
    texture::Texture,
};

pub struct Sprite {
    pub(crate) ptr: *mut sfSprite,
}

impl Drop for Sprite {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Sprite {
    pub fn create() -> Result<Self> {
        let sprite = unsafe { sfSprite_create() };
        if sprite.is_null() {
            return Err("Failed to create sprite".into());
        }

        Ok(Self { ptr: sprite })
    }

    pub fn create_from_texture(texture: &Texture) -> Result<Self> {
        let sprite = unsafe { sfSprite_create() };
        if sprite.is_null() {
            return Err("Failed to create sprite".into());
        }

        unsafe {
            sfSprite_setTexture(sprite, texture.ptr(), 1);
        }

        Ok(Self { ptr: sprite })
    }

    pub fn destroy(&self) {
        if !self.ptr.is_null() {
            unsafe {
                sfSprite_destroy(self.ptr);
            }
        }
    }

    #[must_use]
    pub fn get_position(&self) -> Vector2f {
        unsafe {
            let pos = sfSprite_getPosition(self.ptr);
            Vector2f::from(pos)
        }
    }

    pub fn set_position(&mut self, pos: Vector2f) {
        unsafe {
            sfSprite_setPosition(self.ptr, sfVector2f::from(pos));
        }
    }

    pub fn move_sprite(&mut self, offset: Vector2f) {
        unsafe {
            sfSprite_move(self.ptr, sfVector2f::from(offset));
        }
    }

    #[must_use]
    pub fn get_scale(&self) -> Vector2f {
        unsafe {
            let scale = sfSprite_getScale(self.ptr);
            Vector2f::from(scale)
        }
    }

    pub fn set_scale(&mut self, factor: Vector2f) {
        unsafe {
            sfSprite_setScale(self.ptr, sfVector2f::from(factor));
        }
    }

    pub fn scale_sprite(&mut self, factor: Vector2f) {
        unsafe {
            sfSprite_scale(self.ptr, sfVector2f::from(factor));
        }
    }

    #[must_use]
    pub fn get_origin(&self) -> Vector2f {
        unsafe {
            let origin = sfSprite_getOrigin(self.ptr);
            Vector2f::from(origin)
        }
    }

    pub fn set_origin(&mut self, origin: Vector2f) {
        unsafe {
            sfSprite_setOrigin(self.ptr, sfVector2f::from(origin));
        }
    }

    #[must_use]
    pub fn get_rotation(&self) -> f32 {
        unsafe { sfSprite_getRotation(self.ptr) }
    }

    pub fn set_rotation(&mut self, angle: f32) {
        unsafe {
            sfSprite_setRotation(self.ptr, angle);
        }
    }

    pub fn rotate_sprite(&mut self, angle: f32) {
        unsafe {
            sfSprite_rotate(self.ptr, angle);
        }
    }

    #[must_use]
    pub fn get_color(&self) -> Color {
        unsafe {
            let color = sfSprite_getColor(self.ptr);
            Color::from_csfml(color)
        }
    }

    pub fn set_color(&mut self, color: Color) {
        unsafe {
            sfSprite_setColor(self.ptr, color.to_csfml());
        }
    }

    #[must_use]
    pub fn get_texture(&self) -> Option<Texture> {
        unsafe {
            let texture = sfSprite_getTexture(self.ptr);
            if texture.is_null() {
                None
            } else {
                Some(Texture::Const(texture))
            }
        }
    }

    pub fn set_texture(&mut self, texture: Option<&Texture>) {
        let tex_ptr = texture.map_or(ptr::null(), super::texture::Texture::ptr);
        unsafe {
            sfSprite_setTexture(self.ptr, tex_ptr, 1);
        }
    }

    #[must_use]
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            let rect = sfSprite_getTextureRect(self.ptr);
            IntRect::from_csfml(rect)
        }
    }

    pub fn set_texture_rect(&mut self, rect: IntRect) {
        unsafe {
            sfSprite_setTextureRect(self.ptr, rect.to_csfml());
        }
    }

    #[must_use]
    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            let bounds = sfSprite_getLocalBounds(self.ptr);
            FloatRect::from_csfml(bounds)
        }
    }

    #[must_use]
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe {
            let bounds = sfSprite_getGlobalBounds(self.ptr);
            FloatRect::from_csfml(bounds)
        }
    }
}

// Usage of this structure in Rust would look similar to the Zig test case, for example:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_getters_setters() {
        let mut spr = Sprite::create().unwrap();

        spr.set_color(Color::YELLOW);
        spr.set_rotation(15.0);
        spr.set_position(Vector2f::new(1.0, 2.0));
        spr.set_origin(Vector2f::new(20.0, 25.0));
        spr.set_scale(Vector2f::new(2.0, 2.0));
        spr.set_texture(None);

        assert_eq!(spr.get_color(), Color::YELLOW);
        assert_eq!(spr.get_position(), Vector2f::new(1.0, 2.0));
        assert_eq!(spr.get_origin(), Vector2f::new(20.0, 25.0));
        assert_eq!(spr.get_texture(), None);
        assert_eq!(spr.get_scale(), Vector2f::new(2.0, 2.0));

        spr.rotate_sprite(5.0);
        spr.move_sprite(Vector2f::new(-5.0, 5.0));
        spr.scale_sprite(Vector2f::new(5.0, 5.0));

        assert_eq!(spr.get_rotation(), 20.0);
        assert_eq!(spr.get_position(), Vector2f::new(-4.0, 7.0));
        assert_eq!(spr.get_scale(), Vector2f::new(10.0, 10.0));
    }
}
