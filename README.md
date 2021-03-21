**Don't use this! When I made it, I didn't know about memory alignment.**

# as_with_bytes
Simple serialization for simple values.

Using these traits, values can be serialized as bytes without any copying of data whatsoever.

## Implementations
`AsBytes`, `WithBytes`, and `TryWithBytes` are all implemented for types `T` and `[T]` where `T: Copy`.
`try_with_bytes` always returns `Some(&[T])` when used on a dynamically sized slice, although the slice
referenced can be empty. The trait is only implemented there for genericity.

## Examples
```rust
let n: u32 = 0;

assert_eq!(n.as_bytes(), &[0, 0, 0, 0]);
```
#### It's all the same block of memory
```rust
use core::ptr;

let arr = [10, -11];

// They reference the same memory address
assert!(ptr::eq(unsafe { <[i32; 2]>::with_bytes(arr.as_bytes()) }, &arr));
```

## How to obtain
This is available on crates.io [here](https://crates.io/crates/as_with_bytes). More documentation can
also be found at that location.
