use std::ptr::{self, null_mut};

use csfml_sys::{
    sfBool, sfColor, sfRenderTexture, sfRenderTexture_clear, sfRenderTexture_create,
    sfRenderTexture_destroy, sfRenderTexture_display, sfRenderTexture_drawCircleShape,
    sfRenderTexture_drawConvexShape, sfRenderTexture_drawRectangleShape,
    sfRenderTexture_drawSprite, sfRenderTexture_drawText, sfRenderTexture_drawVertexArray,
    sfRenderTexture_drawVertexBuffer, sfRenderTexture_generateMipmap, sfRenderTexture_getSize,
    sfRenderTexture_getTexture, sfRenderTexture_isRepeated, sfRenderTexture_isSmooth,
    sfRenderTexture_setRepeated, sfRenderTexture_setSmooth, sfVector2f, sfVector2i,
};

use crate::{
    system::{Vector2f, Vector2i, Vector2u},
    types::Result,
    utils::HasCsfmlPointer,
};

use super::{
    color::Color,
    rect::IntRect,
    text::Text,
    texture::Texture,
    vertex::{VertexArray, VertexBuffer},
    CircleShape, ConvexShape, RectangleShape, RenderStates, Sprite, View,
};

// Define a Drawable trait
pub trait RenderTextureDrawable {
    fn draw_to_render_texture(
        &self,
        render_texture: &mut RenderTexture,
        states: Option<&RenderStates>,
    );
}

#[repr(C)]
pub struct RenderTexture {
    ptr: *mut sfRenderTexture,
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl RenderTexture {
    pub fn create(size: Vector2u) -> Result<Self> {
        unsafe {
            let render_texture = sfRenderTexture_create(size.x, size.y, 0);
            if render_texture.is_null() {
                Err("Failed to create render texture".into())
            } else {
                Ok(Self {
                    ptr: render_texture,
                })
            }
        }
    }

    pub fn create_with_depth_buffer(size: Vector2u) -> Result<Self> {
        unsafe {
            let render_texture = sfRenderTexture_create(size.x, size.y, 1);
            if render_texture.is_null() {
                Err("Failed to create render texture with depth buffer".into())
            } else {
                Ok(Self {
                    ptr: render_texture,
                })
            }
        }
    }

    pub fn destroy(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                sfRenderTexture_destroy(self.ptr);
                self.ptr = null_mut();
            }
        }
    }

    pub fn clear(&mut self, color: Color) {
        unsafe {
            sfRenderTexture_clear(self.ptr, sfColor::from(color));
        }
    }

    pub fn display(&mut self) {
        unsafe {
            sfRenderTexture_display(self.ptr);
        }
    }

    pub fn draw<T: RenderTextureDrawable>(&mut self, drawable: &T, states: Option<&RenderStates>) {
        drawable.draw_to_render_texture(self, states);
    }

    #[must_use]
    pub fn texture(&self) -> Texture {
        unsafe {
            let tex_ptr = sfRenderTexture_getTexture(self.ptr);
            Texture::Const(tex_ptr)
        }
    }

    /// Generates a mipmap for the current texture data, returns true if the operation succeeded
    #[must_use]
    pub fn generate_mipmap(&self) -> bool {
        unsafe { sfRenderTexture_generateMipmap(self.ptr) != 0 }
    }

    pub fn set_smooth(&mut self, smooth: bool) {
        unsafe {
            sfRenderTexture_setSmooth(self.ptr, sfBool::from(smooth));
        }
    }

    #[must_use]
    pub fn is_smooth(&self) -> bool {
        unsafe { sfRenderTexture_isSmooth(self.ptr) != 0 }
    }

    pub fn set_repeated(&mut self, repeated: bool) {
        unsafe {
            sfRenderTexture_setRepeated(self.ptr, sfBool::from(repeated));
        }
    }

    #[must_use]
    pub fn is_repeated(&self) -> bool {
        unsafe { sfRenderTexture_isRepeated(self.ptr) != 0 }
    }

    #[must_use]
    pub fn size(&self) -> Vector2u {
        Vector2u::from(unsafe { sfRenderTexture_getSize(self.ptr) })
    }

    #[must_use]
    pub fn view(&self) -> View {
        unsafe {
            let view_ptr = csfml_sys::sfRenderTexture_getView(self.ptr);
            View::from_csfml(view_ptr)
        }
    }

    #[must_use]
    pub fn default_view(&self) -> View {
        unsafe {
            let view_ptr = csfml_sys::sfRenderTexture_getDefaultView(self.ptr);
            View::from_csfml(view_ptr)
        }
    }

    pub fn set_view(&mut self, view: View) {
        unsafe {
            csfml_sys::sfRenderTexture_setView(self.ptr, view.to_csfml());
        }
    }

    #[must_use]
    pub fn viewport(&self, view: View) -> IntRect {
        IntRect::from_csfml(unsafe {
            csfml_sys::sfRenderTexture_getViewport(self.ptr, view.to_csfml())
        })
    }

    #[must_use]
    pub fn map_pixel_to_coords(&self, pixel: Vector2i, view: Option<View>) -> Vector2f {
        unsafe {
            let cview = view.map_or(null_mut(), |v| v.to_csfml());
            let coords_ptr = csfml_sys::sfRenderTexture_mapPixelToCoords(
                self.ptr,
                sfVector2i::from(pixel),
                cview,
            );
            Vector2f::from(coords_ptr)
        }
    }

    #[must_use]
    pub fn map_coords_to_pixel(&self, coords: Vector2f, view: Option<View>) -> Vector2i {
        unsafe {
            let cview = view.map_or(null_mut(), |v| v.to_csfml());
            let pixel_ptr = csfml_sys::sfRenderTexture_mapCoordsToPixel(
                self.ptr,
                sfVector2f::from(coords),
                cview,
            );
            Vector2i::from(pixel_ptr)
        }
    }
}

