use std::ffi::CString;
use std::ptr;

use csfml_sys::{
    sfSound, sfSoundBuffer, sfSoundBuffer_createFromFile, sfSoundBuffer_createFromMemory,
    sfSoundBuffer_createFromSamples, sfSoundBuffer_destroy, sfSoundBuffer_getChannelCount,
    sfSoundBuffer_getDuration, sfSoundBuffer_getSampleCount, sfSoundBuffer_getSampleRate,
    sfSoundBuffer_saveToFile, sfSound_create, sfSound_destroy, sfSound_getAttenuation,
    sfSound_getBuffer, sfSound_getLoop, sfSound_getMinDistance, sfSound_getPitch,
    sfSound_getPlayingOffset, sfSound_getPosition, sfSound_getStatus, sfSound_getVolume,
    sfSound_isRelativeToListener, sfSound_pause, sfSound_play, sfSound_setAttenuation,
    sfSound_setBuffer, sfSound_setLoop, sfSound_setMinDistance, sfSound_setPitch,
    sfSound_setPlayingOffset, sfSound_setPosition, sfSound_setRelativeToListener,
    sfSound_setVolume, sfSound_stop,
};

use crate::{
    system::{time::Time, Vector3f},
    types::Result,
};

/// Represents the status of a sound playing
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SoundStatus {
    Stopped,
    Paused,
    Playing,
}

#[derive(Debug, Clone)]
pub struct SoundBuffer {
    ptr: *mut sfSoundBuffer,
}

impl Drop for SoundBuffer {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl SoundBuffer {
    /// Loads a sound from a file
    pub fn create_from_file(path: &str) -> Result<Self> {
        let c_path = CString::new(path).unwrap();
        let sound = unsafe { sfSoundBuffer_createFromFile(c_path.as_ptr()) };

        (!sound.is_null())
            .then_some(Self { ptr: sound })
            .ok_or_else(|| "Error loading resource".into())
    }

    /// Loads a sound from memory
    pub fn create_from_memory(data: &[u8]) -> Result<Self> {
        let sound = unsafe {
            sfSoundBuffer_createFromMemory(data.as_ptr().cast::<std::ffi::c_void>(), data.len())
        };

        (!sound.is_null())
            .then_some(Self { ptr: sound })
            .ok_or_else(|| "Error loading resource".into())
    }

    /// Creates a sound buffer from sample data
    pub fn create_from_samples(
        samples: &[i16],
        channel_count: usize,
        sample_rate: usize,
    ) -> Result<Self> {
        let sound = unsafe {
            sfSoundBuffer_createFromSamples(
                samples.as_ptr().cast::<i16>(),
                samples.len() as u64,
                channel_count as u32,
                sample_rate as u32,
            )
        };

        (!sound.is_null())
            .then_some(Self { ptr: sound })
            .ok_or_else(|| "Error loading resource".into())
    }

