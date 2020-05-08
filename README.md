# array-lit

[Documentation](https://docs.rs/array-lit/0.1.0/array_lit/index.html) · [Crates.io](https://crates.io/crates/array-lit) · [Lib.rs](https://lib.rs/crates/array-lit)


[Rust](https://www.rust-lang.org/) macros for array and `Vec` literals with superpowers.
The macro for arrays is called `arr!`; the macro for `Vec`
is called `vec!` and shadows the macro from the standard library.

They allow creating an array or `Vec` where only some elements are specified,
and the rest is set to a default value:

```rust
let a = arr![1, 2, 3; default: 0; 8];
assert_eq!(a, [1, 2, 3, 0, 0, 0, 0, 0]);
```

You can also specify elements at specific array indices:

```rust
let a = arr![default: 1, 6: 0; 8];
assert_eq!(a, [1, 1, 1, 1, 1, 1, 0, 1]);
```

You can even specify the first _n_ elements as well as specific indices:

```rust
let a = arr![1, 2; default: 3, 6: 0; 8];
assert_eq!(a, [1, 2, 3, 3, 3, 3, 0, 3]);
```

They also support normal array syntax (`arr![a; N]` and `arr![a, b, c]`). This means that
the `vec!` macro from this crate is a drop-in replacement for `std::vec!`.

## `Copy` bound and default values

Unless the literal specifies all elements explicitly (e.g. `arr![1, 2, 3]`), the
type in the array or `Vec` must implement the `Copy` trait, and the literal must
specify a default value.

## How does it work?

The macros generate a block that first creates a array or `Vec`, and then
inserts the specified values:

```rust
arr![default: 4, 0: 0, 1: 1; 5]
// is expanded to
{
    let mut arr = [4; 5];  // in the vec! macro, std::vec! is used
    arr[0] = 0;
    arr[1] = 1;
    arr
}
```

## What about array lifetimes?

In trivial cases such as `arr![3; 5]`, the `'static` lifetime is inferred for array
literals. This means that they have the exact same behavior as normal array literals.

In the other cases, the `'static` lifetime is not inferred. This means that the
array literal is computed at runtime and doesn't have a fixed memory location.
It also means that the following

```rust
// does NOT compile!
fn return_temporary() -> &'static [i32; 4] {
    &arr![default: 0, 0: 1; 4]
}
```

produces `error[E0515]: cannot return reference to temporary value`. This can be solved
by assigning the literal to a `const` or `static` variable first:

```rust
fn return_temporary() -> &'static [i32; 4] {
    static ARR: &[i32; 4] = &arr![default: 0, 0: 1; 4];
    ARR
}
```

Note that `const` enforces **const evaluation**, which means that the whole array is
included in the application binary. This might not be desirable if the array is large.

## Order of assignments

When positional elements are combined with elements at specific indices, the positional
elements are assigned first, so they might be overwritten later:

```rust
let a = arr![1, 2; default: 1, 0: 0; 4];
assert_eq!(a, [0, 2, 1, 1]);
```

## Usage

If you use the 2018 edition, import the macros with

```rust
use array_lit::{arr, vec};
```

If you don't want to shadow `std::vec!`, you can rename the macro:

```rust
use array_lit::{arr, vec as vector};
```

If you're on the 2015 edition, or want to import the macros globally,
you can add this to your crate root instead:

```rust
#[macro_use]
extern crate array_lit;
```

## `no_std` support

This library supports `no_std`, if default features are disabled.
This makes the `vec!` macro unavailable.

## Minimum required Rust version

This library supports the macros in `const` and `static` contexts since
Rust 1.33. Without them, Rust 1.30 is supported (but 1 doc-test fails).

# License

Licensed under **MIT** or **Apache 2.0** at your choice.
