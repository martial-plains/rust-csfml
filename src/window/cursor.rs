//! Cursor defines the appearance of a system cursor.

use csfml_sys::{
    sfCursor, sfCursorType, sfCursor_createFromPixels, sfCursor_createFromSystem, sfCursor_destroy,
    sfVector2u,
};
use std::{ffi::c_uint, ptr::NonNull};

use crate::{system::Vector2u, types::Result};

/// Cursor defines the appearance of a system cursor.
#[derive(Debug)]
pub struct Cursor {
    ptr: Option<NonNull<sfCursor>>,
}

/// Enumeration of the native system cursor types.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Type {
    /// Arrow cursor (default)
    Arrow = 0,
    /// Busy arrow cursor
    ArrowWait = 1,
    /// Busy cursor
    Wait = 2,
    /// I-beam, cursor when hovering over a field allowing text entry
    Text = 3,
    /// Pointing hand cursor
    Hand = 4,
    /// Horizontal double arrow cursor
    SizeHorizontal = 5,
    /// Vertical double arrow cursor
    SizeVertical = 6,
    /// Double arrow cursor going from top-left to bottom-right
    SizeTopLeftBottomRight = 7,
    /// Double arrow cursor going from bottom-left to top-right
    SizeBottomLeftTopRight = 8,
    /// Left arrow cursor on Linux, same as sizeHorizontal on other platforms
    SizeLeft = 9,
    /// Right arrow cursor on Linux, same as sizeHorizontal on other platforms
    SizeRight = 10,
    /// Up arrow cursor on Linux, same as sizeVertical on other platforms
    SizeTop = 11,
    /// Down arrow cursor on Linux, same as sizeVertical on other platforms
    SizeBottom = 12,
    /// Top-left arrow cursor on Linux, same as sizeTopLeftBottomRight on other platforms
    SizeTopLeft = 13,
    /// Bottom-right arrow cursor on Linux, same as sizeTopLeftBottomRight on other platforms
    SizeBottomRight = 14,
    /// Bottom-left arrow cursor on Linux, same as sizeBottomLeftTopRight on other platforms
    SizeBottomLeft = 15,
    /// Top-right arrow cursor on Linux, same as sizeBottomLeftTopRight on other platforms
    SizeTopRight = 16,
    /// Combination of sizeHorizontal and sizeVertical
    SizeAll = 17,
    /// Crosshair cursor
    Cross = 18,
    /// Help cursor
    Help = 19,
    /// Action not allowed cursor
    NotAllowed = 20,
}

impl Cursor {
    /// Create a cursor with the provided image.
    pub fn create_from_pixels(pixels: &[u8], size: Vector2u, hotspot: Vector2u) -> Result<Self> {
        if pixels.len()
            != (size.x
                * size.y
                * u32::try_from(std::mem::size_of::<u32>()).map_err(|e| e.to_string())?)
                as usize
        {
            return Err("Wrong data size".to_string().into());
        }

        let cursor = unsafe {
            sfCursor_createFromPixels(
                pixels.as_ptr().cast(),
                sfVector2u::from(size),
                sfVector2u::from(hotspot),
            )
        };

        if cursor.is_null() {
            return Err("Failed to create cursor: nullptr returned"
                .to_string()
                .into());
        }

        Ok(Self {
            ptr: Some(unsafe { NonNull::new_unchecked(cursor) }),
        })
    }

    /// Create a native system cursor.
    pub fn create_from_system(cursor_type: Type) -> Result<Self> {
        let cursor = unsafe { sfCursor_createFromSystem(cursor_type as sfCursorType) };

        if cursor.is_null() {
            return Err("Failed to create system cursor: nullptr returned"
                .to_string()
                .into());
        }

        Ok(Self {
            ptr: Some(unsafe { NonNull::new_unchecked(cursor) }),
        })
    }

    /// Destroy the cursor.
    pub fn destroy(&mut self) {
        if let Some(ptr) = self.ptr.take() {
            unsafe { sfCursor_destroy(ptr.as_ptr()) };
        }
    }
}

impl Drop for Cursor {
    fn drop(&mut self) {
        self.destroy();
    }
}
