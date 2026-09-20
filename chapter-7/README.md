# Chapter 7 - Error Handling & Options

## Options - Is it `None` or is it `Some`

Missing (optional) values -> Result of actions that might not return a value (`search` for example)

Most languages solves this with `Null / Nill etc..` but it's a RUNTIME error, not compile time.

Rust solves this with a special Enum (builtin) that looks like this:

```rust
pub enum Option<T> {
    None, // No data, random name
    Some(T) // Value `T`
}
```

It's a generic type that is either `None` or something (anything that isn't `None` -> aka **Some**thing )

## Panic - Unwind and clean exit

NOTE: Its for error and not like exception, it's an `halt` stage.

panic is NOT undefined behavior!

There is a *macro* of `panic!` which it's the easiest way to trigger a *panic* flow

```rust
fn main() {
    panic!("ARGGGH!");
}
```

Unlike the `option.unwrap` which raises panic if the `option` is `None`, there is `option.expect(msg)` which returns a `msg` and then panics

## Exceptions Handling

We have a `Result` (enum, stdlib)

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Enum that has 2 variants (success & error)
