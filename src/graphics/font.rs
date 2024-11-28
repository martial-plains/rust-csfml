use std::ffi::{CStr, CString};
use std::ptr::null_mut;

use csfml_sys::{
    sfBool, sfFont, sfFont_createFromFile, sfFont_createFromMemory, sfFont_destroy, sfFont_getInfo,
    sfFont_getKerning, sfFont_getLineSpacing, sfFont_getUnderlinePosition,
    sfFont_getUnderlineThickness, sfFont_hasGlyph, sfFont_isSmooth, sfFont_setSmooth,
};

#[derive(Debug, Clone)]
pub struct Font {
    pub(crate) ptr: *mut sfFont,
}

impl Drop for Font {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Font {
    /// Loads a font from a file
    pub fn create_from_file(path: &str) -> Result<Self, String> {
        unsafe {
            let c_path = CString::new(path).unwrap();
            let font = sfFont_createFromFile(c_path.as_ptr());
            if font.is_null() {
                Err("Resource loading error".to_string())
            } else {
                Ok(Self { ptr: font })
            }
        }
    }

    /// Loads a font from memory
    pub fn create_from_memory(data: &[u8]) -> Result<Self, String> {
        unsafe {
            let font = sfFont_createFromMemory(data.as_ptr().cast(), data.len());
            if font.is_null() {
                Err("Resource loading error".to_string())
            } else {
                Ok(Self { ptr: font })
            }
        }
    }

    /// Destroys a font
    pub fn destroy(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                sfFont_destroy(self.ptr);
                self.ptr = null_mut();
            }
        }
    }

    /// Gets the family name of the font
    #[must_use]
    pub fn get_family(&self) -> String {
        unsafe {
            let info = sfFont_getInfo(self.ptr);
            let family_cstr = CStr::from_ptr(info.family);
            family_cstr.to_string_lossy().into_owned()
        }
    }

    /// Gets the kerning offset of two glyphs
    #[must_use]
    pub fn get_kerning(&self, first: u32, second: u32, character_size: usize) -> f32 {
        unsafe { sfFont_getKerning(self.ptr, first, second, character_size as u32) }
    }

    /// Gets the default spacing between two lines
    #[must_use]
    pub fn get_line_spacing(&self, character_size: usize) -> f32 {
        unsafe { sfFont_getLineSpacing(self.ptr, character_size as u32) }
    }

    /// Gets the vertical offset of the underline
    #[must_use]
    pub fn get_underline_position(&self, character_size: usize) -> f32 {
        unsafe { sfFont_getUnderlinePosition(self.ptr, character_size as u32) }
    }

    /// Gets the underline thickness
    #[must_use]
    pub fn get_underline_thickness(&self, character_size: usize) -> f32 {
        unsafe { sfFont_getUnderlineThickness(self.ptr, character_size as u32) }
    }

    /// Checks if the font has the given glyph
    #[must_use]
    pub fn has_glyph(&self, codepoint: u32) -> bool {
        unsafe { sfFont_hasGlyph(self.ptr, codepoint) != 0 }
    }

    /// Enable or disable the smooth filter
    pub fn set_smooth(&mut self, smooth: bool) {
        unsafe {
            sfFont_setSmooth(self.ptr, sfBool::from(smooth));
        }
    }

    /// Checks if the smooth filter is enabled
    #[must_use]
    pub fn is_smooth(&self) -> bool {
        unsafe { sfFont_isSmooth(self.ptr) != 0 }
    }
}
