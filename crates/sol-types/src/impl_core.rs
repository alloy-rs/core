//! Modified implementations of unstable libcore functions.

use alloc::vec::Vec;
use core::mem::{self, MaybeUninit};

trait Ext {
    const IS_ZST: bool;
}

impl<T> Ext for T {
    const IS_ZST: bool = mem::size_of::<Self>() == 0;
}

/// [`core::array::try_from_fn`]
#[inline]
pub(crate) fn try_from_fn<F, T, E, const N: usize>(mut cb: F) -> Result<[T; N], E>
where
    F: FnMut(usize) -> Result<T, E>,
{
    if N == 0 {
        // SAFETY: An empty array is always inhabited and has no validity invariants.
        return unsafe { Ok(mem::zeroed()) };
    }

    struct Guard<'a, T, const N: usize> {
        array_mut: &'a mut [MaybeUninit<T>; N],
        initialized: usize,
    }

    impl<T, const N: usize> Drop for Guard<'_, T, N> {
        fn drop(&mut self) {
            debug_assert!(self.initialized <= N);

            // SAFETY: this slice will contain only initialized objects.
            unsafe {
                core::ptr::drop_in_place(slice_assume_init_mut(
                    self.array_mut.get_unchecked_mut(..self.initialized),
                ));
            }
        }
    }

    let mut array = uninit_array::<T, N>();
    let mut guard = Guard { array_mut: &mut array, initialized: 0 };

    for _ in 0..N {
        // SAFETY: `guard.initialized` starts at 0, is increased by one in the
        // loop and the loop is aborted once it reaches N (which is `array.len()`).
        unsafe {
            guard.array_mut.get_unchecked_mut(guard.initialized).write(cb(guard.initialized)?);
        }
        guard.initialized += 1;
    }

    mem::forget(guard);
    // SAFETY: all elements are initialized.
    Ok(unsafe { array_assume_init(array) })
}

/// [`array::split_array_ref`]
#[inline]
#[track_caller]
pub(crate) fn split_array_ref<T, const N: usize>(slice: &[T]) -> (&[T; N], &[T]) {
    let (a, b) = slice.split_at(N);
    // SAFETY: a points to [T; N]? Yes it's [T] of length N (checked by split_at)
    unsafe { (&*a.as_ptr().cast::<[T; N]>(), b) }
}

/// [`MaybeUninit::slice_assume_init_mut`]
#[inline(always)]
unsafe fn slice_assume_init_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
    // SAFETY: similar to safety notes for `slice_get_ref`, but we have a
    // mutable reference which is also guaranteed to be valid for writes.
    unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) }
}

/// [`MaybeUninit::uninit_array`]
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
    mem::transmute_copy::<[MaybeUninit<T>; N], MaybeUninit<[T; N]>>(&mem::ManuallyDrop::new(&array))
}

/// [`Vec::into_flattened`].
#[inline]
pub(crate) fn into_flattened<T, const N: usize>(vec: Vec<[T; N]>) -> Vec<T> {
    let (ptr, len, cap) = into_raw_parts(vec);
    let (new_len, new_cap) = if T::IS_ZST {
        (len.checked_mul(N).expect("vec len overflow"), usize::MAX)
    } else {
        // SAFETY:
        // - `cap * N` cannot overflow because the allocation is already in
        // the address space.
        // - Each `[T; N]` has `N` valid elements, so there are `len * N`
        // valid elements in the allocation.
        unsafe { (len.checked_mul(N).unwrap_unchecked(), cap.checked_mul(N).unwrap_unchecked()) }
    };
    // SAFETY:
    // - `ptr` was allocated by `self`
    // - `ptr` is well-aligned because `[T; N]` has the same alignment as `T`.
    // - `new_cap` refers to the same sized allocation as `cap` because
    // `new_cap * size_of::<T>()` == `cap * size_of::<[T; N]>()`
    // - `len` <= `cap`, so `len * N` <= `cap * N`.
    unsafe { Vec::from_raw_parts(ptr.cast(), new_len, new_cap) }
}

/// [`Vec::into_raw_parts`]
#[inline(always)]
fn into_raw_parts<T>(vec: Vec<T>) -> (*mut T, usize, usize) {
    let mut me = mem::ManuallyDrop::new(vec);
    (me.as_mut_ptr(), me.len(), me.capacity())
}
