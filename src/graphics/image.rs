use std::{ffi::CString, ptr, str::FromStr};

use csfml_sys::{
    sfImage, sfImage_createFromColor, sfImage_createFromFile, sfImage_createFromMemory,
    sfImage_createFromPixels, sfImage_createMaskFromColor, sfImage_destroy,
    sfImage_flipHorizontally, sfImage_flipVertically, sfImage_getPixel, sfImage_getPixelsPtr,
    sfImage_getSize, sfImage_saveToFile, sfImage_saveToMemory, sfImage_setPixel,
};
use derive_more::derive::Display;

use crate::system::{Buffer, Vector2u};

use super::color::Color;

#[derive(Debug)]
pub enum ImageError {
    NullPointer,
    NotEnoughData,
    PathIsNotAnImage,
    ResourceLoadingError,
    SavingInFileFailed,
}

pub struct Image {
    pub ptr: *mut sfImage,
}

impl Drop for Image {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Image {
    /// Creates a new image
    pub fn create(size: Vector2u, color: Color) -> Result<Self, ImageError> {
        unsafe {
            let img = sfImage_createFromColor(size.x, size.y, color.to_csfml());

            if img.is_null() {
                Err(ImageError::NullPointer)
            } else {
                Ok(Self { ptr: img })
            }
        }
    }

    /// Creates an image from a pixel array
    pub fn create_from_pixels(size: Vector2u, pixels: &[Color]) -> Result<Self, ImageError> {
        if pixels.len() < (size.x * size.y) as usize {
            return Err(ImageError::NotEnoughData);
        }

        unsafe {
            let img = sfImage_createFromPixels(size.x, size.y, pixels.as_ptr().cast::<u8>());
            if img.is_null() {
                Err(ImageError::NullPointer)
            } else {
                Ok(Self { ptr: img })
            }
        }
    }

    /// Loads an image from a file
    pub fn create_from_file(path: &str) -> Result<Self, ImageError> {
        let c_path = std::ffi::CString::new(path).map_err(|_| ImageError::PathIsNotAnImage)?;
        unsafe {
            let img = sfImage_createFromFile(c_path.as_ptr());
            if img.is_null() {
                Err(ImageError::ResourceLoadingError)
            } else {
                Ok(Self { ptr: img })
            }
        }
    }

    /// Loads an image from a file in memory
    pub fn create_from_memory(data: &[u8]) -> Result<Self, ImageError> {
        unsafe {
            let img =
                sfImage_createFromMemory(data.as_ptr().cast::<std::ffi::c_void>(), data.len());
            if img.is_null() {
                Err(ImageError::ResourceLoadingError)
            } else {
                Ok(Self { ptr: img })
            }
        }
    }

    /// Save the image to a buffer in memory
    pub fn save_to_memory(
        &self,
        buffer: &mut Buffer,
        format: FileFormat,
    ) -> Result<(), ImageError> {
        let format =
            CString::from_str(&format.to_string()).map_err(|_| ImageError::PathIsNotAnImage)?;
        unsafe {
            if sfImage_saveToMemory(self.ptr, buffer.ptr, format.as_ptr()) == 0 {
                Err(ImageError::SavingInFileFailed)
            } else {
                Ok(())
            }
        }
    }

    /// Destroys an image
    pub fn destroy(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                sfImage_destroy(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }

    /// Save the image to a file
    pub fn save_to_file(&self, path: &str) -> Result<(), ImageError> {
        let c_path = std::ffi::CString::new(path).map_err(|_| ImageError::PathIsNotAnImage)?;
        unsafe {
            if sfImage_saveToFile(self.ptr, c_path.as_ptr()) == 1 {
                Ok(())
            } else {
                Err(ImageError::SavingInFileFailed)
            }
        }
    }

    /// Gets a pixel from this image (bounds are only checked in an assertion)
    #[must_use]
    pub fn get_pixel(&self, pixel_pos: Vector2u) -> Color {
        let size = self.get_size();
        assert!(pixel_pos.x < size.x && pixel_pos.y < size.y);

        unsafe { Color::from_csfml(sfImage_getPixel(self.ptr, pixel_pos.x, pixel_pos.y)) }
    }

    /// Sets a pixel on this image (bounds are only checked in an assertion)
    pub fn set_pixel(&mut self, pixel_pos: Vector2u, color: Color) {
        let size = self.get_size();
        assert!(pixel_pos.x < size.x && pixel_pos.y < size.y);

        unsafe {
            sfImage_setPixel(self.ptr, pixel_pos.x, pixel_pos.y, color.to_csfml());
        }
    }

    /// Gets the size of this image
    #[must_use]
    pub fn get_size(&self) -> Vector2u {
        unsafe {
            let size = sfImage_getSize(self.ptr);
            Vector2u {
                x: size.x,
                y: size.y,
            }
        }
    }

    /// Changes the pixels of the image matching color to be transparent
    pub fn create_mask_from_color(&mut self, color: Color, alpha: u8) {
        unsafe {
            sfImage_createMaskFromColor(self.ptr, color.to_csfml(), alpha);
        }
    }

    /// Flip an image horizontally (left <-> right)
    pub fn flip_horizontally(&mut self) {
        unsafe {
            sfImage_flipHorizontally(self.ptr);
        }
    }

    /// Flip an image vertically (top <-> bottom)
    pub fn flip_vertically(&mut self) {
        unsafe {
            sfImage_flipVertically(self.ptr);
        }
    }

    /// Get a read-only pointer to the array of pixels of the image
    #[must_use]
    pub fn get_pixels_slice(&self) -> &[Color] {
        unsafe {
            let ptr = sfImage_getPixelsPtr(self.ptr);
            let size = self.get_size();
            let len = (size.x * size.y) as usize;

            std::slice::from_raw_parts(ptr.cast::<Color>(), len)
        }
    }
}

/// `FileFormat` Enum
#[derive(Debug, Display, Clone, Copy)]
pub enum FileFormat {
    #[display("bmp")]
    Bmp = 0,
    #[display("png")]
    Png = 1,
    #[display("tga")]
    Tga = 2,
    #[display("jpg")]
    Jpg = 3,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn image_sane_getters_and_setters() -> Result<(), ImageError> {
        let pixel_data: Vec<Color> = (0..30)
            .map(|i| Color::from_hsva(i as f32 * 360.0 / 30.0, 100.0, 100.0, 1.0))
            .collect();
        let size = Vector2u { x: 5, y: 6 };

        let mut img = Image::create_from_pixels(size, &pixel_data)?;

        assert_eq!(img.get_size(), size);

        img.set_pixel(Vector2u { x: 1, y: 2 }, Color::CYAN);
        assert_eq!(img.get_pixel(Vector2u { x: 1, y: 2 }), Color::CYAN);

        img.set_pixel(Vector2u { x: 1, y: 2 }, Color::RED);

        let slice = img.get_pixels_slice();
        assert_eq!(slice[0], Color::RED);

        Ok(())
    }

    #[test]
    fn image_save_to_memory() -> Result<(), ImageError> {
        let img = Image::create(Vector2u { x: 50, y: 50 }, Color::CYAN)?;

        for format in &[
            FileFormat::Bmp,
            FileFormat::Png,
            FileFormat::Tga,
            FileFormat::Jpg,
        ] {
            let mut buf = Buffer::new();
            img.save_to_memory(&mut buf, *format)?;
            assert!(buf.size() != 0);
        }

        Ok(())
    }
}
