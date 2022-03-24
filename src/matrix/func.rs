pub struct Func<T: Copy + PartialEq>(pub Box<dyn Fn(T) -> T>);

// 'static ensures F is a type that allows f being moved.
impl<T: Copy + PartialEq> Func<T> {
  pub fn new<F: 'static + Fn(T) -> T>(f: F) -> Func<T> {
    return Func(Box::new(f));
  }
}
