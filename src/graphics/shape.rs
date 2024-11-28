use csfml_sys::{
    sfCircleShape, sfCircleShape_create, sfCircleShape_destroy, sfCircleShape_getFillColor,
    sfCircleShape_getGlobalBounds, sfCircleShape_getLocalBounds, sfCircleShape_getOrigin,
    sfCircleShape_getOutlineColor, sfCircleShape_getOutlineThickness, sfCircleShape_getPoint,
    sfCircleShape_getPointCount, sfCircleShape_getPosition, sfCircleShape_getRotation,
    sfCircleShape_getTexture, sfCircleShape_getTextureRect, sfCircleShape_move,
    sfCircleShape_rotate, sfCircleShape_setFillColor, sfCircleShape_setOrigin,
    sfCircleShape_setOutlineColor, sfCircleShape_setOutlineThickness, sfCircleShape_setPosition,
    sfCircleShape_setRadius, sfCircleShape_setRotation, sfCircleShape_setTexture,
    sfCircleShape_setTextureRect, sfColor, sfConvexShape, sfConvexShape_create,
    sfConvexShape_destroy, sfConvexShape_getFillColor, sfConvexShape_getGlobalBounds,
    sfConvexShape_getLocalBounds, sfConvexShape_getOrigin, sfConvexShape_getOutlineColor,
    sfConvexShape_getOutlineThickness, sfConvexShape_getPoint, sfConvexShape_getPointCount,
    sfConvexShape_getPosition, sfConvexShape_getRotation, sfConvexShape_getTexture,
    sfConvexShape_getTextureRect, sfConvexShape_move, sfConvexShape_rotate,
    sfConvexShape_setFillColor, sfConvexShape_setOrigin, sfConvexShape_setOutlineColor,
    sfConvexShape_setOutlineThickness, sfConvexShape_setPosition, sfConvexShape_setRotation,
    sfConvexShape_setTexture, sfConvexShape_setTextureRect, sfRectangleShape,
    sfRectangleShape_create, sfRectangleShape_destroy, sfRectangleShape_getFillColor,
    sfRectangleShape_getGlobalBounds, sfRectangleShape_getLocalBounds, sfRectangleShape_getOrigin,
    sfRectangleShape_getOutlineColor, sfRectangleShape_getOutlineThickness,
    sfRectangleShape_getPoint, sfRectangleShape_getPointCount, sfRectangleShape_getPosition,
    sfRectangleShape_getRotation, sfRectangleShape_getTexture, sfRectangleShape_getTextureRect,
    sfRectangleShape_move, sfRectangleShape_rotate, sfRectangleShape_setFillColor,
    sfRectangleShape_setOrigin, sfRectangleShape_setOutlineColor,
    sfRectangleShape_setOutlineThickness, sfRectangleShape_setPosition,
    sfRectangleShape_setRotation, sfRectangleShape_setSize, sfRectangleShape_setTexture,
    sfRectangleShape_setTextureRect, sfVector2f, sfWhite,
};

use crate::{system::Vector2f, types::Result, utils::HasCsfmlPointer};

use super::{
    color::Color,
    rect::{FloatRect, IntRect},
    texture::Texture,
};

pub trait Shape: HasCsfmlPointer {
    fn get_fill_color(&self) -> Color;
    fn set_fill_color(&mut self, color: Color);

    fn get_outline_color(&self) -> Color;
    fn set_outline_color(&mut self, color: Color);

    fn get_outline_thickness(&self) -> f32;
    fn set_outline_thickness(&mut self, thickness: f32);

    fn get_position(&self) -> Vector2f;
    fn set_position(&mut self, position: Vector2f);

    fn move_shape(&mut self, offset: Vector2f);

    fn get_origin(&self) -> Vector2f;
    fn set_origin(&mut self, origin: Vector2f);

    fn get_rotation(&self) -> f32;
    fn set_rotation(&mut self, angle: f32);
    fn rotate(&mut self, angle: f32);

    fn get_texture(&self) -> Option<Texture>;
    fn set_texture(&mut self, texture: Option<Texture>);

    fn get_texture_rect(&self) -> IntRect;
    fn set_texture_rect(&mut self, rect: IntRect);

    fn get_local_bounds(&self) -> FloatRect;
    fn get_global_bounds(&self) -> FloatRect;

