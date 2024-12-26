//! Defines a system event and its parameters.

use csfml_sys::{
    sfEvent, sfEventType, sfEvtClosed, sfEvtCount, sfEvtGainedFocus, sfEvtJoystickButtonPressed,
    sfEvtJoystickButtonReleased, sfEvtJoystickConnected, sfEvtJoystickDisconnected,
    sfEvtJoystickMoved, sfEvtKeyPressed, sfEvtKeyReleased, sfEvtLostFocus, sfEvtMouseButtonPressed,
    sfEvtMouseButtonReleased, sfEvtMouseEntered, sfEvtMouseLeft, sfEvtMouseMoved,
    sfEvtMouseWheelMoved, sfEvtMouseWheelScrolled, sfEvtResized, sfEvtSensorChanged,
    sfEvtTextEntered, sfEvtTouchBegan, sfEvtTouchEnded, sfEvtTouchMoved, sfJoystickAxis, sfKeyCode,
    sfMouseButton, sfMouseWheel, sfSensorType,
};

use std::error::Error;
use std::fmt::Formatter;
use std::fmt::{self, Debug};

use crate::{
    system::{Vector2i, Vector2u, Vector3f},
    types::Result,
};

// Event types enumeration (same as Zig enum)
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EventType {
    Closed = sfEvtClosed,
    Resized = sfEvtResized,
    LostFocus = sfEvtLostFocus,
    GainedFocus = sfEvtGainedFocus,
    TextEntered = sfEvtTextEntered,
    KeyPressed = sfEvtKeyPressed,
    KeyReleased = sfEvtKeyReleased,
    MouseWheelMoved = sfEvtMouseWheelMoved,
    MouseWheelScrolled = sfEvtMouseWheelScrolled,
    MouseButtonPressed = sfEvtMouseButtonPressed,
    MouseButtonReleased = sfEvtMouseButtonReleased,
    MouseMoved = sfEvtMouseMoved,
    MouseEntered = sfEvtMouseEntered,
    MouseLeft = sfEvtMouseLeft,
    JoystickButtonPressed = sfEvtJoystickButtonPressed,
    JoystickButtonReleased = sfEvtJoystickButtonReleased,
    JoystickMoved = sfEvtJoystickMoved,
    JoystickConnected = sfEvtJoystickConnected,
    JoystickDisconnected = sfEvtJoystickDisconnected,
    TouchBegan = sfEvtTouchBegan,
    TouchMoved = sfEvtTouchMoved,
    TouchEnded = sfEvtTouchEnded,
    SensorChanged = sfEvtSensorChanged,
}

// The Event struct and all its possible types
#[derive(Debug)]
pub enum Event {
    Closed,
    Resized(SizeEvent),
    LostFocus,
    GainedFocus,
    TextEntered(TextEvent),
    KeyPressed(KeyEvent),
    KeyReleased(KeyEvent),
    MouseWheelMoved(MouseWheelEvent),
    MouseWheelScrolled(MouseWheelScrollEvent),
    MouseButtonPressed(MouseButtonEvent),
    MouseButtonReleased(MouseButtonEvent),
    MouseMoved(MouseMoveEvent),
    MouseEntered,
    MouseLeft,
    JoystickButtonPressed(JoystickButtonEvent),
    JoystickButtonReleased(JoystickButtonEvent),
    JoystickMoved(JoystickMoveEvent),
    JoystickConnected(JoystickConnectEvent),
    JoystickDisconnected(JoystickConnectEvent),
    TouchBegan(TouchEvent),
    TouchMoved(TouchEvent),
    TouchEnded(TouchEvent),
    SensorChanged(SensorEvent),
    Unimplemented,
}

