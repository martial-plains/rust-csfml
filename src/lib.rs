#![warn(clippy::pedantic, clippy::nursery)]

#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "graphics")]
pub mod graphics;
#[cfg(feature = "network")]
pub mod network;
pub mod system;
pub mod types;
#[cfg(feature = "window")]
pub mod window;

mod utils;
