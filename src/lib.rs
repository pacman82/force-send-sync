//! Please do not use this crate. The Rust compiler tries to protect you for a reason. Do under no
//! circumstances use this to silence some compiler error you do not understand. Only use this if
//! you do understand why your type is `Send` and or `Sync`, and also understand why the compiler
//! disagrees with you.
use std::ops::{Deref, DerefMut};

/// Wraps a type to make it implement `Send`.
#[repr(transparent)]
pub struct Send<T>(T);

impl<T> Send<T> {

    /// # Safety
    ///
    /// This is not a magic way to make `t` `Send`. It is a way to tell the compiler `t` is `Send`
    /// and you should only call this method if you are sure this is not a lie.
    pub unsafe fn new(t: T) -> Self {
        Send(t)
    }

    /// Destroy wrapper and get original type.
    pub fn unwrap(self) -> T {
        self.0
    }
}

unsafe impl<T> std::marker::Send for Send<T> {}

impl<T> Deref for Send<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Send<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Wraps a type to make it implement `Sync`.
#[repr(transparent)]
pub struct Sync<T>(T);

impl<T> Sync<T> {

    /// # Safety
    ///
    /// This is not a magic way to make `t` `Sync`. It is a way to tell the compiler `t` is `Sync`
    /// and you should only call this method if you are sure this is not a lie.
    pub unsafe fn new(t: T) -> Self {
        Sync(t)
    }

    /// Destroy wrapper and get original type.
    pub fn unwrap(self) -> T {
        self.0
    }
}

unsafe impl<T> std::marker::Sync for Sync<T> {}

impl<T> Deref for Sync<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Sync<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Wraps a type to make it implement `Send` and `Sync`.
#[repr(transparent)]
pub struct SendSync<T>(T);

impl<T> SendSync<T> {

    /// # Safety
    ///
    /// This is not a magic way to make `t` `Send` and `Sync`. It is a way to tell the compiler `t`
    /// is `Send` and `Sync` and you should only call this method if you are sure this is not a lie.
    pub unsafe fn new(t: T) -> Self {
        SendSync(t)
    }

    /// Destroy wrapper and get original type.
    pub fn unwrap(self) -> T {
        self.0
    }
}

unsafe impl<T> std::marker::Send for SendSync<T> {}
unsafe impl<T> std::marker::Sync for SendSync<T> {}

impl<T> Deref for SendSync<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for SendSync<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
