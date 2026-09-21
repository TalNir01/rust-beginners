# Chapter 11 - Iterators & Functional Programming

## Iterators

It's convert an type to a type that you can *iterate* over!

The `iterator` is basically a `trait` that object can implement.

Rhw only must method that we must implement is `fn next(&self) -> ...`

Another useful *trait* is the `IntoIterator` which supply the `.into_iter()` function which returns an Iterator from the object.

```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    // required method
    fn into_iter(self) -> Self::IntoIter
}
```

Why iterators are so usefull? because they supply a vast range of pre-built / implemented default method that we can use to have fun...
 (Map, take, skip...)
