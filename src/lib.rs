use std::mem;
use std::slice;

pub trait AsBytes {
    fn as_bytes<'a>(&'a self) -> &'a [u8];
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

pub trait FromBytes {
    unsafe fn from_bytes<'a>(bytes: &'a [u8]) -> &'a Self;
}

impl <T: Copy> FromBytes for T {
    unsafe fn from_bytes<'a>(bytes: &'a [u8]) -> &'a T {
        mem::transmute::<_, &'a T>(bytes.as_ptr())
    }
}
