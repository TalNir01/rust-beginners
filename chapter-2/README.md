# Chapter 2 - Start Writing Rust

## Setup

We should download `rust` toolchain
2 Core components of the `toolchain`

1. rustup - toolchain manager
2. rustc - toolchains

View [rust-official-website](https://rust-lang.org) or [rustup-rs](https://rustup.rs) (Quick setup)

Rust `cargo` is the best-multi-tool utility!

Has `cargo new <project-name>` generates a new cargo project with certain format.

## Working with variables

Integers (`i` - signed, `u` - unsigned) numbers with `64/32/../` (Width) amount of bits a number is taking!

f32 / f64 - Floating points (always signed)

`char` -> Textual signs holder, a `unicode` symbol (32 bit =  4 bytes)
usize / isize -> Like `size_t`;

`bool` - Is `true` or `false`;

Literals values (like i32) or `str`
If we want a `0` in float we can use `0f64`.

We can also do `'X'` which is char, if it's with `b` at begging it's a binary value (u8 / array of u8) - Only ASCII.

we can use `into` function to convert types, or using `x as i64` -> Convert `x` to `i64` type (Might loose data!).

Functions like `println!` is actually a `macro` and not regular function!

## Arrays

A collection of elements of the same type

```rust
let arr: [i32,3] = [1, 2, 3]; // Type [element-type, length]
println!(arr[0]); // arr[INDEX] -> return value
```

tuple -> More complex type of pairs. It's more like `struct` with `.INDEX` for properties!

For being able to modify variables value we must make them `mut` (mutable) -> `let mut x: i64 = ...`

We can use rust `const` for global constants (probably entire program lifetime...)

In `rust` there is a `match` which is the strong cousin of `if`. Every `match VAL` must implement all the possible branches for `VAL` (There is `_` for default, should be the last). It's does pattern matching!
