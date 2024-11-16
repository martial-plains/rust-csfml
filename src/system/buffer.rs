use std::{
    ptr::{self},
    slice,
};

use derive_more::derive::{AsMut, AsRef, Deref, DerefMut};
use sfml_sys::{sfBuffer, sfBuffer_create, sfBuffer_destroy, sfBuffer_getData, sfBuffer_getSize};

#[derive(Debug, Clone, Deref, DerefMut, AsRef, AsMut)]
pub struct Buffer {
    __ptr: *mut sfBuffer,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            __ptr: unsafe { sfBuffer_create() },
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { sfBuffer_destroy(self.__ptr) };
        self.__ptr = ptr::null_mut();
    }
}

impl Buffer {
    #[must_use]
    pub fn size(&self) -> usize {
        unsafe { sfBuffer_getSize(self.__ptr) }
    }

    #[must_use]
    pub fn data(&self) -> Option<&'static [u8]> {
        let ptr = unsafe { sfBuffer_getData(self.__ptr) };

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
