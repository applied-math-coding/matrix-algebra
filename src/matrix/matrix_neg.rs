use super::Matrix;
use core::ops::Neg;

#[macro_export]
macro_rules! matrix_neg {
  ($RHS:ty, $ScalarType:tt ) => {
    impl<$ScalarType: Neg<Output = $ScalarType> + Copy + PartialEq> Neg for $RHS {
      type Output = Matrix<$ScalarType>;
      fn neg(self) -> Self::Output {
        let mut res: Vec<$ScalarType> = vec![];
        for (e, _, _) in self.iter() {
          res.push(-*e);
        }
        return Matrix::create_from_data(res, self.n_rows, self.n_cols);
      }
    }
  };
}
matrix_neg!(&Matrix<T>, T);
matrix_neg!(Matrix<T>, T);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_neg_1() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    assert_eq!(-&m1, Matrix::create_from_data(vec![-1, 0, 0, -1], 2, 2));
  }

  #[test]
  fn test_neg_2() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    assert_eq!(--&m1, Matrix::create_from_data(vec![1, 0, 0, 1], 2, 2));
  }
}
