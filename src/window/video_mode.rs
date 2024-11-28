use core::slice;
use std::ffi::c_uint;

use csfml_sys::{
    sfVideoMode, sfVideoMode_getDesktopMode, sfVideoMode_getFullscreenModes, sfVideoMode_isValid,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VideoMode {
    pub width: c_uint,
    pub height: c_uint,
    pub bits_per_pixel: c_uint,
}

impl VideoMode {
    #[must_use]
    pub const fn from_csfml(
        sfVideoMode {
            width,
            height,
            bitsPerPixel,
        }: sfVideoMode,
    ) -> Self {
        Self {
            width,
            height,
            bits_per_pixel: bitsPerPixel,
        }
    }

    #[must_use]
    pub const fn to_csfml(self) -> sfVideoMode {
        sfVideoMode {
            width: self.width,
            height: self.height,
            bitsPerPixel: self.bits_per_pixel,
        }
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        unsafe { sfVideoMode_isValid(self.to_csfml()) != 0 }
    }

    #[must_use]
    pub fn desktop_mode() -> Self {
        unsafe { Self::from_csfml(sfVideoMode_getDesktopMode()) }
    }

    #[must_use]
    pub fn fullscreen_modes() -> &'static [Self] {
        let mut count = 0;
        unsafe {
            slice::from_raw_parts(sfVideoMode_getFullscreenModes(&raw mut count).cast(), count)
        }
    }
}

impl From<sfVideoMode> for VideoMode {
    fn from(value: sfVideoMode) -> Self {
        Self::from_csfml(value)
    }
}

impl From<VideoMode> for sfVideoMode {
    fn from(value: VideoMode) -> Self {
        value.to_csfml()
    }
}
