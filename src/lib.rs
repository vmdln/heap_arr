#![warn(clippy::pedantic)]
#![no_std]

use core::ptr::{self, NonNull};

use alloc::{
    alloc::{handle_alloc_error, Layout, LayoutError},
    boxed::Box,
};

extern crate alloc;

/// Allocates `[T; N]` on the heap and initializes its entries to `initial`.
///
/// For now, this function uses the global allocator. This will change once the
/// [`Allocator`](core::alloc::Allocator) trait becomes stable.
///
/// # Safety
/// The result of this function is undefined if `mem::size_of::<T>() == 0`, or
/// if `N == 0`
///
/// # Errors
/// See [`Layout::array`]
pub unsafe fn new<T, const N: usize>(initial: &T) -> Result<Box<[T; N]>, LayoutError>
where
    T: Clone,
{
    unsafe {
        let mut ptr = new_uninit()?;
        for v in ptr.as_mut() {
            ptr::write(v, initial.clone());
        }

        Ok(Box::from_raw(ptr.as_ptr()))
    }
}

/// Allocates `[T; N]` on the heap and initializes its entries to `T::default()`.
///
/// For now, this function uses the global allocator. This will change once the
/// [`Allocator`](core::alloc::Allocator) trait becomes stable.
///
/// # Safety
/// The result of this function is undefined if `mem::size_of::<T>() == 0`, or
/// if `N == 0`
///
/// # Errors
/// See [`Layout::array`]
pub unsafe fn new_default<T, const N: usize>() -> Result<Box<[T; N]>, LayoutError>
where
    T: Default,
{
    unsafe {
        let mut ptr = new_uninit()?;
        for v in ptr.as_mut() {
            ptr::write(v, T::default());
        }

        Ok(Box::from_raw(ptr.as_ptr()))
    }
}

/// Allocates `[T; N]` on the heap.
///
/// The returned pointer is guaranteed to be non-null. The caller is required
/// to handle deallocation if he wants to avoid memory leaks.
///
/// For now, this function uses the global allocator. This will change once the
/// [`Allocator`](core::alloc::Allocator) trait becomes stable.
///
/// # Safety
/// - The elements are uninitialized
/// - The result of this function is undefined if `mem::size_of::<T>() == 0`, or
/// if `N == 0`
///
/// # Errors
/// See [`Layout::array`]
///
/// # Examples
/// ```
/// use std::{alloc::{self, Layout}, ptr};
/// use heap_arr::new_uninit;
///
/// unsafe {
///     let mut ptr = new_uninit::<usize, 16>().unwrap();
///     for (i, v) in ptr.as_mut().iter_mut().enumerate() {
///         ptr::write(v, i);
///     }
///
///     let _ = Box::from_raw(ptr.as_ptr());
/// }
/// ```
pub unsafe fn new_uninit<T, const N: usize>() -> Result<NonNull<[T; N]>, LayoutError> {
    let layout = Layout::array::<T>(N)?;
    let ptr = alloc::alloc::alloc(layout);

    match NonNull::new(ptr) {
        Some(v) => Ok(v.cast()),
        None => handle_alloc_error(layout),
    }
}
