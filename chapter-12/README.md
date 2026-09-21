# Chapter 12 - Data Sharing

Function `std::mem::size_of::<T>()` -> Amount of bytes of `T`

Dynamic Memory - `Box<T>`

`Box::new(T)` - Allocate dynamic memory of object at memory, and return a pointer!

Box is the single owner of the data it's hold a pointer to.
It's incharge of `drop` or other managerial duities of the owned data (even when it allocated on the heap).

Rc - Smart pointer that tracks amount of subscribers to it, like box it's uses dynamic memory!
