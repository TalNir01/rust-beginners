# Rust-Beginners

A follow-up of a great rust introduction course.

Course [rust-from-scratch](https://turbofishandchips.com/rust-from-scratch/index.html)

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
    2. Using of `map`on `Result` / `Option`
5. Iterators
    1. `and_than`
    2. `collect`
    3. `copied` - Uses `copy`?
6. Data Sharing - `Box`
    1. Like `dyn`
    2. Rc / Arc / RefCell / &* / Guards (Smart Pointer)
7. Lifetime
8. Threads
    1. Scope VS Spawn

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

### What is closure?

A closure is an anonymous function that can capture variables from the scope where it was created (a bit like python `lambda` functions)

```python
factor = 3
triple = lambda x: x * factor   # captures `factor`
print(triple(5))                # 15
```

And the rust equivalent

```rust
let factor = 3;
let triple = |x| x * factor;    // captures `factor`
println!("{}", triple(5));      // 15
```

The syntax works likethis. Parameters go between the pipes `|...|` instead of after `lambda`; Types are usually inferred, but you can write them `|x: i32| -> i32 {x * factor}`. Unlike python's lambda, which is limited to a single expression, a RUST closure can contain a full block!

Unlike lambdas, rust `closures` are like nested `def` functions.

### How Variables Are Captured

This is the unique thing that isn't like `closure`.

```rust
// Example 1 - `Fn` - Borrow Immutably
/// The closure only reads the variable. You can call it as many times as you like.
let count = 0;
let show = || println!("{count}");
show();
show();

// Example 2 - `Fn` - Borrow Immutably
/// The borrow checker also prevents the kind of surprise Python allows:
let mut count = 0;
let show = || println!("{count}");
count += 1;   // ❌ compile error: `count` is borrowed by `show`
show();

// Example 3 - `FnMut` - Borrow Mutably
/// The closure changes the variable. The closure itself must be declared `mut`.
let mut total = 0;
let mut add = |x: i32| total += x;
add(5);
add(10);
println!("{total}"); // 15

// Example 4 - Take ownership (`FnOnce`)
/// The closure consumes a captured value, so it can only be called once
let name = String::from("Ada");
let consume = move || name;   // moves `name` out when called
let n = consume();
// consume();  // ❌ error: closure already used up its `name`
```

**The `move` keyword**
A `move` forces the closure to take **ownership** of everything it captures, **even if it only reads it.** You need this when the closure must outlive the current scope, and the classic case is threads...

```rust
use std::thread;

let data = vec![1, 2, 3];
let handle = thread::spawn(move || data.iter().sum::<i32>());
println!("{}", handle.join().unwrap()); // 6
// `data` is no longer usable here, because the thread owns it now
```

Without `move`, the compiler refuses, because the thread might outlive `data`. In python this would just work and you'd find out about any sharing bugs at runtime.

***Rule Of Thumb*** - `Fn` < `FnMut` < `FnOnce`. A function that accepts FnOnce accepts any closure. A function that accepts Fn only accepts closures that don't mutate or consume.

### Iterators & `MAP`

In `python` there is a magic function named `map` - applies a function to every element of one or more iterables and returns a map object (iterator) containing the transformed results.

In rust the `map` is quite similar, and it's uses `closures` to define the `transformation` function...

```rust
let nums = vec![1, 2, 3, 4];
let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
```

The rust line has 3 parts:

1. A `.iter()` - Creates an iterator over the vector.
2. A `.map(|x| x * 2)` Describes a transformation. Nothing runs yet.
3. A `.collect()` - Actually runs the pipeline and builds a collection

***Laziness*** - Nothing happens until jyou *"consume"*.

A `map` only builds a recipe. You need a ***consumer*** like `collect()`, `sum()`, `count()`, `for_each()`, or a `for` loop to execute it.

Unlike regular python which doesn't really care about the `iteration flow` but rust does

| Method | You Get | Original Collection Afterward |
| ------ | ------- | ----------------------------- |
| `.iter()` | `&T` (Read-only references) | Still usable |
| `.iter_mut()` | `&mut T` (Can modify in place) | Still usable, modified |
| `.into_iter()` | `T` (Owned values) | Consumed, gone |

```rust
let mut v = vec![1, 2, 3];
v.iter_mut().for_each(|x| *x *= 10);      // v is now [10, 20, 30]

let words = vec![String::from("a"), String::from("b")];
let upper: Vec<String> = words.into_iter().map(|s| s.to_uppercase()).collect();
// `words` can't be used anymore
```

Telling `collect` what to build It's can findout that data strcture it should build based on annotation / "turbofish" syntax

```rust
// Type Annotation
let upper: Vec<String> = words.into_iter().map(|s| s.to_uppercase()).collect();

// Turbofish Syntax
let b = nums.iter().map(|x| x + 1).collect::<Vec<i32>>();
let c = nums.iter().map(|x| x + 1).collect::<Vec<_>>(); // let Rust infer the element type
```

Here is the mapping

```rust
let words = vec!["apple", "kiwi", "banana", "fig", "cherry"];

let result: Vec<String> = words
    .iter()
    .filter(|w| w.len() > 3)          // keep longer words
    .enumerate()                      // add an index
    .map(|(i, w)| format!("{i}: {}", w.to_uppercase()))
    .collect();

// ["0: APPLE", "1: KIWI", "2: BANANA", "3: CHERRY"]
```

```rust
let maybe_name: Option<&str> = Some("Ada");
let len: Option<usize> = maybe_name.map(|s| s.len());   // Some(3)

let nothing: Option<&str> = None;
let len2 = nothing.map(|s| s.len());                    // None, no crash
```

*The one-sentence summary for your team: a Rust closure is a Python nested function whose captured variables follow ownership rules, and map and its relatives are Python comprehensions that are lazy, type-checked, and compile down to loop-speed code, but they're a tool for transforming data, not a replacement for every loop.*

### MAP @ Result

Results as we know is rust exception concept (Something either succeeded "OK" or failed "ERR")

So when we use `map` on a `result` it's only act on the value *inside* the `Ok`, and leaves an `Err` completely untouched

```rust
let doubled = "21".parse::<i32>().map(|n| n * 2);   // Ok(42)
let failed  = "abc".parse::<i32>().map(|n| n * 2);  // Err(ParseIntError { .. })
```

A helpful way to picture it is two parallel tracks. Values on the `Ok` track get transformed by each `map`. Once something is on the `Err` track, it slides past every `map` without being touched:

```rust
let result = "5".parse::<i32>()
    .map(|n| n * 10)          // Ok(50)
    .map(|n| n + 1)           // Ok(51)
    .map(|n| n.to_string());  // Ok("51")
```

**The mental model to take away:** map works on the happy path and lets errors pass through untouched. Use map_err to reshape errors, and_then when the next step can also fail, and ? when the chain gets long enough that plain statements read better.

## Iterators

The `and_than`, `collect` and `copied`

### Big Picture

Python iterators are mot similar to Rust's than you might expect.
Rust uses a `traits` to implement the iterators property.

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;  // `None` means "done", no exception
}

