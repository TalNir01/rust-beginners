# Rust-Beginners

A follow-up of a great rust introduction course.

## Things I should go over again

1. Results & Errors
    1. Specially how `?` acts
    2. Use of `anyhow` and `thiserror`
2. Again `&str` vs `String`
    1. includes `to_string()` and `into()`
3. Standard Lib
    1. `Clone` vs `Copy` traits
4. Closures
    1. Using of `map`

Link: [Rust-From-Scratch](https://turbofishandchips.com/rust-from-scratch/index.html)

## Results & Errors - Anyhow VS Thiserror

After meeting with `Option` and `Result` the next question is always *"Okay, but what do I put in the `E` slot?"*.

The actual problem
The `Result<T, E>` lets you pick any error type. The stdlib gives you `std::error::Error` which is just a trait, so in practice you have three options and all of them hurts

1. Use `String` as the error - Cheap, but you lose all structure and callers can't react to specific failures.
2. Define your own custom enum, correct but you must hand-write `Display`, `Error` and a `From` impl for every underlying error just so `?` works...
3. Use `Box<dyn Error>` - Works but it's clumsy and a bit overkill.

The `thiserror` makes option 2 painless, `anyhow` makes option 3 pleasant.

### Thiserror

Thiserror - Define your own custom error type, without boilerplate
It's a dervice macro. It's automatically generates (`Impl`) several key traits.

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("no config file at {path}")]
    NotFound { path: String },

    #[error("could not read config file")]
    Io(#[from] std::io::Error),

    #[error("invalid TOML")]
    Parse(#[from] toml::de::Error),
}

pub fn load(path: &str) -> Result<Config, ConfigError> {
    let text = std::fs::read_to_string(path)?;  // io::Error -> ConfigError via #[from]
    let cfg = toml::from_str(&text)?;           // same for the parse error
    Ok(cfg)
}

...

match load("app.toml") {
    Err(ConfigError::NotFound { .. }) => create_default_config(),
    Err(e) => return Err(e),
    Ok(cfg) => cfg,
}
```

### Anyhow

Anyhow - One error type that swallows everything!
The `anyhow::Error` is a single opaque type that any `std::error::Error + Send + Sync + 'static` can convert into. So `?` just works on anything, everywhere, with no `From` impls and no enum to maintain.

```rust
use anyhow::{Context, Result};  // anyhow::Result<T> = Result<T, anyhow::Error>

fn main() -> Result<()> {
    let text = std::fs::read_to_string("app.toml")
        .context("reading app.toml")?;
    let cfg: Config = toml::from_str(&text)
        .context("parsing app.toml")?;
    run(cfg)
}
```

### When to use each

The conventional rule is *libraries use `thiserror`, binaries use `anyhow`*

### The `?` Operator

The `?` - *If this is `Ok`, give me the value inside, If it's `Err`, stop here and return that error from the whole function.*

it's must be called inside a function that returns a `Result`.

Here is two "identical" functions.. One without `?` and one with...

```rust
fn read_config() -> Result<String, io::Error> {
    let text = match std::fs::read_to_string("app.toml") {
        Ok(t) => t,
        Err(e) => return Err(e),
    };
    Ok(text)
}
```

```rust
fn read_config() -> Result<String, io::Error> {
    let text = std::fs::read_to_string("app.toml")?;
    Ok(text)
}
```

Basically the `?` is the `match` in this context..

NOTE: The `?` is an early return. It doesn't *"handle"* the error,it forwards it upwards. So function containing the `?` must itself return a `Result` (Or `Option`), because `?` needs somewhere to return the error to.

Can think of `?` as *"or bail"* (If it works, fine, if not BAIL! return error and break...)

## The `&str` vs `String` vs `str`

We can think of them as following:

1. `str` - ***it's the data itself.***  A rn of UTF-8 bytes sitting somewhere in memory, size is unknown at compile time, so you cant put a bare `str` in a variable or pass one to a function. Only be touched via a pointer / reference. It's the *abstract* of *"Some text lives here"* not something you can hold!
2. `&str` - ***Is a borrowed view of the data***. It's a pointer + data length - it says *"test starts at this address and runs for N bytes"*. It's doesn't own the bytes, can't resize them, and can't outlive them. - Like python `memoryview` over text (?!?)
3. `String` - ***Owns a heap buffer***. Pointer, length and capacity. It allocated the bytes, it frees them when it goes out of scop, and it can grow!

```rust
let owned: String = String::from("hello");  // owns a heap allocation
let view:  &str   = &owned;                 // borrows it, no allocation
let lit:   &str   = "hello";                // points into the binary itself
```

NOTE: In rust, *string literals* aren't `String`. They are `&'static str` pointing at bytes baked into your executable, which is why you can't mutate them.

**Rule of thumb**: Take `&str` in a function parameters, return `String` when you are producing new text.

```rust
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

let s = String::from("Ada");
greet(&s);       // &String auto-coerces to &str via Deref
greet("Grace");  // literal works too
```

Taking `&str` means callers can pass either kind without cloning. If you took `String` instead, every caller with a literal would have to allocate it just to call you!

Few cavitates

```rust
s[0]; // Doesn't compile, UTF-8 is variable-width. For specific chars use `s.chars().nth(0)`. And `s.len()` returns a byte-count, not character count!
```

The mental shift from Python is that String vs &str isn't about the text at all — it's about who is responsible for freeing the memory. Python's garbage collector made that question invisible; Rust puts it in the type.

## Traits - `Clone` VS `Copy`

Unlike regular languages with `GC` (python for example) this code raises an error...

```rust
let a = String::from("hi");
let b = a;
println!("{a}");  // error: value borrowed here after move
```

The *assignment* is basically an `move` of the actual object (and it's ownership), not a copy / multi-owners to the same data...

The `Copy` Traits - *Duplication is a memcpy, so do it implicitly*. It's a marker trait with no methods. If a type is `Copy`, assignment and passing by value **duplicates** the bits instead of moving, and the original stays valid!

```rust
let a: i32 = 5;
let b = a;
println!("{a}");  // fine, i32 is Copy
```

Some common types that implement this `Copy` trait - `i32`, `bool`, `char`, `f64`, shared references `&T`, tuples / arrays of `Copy` implementing types.

Common though line is that where the value is entirely *"self contained"* (No heap pointer, nothing to free) it's a `Copy` implementing type.

The `Clone` Trait - *Duplication might be expensive, so ask for it explicitly*. It's a real trait (with method you must implement - `clone()`)

```rust
let a = String::from("hi");
let b = a.clone();   // new heap allocation
println!("{a} {b}"); // both alive -> Represent 2 different string in HEAP
```

The two are linked `trait Copy: Clone` - *Every `Copy` type must also be `Clone`, which is why you almost always see them derived together.*

```rust
#[derive(Clone, Copy)]
struct Point { x: f64, y: f64 }

#[derive(Clone)]           // Copy impossible: String owns a heap buffer
struct User { name: String, age: u32 }
```

You can't derive Copy if any field isn't Copy, and you can't implement both Copy and Drop on the same type. A destructor implies there's something to clean up, which implies copying the bits is wrong.

### Problematic Part

`.clone()` as a borrow-checker crutch - This is the big problem. You have hit a *cannot borrow* error, you just put `.clone()`, the compiler shuts up and you are happy. This sleazy trick has great performance cost! - If you have used `.clone()` just ask *Do I really need a second copy of the data? Or just a small restructure of who owns the data?*

```rust
#[derive(Clone, Copy)]
struct Counter { n: u32 }

fn bump(mut c: Counter) { c.n += 1; }   // mutates a copy, does nothing

let mut c = Counter { n: 0 };
bump(c);
assert_eq!(c.n, 0);  // passes
```

the fix is passing `&mut c` to the `bump` function

***`Copy` is a public API promise*** - Once a struct is `Copy`, callers write code that relies on teh values staying useable after being passed along (pass-by-value, not pass-by-reference). Adding a `Clone` is cheap; Adding `Copy` is a commitment!

Rule Of Thumb: Derive `Clone` on nearly everything, derive `Copy` only on small, plain-data types where a bitwise duplicate is  obviously the right meaning.

## Closures - Use of `MAP`

TBD
