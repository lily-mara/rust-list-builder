Rust List Builder
===

List/set/generator comprehensions are one of the best features of Python and
Haskell. They make creating complex iterables simpler and more intuitive. This
crate attempts to add similar functionality to the Rust language.

This crate provides the `gen!` macro. Its syntax is similar to Python list
comprehensions:

```rust
#[macro_use(gen)]
#[no_link]
extern crate list_builder;

fn main() {
	let x: Vec<i32> = gen![i*1000 => i in [1, 2, 3, 4, 5, 6]];
}
```

You can use conditionals just like in Python:

```rust
let x: Vec<i32> = gen![i*1000 => i in [1, 2, 3, 4, 5, 6], x % 2 == 0];
assert_eq!(x, vec![2000, 4000, 6000]);
```
