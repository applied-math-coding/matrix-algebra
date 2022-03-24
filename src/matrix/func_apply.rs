use super::{func::Func, Matrix};
use core::ops::Rem;

macro_rules! elementw_func_apply {
  ($LHS:ty, $RHS:ty, $ScalarType:tt ) => {
    impl<$ScalarType: Copy + PartialEq> Rem<$RHS> for $LHS {
      type Output = Matrix<$ScalarType>;
      fn rem(self, rhs: $RHS) -> Self::Output {
        let mut res: Vec<$ScalarType> = vec![];
        for (e, ..) in rhs.iter() {
          res.push(self.0(*e));
        }
        return Matrix::create_from_data(res, rhs.n_rows, rhs.n_cols);
      }
    }
  };
}
elementw_func_apply!(Func<T>, Matrix<T>, T);
elementw_func_apply!(Func<T>, &Matrix<T>, T);
elementw_func_apply!(&Func<T>, Matrix<T>, T);
elementw_func_apply!(&Func<T>, &Matrix<T>, T);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_func_apply_1() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    let double = Func::new(|x| 2 * x);
    assert_eq!(double % m, Matrix::new(vec![vec![0, 2], vec![4, 6]]));
  }

  #[test]
  fn test_func_apply_2() {
    let m = &Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    let double = Func::new(|x| 2 * x);
    assert_eq!(double % m, Matrix::new(vec![vec![0, 2], vec![4, 6]]));
  }

  #[test]
  fn test_func_apply_3() {
    let m = &Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    let double = &Func::new(|x| 2 * x);
    assert_eq!(double % m, Matrix::new(vec![vec![0, 2], vec![4, 6]]));
  }

  #[test]
  fn test_func_apply_4() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    let double = &Func::new(|x| 2 * x);
    assert_eq!(double % m, Matrix::new(vec![vec![0, 2], vec![4, 6]]));
  }
}