...

v.iter().map(|x| println!("{x}"));  // warning: iterators are lazy and do nothing unless consumed
```

The one real difference, and the reason copied exists at all, is ownership. In Python, every variable is a reference to an object, so you never ask "am I getting the value or a pointer to it?" In Rust, a Vec can give you three kinds of iterator:

```rust
v.iter()       // yields &T      (borrow each element, read-only)
v.iter_mut()   // yields &mut T  (borrow each element, can modify)
v.into_iter()  // yields T       (take ownership; v is consumed and gone afterward)
```

### And_Then

**How it solves it:** `and_then` says *"if I have a value, feed it into the next step, and that step may itself return `None`. If I already have `None`, skip everything and stay `None`."*

```rust
fn parse_positive(s: &str) -> Option<i32> {
    s.parse::<i32>().ok()                                    // Option<i32>
        .and_then(|n| if n > 0 { Some(n) } else { None })   // still Option<i32>, not Option<Option<i32>>
}
```

The key point is the flattening. If you used `map` with a closure that returns an `Option`, you'd get `Option<Option<i32>>`. `and_then` gives you a single, **flat `Option`.**

### Collect

`collect` - Turning a *lazy* iterator into an actual container.

***The problem:*** Since iterators are lazy, at some point you need real data you can store, index, or return.

***How It Solves It:*** `collect()` runs the iterator to completion and builds a collection.  You must chose the type target explicitly!

```rust
// Via Annotations
let v: Vec<i32>         = (1..4).collect();
let s: HashSet<i32>     = (1..4).collect();
let text: String        = vec!['h', 'i'].into_iter().collect();
let m: HashMap<&str, i32> = vec![("a", 1), ("b", 2)].into_iter().collect();
// or the "turbofish" syntax:
let v = (1..4).collect::<Vec<i32>>();
```

This works for any type that implements the `FromIterator` trait.

You can also use `collect` to *"fold"* all the `Results` into a single results, a *"all or nothing"* mentality!

```rust
let all: Result<Vec<i32>, _> = ["1", "2", "3"].iter().map(|s| s.parse::<i32>()).collect();
// Ok([1, 2, 3])

