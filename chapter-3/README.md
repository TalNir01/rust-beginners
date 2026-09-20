# Chapter 3 - Data Ownership

Rust *borrowing* is defined by `&` (*Borrow Operator*, In other languages it's known as *reference operator*)
To manually force *dereference* we can use `*` before the variable / object.

Rust *slice* is *borrowing* only a part of the structure / object, not it entirety.
Unlike regular *pointers* rust *slice* is "fat" it's includes a `ptr` to the data and also the `data-len`
