use csfml_sys::sfContextSettings;
use std::ffi::c_uint;
use std::mem::{self};

/// Settings used for creating OpenGL contexts
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ContextSettings {
    depth_bits: c_uint,
    stencil_bits: c_uint,
    antialiasing_level: c_uint,
    major_version: c_uint,
    minor_version: c_uint,
    attribute_flags: u32,
    srgb_capable: bool,
    _padding: u8,
}

impl Default for ContextSettings {
    fn default() -> Self {
        Self {
            depth_bits: 0,
            stencil_bits: 0,
            antialiasing_level: 0,
            major_version: 1,
            minor_version: 1,
            attribute_flags: 0,
            srgb_capable: false,
            _padding: 0,
        }
    }
}

impl From<sfContextSettings> for ContextSettings {
    fn from(value: sfContextSettings) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<ContextSettings> for sfContextSettings {
    fn from(value: ContextSettings) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl ContextSettings {
    #[must_use]
    pub const fn attribute_default() -> u32 {
        0
    }

    #[must_use]
    pub const fn attribute_core() -> u32 {
        1
    }

    #[must_use]
    pub const fn attribute_debug() -> u32 {
        4
    }
}

#[cfg(test)]
mod tests {
    use csfml_sys::sfContextSettings;

    use super::ContextSettings;

    #[test]
    fn context_settings_default() {
        let settings = ContextSettings::default();
        assert_eq!(settings.major_version, 1);
        assert_eq!(settings.minor_version, 1);
        assert_eq!(settings.depth_bits, 0);
    }

    #[test]
    fn context_settings_to_and_from_csfml() {
        let settings = ContextSettings::default();
        let csfml = sfContextSettings::from(settings);
        let round_trip = ContextSettings::from(csfml);
        assert_eq!(settings.major_version, round_trip.major_version);
        assert_eq!(settings.minor_version, round_trip.minor_version);
    }
}