    fn get_point_count(&self) -> usize;
    fn get_point(&self, index: usize) -> Vector2f;
}

pub struct CircleShape {
    ptr: *mut sfCircleShape,
}

impl Drop for CircleShape {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl CircleShape {
    pub fn new(radius: f32) -> Result<Self> {
        let shape = unsafe { sfCircleShape_create() };
        if shape.is_null() {
            return Err("Failed to create CircleShape".into());
        }
        unsafe {
            sfCircleShape_setFillColor(shape, sfWhite);
            sfCircleShape_setRadius(shape, radius);
        }
        Ok(Self { ptr: shape })
    }

    pub fn destroy(&mut self) {
        unsafe {
            sfCircleShape_destroy(self.ptr);
        }
        self.ptr = std::ptr::null_mut();
    }
}

impl HasCsfmlPointer for CircleShape {
    type Output = sfCircleShape;

    fn mut_ptr(&self) -> *mut Self::Output {
        self.ptr
    }
}

impl Shape for CircleShape {
    fn get_fill_color(&self) -> Color {
        unsafe { Color::from(sfCircleShape_getFillColor(self.ptr)) }
    }

    fn set_fill_color(&mut self, color: Color) {
        unsafe {
            sfCircleShape_setFillColor(self.ptr, color.to_csfml());
        }
    }

    fn get_outline_color(&self) -> Color {
        unsafe { Color::from(sfCircleShape_getOutlineColor(self.ptr)) }
    }

    fn set_outline_color(&mut self, color: Color) {
        unsafe {
            sfCircleShape_setOutlineColor(self.ptr, color.to_csfml());
        }
    }

    fn get_outline_thickness(&self) -> f32 {
        unsafe { sfCircleShape_getOutlineThickness(self.ptr) }
    }

    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe {
            sfCircleShape_setOutlineThickness(self.ptr, thickness);
        }
    }

    fn get_position(&self) -> Vector2f {
        unsafe { Vector2f::from(sfCircleShape_getPosition(self.ptr)) }
    }

    fn set_position(&mut self, position: Vector2f) {
        unsafe {
            sfCircleShape_setPosition(self.ptr, sfVector2f::from(position));
        }
    }

    fn move_shape(&mut self, offset: Vector2f) {
        unsafe {
            sfCircleShape_move(self.ptr, sfVector2f::from(offset));
        }
    }

    fn get_origin(&self) -> Vector2f {
        unsafe { Vector2f::from(sfCircleShape_getOrigin(self.ptr)) }
    }

    fn set_origin(&mut self, origin: Vector2f) {
        unsafe {
            sfCircleShape_setOrigin(self.ptr, sfVector2f::from(origin));
        }
    }

    fn get_rotation(&self) -> f32 {
        unsafe { sfCircleShape_getRotation(self.ptr) }
    }

    fn set_rotation(&mut self, angle: f32) {
        unsafe {
            sfCircleShape_setRotation(self.ptr, angle);
        }
    }

    fn rotate(&mut self, angle: f32) {
        unsafe {
            sfCircleShape_rotate(self.ptr, angle);
        }
    }

    fn get_texture(&self) -> Option<Texture> {
        let texture_ptr = unsafe { sfCircleShape_getTexture(self.ptr) };
        if texture_ptr.is_null() {
            None
        } else {
            Some(Texture::Const(texture_ptr))
        }
    }

    fn set_texture(&mut self, texture: Option<Texture>) {
        unsafe {
            sfCircleShape_setTexture(
                self.ptr,
                texture.map_or(std::ptr::null_mut(), |tex| tex.ptr()),
                0,
            );
        }
    }

    fn get_texture_rect(&self) -> IntRect {
        unsafe { IntRect::from_csfml(sfCircleShape_getTextureRect(self.ptr)) }
    }

    fn set_texture_rect(&mut self, rect: IntRect) {
        unsafe {
            sfCircleShape_setTextureRect(self.ptr, rect.to_csfml());
        }
    }

    fn get_local_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_csfml(sfCircleShape_getLocalBounds(self.ptr)) }
    }

    fn get_global_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_csfml(sfCircleShape_getGlobalBounds(self.ptr)) }
    }

    fn get_point_count(&self) -> usize {
        unsafe { sfCircleShape_getPointCount(self.ptr) as usize }
    }

    fn get_point(&self, index: usize) -> Vector2f {
        unsafe { Vector2f::from(sfCircleShape_getPoint(self.ptr, index)) }
    }
}

