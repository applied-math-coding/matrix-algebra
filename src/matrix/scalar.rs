/// The struct `Scalar` is intended to wrap scalar types like `f64`
/// in order to allow the implementation of arithmetic operations between matrix types.
#[derive(PartialEq, Debug, Clone)]
pub struct Scalar<T>(pub T);
