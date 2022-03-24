//! This crate intends to support basic matrix operations and arithmetic.
//! It is quite open in the choice of the underlying scalar type.
//!
//! For instance, the scalar type can be chosen to be anything that implements
//! common arithmetic traits like `Add`, `Sub`, `Div`, ...
//!
//! The two core types of this crate are `Matrix` and `Scalar`.
//! A matrix can created like this:
//!
//! ```
//! use matrix_algebra::matrix::Matrix;
//! let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
//! ```
//!
//! Common arithmetic operators are implemented on the type `Matrix`:
//!
//! ```
//! use matrix_algebra::matrix::Matrix;
//! let m1 = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
//! let m2 = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
//!
//! assert_eq!(m1 + m2, Matrix::new(vec![vec![0, 2], vec![4, 6]]))
//!
//!
pub mod matrix;
pub use matrix::scalar::Scalar;
pub use matrix::Matrix;
