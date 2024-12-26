//! Give access to the real-time state of the mouse.

use std::ptr;

use csfml_sys::{
    sfMouseButton, sfMouse_getPosition, sfMouse_isButtonPressed, sfMouse_setPosition, sfVector2i,
    sfWindow,
};

use crate::system::Vector2i;

/// Mouse buttons
#[repr(u32)]
pub enum Button {
    Left = 0,
    Right,
    Middle,
    XButton1,
    XButton2,
}

/// Mouse wheels
#[repr(u32)]
pub enum Wheel {
    Vertical = 0,
    Horizontal,
}

impl Button {
    /// Returns true if the specified mouse button is pressed
    #[must_use]
    pub fn is_pressed(self) -> bool {
        unsafe { sfMouse_isButtonPressed(self as sfMouseButton) == 1 }
    }
}

/// Get the position of the mouse cursor relative to the window passed or desktop
#[must_use]
pub fn get_position(window: Option<*const sfWindow>) -> Vector2i {
    window.map_or_else(
        || {
            let vec = unsafe { sfMouse_getPosition(ptr::null_mut()) };
            Vector2i::from(vec)
        },
        |w| {
            let vec = unsafe { sfMouse_getPosition(w) };
            Vector2i::from(vec)
        },
    )
}

/// Set the position of the mouse cursor relative to the window passed or desktop
pub fn set_position(position: Vector2i, window: Option<*const sfWindow>) {
    if let Some(w) = window {
        unsafe { sfMouse_setPosition(sfVector2i::from(position), w) };
    } else {
        unsafe { sfMouse_setPosition(sfVector2i::from(position), ptr::null()) };
    }
}