let bad: Result<Vec<i32>, _> = ["1", "x", "3"].iter().map(|s| s.parse::<i32>()).collect();
// Err(ParseIntError); stops at the first failure
```

### `copied` - Turn `&T` into `&`

The `copied` basically turns `&T` into `T` by doing a `Copy`.
The original problem:

```rust
let v = vec![1, 2, 3, 4];
let evens: Vec<i32> = v.iter().filter(|x| *x % 2 == 0).collect();
// ERROR: a Vec<i32> cannot be built from an iterator over &i32
```

A `v.iter()` yields references (`&i32`), so the natural result is a `Vec<i32>`, meaning *"a list of pointers into `v`"*.  

```rust
// Solution
let evens: Vec<i32> = v.iter().copied().filter(|x| x % 2 == 0).collect();
// [2, 4]
```

***What it's doesn't solve?:***

* It only works for `Copy` types.

## Summary

* A `and_than` chains steps that might produce nothing, without nesting. It's on `Option` / `Result`, not iterators versions are `flat_map` and `filter_map`. It's doesn't tell why something failed.

* A `collect` makes a lazy iterator actually run and builds any container you name. It can't infer the container type on its own, and it costs an allocation.

* A `copied` turns borrowed values into  owned copies for cheap types. It fails for types like `String`; use `cloned` or `into_iter` there.

## Data-Sharing & Smart-Pointers

One sentence -
Rust gives you none of that by default. A value has one owner, lives wherever you put it (usually the stack), and you can have either many readers or one writer, never both. Smart pointers are how you opt back into specific pieces of Python's behavior, one piece at a time, paying only for what you use:

1. `Box<T>`: "put this on the heap"
2. `Rc<T>`: "let multiple owners share this" (reference counting, like Python)
3. `RefCell<T>`: "let me mutate this even through shared access" (checked at runtime)
4. `Weak<T>`: "point at this without keeping it alive" (like Python's weakref)
5. `Arc<T>` + `Mutex<T>`: the thread-safe versions of the above

What make a pointer *"Smart"*?

1. Having `Deref` - Lets it behave lie an reference, so `*my_box` and method calls pass through to inner value. This is why you can call `.len()` on a `Box<String>` directly
2. A `Drop` runs cleanup when the owner goes out of scope.

### Box - Heap allocation

The problem - Rust needs to know the size of every type at compile time. Some things don't have a known size. The classic is recursive type

```rust
enum List {
    Cons(i32, List), // ERROR - Recursive type has infinite size
    Nil
}
```

A `List` contains a `List`... The compiler canno't compute size for a recursive type! In python everything is a pointer.

How it solves this? A `BOX` is always pointer-size, whatever it points to, The actual data live son the heap!

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
```

The second big use is *trait objects*: a collection of different types that share behavior.

Let's say we have 2 objects (`Circle` & `Square`) that implements a `trait` that declares a `fn area(&self) -> ...`

```rust
trait Shape { fn area(&self) -> f64; }
struct Circle(f64);
struct Square(f64);
impl Shape for Circle { fn area(&self) -> f64 { 3.14159 * self.0 * self.0 } }
impl Shape for Square { fn area(&self) -> f64 { self.0 * self.0 } }

let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle(1.0)), Box::new(Square(2.0))];
for s in &shapes {
    println!("{}", s.area());
}
```

### Rc<T>

The `Rc<T>` shared ownership through reference counting
The problem: Sometimes a value genuinely has multiple owners and no single of tem should decide when it dies. With plain ownership, you'd have to pick one onerand make everyone else *borrow*, aht the borrow checker often won't let those borrows live long enough.

How it solves it: `Rc` ("Reference Counted") is exactly what python does to every object. Each `Rc::clone` bumps a counter; each drops decrements it; When it's hits zero, the value is freed!!!

```rust
use std::rc::Rc;

let a = Rc::new(String::from("shared config"));
let b = Rc::clone(&a);  // does NOT copy the string, just bumps the count
let c = Rc::clone(&a);

println!("{}", Rc::strong_count(&a));  // 3
drop(b);
println!("{}", Rc::strong_count(&a));  // 2
```

Here is the same idea in python (native)

```python
import sys
a = "shared config"
b = a  # sys.getrefcount(a) goes up
```

We should use `Rc::clone(&a)` over `a.clone()` - It's just a visual hint but it's help prevent confusion.

What it doesn't solve:

* Mutation - The `RC` only gives you shared **read** access.

Many owners plus mutation is exactly the situation the borrow check exists to prevent.

To gather the POWER you should use `RefCell`.

### RefCell<T>

A `RefCell<T>` - Mutation checked at runtime instead of compile time.

How is it being solved? `RefCell` moves the borrowing rules ("Many readers or one writer") from compile time to runtime.

```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2]);
data.borrow_mut().push(3);          // mutable access, even though `data` isn't `mut`
println!("{:?}", data.borrow());    // [1, 2, 3]
```

This is called ***interior mutability*** the outside looks immutable, but the inside can change.

The catch: break the rules, and instead of a compile error you get a `panic` (a crash) at runtime:

```rust
let data = RefCell::new(vec![1, 2]);
let reader = data.borrow();         // a reader is still alive...
data.borrow_mut().push(3);          // PANIC: already borrowed 
```

