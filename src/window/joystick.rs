//! Give access to the real-time state of the joysticks.

use csfml_sys::{
    sfJoystickAxis, sfJoystick_getAxisPosition, sfJoystick_getButtonCount, sfJoystick_hasAxis,
    sfJoystick_isButtonPressed, sfJoystick_isConnected,
};

/// Constants related to joysticks capabilities
pub const MAX_JOYSTICK_COUNT: u32 = 8;
pub const MAX_BUTTON_COUNT: u32 = 32;
pub const MAX_AXIS_COUNT: u32 = 8;

/// Joystick axis
#[repr(u32)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
    R = 3,
    U = 4,
    V = 5,
    PovX = 6,
    PovY = 7,
}

/// Joystick structure holding a joystick number
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Joystick {
    joystick_number: u32,
}

impl Joystick {
    /// Gets a joystick if it is connected, `None` if it is not.
    /// You can also construct the Joystick struct manually with a joystick number
    /// if you want to check its state even when it is not connected.
    #[must_use]
    pub fn get(joystick: u32) -> Option<Self> {
        assert!(joystick < MAX_JOYSTICK_COUNT);

        if unsafe { sfJoystick_isConnected(joystick) } != 0 {
            Some(Self {
                joystick_number: joystick,
            })
        } else {
            None
        }
    }

    /// Check if this joystick is still connected
    #[must_use]
    pub fn is_connected(&self) -> bool {
        unsafe { sfJoystick_isConnected(self.joystick_number) != 0 }
    }

    /// Gets the button count of this joystick
    #[must_use]
    pub fn get_button_count(&self) -> usize {
        unsafe { sfJoystick_getButtonCount(self.joystick_number) as usize }
    }

    /// Checks if the joystick has a given axis
    #[must_use]
    pub fn has_axis(&self, axis: Axis) -> bool {
        unsafe { sfJoystick_hasAxis(self.joystick_number, axis as sfJoystickAxis) != 0 }
    }

    /// Gets the value of an axis
    #[must_use]
    pub fn get_axis_position(&self, axis: Axis) -> f32 {
        unsafe { sfJoystick_getAxisPosition(self.joystick_number, axis as sfJoystickAxis) }
    }

    /// Checks if a joystick button is pressed
    #[must_use]
    pub fn is_button_pressed(&self, button: u32) -> bool {
        assert!(button < MAX_BUTTON_COUNT);
        unsafe { sfJoystick_isButtonPressed(self.joystick_number, button) != 0 }
    }
}
