pub struct Func<T: Copy + PartialEq>(pub Box<dyn Fn(T) -> T>);

/// Intended to wrap closures and function pointers for making them
/// element-wise applicable on a matrix.
// 'static ensures F is a type that allows f being moved.
impl<T: Copy + PartialEq> Func<T> {
  pub fn new<F: 'static + Fn(T) -> T>(f: F) -> Func<T> {
    return Func(Box::new(f));
  }
}
