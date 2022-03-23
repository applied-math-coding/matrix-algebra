use super::Matrix;
use core::ops::Add;

#[macro_export]
macro_rules! matrix_add {
  ($LHS:ty, $RHS:ty, $ScalarType:tt ) => {
    impl<$ScalarType: Add<Output = $ScalarType> + Copy + PartialEq> Add<$RHS> for $LHS {
      type Output = Matrix<$ScalarType>;
      fn add(self, rhs: $RHS) -> Self::Output {
        let mut res: Vec<$ScalarType> = vec![];
        for (e, row, col) in self.iter() {
          res.push(*e + rhs.get(row, col));
        }
        return Matrix::create_from_data(res, self.n_rows, self.n_cols);
      }
    }
  };
}
matrix_add!(&Matrix<T>, &Matrix<T>, T);
matrix_add!(Matrix<T>, Matrix<T>, T);
matrix_add!(&Matrix<T>, Matrix<T>, T);
matrix_add!(Matrix<T>, &Matrix<T>, T);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_1() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(&m1 + &m2, Matrix::create_from_data(vec![2, 1, 1, 2], 2, 2));
  }

  #[test]
  fn test_add_2() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m3 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(
      &m1 + &m2 + &m3,
      Matrix::create_from_data(vec![3, 2, 2, 3], 2, 2)
    );
  }

  #[test]
  fn test_add_3() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m3 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(
      &m1 + (&m2 + &m3),
      Matrix::create_from_data(vec![3, 2, 2, 3], 2, 2)
    );
  }

  #[test]
  fn test_add_4() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m3 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m4 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(
      (&m1 + &m2) + (&m3 + &m4),
      Matrix::create_from_data(vec![4, 3, 3, 4], 2, 2)
    );
  }
}
