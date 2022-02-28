use super::Matrix;
use core::ops::{Mul, Rem};

impl<T: Mul<Output = T> + Copy + PartialEq> Rem for &Matrix<T> {
  type Output = Matrix<T>;
  fn rem(self, rhs: Self) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for (e, row, col) in self.iter() {
      res.push(*e * rhs.get(row, col));
    }
    return Matrix::create_from_data(res, self.n_rows, self.n_cols);
  }
}

impl<T: Mul<Output = T> + Copy + PartialEq> Rem<&Matrix<T>> for Matrix<T> {
  type Output = Matrix<T>;
  fn rem(self, rhs: &Matrix<T>) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for (e, row, col) in self.iter() {
      res.push(*e * rhs.get(row, col));
    }
    return Matrix::create_from_data(res, self.n_rows, self.n_cols);
  }
}

impl<T: Mul<Output = T> + Copy + PartialEq> Rem<Matrix<T>> for &Matrix<T> {
  type Output = Matrix<T>;
  fn rem(self, rhs: Matrix<T>) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for (e, row, col) in self.iter() {
      res.push(*e * rhs.get(row, col));
    }
    return Matrix::create_from_data(res, self.n_rows, self.n_cols);
  }
}

impl<T: Mul<Output = T> + Copy + PartialEq> Rem<Matrix<T>> for Matrix<T> {
  type Output = Matrix<T>;
  fn rem(self, rhs: Matrix<T>) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for (e, row, col) in self.iter() {
      res.push(*e * rhs.get(row, col));
    }
    return Matrix::create_from_data(res, self.n_rows, self.n_cols);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rem_1() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(&m1 % &m2, Matrix::create_from_data(vec![1, 0, 0, 1], 2, 2));
  }

  #[test]
  fn test_rem_2() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m3 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(
      &m1 % &m2 % &m3,
      Matrix::create_from_data(vec![1, 0, 0, 1], 2, 2)
    );
  }

  #[test]
  fn tes_rem_3() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m3 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(
      &m1 % (&m2 % &m3),
      Matrix::create_from_data(vec![1, 0, 0, 1], 2, 2)
    );
  }

  #[test]
  fn tes_rem_4() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m3 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m4 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(
      (&m1 % &m2) % (&m3 % m4),
      Matrix::create_from_data(vec![1, 0, 0, 1], 2, 2)
    );
  }
}
