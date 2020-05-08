//! A macro for array literals with superpowers.
//!
//! It allows creating an array where only some elements are specified,
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
//! You can even specify the first _n_ elements as well as specific array indices:
//!
//! ```rust
//! # use array_lit::arr;
//! let a = arr![1, 2; default: 3, 6: 0; 8];
//! assert_eq!(a, [1, 2, 3, 3, 3, 3, 0, 3]);
//! ```
//!
//! It also supports normal array syntax. I. e. `arr![t; N]` is equivalent to `[t; N]`,
//! and `arr![a, b, c]` is equivalent to `[a, b, c]`.
//!
//! ## `Copy` bound and default values
//!
//! Unless the literal specifies all elements explicitly (e.g. `arr![1, 2, 3]`), the
//! type inside the array must implement the `Copy` trait, and the literal must
//! specify a default value.
//!
//! ## Order of assignment
//!
//! When positional elements are combined with elements at specific indices, the positional
//! elements are assigned first, so they can be overwritten by elements at specific indices:
//!
//!```rust
//! # use array_lit::arr;
//! let a = arr![1, 2; default: 1, 0: 0; 4];
//! assert_eq!(a, [0, 2, 1, 1]);
//! ```
//!
//! ## Usage
//!
//! If you use the 2018 edition, import it with
//!
//! ```rust
//! use array_lit::vec;
//! ```
//!
//! If you're on the 2015 edition, or want to import the macro globally,
//! you can add this to your crate root instead:
//!
//! ```rust
//! #[macro_use]
//! extern crate array_lit;
//! ```
//!

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple_literals() {
        assert_eq!(arr![3; 5], [3; 5]);
        assert_eq!(arr![5, 4, 3, 2, 1], [5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_advanced_literals() {
        assert_eq!(arr![default: 4; 5], [4; 5]);
        assert_eq!(arr![default: 4, 3: 3, 2: 2; 5], [4, 4, 2, 3, 4]);
        assert_eq!(arr![1, 2; default: 3, 4: 4; 5], [1, 2, 3, 3, 4]);
    }

    #[test]
    fn test_assignment_order() {
        assert_eq!(arr![1, 2, 3; default: 4, 1: 5; 5], [1, 5, 3, 4, 4]);
    }

    #[test]
    fn test_non_copy_types() {
        #[derive(PartialEq, Debug)]
        struct X; // does NOT implement Copy

        assert_eq!(arr![X, X, X], [X, X, X]);
    }
}