pub struct RectangleShape {
    ptr: *mut sfRectangleShape,
}

impl Drop for RectangleShape {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl RectangleShape {
    pub fn create(size: Vector2f) -> Result<Self> {
        let shape = unsafe { sfRectangleShape_create() };

        if shape.is_null() {
            return Err("Failed to create Shape".into());
        }

        unsafe {
            sfRectangleShape_setFillColor(shape, sfWhite);
            sfRectangleShape_setSize(shape, sfVector2f::from(size));
        }

        Ok(Self { ptr: shape })
    }

    pub fn destroy(&mut self) {
        unsafe {
            sfRectangleShape_destroy(self.ptr);
        }
        self.ptr = std::ptr::null_mut();
    }
}

impl HasCsfmlPointer for RectangleShape {
    type Output = sfRectangleShape;

    fn mut_ptr(&self) -> *mut Self::Output {
        self.ptr
    }
}

impl Shape for RectangleShape {
    fn get_fill_color(&self) -> Color {
        Color::from(unsafe { sfRectangleShape_getFillColor(self.ptr) })
    }

    fn set_fill_color(&mut self, color: Color) {
        unsafe { sfRectangleShape_setFillColor(self.ptr, sfColor::from(color)) };
    }

    fn get_outline_color(&self) -> Color {
        Color::from(unsafe { sfRectangleShape_getOutlineColor(self.ptr) })
    }

    fn set_outline_color(&mut self, color: Color) {
        unsafe { sfRectangleShape_setOutlineColor(self.ptr, sfColor::from(color)) };
    }

    fn get_outline_thickness(&self) -> f32 {
        unsafe { sfRectangleShape_getOutlineThickness(self.ptr) }
    }

    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe {
            sfRectangleShape_setOutlineThickness(self.ptr, thickness);
        }
    }

    fn get_position(&self) -> Vector2f {
        Vector2f::from(unsafe { sfRectangleShape_getPosition(self.ptr) })
    }

    fn set_position(&mut self, position: Vector2f) {
        unsafe { sfRectangleShape_setPosition(self.ptr, sfVector2f::from(position)) };
    }

    fn move_shape(&mut self, offset: Vector2f) {
        unsafe {
            sfRectangleShape_move(self.ptr, sfVector2f::from(offset));
        }
    }

    fn get_origin(&self) -> Vector2f {
        Vector2f::from(unsafe { sfRectangleShape_getOrigin(self.ptr) })
    }

    fn set_origin(&mut self, origin: Vector2f) {
        unsafe {
            sfRectangleShape_setOrigin(self.ptr, sfVector2f::from(origin));
        }
    }

    fn get_rotation(&self) -> f32 {
        unsafe { sfRectangleShape_getRotation(self.ptr) }
    }

    fn set_rotation(&mut self, angle: f32) {
        unsafe { sfRectangleShape_setRotation(self.ptr, angle) };
    }

    fn rotate(&mut self, angle: f32) {
        unsafe {
            sfRectangleShape_rotate(self.ptr, angle);
        }
    }

    fn get_texture(&self) -> Option<Texture> {
        let texture = unsafe { sfRectangleShape_getTexture(self.ptr) };

        if self.ptr.is_null() {
            None
        } else {
            Some(Texture::Const(texture))
        }
    }

    fn set_texture(&mut self, texture: Option<Texture>) {
        unsafe {
            sfRectangleShape_setTexture(
                self.ptr,
                texture.map_or(std::ptr::null_mut(), |tex| tex.ptr()),
                0,
            );
        }
    }

    fn get_texture_rect(&self) -> IntRect {
        IntRect::from_csfml(unsafe { sfRectangleShape_getTextureRect(self.ptr) })
    }

    fn set_texture_rect(&mut self, rect: IntRect) {
        unsafe { sfRectangleShape_setTextureRect(self.ptr, rect.to_csfml()) };
    }

    fn get_local_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from(sfRectangleShape_getLocalBounds(self.ptr)) }
    }

    fn get_global_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from(sfRectangleShape_getGlobalBounds(self.ptr)) }
    }

    fn get_point_count(&self) -> usize {
        unsafe { sfRectangleShape_getPointCount(self.ptr) }
    }

    fn get_point(&self, index: usize) -> Vector2f {
        unsafe { Vector2f::from(sfRectangleShape_getPoint(self.ptr, index)) }
    }
}

