//! A module providing synchronization primitives, specifically a `Mutex`
//! for mutual exclusion in a multithreaded environment. This module is
//! a Rust implementation inspired by SFML's `sf::Mutex` class and its
//! related helper `sf::Lock` class.
//!
//! ## Notes on Deadlock and Best Practices
//!
//! Be cautious with how you use `Mutex` and `Lock`. A common pitfall is
//! **deadlock**, where two or more threads are waiting on each other to
//! release a mutex, causing the program to get stuck. Avoid situations
//! where a thread locks multiple mutexes in a nested manner unless
//! absolutely necessary. Always try to lock mutexes in the same order
//! to minimize the risk of deadlock.
//!
//! In general, it's best practice to keep the scope of locked mutexes
//! as small as possible to reduce contention between threads and to
//! avoid performance bottlenecks.

use std::ptr;

use csfml_sys::{sfMutex, sfMutex_create, sfMutex_destroy, sfMutex_lock, sfMutex_unlock};
use derive_more::derive::{AsMut, AsRef, Deref, DerefMut};

#[derive(Debug, Clone, Deref, DerefMut, AsRef, AsMut)]
pub struct Mutex {
    ptr: *mut sfMutex,
}

impl Default for Mutex {
    fn default() -> Self {
        Self {
            ptr: unsafe { sfMutex_create() },
        }
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        unsafe { sfMutex_destroy(self.ptr) };
    }
}

impl Mutex {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn destroy(&mut self) {
        unsafe { sfMutex_destroy(self.ptr) };
        self.ptr = ptr::null_mut();
    }

    pub fn lock(&self) {
        unsafe { sfMutex_lock(self.ptr) };
    }

    pub fn unlock(&self) {
        unsafe { sfMutex_unlock(self.ptr) };
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Lock<'a> {
    mutex: &'a Mutex,
}

impl<'a> Lock<'a> {
    #[must_use]
    pub fn new(mutex: &'a Mutex) -> Self {
        mutex.lock();

        Self { mutex }
    }
}

impl Drop for Lock<'_> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}
