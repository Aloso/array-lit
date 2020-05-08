#![cfg_attr(not(feature = "std"), no_std)]

//! Macros for array and `Vec` literals with superpowers.
//! The macro for arrays is called `arr!`; the macro for `Vec`
//! is called `vec!` and shadows the macro from the standard library.
//!
//! They allow creating an array or `Vec` where only some elements are specified,
//! and the rest is set to a default value:
//!
//! ```rust
//! # use array_lit::arr;
//! let a = arr![1, 2, 3; default: 0; 8];
//! assert_eq!(a, [1, 2, 3, 0, 0, 0, 0, 0]);
//! ```
//!
//! You can also specify elements at specific array indices:
//!
//! ```rust
//! # use array_lit::arr;
//! let a = arr![default: 1, 6: 0; 8];
//! assert_eq!(a, [1, 1, 1, 1, 1, 1, 0, 1]);
//! ```
//!
//! You can even specify the first _n_ elements as well as specific indices:
//!
//! ```rust
//! # use array_lit::arr;
//! let a = arr![1, 2; default: 3, 6: 0; 8];
//! assert_eq!(a, [1, 2, 3, 3, 3, 3, 0, 3]);
//! ```
//!
//! They also support normal array syntax (`arr![a; N]` and `arr![a, b, c]`). This means that
//! the `vec!` macro from this crate is a drop-in replacement for `std::vec!`.
//!
//! ## `Copy` bound and default values
//!
//! Unless the literal specifies all elements explicitly (e.g. `arr![1, 2, 3]`), the
//! type in the array or `Vec` must implement the `Copy` trait, and the literal must
//! specify a default value.
//!
//! ## Order of assignment
//!
//! When positional elements are combined with elements at specific indices, the positional
//! elements are assigned first, so they might be overwritten later:
//!
//!```rust
//! # use array_lit::arr;
//! let a = arr![1, 2; default: 1, 0: 0; 4];
//! assert_eq!(a, [0, 2, 1, 1]);
//! ```
//!
//! ## Usage
//!
//! If you use the 2018 edition, import the macros with
//!
//! ```rust,ignore
//! use array_lit::{arr, vec};
//! ```
//!
//! If you don't want to shadow `std::vec!`, you can rename the macro:
//!
//! ```rust,ignore
//! use array_lit::{arr, vec as vector};
//! ```
//!
//! If you're on the 2015 edition, or want to import the macros globally,
//! you can add this to your crate root instead:
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate array_lit;
//! ```
//!
//! ## `no_std` support
//!
//! This library supports `no_std`, if default features are disabled.
//! This makes the `vec!` macro unavailable.
//!
//! ## Minimum required Rust version
//!
//! This library should support fairly old compiler versions,
//! although I haven't tested it yet.

/// A macro for array literals with superpowers.
///
/// See [the module level documentation](index.html) for more.
///
/// # Example
///
///```rust
/// # use array_lit::arr;
/// let a = arr![1, 2; default: 1, 4: 0; 5];
/// assert_eq!(a, [1, 2, 1, 1, 0]);
/// ```
#[macro_export]
macro_rules! arr {
    [default: $def:expr $(, $idx:literal : $sparse:expr )* ; $len:expr] => {
        {
            #[allow(unused_mut)]
            let mut arr = [$def ; $len];
            $( arr[$idx] = $sparse; )*
            arr
        }
    };
    [$( $item:expr ),* ; default: $def:expr $(, $idx:literal : $sparse:expr )* ; $len:expr] => {
        {
            #[allow(unused_mut, unused_assignments)]
            {
                let mut arr = [$def ; $len];
                let mut i = 0;
                $(
                    arr[i] = $item;
                    i += 1;
                )*
                $( arr[$idx] = $sparse; )*
                arr
            }
        }
    };
    [$item:expr ; $len:expr] => {
        [$item ; $len]
    };
    [$( $item:expr ),* $(,)?] => {
        [ $($item),* ]
    };
}

/// A macro for `Vec` literals with superpowers.
///
/// See [the module level documentation](index.html) for more.
///
/// > This macro requires the **`std`** feature (enabled by default)
///
/// # Example
///
///```rust
/// # use array_lit::vec;
/// let a = vec![1, 2; default: 1, 4: 0; 5];
/// assert_eq!(a, std::vec![1, 2, 1, 1, 0]);
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! vec {
    [default: $def:expr $(, $idx:literal : $sparse:expr )* ; $len:expr] => {
        {
            #[allow(unused_mut)]
            let mut arr = std::vec![$def ; $len];
            $( arr[$idx] = $sparse; )*
            arr
        }
    };
    [$( $item:expr ),* ; default: $def:expr $(, $idx:literal : $sparse:expr )* ; $len:expr] => {
        {
            #[allow(unused_mut, unused_assignments)]
            {
                let mut arr = std::vec![$def ; $len];
                let mut i = 0;
                $(
                    arr[i] = $item;
                    i += 1;
                )*
                $( arr[$idx] = $sparse; )*
                arr
            }
        }
    };
    [$item:expr ; $len:expr] => {
        std::vec![$item ; $len]
    };
    [$( $item:expr ),* $(,)?] => {
        std::vec![ $($item),* ]
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple_literals() {
        assert_eq!(arr![3; 5], [3; 5]);
        assert_eq!(arr![5, 4, 3, 2, 1], [5, 4, 3, 2, 1]);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_simple_vec_literals() {
        assert_eq!(vec![3; 5], std::vec![3; 5]);
        assert_eq!(vec![5, 4, 3, 2, 1], std::vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_advanced_literals() {
        assert_eq!(arr![default: 4; 5], [4; 5]);
        assert_eq!(arr![default: 4, 3: 3, 2: 2; 5], [4, 4, 2, 3, 4]);
        assert_eq!(arr![1, 2; default: 3, 4: 4; 5], [1, 2, 3, 3, 4]);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_advanced_vec_literals() {
        assert_eq!(vec![default: 4; 5], std::vec![4; 5]);
        assert_eq!(vec![default: 4, 3: 3, 2: 2; 5], std::vec![4, 4, 2, 3, 4]);
        assert_eq!(vec![1, 2; default: 3, 4: 4; 5], std::vec![1, 2, 3, 3, 4]);
    }

    #[test]
    fn test_assignment_order() {
        assert_eq!(arr![1, 2, 3; default: 4, 1: 5; 5], [1, 5, 3, 4, 4]);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_assignment_order_vec() {
        assert_eq!(vec![1, 2, 3; default: 4, 1: 5; 5], std::vec![1, 5, 3, 4, 4]);
    }

    #[test]
    fn test_non_copy_types() {
        #[derive(PartialEq, Debug)]
        struct X; // does NOT implement Copy

        assert_eq!(arr![X, X, X], [X, X, X]);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_non_copy_types_vec() {
        #[derive(PartialEq, Debug)]
        struct X; // does NOT implement Copy

        assert_eq!(vec![X, X, X], std::vec![X, X, X]);
    }
}
