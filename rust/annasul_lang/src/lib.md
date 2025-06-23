# Annasul Language

> a rust-like language

## Example

```
let number = box 1; // let number: i32.Box = box 1;
println!("{*number}"); // output `1` and newline
```

equivalent to

```rust
fn main() {
    let number = box 1; // let number = Box::new(1); 
    println!("{}, *number")
}
```

## Basic Type

|    type \ bit    |    8    |    16    |      32       | 64  | 128  | size(32\|64) |
|:----------------:|:-------:|:--------:|:-------------:|:---:|:----:|:------------:|
|     integer      |   i8    |   i16    |      i32      | i64 | i128 |    isize     |
| unsigned integer |   u8    |   u16    |      u32      | u64 | u128 |    usize     |
|   float number   |    \    |   f16    |      f32      | f64 | f128 |      \       |
|     boolean      |  bool   |    \     |       \       |  \  |  \   |      \       |
|    character     | c_char8 | c_char16 | char/c_char32 |  \  |  \   |      \       |
|     pointer      |    \    |    \     |       \       |  \  |  \   |   *const T   |
| mutable pointer  |    \    |    \     |       \       |  \  |  \   |    *mut T    |
|       ref        |    \    |    \     |       \       |  \  |  \   |      &T      |
|   mutable ref    |    \    |    \     |       \       |  \  |  \   |    &mut T    |

|     type      |          define          |                            size                            |
|:-------------:|:------------------------:|:----------------------------------------------------------:|
| dynamic array |         arr: [T]         |                size_of::\<T\>() * arr.len()                |
| static array  |      arr: [T; len]       |                   size_of::\<T\>() * len                   |
|  empty tuple  |        tuple: ()         |                             \                              |
|     tuple     | tuple: (T1, T2, T3, ...) | size_of::\<T1>() +size_of::\<T2>() +size_of::\<T3>() + ... |
