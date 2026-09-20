# Chapter 4 - Strings & Text

## Encoding

*Encoding* - Mapping a numeric value to a character

Notable options:

* ASCII (Up to 7 bits) ~ 1 byte (8 bits)
* Unicode (Up 21 bits) ~ 4 byte (32) - `char`

utf-8 implement's both(?)

`&str` -> Rust's *string* slice (Due to rust using variable-length it can't just be an array of `chars`)