impl RenderTextureDrawable for Sprite {
    fn draw_to_render_texture(
        &self,
        render_texture: &mut RenderTexture,
        states: Option<&RenderStates>,
    ) {
        let states = states.map_or_else(ptr::null, |state| {
            let cstate = state.to_csfml();
            &raw const cstate
        });

        unsafe {
            sfRenderTexture_drawSprite(render_texture.ptr, self.ptr, states);
        }
    }
}

impl RenderTextureDrawable for Text {
    fn draw_to_render_texture(
        &self,
        render_texture: &mut RenderTexture,
        states: Option<&RenderStates>,
    ) {
        let states = states.map_or_else(ptr::null, |state| {
            let cstate = state.to_csfml();
            &raw const cstate
        });

        unsafe {
            sfRenderTexture_drawText(render_texture.ptr, self.ptr, states);
        }
    }
}

impl RenderTextureDrawable for CircleShape {
    fn draw_to_render_texture(
        &self,
        render_texture: &mut RenderTexture,
        states: Option<&RenderStates>,
    ) {
        let states = states.map_or_else(ptr::null, |state| {
            let cstate = state.to_csfml();
            &raw const cstate
        });

        unsafe {
            sfRenderTexture_drawCircleShape(render_texture.ptr, self.mut_ptr(), states);
        }
    }
}

impl RenderTextureDrawable for ConvexShape {
    fn draw_to_render_texture(
        &self,
        render_texture: &mut RenderTexture,
        states: Option<&RenderStates>,
    ) {
        let states = states.map_or_else(ptr::null, |state| {
            let cstate = state.to_csfml();
            &raw const cstate
        });

        unsafe {
            sfRenderTexture_drawConvexShape(render_texture.ptr, self.mut_ptr(), states);
        }
    }
}

impl RenderTextureDrawable for RectangleShape {
    fn draw_to_render_texture(
        &self,
        render_texture: &mut RenderTexture,
        states: Option<&RenderStates>,
    ) {
        let states = states.map_or_else(ptr::null, |state| {
            let cstate = state.to_csfml();
            &raw const cstate
        });

        unsafe {
            sfRenderTexture_drawRectangleShape(render_texture.ptr, self.mut_ptr(), states);
        }
    }
}

impl RenderTextureDrawable for VertexArray {
    fn draw_to_render_texture(
        &self,
        render_texture: &mut RenderTexture,
        states: Option<&RenderStates>,
    ) {
        let states = states.map_or_else(ptr::null, |state| {
            let cstate = state.to_csfml();
            &raw const cstate
        });

        unsafe {
            sfRenderTexture_drawVertexArray(render_texture.ptr, self.ptr, states);
        }
    }
}

impl RenderTextureDrawable for VertexBuffer {
    fn draw_to_render_texture(
        &self,
        render_texture: &mut RenderTexture,
        states: Option<&RenderStates>,
    ) {
        let states = states.map_or_else(ptr::null, |state| {
            let cstate = state.to_csfml();
            &raw const cstate
        });

        unsafe {
            sfRenderTexture_drawVertexBuffer(render_texture.ptr, self.ptr, states);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        graphics::{color::Color, render_texture::RenderTexture, RectangleShape, Shape},
        system::{Vector2f, Vector2u},
    };

    #[test]
    fn render_texture_tests() {
        let mut rentex = RenderTexture::create(Vector2u { x: 10, y: 10 }).unwrap();

        rentex.set_repeated(true);
        rentex.set_smooth(true);

        rentex.clear(Color::RED);

        let mut rect = RectangleShape::create(Vector2f { x: 5.0, y: 5.0 }).unwrap();
        rect.set_fill_color(Color::BLUE);

        rentex.draw(&rect, None);

        rentex.display();

        rentex.generate_mipmap();

        assert!(rentex.is_repeated());
        assert!(rentex.is_smooth());

        let tex = rentex.texture();

        assert_eq!(tex.size(), Vector2u::new(10, 10));
        assert_eq!(rentex.size(), Vector2u::new(10, 10));

        let img = tex.copy_to_image();

        assert_eq!(img.get_pixel(Vector2u::new(1, 1)), Color::BLUE);
        assert_eq!(img.get_pixel(Vector2u::new(6, 3)), Color::RED);
    }
}
