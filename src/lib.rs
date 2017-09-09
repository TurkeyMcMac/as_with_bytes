#![no_std]

use core::mem;
use core::slice;

pub trait AsBytes {
    fn as_bytes<'a>(&'a self) -> &'a [u8];
}

pub trait WithBytes {
    unsafe fn with_bytes<'a>(bytes: &'a [u8]) -> &'a Self;
    
    unsafe fn try_with_bytes<'a>(bytes: &'a [u8]) -> Option<&'a Self>;
}

impl <T: Copy> AsBytes for T {
    fn as_bytes<'a>(&'a self) -> &'a [u8] {
        unsafe {
            slice::from_raw_parts(
                mem::transmute::<_, *const u8>(self),
                mem::size_of::<T>(),
            )
        }
    }
}

impl <T: Copy> WithBytes for T {
    unsafe fn with_bytes<'a>(bytes: &'a [u8]) -> &'a T {
        mem::transmute::<_, &'a T>(bytes.as_ptr())
    }
    
    unsafe fn try_with_bytes<'a>(bytes: &'a [u8]) -> Option<&'a T> {
        if bytes.len() < mem::size_of::<T>() {
            None
        } else {
            Some(T::with_bytes(bytes))
        }
    }
}

impl <T: Copy> AsBytes for [T] {
    fn as_bytes<'a>(&'a self) -> &'a [u8] {
        unsafe {
            slice::from_raw_parts(
                mem::transmute::<_, *const u8>(self.as_ptr()),
                self.len() * mem::size_of::<T>(),
            )
        }
    }
}

impl <T: Copy> WithBytes for [T] {    
    unsafe fn with_bytes<'a>(bytes: &'a [u8]) -> &'a [T] {
        slice::from_raw_parts(
            mem::transmute::<_, *const T>(bytes.as_ptr()),
            bytes.len() / mem::size_of::<T>(),
        )
    }
    
    unsafe fn try_with_bytes<'a>(bytes: &'a [u8]) -> Option<&'a [T]> {
        Some(<[T]>::with_bytes(bytes))
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
}
