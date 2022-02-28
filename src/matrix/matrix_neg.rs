use super::Matrix;
use core::ops::Neg;

impl<T: Neg<Output = T> + Copy + PartialEq> Neg for &Matrix<T> {
  type Output = Matrix<T>;
  fn neg(self) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for (e, _, _) in self.iter() {
      res.push(-*e);
    }
    return Matrix::create_from_data(res, self.n_rows, self.n_cols);
  }
}

impl<T: Neg<Output = T> + Copy + PartialEq> Neg for Matrix<T> {
  type Output = Matrix<T>;
  fn neg(self) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for (e, _, _) in self.iter() {
      res.push(-*e);
    }
    return Matrix::create_from_data(res, self.n_rows, self.n_cols);
  }
}

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