    pub fn destroy(&mut self) {
        if !self.ptr.is_null() {
            unsafe { sfSoundBuffer_destroy(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }

    #[must_use]
    pub fn duration(&self) -> Time {
        unsafe { Time::from(sfSoundBuffer_getDuration(self.ptr)) }
    }

    pub fn sample_count(&self) -> Result<usize> {
        unsafe { Ok(usize::try_from(sfSoundBuffer_getSampleCount(self.ptr))?) }
    }

    /// Gets the sample rate of this sound
    #[must_use]
    pub fn sample_rate(&self) -> usize {
        unsafe { sfSoundBuffer_getSampleRate(self.ptr) as usize }
    }

    /// Gets the channel count (e.g., 2 for stereo)
    #[must_use]
    pub fn channel_count(&self) -> usize {
        unsafe { sfSoundBuffer_getChannelCount(self.ptr) as usize }
    }

    /// Save the sound buffer to an audio file
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let c_path = CString::new(path).unwrap();
        let result = unsafe { sfSoundBuffer_saveToFile(self.ptr, c_path.as_ptr()) };

        (result == 1)
            .then_some(())
            .ok_or_else(|| "Error saving the file".into())
    }
}

#[derive(Debug, Clone)]
pub struct Sound {
    ptr: *mut sfSound,
}

impl Drop for Sound {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Sound {
    /// Inits an empty sound
    pub fn create() -> Result<Self> {
        let sound = unsafe { sfSound_create() };

        (!sound.is_null())
            .then_some(Self { ptr: sound })
            .ok_or_else(|| "Error creating sound".into())
    }

    /// Inits a sound with a `SoundBuffer` object
    pub fn create_from_buffer(buffer: &SoundBuffer) -> Result<Self> {
        let mut sound = Self::create()?;
        sound.set_buffer(buffer);
        Ok(sound)
    }

    /// Destroys this sound object
    pub fn destroy(&mut self) {
        if !self.ptr.is_null() {
            unsafe { sfSound_destroy(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }

    /// Plays the sound
    pub fn play(&self) {
        unsafe { sfSound_play(self.ptr) }
    }

    /// Pauses the sound
    pub fn pause(&self) {
        unsafe { sfSound_pause(self.ptr) }
    }

    /// Stops the sound and resets the player position
    pub fn stop(&self) {
        unsafe { sfSound_stop(self.ptr) }
    }

    /// Gets the buffer this sound is attached to
    /// Not valid if a buffer was never assigned (but not null?)
    #[must_use]
    pub fn buffer(&self) -> Option<SoundBuffer> {
        let buf = unsafe { sfSound_getBuffer(self.ptr) };
        if buf.is_null() {
            None
        } else {
            Some(SoundBuffer {
                ptr: buf.cast_mut(),
            })
        }
    }

    /// Sets the buffer this sound will play
    pub fn set_buffer(&mut self, buffer: &SoundBuffer) {
        unsafe { sfSound_setBuffer(self.ptr, buffer.ptr) }
    }

    /// Gets the current playing offset of the sound
    #[must_use]
    pub fn playing_offset(&self) -> Time {
        unsafe { Time::from(sfSound_getPlayingOffset(self.ptr)) }
    }

    /// Sets the current playing offset of the sound
    pub fn set_playing_offset(&mut self, offset: Time) {
        unsafe { sfSound_setPlayingOffset(self.ptr, offset.into()) }
    }

    /// Tells whether or not this sound is in loop mode
    #[must_use]
    pub fn r#loop(&self) -> bool {
        unsafe { sfSound_getLoop(self.ptr) != 0 }
    }

    /// Enable or disable auto loop
    pub fn set_loop(&mut self, loop_enabled: bool) {
        unsafe { sfSound_setLoop(self.ptr, i32::from(loop_enabled)) }
    }

    /// Gets the pitch of the sound
    #[must_use]
    pub fn pitch(&self) -> f32 {
        unsafe { sfSound_getPitch(self.ptr) }
    }

    /// Sets the pitch of the sound
    pub fn set_pitch(&mut self, pitch: f32) {
        unsafe { sfSound_setPitch(self.ptr, pitch) }
    }

    /// Gets the volume of the sound
    #[must_use]
    pub fn volume(&self) -> f32 {
        unsafe { sfSound_getVolume(self.ptr) }
    }

    /// Sets the volume of the sound
    pub fn set_volume(&mut self, volume: f32) {
        unsafe { sfSound_setVolume(self.ptr, volume) }
    }

    /// Gets the current status of the sound (stopped, paused, playing)
    #[must_use]
    pub fn status(&self) -> SoundStatus {
        unsafe { std::mem::transmute(sfSound_getStatus(self.ptr)) }
    }

    /// Tell whether the sound's position is relative to the listener or is absolute
    #[must_use]
    pub fn is_relative_to_listener(&self) -> bool {
        unsafe { sfSound_isRelativeToListener(self.ptr) != 0 }
    }

    /// Make the sound's position relative to the listener or absolute
    pub fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe { sfSound_setRelativeToListener(self.ptr, i32::from(relative)) }
    }

    /// Set the minimum distance of a sound
    pub fn set_min_distance(&mut self, min_distance: f32) {
        unsafe { sfSound_setMinDistance(self.ptr, min_distance) }
    }

    /// Get the minimum distance of a sound
    #[must_use]
    pub fn min_distance(&self) -> f32 {
        unsafe { sfSound_getMinDistance(self.ptr) }
    }

    /// Set the attenuation factor of a sound
    pub fn set_attenuation(&mut self, attenuation: f32) {
        unsafe { sfSound_setAttenuation(self.ptr, attenuation) }
    }

    /// Get the attenuation factor of a sound
    #[must_use]
    pub fn attenuation(&self) -> f32 {
        unsafe { sfSound_getAttenuation(self.ptr) }
    }

    /// Set the 3D position of a sound in the audio scene
    pub fn set_position(&mut self, position: Vector3f) {
        unsafe { sfSound_setPosition(self.ptr, position.into()) }
    }

    /// Get the 3D position of a sound in the audio scene
    #[must_use]
    pub fn position(&self) -> Vector3f {
        unsafe { Vector3f::from(sfSound_getPosition(self.ptr)) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_approx_eq,
        audio::SoundStatus,
        system::{time::Time, Vector3f},
    };

    use super::{Sound, SoundBuffer};

    #[test]
    fn sound_buffers_getters_and_setters() {
        let samples = vec![0; 44100 * 3].into_boxed_slice();

        let buffer = SoundBuffer::create_from_samples(&samples, 1, 44100).unwrap();

        assert_approx_eq!(3.0, buffer.duration().as_seconds(), 0.001);
        assert_eq!(44100 * 3, buffer.sample_count().unwrap());
        assert_eq!(44100, buffer.sample_rate());
        assert_eq!(1, buffer.channel_count());
    }

    #[test]
    fn sound_getters_and_setters() -> crate::types::Result<()> {
        let mut sound = Sound::create()?;
        sound.set_loop(true);
        sound.set_attenuation(0.5);
        sound.set_min_distance(10.0);
        sound.set_pitch(1.2);
        sound.set_relative_to_listener(true);
        sound.set_volume(2.0);
        sound.set_position(Vector3f::new(1.0, 2.0, 3.0));

        assert_eq!(SoundStatus::Stopped, sound.status());
        assert_eq!(Time::seconds(0.0), sound.playing_offset());
        assert!(sound.r#loop());
        assert_approx_eq!(0.5, sound.attenuation(), 0.001);
        assert_approx_eq!(10.0, sound.min_distance(), 0.001);
        assert_approx_eq!(1.2, sound.pitch(), 0.001);
        assert!(sound.is_relative_to_listener());
        assert_approx_eq!(2.0, sound.volume(), 0.001);

        let pos = sound.position();
        assert_approx_eq!(1.0, pos.x, 0.001);
        assert_approx_eq!(2.0, pos.y, 0.001);
        assert_approx_eq!(3.0, pos.z, 0.001);

        Ok(())
    }
}