This bit me while experimenting. The fix is usually to make sure the first borrow ends before the second starts, for example by putting it in its own `{` `}` block or not storing it in a variable.

There's also a lightweight sibling, `Cell<T>`, for small `Copy` types like integers and bools. It never hands out references at all; you just .`get()` a copy and `.set()` a new value, so it can't panic:

```rust
use std::cell::Cell;
let counter = Cell::new(0);
counter.set(counter.get() + 1);
```

Let's say we will combine `Rc` (Multiple Owners) and `RefCell` (Interior Mutable) leads `Rc<RefCell<T>>`
The 2 gives you *shared ownership & mutation*. This is most literally, what every Python object is.

### Thread Safety

There are thread safe versions for these smart pointers

* A `Arc<T>` (*"Atomically reference counted"*) is `Rc` with an atomic counter, safe across threads.

* A `Mutex<T>` is `RefCell` for threads. Instead of panicking on conflicting access, it makes the other thread *wait* for its turn.

### Ownership VS Borrowing

**Core difference (analogy):**
Think of a library book

1. `The owner` bought the book. They decided when it gets thrown away, they can give it away permanently, and they can lend it out.
2. `A borrower` - has it temporarily. They must give it back, they can't throw it away, and they can't give it to someone else permanently!

Rust has another rule in addition of rules of libraries don't hvae. You can lend a book to ***many people to read at once***, or to ***exactly one person to write in***, but never both at the same time.

The 3 ownership rules:

1. Each value in `Rust` has an `owner` (a variable)
2. There can only be `one owner at a time`. **
    * Think of `Rc` as an exception to the rule!
3. When the owner goes out of scope, `the value is dropped` (aka *"freed"*)

**Moving**: giving ownership away!
In rust `let s2 = s1` means *"S1 gibes the string (object) to s2"*

**Borrowing**: Using data without taking it
If every function took ownership, you'd have to pass value in and return them back out constantly.
 It's being solved by handing out *reference* (`&`), and the owner keeps ownership.

There are 2 core types of borrow:

1. Shared borrow (read-only) - `&T`
Many can exist in once, nobody can modify the data while any of them is alive!

2. Exclusive borrow (read-write) - `&mut T`
Only one can exist at a time, and no shared borrows can exist alongside it.
Notice that `mut` appears in 3 places - the variable (`let mut s`), the parameter type (`&mut String`), and the call site (`&mut s`). Mutation is never hidden!

| Capability | Owner (`T`) | Shared Borrow (`&T`) | Exclusive borrow (`&mut T`) |
| ------ | ------ | ------ | ------ |
| Can read | Yes | Yes | Yes |
| Can modify | Yes (if `mut`) | No | Yes |
| Can give it away (`move`) | Yes | No | No |
| Responsible for `Dropping` It | Yes | No | No |
| How many at once | Exactly one | Many | Exactly one, with no `&T` alongside |
| Must end before | Its scope ends | The owner is gone | The owner is gone |

### The `dyn`

The `dyn Trait` means *"Some type that implements this trait, but I don't know which one until the program is running"*

`dyn` is used to holds a several **different** types in one collection.

With `dyn`, you say "a vector of things that can speak, whatever they are":

```rust
let animals: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Cat)];

for a in &animals {
    println!("{}", a.speak());   // Woof, then Meow
}
```

The `&dyn Speak` is actually 2 pointers stuck together, aka *"fat pointer"*

1. A pointer to **the data** (Actual dog / cat)
2. A pointer to a **vtable**: a small table, generated by the compiler, listing the method implementations for that concrete type!

## Lifetimes

A `lifetime` is the span of a program during which a reference is guaranteed to be valid.
Every reference in Rust has one. In most code the compiler infers lifetimes automatically,and developers can ignore them!

Lifetime annotations do not change how long any value lives. They describe relationships between references so that the compiler can verify those relationships are sound.

Basically it's a proof we write to the compiler

Actual problem:

```rust
let r;
{
    let x = 5;
    r = &x;
}                   // x is dropped here
println!("{r}");    // ERROR: `x` does not live long enough
```

Here is an example for a need of annotations

```rust
fn longest(a: &str, b: &str) -> &str {   // ERROR: missing lifetime specifier
    if a.len() >= b.len() { a } else { b }
}
```

The compiler rejects this signature because the returned reference could come from either `a` or `b`, depending on runtime values. Callers need to know how long the result remains valid, and the compiler checks each function using only its signature, not its body. The signature must therefore state the relationship explicitly.

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

This can be translated at *"For some lifetime `'a`, both inputs are valid for at least `'a`, and the output is valid for `'a` as well."*

In practice, `'a` resolves to the shorter of the two input lifetimes, since that is the only span during which both are guaranteed valid.
