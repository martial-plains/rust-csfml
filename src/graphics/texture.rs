use std::ffi::{c_void, CString};
use std::ptr;

use csfml_sys::{sfBool, sfTexture};

use crate::{system::Vector2u, types::Result};

use super::color::Color;
use super::image::Image;
use super::rect::{IntRect, Rect};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Texture {
    Mutable(*mut sfTexture),
    Const(*const sfTexture),
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Texture {
    /// Creates a texture from nothing
    pub fn create(size: Vector2u) -> Result<Self> {
        let tex = unsafe { csfml_sys::sfTexture_create(size.x, size.y) };
        if tex.is_null() {
            Err("resourceLoadingError".into())
        } else {
            Ok(Self::Mutable(tex))
        }
    }

    /// Loads a texture from a file
    pub fn create_from_file(path: &str) -> Result<Self> {
        let c_path = CString::new(path).unwrap();
        let tex = unsafe { csfml_sys::sfTexture_createFromFile(c_path.as_ptr(), ptr::null()) };
        if tex.is_null() {
            Err("resourceLoadingError".into())
        } else {
            Ok(Self::Mutable(tex))
        }
    }

    /// Loads a texture from a file in memory
    pub fn create_from_memory(data: &[u8], area: IntRect) -> Result<Self> {
        let tex: *mut sfTexture = unsafe {
            csfml_sys::sfTexture_createFromMemory(
                data.as_ptr().cast::<c_void>(),
                data.len(),
                &area.to_csfml(),
            )
        };
        if tex.is_null() {
            Err("resourceLoadingError".into())
        } else {
            Ok(Self::Mutable(tex))
        }
    }

    /// Creates a texture from an image
    pub fn create_from_image(image: &Image, area: Option<IntRect>) -> Result<Self> {
        let tex = unsafe {
            area.map_or_else(
                || csfml_sys::sfTexture_createFromImage(image.ptr, ptr::null()),
                |a| csfml_sys::sfTexture_createFromImage(image.ptr, &a.to_csfml()),
            )
        };
        if tex.is_null() {
            Err("nullptrUnknownReason".into())
        } else {
            Ok(Self::Mutable(tex))
        }
    }

    /// Destroys a texture
    pub fn destroy(&mut self) {
        if let Self::Mutable(tex) = self {
            if !tex.is_null() {
                unsafe {
                    csfml_sys::sfTexture_destroy(*tex);
                }
                *tex = ptr::null_mut();
            }
        }
    }

    #[must_use]
    pub const fn ptr(&self) -> *const sfTexture {
        match *self {
            Self::Mutable(ptr) => ptr.cast_const(),
            Self::Const(ptr) => ptr,
        }
    }

    #[must_use]
    pub const fn mut_ptr(&self) -> *mut sfTexture {
        match *self {
            Self::Mutable(ptr) => ptr,
            Self::Const(ptr) => ptr.cast_mut(),
        }
    }

    /// Clones the texture (the clone won't be const)
    pub fn copy(&self) -> Result<Self> {
        match *self {
            Self::Mutable(ptr) => {
                let cpy = unsafe { csfml_sys::sfTexture_copy(ptr) };
                if cpy.is_null() {
                    Err("nullptrUnknownReason".into())
                } else {
                    Ok(Self::Mutable(cpy))
                }
            }

            Self::Const(ptr) => {
                let cpy = unsafe { csfml_sys::sfTexture_copy(ptr.cast_mut()) };
                if cpy.is_null() {
                    Err("nullptrUnknownReason".into())
                } else {
                    Ok(Self::Mutable(cpy))
                }
            }
        }
    }

    #[must_use]
    pub fn copy_to_image(&self) -> Image {
        Image {
            ptr: unsafe { csfml_sys::sfTexture_copyToImage(self.ptr()) },
        }
    }

    #[must_use]
    pub fn size(&self) -> Vector2u {
        let size = unsafe { csfml_sys::sfTexture_getSize(self.ptr()) };
        Vector2u::new(size.x, size.y)
    }

    #[must_use]
    pub fn pixel_count(&self) -> usize {
        let size = self.size();
        (size.x * size.y) as usize
    }

    /// Updates the texture from pixel data
    pub fn update_from_pixels(&mut self, pixels: &[Color], zone: Option<IntRect>) -> Result<()> {
        match *self {
            Self::Const(_) => Err("Can't set pixels on a const texture".into()),
            Self::Mutable(tex) => {
                let size = self.size();
                let real_zone =
                    zone.unwrap_or_else(|| Rect::new(0, 0, size.x as i32, size.y as i32));

                if pixels.len() < (real_zone.width * real_zone.height) as usize {
                    return Err("Not enough data".into());
                }

                unsafe {
                    csfml_sys::sfTexture_updateFromPixels(
                        tex,
                        pixels.as_ptr().cast::<u8>(),
                        real_zone.width as u32,
                        real_zone.height as u32,
                        real_zone.left as u32,
                        real_zone.top as u32,
                    );
                }
                Ok(())
            }
        }
    }

    #[must_use]
    pub fn is_smooth(&self) -> bool {
        unsafe { csfml_sys::sfTexture_isSmooth(self.ptr()) != 0 }
    }

    pub fn set_smooth(&mut self, smooth: bool) {
        match self {
            Self::Const(_) => panic!("Can't set properties on a const texture"),
            Self::Mutable(tex) => unsafe {
                csfml_sys::sfTexture_setSmooth(*tex, smooth as sfBool);
            },
        }
    }

    #[must_use]
    pub fn is_repeated(&self) -> bool {
        unsafe { csfml_sys::sfTexture_isRepeated(self.ptr()) != 0 }
    }

    pub fn set_repeated(&mut self, repeated: bool) {
        match self {
            Self::Const(_) => panic!("Can't set properties on a const texture"),
            Self::Mutable(tex) => unsafe {
                csfml_sys::sfTexture_setRepeated(*tex, repeated as sfBool);
            },
        }
    }

    #[must_use]
    pub fn is_srgb(&self) -> bool {
        unsafe { csfml_sys::sfTexture_isSrgb(self.ptr()) != 0 }
    }

    pub fn swap(&mut self, other: &mut Self) {
        match (self, other) {
            (Self::Mutable(tex1), Self::Mutable(tex2)) => unsafe {
                csfml_sys::sfTexture_swap(*tex1, *tex2);
            },
            _ => panic!("Texture swapping must be done between two non-const textures"),
        }
    }

    pub fn generate_mipmap(&mut self) -> bool {
        match self {
            Self::Const(_) => panic!("Can't act on a const texture"),
            Self::Mutable(tex) => unsafe { csfml_sys::sfTexture_generateMipmap(*tex) != 0 },
        }
    }

    #[must_use]
    pub const fn is_const(&self) -> bool {
        matches!(self, Self::Const(_))
    }
}

// Test Functionality
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_sane_getters_and_setters() {
        let mut tex = Texture::create(Vector2u::new(12, 10)).unwrap();
        let size = tex.size();

        tex.set_smooth(true);
        tex.set_repeated(true);

        assert_eq!(size.x, 12);
        assert_eq!(size.y, 10);
        assert_eq!(tex.pixel_count(), 120);

        let mut pixel_data: Vec<Color> = vec![Color::BLACK; 120];
        pixel_data[0] = Color::GREEN;

        tex.update_from_pixels(&pixel_data, None).unwrap();

        assert!(!tex.is_srgb());
        assert!(tex.is_smooth());
        assert!(tex.is_repeated());

        let img = tex.copy_to_image();
        assert_eq!(img.get_pixel(Vector2u::new(0, 0)), Color::GREEN);

        let mut tex2 = Texture::create(Vector2u::new(100, 100)).unwrap();
        tex.swap(&mut tex2);
        assert_eq!(tex2.pixel_count(), 120);
        assert_eq!(tex.pixel_count(), 100 * 100);
    }
}