impl Event {
    #[allow(nonstandard_style)]
    pub fn from_csfml(event: sfEvent) -> Result<Self> {
        unsafe {
            match event.type_ {
                sfEvtClosed => Ok(Self::Closed),
                sfEvtResized => Ok(Self::Resized(SizeEvent {
                    size: Vector2u {
                        x: event.size.width,
                        y: event.size.height,
                    },
                })),
                sfEvtLostFocus => Ok(Self::LostFocus),
                sfEvtGainedFocus => Ok(Self::GainedFocus),
                sfEvtTextEntered => Ok(Self::TextEntered(TextEvent {
                    unicode: event.text.unicode,
                })),
                sfEvtKeyPressed | sfEvtKeyReleased => Ok(Self::KeyPressed(KeyEvent {
                    code: event.key.code,
                    alt: event.key.alt != 0,
                    control: event.key.control != 0,
                    shift: event.key.shift != 0,
                    system: event.key.system != 0,
                })),
                sfEvtMouseWheelMoved => Ok(Self::MouseWheelMoved(MouseWheelEvent {
                    delta: event.mouseWheel.delta,
                    x: event.mouseWheel.x,
                    y: event.mouseWheel.y,
                })),
                sfEvtMouseWheelScrolled => Ok(Self::MouseWheelScrolled(MouseWheelScrollEvent {
                    wheel: event.mouseWheelScroll.wheel,
                    delta: event.mouseWheelScroll.delta,
                    pos: Vector2i {
                        x: event.mouseWheelScroll.x,
                        y: event.mouseWheelScroll.y,
                    },
                })),
                sfEvtMouseButtonPressed | sfEvtMouseButtonReleased => {
                    Ok(Self::MouseButtonPressed(MouseButtonEvent {
                        button: event.mouseButton.button,
                        pos: Vector2i {
                            x: event.mouseButton.x,
                            y: event.mouseButton.y,
                        },
                    }))
                }
                sfEvtMouseMoved => Ok(Self::MouseMoved(MouseMoveEvent {
                    pos: Vector2i {
                        x: event.mouseMove.x,
                        y: event.mouseMove.y,
                    },
                })),
                sfEvtMouseEntered => Ok(Self::MouseEntered),
                sfEvtMouseLeft => Ok(Self::MouseLeft),
                sfEvtJoystickButtonPressed | sfEvtJoystickButtonReleased => {
                    Ok(Self::JoystickButtonPressed(JoystickButtonEvent {
                        joystick_id: event.joystickButton.joystickId,
                        button: event.joystickButton.button,
                    }))
                }
                sfEvtJoystickMoved => Ok(Self::JoystickMoved(JoystickMoveEvent {
                    joystick_id: event.joystickMove.joystickId,
                    axis: event.joystickMove.axis,
                    position: event.joystickMove.position,
                })),
                sfEvtJoystickConnected | sfEvtJoystickDisconnected => {
                    Ok(Self::JoystickConnected(JoystickConnectEvent {
                        joystick_id: event.joystickConnect.joystickId,
                    }))
                }
                sfEvtTouchBegan | sfEvtTouchMoved | sfEvtTouchEnded => {
                    Ok(Self::TouchBegan(TouchEvent {
                        finger: event.touch.finger,
                        pos: Vector2i {
                            x: event.touch.x,
                            y: event.touch.y,
                        },
                    }))
                }
                sfEvtSensorChanged => Ok(Self::SensorChanged(SensorEvent {
                    sensor_type: event.sensor.sensorType,
                    vector: Vector3f {
                        x: event.sensor.x,
                        y: event.sensor.y,
                        z: event.sensor.z,
                    },
                })),
                _ => Err(EventError::Unimplemented(event.type_).into()),
            }
        }
    }

    #[must_use]
    pub const fn event_count() -> sfEventType {
        sfEvtCount as sfEventType
    }
}

#[derive(Debug)]
pub enum EventError {
    Unimplemented(u32),
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unimplemented(type_code) => {
                write!(f, "Unimplemented event type: {type_code}")
            }
        }
    }
}

impl Error for EventError {}

// Event parameter structs

#[derive(Debug)]
pub struct SizeEvent {
    pub size: Vector2u,
}

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct KeyEvent {
    pub code: sfKeyCode,
    pub alt: bool,
    pub control: bool,
    pub shift: bool,
    pub system: bool,
}

#[derive(Debug)]
pub struct TextEvent {
    pub unicode: u32,
}

#[derive(Debug)]
pub struct MouseMoveEvent {
    pub pos: Vector2i,
}

#[derive(Debug)]
pub struct MouseButtonEvent {
    pub button: sfMouseButton,
    pub pos: Vector2i,
}

#[derive(Debug)]
pub struct MouseWheelEvent {
    pub delta: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct MouseWheelScrollEvent {
    pub wheel: sfMouseWheel,
    pub delta: f32,
    pub pos: Vector2i,
}

#[derive(Debug)]
pub struct JoystickMoveEvent {
    pub joystick_id: u32,
    pub axis: sfJoystickAxis,
    pub position: f32,
}

#[derive(Debug)]
pub struct JoystickButtonEvent {
    pub joystick_id: u32,
    pub button: u32,
}

#[derive(Debug)]
pub struct JoystickConnectEvent {
    pub joystick_id: u32,
}

#[derive(Debug)]
pub struct TouchEvent {
    pub finger: u32,
    pub pos: Vector2i,
}

#[derive(Debug)]
pub struct SensorEvent {
    pub sensor_type: sfSensorType,
    pub vector: Vector3f,
}
