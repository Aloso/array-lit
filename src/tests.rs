use crate::arr;
#[cfg(feature = "std")]
use crate::vec;

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
    assert_eq!(arr![4; 5; {}], [4; 5]);
    assert_eq!(arr![4; 5; { [0]: [] }], [4; 5]);
    assert_eq!(arr![4; 5; { 3: 3, 2: 2 }], [4, 4, 2, 3, 4]);
    assert_eq!(arr![3; 5; { [0]: [1, 2], 4: 4 }], [1, 2, 3, 3, 4]);
    assert_eq!(arr![0; 5; { [2]: [1, 2] }], [0, 0, 1, 2, 0]);
}

#[test]
#[cfg(feature = "std")]
fn test_advanced_vec_literals() {
    assert_eq!(vec![4; 5], std::vec![4; 5]);
    assert_eq!(vec![4; 5; { 3: 3, 2: 2 }], std::vec![4, 4, 2, 3, 4]);
    assert_eq!(vec![3; 5; { [0]: [1, 2], 4: 4 }], std::vec![1, 2, 3, 3, 4]);
}

#[test]
fn test_runtime_values() {
    assert_eq!(arr![4; 5; { [0]: [1; 3] }], [1, 1, 1, 4, 4]);
    let a = [1, 2, 3];
    assert_eq!(arr![0; 5; { [1]: a }], [0, 1, 2, 3, 0]);
    let a = &[1, 2, 3];
    assert_eq!(arr![0; 5; { [1]: a }], [0, 1, 2, 3, 0]);
}

#[test]
fn test_assignment_order() {
    assert_eq!(arr![4; 5; { [0]: [1, 2, 3], 1: 5 }], [1, 5, 3, 4, 4]);
}

#[test]
#[cfg(feature = "std")]
fn test_assignment_order_vec() {
    assert_eq!(
        vec![4; 5; { [0]: [1, 2, 3], 1: 5 }],
        std::vec![1, 5, 3, 4, 4]
    );
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

#[test]
#[cfg(feature = "std")]
fn test_custom_index() {
    #[derive(PartialEq, Debug, Copy, Clone)]
    struct S(bool);

    struct Idx(usize);

    impl core::ops::Index<Idx> for Vec<S> {
        type Output = bool;

        fn index(&self, index: Idx) -> &Self::Output {
            &self.get(index.0).unwrap().0
        }
    }

    impl core::ops::IndexMut<Idx> for Vec<S> {
        fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
            &mut self.get_mut(index.0).unwrap().0
        }
    }

    assert_eq!(
        vec![S(true); 4; { (Idx(3)): false }],
        vec![S(true), S(true), S(true), S(false)]
    );
}
