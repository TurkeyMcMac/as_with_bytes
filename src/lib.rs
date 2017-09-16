#![no_std]

//! Using these traits, values can be serialized as bytes without any copying of data whatsoever.
//! 
//! The generalization that is used here only works for data which contains no pointers to other
//! data. As such, the traits are only implemented for types which implement `Copy` and for slices
//! whose contents implement `Copy`.
//! 
//! This crate makes no guarantees about portability across systems; it simply encodes the raw
//! bytes of values.
//! 
//! #### It's all the same block of memory
//! ```rust
//! use as_with_bytes::{AsBytes, WithBytes};
//! 
//! use std::ptr;
//! 
//! let arr = [10, -11];
//! 
//! // They reference the same memory address
//! assert!(ptr::eq(unsafe { <[i32; 2]>::with_bytes(arr.as_bytes()) }, &arr));
//! ```

use core::mem;
use core::slice;

/// A trait used for converting into bytes.
pub trait AsBytes {
    /// Returns a byte slice representation of `self`.
    fn as_bytes(&self) -> &[u8];
}

/// A trait used for converting from bytes.
pub trait WithBytes {
    /// Returns a `Self` representation of the given slice of bytes.
    /// 
    /// # Panics
    /// This function panics when a slice containing a zero-sized
    /// type is requested.
    /// 
    /// # Unsafe
    /// This function is unsafe for two reasons: Firstly, If the
    /// length of `bytes` is shorter than `size_of::<Self>`,
    /// arbitrary memory is read. Secondly, invalid values can
    /// be returned, such as an instance of an empty enum.
    /// This method will work fine as long as you are careful to
    /// avoid both scenarios.
    unsafe fn with_bytes<'a>(bytes: &'a [u8]) -> &'a Self;
}


/// A trait for converting from bytes while checking that the byte
/// slice is long enough.
pub trait TryWithBytes {
    /// Returns `Some(&Self)` if there are enough bytes to encode `Self`,
    /// or `None` otherwise.
    /// 
    /// # Unsafe
    /// While this protects against reading from memory beyond the boundary
    /// of the bytes, it can still produce invalid data for some types such
    /// as enums. It will work as long as whatever you encode from a type,
    /// you decode into that same type.
    /// 
    /// #### Note
    /// When used to decode dynamically sized slices, `Some` will almost always
    /// be returned, since the slice will be empty if there is not enough data.
    /// The only situation where `None` is ever returned is when you ask for a
    /// slice containing a type with a size of zero, and who would want that?
    unsafe fn try_with_bytes<'a>(bytes: &'a [u8]) -> Option<&'a Self>;
}

impl <T: Copy> AsBytes for T {
    #[inline]
    fn as_bytes<'a>(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                mem::transmute::<_, *const u8>(self),
                mem::size_of::<T>(),
            )
        }
    }
}

impl <T: Copy> WithBytes for T {
    #[inline]
    unsafe fn with_bytes<'a>(bytes: &'a [u8]) -> &'a T {
        mem::transmute::<_, &'a T>(bytes.as_ptr())
    }
}

impl <T: Copy> TryWithBytes for T {
    #[inline]
    unsafe fn try_with_bytes<'a>(bytes: &'a [u8]) -> Option<&'a T> {
        if bytes.len() < mem::size_of::<T>() {
            None
        } else {
            Some(T::with_bytes(bytes))
        }
    }
}

impl <T: Copy> AsBytes for [T] {
    #[inline]
    fn as_bytes<'a>(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                mem::transmute::<_, *const u8>(self.as_ptr()),
                self.len() * mem::size_of::<T>(),
            )
        }
    }
}

impl <T: Copy> WithBytes for [T] {    
    #[inline]
    unsafe fn with_bytes<'a>(bytes: &'a [u8]) -> &'a [T] {
        slice::from_raw_parts(
            mem::transmute::<_, *const T>(bytes.as_ptr()),
            bytes.len() / mem::size_of::<T>(),
        )
    }
}

impl <T: Copy> TryWithBytes for [T] {
    #[inline]
    unsafe fn try_with_bytes<'a>(bytes: &'a [u8]) -> Option<&'a [T]> {
        if mem::size_of::<T>() > 0 {
            Some(<[T]>::with_bytes(bytes))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use core::ptr;
    use super::*;
    
    #[test]
    fn information_preserved() {
        assert_eq!(unsafe { <[i32; 2]>::with_bytes([10, -11].as_bytes()) }, &[10, -11]);
    }
    
    #[test]
    fn ptr_equal() {
        let arr = [10, -11];
        
        assert!(ptr::eq(unsafe { <[i32; 2]>::with_bytes(arr.as_bytes()) }, &arr));
    }
    
    #[test]
    fn try_with_bytes_works() {
        assert_eq!(unsafe { u64::try_with_bytes(&[0; 8]) }, Some(&0));
        assert_eq!(unsafe { u64::try_with_bytes(&[0; 7]) }, None);
    }
    
    #[test]
    fn unsized_slices_work() {
        let arr = [0u8; 7];
        
        assert_eq!(unsafe { <[u16]>::with_bytes(&arr) }, &[0; 3]);
    }
    
    #[test]
    fn zero_size_type_slices_work() {
        let byte: u8 = 0;
        
        assert_eq!(unsafe { <[()]>::try_with_bytes(byte.as_bytes()) }, None);
    }
}
