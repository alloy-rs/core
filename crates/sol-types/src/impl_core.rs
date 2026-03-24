//! Modified implementations of unstable libcore functions.

use core::{
    mem::{self, MaybeUninit},
    ptr,
};

/// Initializes each element of `out` by calling `f` for each slot.
///
/// On success, all elements in `out` are initialized.
/// On failure or panic, already-initialized elements are dropped.
#[inline]
pub(crate) fn try_init_each<T, E, F>(out: &mut [MaybeUninit<T>], mut f: F) -> Result<(), E>
where
    F: FnMut() -> Result<T, E>,
{
    struct Guard<'a, T> {
        buf: &'a mut [MaybeUninit<T>],
        initialized: usize,
    }
    impl<T> Drop for Guard<'_, T> {
        fn drop(&mut self) {
            // SAFETY: the first `self.initialized` elements are guaranteed initialized.
            unsafe {
                let ptr = self.buf.as_mut_ptr().cast::<T>();
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(ptr, self.initialized));
            }
        }
    }

    let mut guard = Guard { buf: out, initialized: 0 };
    for i in 0..guard.buf.len() {
        guard.buf[i].write(f()?);
        guard.initialized += 1;
    }
    mem::forget(guard);
    Ok(())
}

/// `MaybeUninit::uninit_array`
#[inline]
pub(crate) fn uninit_array<T, const N: usize>() -> [MaybeUninit<T>; N] {
    // SAFETY: An uninitialized `[MaybeUninit<_>; N]` is valid.
    unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() }
}

/// [`MaybeUninit::array_assume_init`]
#[inline]
pub(crate) unsafe fn array_assume_init<T, const N: usize>(array: [MaybeUninit<T>; N]) -> [T; N] {
    // SAFETY:
    // * The caller guarantees that all elements of the array are initialized
    // * `MaybeUninit<T>` and T are guaranteed to have the same layout
    // * `MaybeUninit` does not drop, so there are no double-frees
    // And thus the conversion is safe
    unsafe { transpose(array).assume_init() }
}

/// [`MaybeUninit::transpose`]
#[inline(always)]
unsafe fn transpose<T, const N: usize>(array: [MaybeUninit<T>; N]) -> MaybeUninit<[T; N]> {
    unsafe {
        mem::transmute_copy::<[MaybeUninit<T>; N], MaybeUninit<[T; N]>>(&mem::ManuallyDrop::new(
            &array,
        ))
    }
}
