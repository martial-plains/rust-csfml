use std::{ffi::CString, os::raw::c_void, ptr};

use derive_more::derive::{AsMut, AsRef, Deref, DerefMut};
use csfml_sys::{
    sfBool, sfMusic, sfMusic_createFromFile, sfMusic_createFromMemory, sfMusic_destroy,
    sfMusic_getAttenuation, sfMusic_getChannelCount, sfMusic_getDuration, sfMusic_getLoop,
    sfMusic_getMinDistance, sfMusic_getPitch, sfMusic_getPlayingOffset, sfMusic_getPosition,
    sfMusic_getSampleRate, sfMusic_getStatus, sfMusic_getVolume, sfMusic_isRelativeToListener,
    sfMusic_pause, sfMusic_play, sfMusic_setAttenuation, sfMusic_setLoop, sfMusic_setMinDistance,
    sfMusic_setPitch, sfMusic_setPlayingOffset, sfMusic_setPosition, sfMusic_setRelativeToListener,
    sfMusic_setVolume, sfMusic_stop,
};

use crate::system::{time::Time, Vector3f};

use super::sound::SoundStatus;

#[derive(Debug, Clone, Deref, DerefMut, AsRef, AsMut)]
pub struct Music {
    ptr: *mut sfMusic,
}

impl Drop for Music {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Music {
    #[must_use]
    pub fn create_from_file(path: &str) -> Self {
        let music = unsafe { sfMusic_createFromFile(CString::new(path).unwrap().as_ptr()) };

        Self { ptr: music }
    }

    #[must_use]
    pub fn create_from_memory(data: &[u8]) -> Self {
        let music = unsafe { sfMusic_createFromMemory(data.as_ptr().cast::<c_void>(), data.len()) };

        Self { ptr: music }
    }

    pub fn destroy(&mut self) {
        unsafe { sfMusic_destroy(self.ptr) };

        self.ptr = ptr::null_mut();
    }

    pub fn play(&self) {
        unsafe { sfMusic_play(self.ptr) };
    }

    pub fn pause(&self) {
        unsafe {
            sfMusic_pause(self.ptr);
        }
    }

    pub fn stop(&self) {
        unsafe {
            sfMusic_stop(self.ptr);
        }
    }

    #[must_use]
    pub fn duration(&self) -> Time {
        unsafe { Time::from(sfMusic_getDuration(self.ptr)) }
    }

    #[must_use]
    pub fn playing_offset(&self) -> Time {
        unsafe { Time::from(sfMusic_getPlayingOffset(self.ptr)) }
    }

    pub fn set_playing_offset(&self, offset: Time) {
        unsafe { sfMusic_setPlayingOffset(self.ptr, offset.into()) }
    }

    #[must_use]
    pub fn loop_enabled(&self) -> bool {
        unsafe { sfMusic_getLoop(self.ptr) != 0 }
    }

    pub fn set_loop(&self, loop_enabled: bool) {
        unsafe { sfMusic_setLoop(self.ptr, sfBool::from(loop_enabled)) }
    }

    #[must_use]
    pub fn pitch(&self) -> f32 {
        unsafe { sfMusic_getPitch(self.ptr) }
    }

    pub fn set_pitch(&self, pitch: f32) {
        unsafe { sfMusic_setPitch(self.ptr, pitch) }
    }

    #[must_use]
    pub fn volume(&self) -> f32 {
        unsafe { sfMusic_getVolume(self.ptr) }
    }

    pub fn set_volume(&self, volume: f32) {
        unsafe { sfMusic_setVolume(self.ptr, volume) }
    }

    #[must_use]
    pub fn sample_rate(&self) -> usize {
        unsafe { sfMusic_getSampleRate(self.ptr) as usize }
    }

    #[must_use]
    pub fn channel_count(&self) -> usize {
        unsafe { sfMusic_getChannelCount(self.ptr) as usize }
    }

    #[must_use]
    pub fn status(&self) -> SoundStatus {
        unsafe { std::mem::transmute(sfMusic_getStatus(self.ptr)) }
    }

    #[must_use]
    pub fn is_relative_to_listener(&self) -> bool {
        unsafe { sfMusic_isRelativeToListener(self.ptr) != 0 }
    }

    pub fn set_relative_to_listener(&self, relative: bool) {
        unsafe { sfMusic_setRelativeToListener(self.ptr, i32::from(relative)) }
    }

    pub fn set_min_distance(&self, min_distance: f32) {
        unsafe { sfMusic_setMinDistance(self.ptr, min_distance) }
    }

    #[must_use]
    pub fn min_distance(&self) -> f32 {
        unsafe { sfMusic_getMinDistance(self.ptr) }
    }

    pub fn set_attenuation(&self, attenuation: f32) {
        unsafe { sfMusic_setAttenuation(self.ptr, attenuation) }
    }

    #[must_use]
    pub fn attenuation(&self) -> f32 {
        unsafe { sfMusic_getAttenuation(self.ptr) }
    }

    pub fn set_position(&self, position: Vector3f) {
        unsafe { sfMusic_setPosition(self.ptr, position.into()) }
    }

    #[must_use]
    pub fn position(&self) -> Vector3f {
        unsafe { Vector3f::from(sfMusic_getPosition(self.ptr)) }
    }
}
