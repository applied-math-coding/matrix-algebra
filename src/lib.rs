//! This crate intends to support basic matrix operations and arithmetic.
//! It is quite open in the choice of the underlying scalar type.
//!
//! For instance, the scalar type can be chosen to be anything that implements
//! common arithmetic traits like `Add`, `Sub`, `Div`, ...
//! <br>
//! <br>
//! The core types of this crate are `Matrix`, `Scalar` and `Func`.
//! A matrix can created like this:
//!
//! ```
//! use matrix_algebra::Matrix;
//!
//! let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
//! ```
//! <br>
//! <br>
//! Common arithmetic operators are implemented on the type `Matrix`:
//!
//! ```
//! use matrix_algebra::Matrix;
//!
//! let m1 = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
//! let m2 = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
//!
//! assert_eq!(m1 + m2, Matrix::new(vec![vec![0, 2], vec![4, 6]]));
//! ```
//! <br>
//! <br>
//! Also, it is possible to apply functions wrapped by `Func` element-wise
//! on a matrix:
//!
//! ```
//! use matrix_algebra::{Matrix, Func};
//!
//! let sin = Func::new(|x| f64::sin(x));
//! let m = Matrix::new(vec![vec![0.0, 1.0], vec![2.0, 3.0]]);
//!
//! assert_eq!(
//!   sin%m,
//!   Matrix::new(
//!    vec![vec![f64::sin(0.0), f64::sin(1.0)], vec![f64::sin(2.0), f64::sin(3.0)]]
//!   )
//! );
//! ```
//! <br>
//! <br>
//! Scalars can by multiplied from left:
//!
//! ```
//! use matrix_algebra::{Matrix, Scalar};
//!
//! let m = &Matrix::new(vec![vec![0, 1], vec![2, 3]]);
//! let s = Scalar(2);
//!
//! assert_eq!(s*m, m*2);
//!
//! ```
//!
pub mod matrix;
pub use matrix::func::Func;
pub use matrix::scalar::Scalar;
pub use matrix::Matrix;
