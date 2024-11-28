use std::{ffi::c_char, ptr};

use csfml_sys::{
    sfGlslVec2, sfGlslVec3, sfShader, sfShader_createFromFile, sfShader_createFromMemory,
    sfShader_destroy, sfShader_isAvailable, sfShader_setBoolUniform, sfShader_setFloatUniform,
    sfShader_setIntUniform, sfShader_setVec2Uniform, sfShader_setVec3Uniform,
};

use crate::types::Result;

use super::glsl;

#[derive(Debug)]
pub enum ShaderError {
    NullPointerUnknownReason,
    InvalidUniformType,
}

#[derive(Debug, Clone)]
pub struct Shader {
    ptr: *mut sfShader,
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Shader {
    pub fn create_from_file(
        vertex_shader_path: Option<&str>,
        geometry_shader_path: Option<&str>,
        fragment_shader_path: Option<&str>,
    ) -> Result<Self> {
        let vertex_shader_ptr =
            vertex_shader_path.map_or(ptr::null(), |s| s.as_ptr().cast::<c_char>());
        let geometry_shader_ptr =
            geometry_shader_path.map_or(ptr::null(), |s| s.as_ptr().cast::<c_char>());
        let fragment_shader_ptr =
            fragment_shader_path.map_or(ptr::null(), |s| s.as_ptr().cast::<c_char>());

        unsafe {
            let shader_ptr = sfShader_createFromFile(
                vertex_shader_ptr,
                geometry_shader_ptr,
                fragment_shader_ptr,
            );

            if shader_ptr.is_null() {
                Err("Null Pointer".into())
            } else {
                Ok(Self { ptr: shader_ptr })
            }
        }
    }

    pub fn create_from_memory(
        vertex_shader: Option<&str>,
        geometry_shader: Option<&str>,
        fragment_shader: Option<&str>,
    ) -> Result<Self> {
        let vertex_shader_ptr = vertex_shader.map_or(ptr::null(), |s| s.as_ptr().cast::<c_char>());
        let geometry_shader_ptr =
            geometry_shader.map_or(ptr::null(), |s| s.as_ptr().cast::<c_char>());
        let fragment_shader_ptr =
            fragment_shader.map_or(ptr::null(), |s| s.as_ptr().cast::<c_char>());

        unsafe {
            let shader_ptr = sfShader_createFromMemory(
                vertex_shader_ptr,
                geometry_shader_ptr,
                fragment_shader_ptr,
            );

            if shader_ptr.is_null() {
                Err("Null pointer".into())
            } else {
                Ok(Self { ptr: shader_ptr })
            }
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            sfShader_destroy(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }

    #[must_use]
    pub fn is_available() -> bool {
        unsafe { sfShader_isAvailable() != 0 }
    }

    #[must_use]
    pub fn is_geometry_available() -> bool {
        unsafe { sfShader_isAvailable() != 0 }
    }

    pub fn set_uniform<T>(&self, name: &str, value: &T)
    where
        T: UniformValue,
    {
        unsafe { value.set_uniform(self.ptr, name) };
    }

    #[must_use]
    pub const fn ptr(&self) -> *mut sfShader {
        self.ptr
    }
}

pub trait UniformValue {
    unsafe fn set_uniform(&self, shader_ptr: *mut sfShader, name: &str);
}

impl UniformValue for f32 {
    unsafe fn set_uniform(&self, shader_ptr: *mut sfShader, name: &str) {
        unsafe {
            sfShader_setFloatUniform(shader_ptr, name.as_ptr().cast::<c_char>(), *self);
        }
    }
}

impl UniformValue for i32 {
    unsafe fn set_uniform(&self, shader_ptr: *mut sfShader, name: &str) {
        unsafe {
            sfShader_setIntUniform(shader_ptr, name.as_ptr().cast::<c_char>(), *self);
        }
    }
}

impl UniformValue for bool {
    unsafe fn set_uniform(&self, shader_ptr: *mut sfShader, name: &str) {
        unsafe {
            sfShader_setBoolUniform(shader_ptr, name.as_ptr().cast::<c_char>(), i32::from(*self));
        }
    }
}

// Add more implementations for other types like Vec2, Vec3, etc.
impl UniformValue for glsl::FVec2 {
    unsafe fn set_uniform(&self, shader_ptr: *mut sfShader, name: &str) {
        unsafe {
            sfShader_setVec2Uniform(
                shader_ptr,
                name.as_ptr().cast::<c_char>(),
                sfGlslVec2::from(*self),
            );
        }
    }
}

impl UniformValue for glsl::FVec3 {
    unsafe fn set_uniform(&self, shader_ptr: *mut sfShader, name: &str) {
        unsafe {
            sfShader_setVec3Uniform(
                shader_ptr,
                name.as_ptr().cast::<c_char>(),
                sfGlslVec3::from(*self),
            );
        }
    }
}
