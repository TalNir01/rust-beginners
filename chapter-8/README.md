# Chapter 8 - Traits & Generic Types

## Generic Types

Let's say we have an object / structure, we don't want to "define" the data

Problem? We can't do "actions" (like `+`) for type `T`, we must be able to "proof" the compiler that the type `T` actually implements the `+` action.

This thing called *Trait*

## Traits

A property of the types that makes sure the type `T` actually implements (supports) certain collection of actions.

It's a more powerful cousin of java `interfaces`

If we want to implement a function that requires it's `T` to implement certain Traits (Like `Drawable`, `HasArea` and such) we can write like this:

```rust
trait HasArea {
    fn area(&self) -> f32;
}

impl HasArea for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius.powf(2.0) // Area of a circle is π * r^2
    }
}

fn print_area<T>(shape: T)
where T: HasArea + Drawable 
{
    ...
    ...
}

fn create_drawable<T: Drawable>() -> T {
    fn create_default()
}
```

## Dynamic Dispatch
