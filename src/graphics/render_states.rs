use std::ptr;

use csfml_sys::sfRenderStates;

use super::{
    blend_mode::{BlendMode, BLEND_ALPHA},
    texture::Texture,
    transform::Transform,
    Shader,
};

#[derive(Debug, Clone)]
pub struct RenderStates {
    pub blend_mode: BlendMode,
    pub transform: Transform,
    pub texture: Option<Texture>,
    pub shader: Option<Shader>,
}

impl Default for RenderStates {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderStates {
    /// Creates a new `RenderStates` instance with default values.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            blend_mode: BLEND_ALPHA,
            transform: Transform::IDENTITY,
            texture: None,
            shader: None,
        }
    }

    /// Converts `RenderStates` to a C-compatible sfRenderStates struct.
    #[must_use]
    pub fn to_csfml(&self) -> sfRenderStates {
        sfRenderStates {
            blendMode: self.blend_mode.to_csfml(),
            transform: self.transform.to_csfml(),
            texture: self.texture.clone().map_or(ptr::null(), |t| t.ptr()),
            shader: self.shader.clone().map_or(ptr::null(), |t| t.ptr()),
        }
    }
}