pub struct ConvexShape {
    ptr: *mut sfConvexShape,
}

impl Drop for ConvexShape {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl ConvexShape {
    pub fn create() -> Result<Self> {
        let shape = unsafe { sfConvexShape_create() };

        if shape.is_null() {
            return Err("Failed to create Shape".into());
        }

        Ok(Self { ptr: shape })
    }

    pub fn destroy(&mut self) {
        unsafe {
            sfConvexShape_destroy(self.ptr);
        }
        self.ptr = std::ptr::null_mut();
    }
}

impl HasCsfmlPointer for ConvexShape {
    type Output = sfConvexShape;

    fn mut_ptr(&self) -> *mut Self::Output {
        self.ptr
    }
}

impl Shape for ConvexShape {
    fn get_fill_color(&self) -> Color {
        Color::from(unsafe { sfConvexShape_getFillColor(self.ptr) })
    }

    fn set_fill_color(&mut self, color: Color) {
        unsafe { sfConvexShape_setFillColor(self.ptr, sfColor::from(color)) };
    }

    fn get_outline_color(&self) -> Color {
        Color::from(unsafe { sfConvexShape_getOutlineColor(self.ptr) })
    }

    fn set_outline_color(&mut self, color: Color) {
        unsafe { sfConvexShape_setOutlineColor(self.ptr, sfColor::from(color)) };
    }

    fn get_outline_thickness(&self) -> f32 {
        unsafe { sfConvexShape_getOutlineThickness(self.ptr) }
    }

    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe {
            sfConvexShape_setOutlineThickness(self.ptr, thickness);
        }
    }

    fn get_position(&self) -> Vector2f {
        Vector2f::from(unsafe { sfConvexShape_getPosition(self.ptr) })
    }

    fn set_position(&mut self, position: Vector2f) {
        unsafe { sfConvexShape_setPosition(self.ptr, sfVector2f::from(position)) };
    }

    fn move_shape(&mut self, offset: Vector2f) {
        unsafe {
            sfConvexShape_move(self.ptr, sfVector2f::from(offset));
        }
    }

    fn get_origin(&self) -> Vector2f {
        Vector2f::from(unsafe { sfConvexShape_getOrigin(self.ptr) })
    }

    fn set_origin(&mut self, origin: Vector2f) {
        unsafe {
            sfConvexShape_setOrigin(self.ptr, sfVector2f::from(origin));
        }
    }

    fn get_rotation(&self) -> f32 {
        unsafe { sfConvexShape_getRotation(self.ptr) }
    }

    fn set_rotation(&mut self, angle: f32) {
        unsafe { sfConvexShape_setRotation(self.ptr, angle) };
    }

    fn rotate(&mut self, angle: f32) {
        unsafe {
            sfConvexShape_rotate(self.ptr, angle);
        }
    }

    fn get_texture(&self) -> Option<Texture> {
        let texture = unsafe { sfConvexShape_getTexture(self.ptr) };

        if self.ptr.is_null() {
            None
        } else {
            Some(Texture::Const(texture))
        }
    }

    fn set_texture(&mut self, texture: Option<Texture>) {
        unsafe {
            sfConvexShape_setTexture(
                self.ptr,
                texture.map_or(std::ptr::null_mut(), |tex| tex.ptr()),
                0,
            );
        }
    }

    fn get_texture_rect(&self) -> IntRect {
        IntRect::from_csfml(unsafe { sfConvexShape_getTextureRect(self.ptr) })
    }

    fn set_texture_rect(&mut self, rect: IntRect) {
        unsafe { sfConvexShape_setTextureRect(self.ptr, rect.to_csfml()) };
    }

    fn get_local_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from(sfConvexShape_getLocalBounds(self.ptr)) }
    }

    fn get_global_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from(sfConvexShape_getGlobalBounds(self.ptr)) }
    }

    fn get_point_count(&self) -> usize {
        unsafe { sfConvexShape_getPointCount(self.ptr) }
    }

    fn get_point(&self, index: usize) -> Vector2f {
        unsafe { Vector2f::from(sfConvexShape_getPoint(self.ptr, index)) }
    }
}
