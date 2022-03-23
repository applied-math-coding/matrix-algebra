use super::Matrix;
use core::ops::Sub;

#[macro_export]
macro_rules! matrix_subtraction {
  ($LHS:ty, $RHS:ty, $ScalarType:tt ) => {
    impl<$ScalarType: Sub<Output = $ScalarType> + Copy + PartialEq> Sub<$RHS> for $LHS {
      type Output = Matrix<$ScalarType>;
      fn sub(self, rhs: $RHS) -> Self::Output {
        let mut res: Vec<$ScalarType> = vec![];
        for (e, row, col) in self.iter() {
          res.push(*e - rhs.get(row, col));
        }
        return Matrix::create_from_data(res, self.n_rows, self.n_cols);
      }
    }
  };
}
matrix_subtraction!(&Matrix<T>, &Matrix<T>, T);
matrix_subtraction!(Matrix<T>, &Matrix<T>, T);
matrix_subtraction!(&Matrix<T>, Matrix<T>, T);
matrix_subtraction!(Matrix<T>, Matrix<T>, T);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sub_1() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(
      &m1 - &m2,
      Matrix::create_from_data(vec![0, -1, -1, 0], 2, 2)
    );
  }

  #[test]
  fn test_sub_2() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m3 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(
      &m1 - &m2 - &m3,
      Matrix::create_from_data(vec![-1, -2, -2, -1], 2, 2)
    );
  }

  #[test]
  fn test_sub_3() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m3 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(
      &m1 - (&m2 - &m3),
      Matrix::create_from_data(vec![1, 0, 0, 1], 2, 2)
    );
  }

  #[test]
  fn test_sub_4() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m3 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m4 = Matrix::new(vec![vec![0, 0], vec![0, 0]]);
    assert_eq!(
      (&m1 - &m4) - (&m2 - &m3),
      Matrix::create_from_data(vec![1, 0, 0, 1], 2, 2)
    );
  }
}
