# array-lit

[Documentation](https://docs.rs/array-lit/0/array_lit/index.html) · [Crates.io](https://crates.io/crates/array-lit) · [Lib.rs](https://lib.rs/crates/array-lit)


[Rust](https://www.rust-lang.org/) macros for array and `Vec` literals with superpowers.
The macro for arrays is called `arr!`; the macro for `Vec` is called `vec!`
and shadows the macro from the standard library.

They allow creating an array or `Vec` where only some elements are
specified, and the rest is set to a default value.

The following macro specifies three consecutive values at index 0:

```rust
let a = arr![0; 8; { [0]: [1, 2, 3] }];
assert_eq!(a, [1, 2, 3, 0, 0, 0, 0, 0]);
```

The square brackets are only needed when specifying multiple consecutive
elements:

```rust
let a = arr![1; 8; { 6: 0 }];
assert_eq!(a, [1, 1, 1, 1, 1, 1, 0, 1]);
```

You can specify as many single and consecutive values as you like:

```rust
let a = arr![0; 8; {
    6: 1,            // 1 at index 6
    [2]: [3, 4],     // 3 and 4 starting from index 2
    7: 5,            // 5 at index 7
}];
assert_eq!(a, [0, 0, 3, 4, 0, 0, 1, 5]);
```

The familiar array syntax (`arr![a; N]` and `arr![a, b, c]`) is also
supported, so the `vec!` macro from this crate is a drop-in replacement for
`std::vec!`.

## How does it work?

The macros generate a block that first creates a array or `Vec`, and then
inserts the specified values:

```rust
arr![4; 10; { 0: 0, 1: 1 }]
// is expanded to
{
    let mut arr = [4; 10];  // in the vec! macro, std::vec! is used
    arr[0] = 0;
    arr[1] = 1;
    arr
}
```

If an array is inserted that is not of the form `[a, b, c, ..]`, a loop is
used:

```rust
arr![4; 10; { [1]: [2; 4] }]
// is expanded to
{
    let mut arr = [4; 10];
    let mut i = 1;
    let end = i + 4;
    while i < end {
        arr[i] = 2;
        i += 1;
    }
    arr
}
```

This even works for slices, arrays and `Vec`s created at runtime:

```rust
let my_slice = &[1, 2, 3, 4];
arr![4; 10; { [1]: my_slice }];
```

## What about array lifetimes?

In trivial cases such as `arr![3; 5]`, the `'static` lifetime is inferred
for array literals. This means that they have the exact same behavior as
normal array literals.

In the other cases, the `'static` lifetime is not inferred. This means that
the array literal is computed at runtime and doesn't have a fixed memory
location. It also means that the following

```rust
// does NOT compile!
fn return_temporary() -> &'static [i32; 4] {
    &arr![0; 4; { 0: 1 }]
}
```

produces `error[E0515]: cannot return reference to temporary value`. This
can be solved by assigning the literal to a `const` or `static` variable
first:

```rust
fn return_temporary() -> &'static [i32; 4] {
    static ARR: &[i32; 4] = &arr![0; 4; { 0: 1 }];
    ARR
}
```

Values assigned to a `static` or `const` variable must be constant. Due to
limitations in the compiler, macros that expand to loops aren't allowed
there:

```rust
// does NOT compile!

const ARR: [i32; 4] = arr![0; 16; { [0]: [1; 8] }];
// this is expanded to a loop ~~~~~~~~~~~^^^^^^
```

Note that `const` enforces **const evaluation**, which means that the whole
array is included in the application binary. This might not be desirable if
the array is large.

## Usage

Import the macros with

```rust
use array_lit::{arr, vec};
```

If you don't want to shadow `std::vec!`, you can rename the macro:

```rust
use array_lit::{arr, vec as vector};
```

Importing the macros globally is also supported, although not recommended:

```rust
#[macro_use]
extern crate array_lit;
```

## Custom indices

If you want to use your own `Index`/`IndexMut` implementation in these
macros, you probably need an extra pair of parentheses:

```rust
#[derive(Copy, Clone)]
struct S(bool);

/// Your custom index
struct Idx(usize);

impl std::ops::Index<Idx> for Vec<S> {
    type Output = bool;
    // etc.
}

impl std::ops::IndexMut<Idx> for Vec<S> {
    // etc.
}

vec![S(true); 1000; { (Idx(16)): false }];
// parens needed ~~~~~^~~~~~~~^
```

## `no_std` support

This library supports `no_std`, if default features are disabled.
This makes the `vec!` macro unavailable.

## Minimum required Rust version

Requires Rust 1.33.

# License

Licensed under **MIT** or **Apache 2.0** at your choice.
