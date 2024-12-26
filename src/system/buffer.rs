use std::{
    ptr::{self},
    slice,
};

use csfml_sys::{sfBuffer, sfBuffer_create, sfBuffer_destroy, sfBuffer_getData, sfBuffer_getSize};
use derive_more::derive::{AsMut, AsRef, Deref, DerefMut};

#[derive(Debug, Clone, Deref, DerefMut, AsRef, AsMut)]
pub struct Buffer {
    pub ptr: *mut sfBuffer,
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Buffer {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ptr: unsafe { sfBuffer_create() },
        }
    }

    pub fn destroy(&mut self) {
        unsafe { sfBuffer_destroy(self.ptr) };
        self.ptr = ptr::null_mut();
    }

    #[must_use]
    pub fn size(&self) -> usize {
        unsafe { sfBuffer_getSize(self.ptr) }
    }

    #[must_use]
    pub fn data(&self) -> Option<&'static [u8]> {
        let ptr = unsafe { sfBuffer_getData(self.ptr) };

        if ptr.is_null() {
            None
        } else {
            let size = self.size();

            unsafe { Some(slice::from_raw_parts(ptr, size)) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Buffer;

    #[test]
    fn functions_compile() {
        let buf = Buffer::default();
        assert_eq!(0, buf.size());
    }
}
