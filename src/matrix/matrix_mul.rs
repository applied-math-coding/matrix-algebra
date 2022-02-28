use super::Matrix;
use core::ops::{Add, Mul};
use std::iter::Sum;

impl<T: Sum + Mul<Output = T> + Add<Output = T> + Copy + PartialEq> Mul for &Matrix<T> {
  type Output = Matrix<T>;
  fn mul(self, rhs: Self) -> Self::Output {
    if rhs.n_rows != self.n_cols {
      panic!("dimensions do not match!");
    }
    let mut res: Vec<T> = vec![];
    for i in 0..self.n_rows {
      for j in 0..rhs.n_cols {
        res.push(
          self.data[i * self.n_cols..(i + 1) * self.n_cols]
            .iter()
            .enumerate()
            .map(|(rhs_row_idx, v)| *v * rhs.data[j + rhs_row_idx * rhs.n_cols])
            .sum(),
        );
      }
    }
    return Matrix::create_from_data(res, self.n_rows, rhs.n_cols);
  }
}

impl<T: Sum + Mul<Output = T> + Add<Output = T> + Copy + PartialEq> Mul<Matrix<T>> for &Matrix<T> {
  type Output = Matrix<T>;
  fn mul(self, rhs: Matrix<T>) -> Self::Output {
    if rhs.n_rows != self.n_cols {
      panic!("dimensions do not match!");
    }
    let mut res: Vec<T> = vec![];
    for i in 0..self.n_rows {
      for j in 0..rhs.n_cols {
        res.push(
          self.data[i * self.n_cols..(i + 1) * self.n_cols]
            .iter()
            .enumerate()
            .map(|(rhs_row_idx, v)| *v * rhs.data[j + rhs_row_idx * rhs.n_cols])
            .sum(),
        );
      }
    }
    return Matrix::create_from_data(res, self.n_rows, rhs.n_cols);
  }
}

impl<T: Sum + Mul<Output = T> + Add<Output = T> + Copy + PartialEq> Mul<&Matrix<T>> for Matrix<T> {
  type Output = Matrix<T>;
  fn mul(self, rhs: &Matrix<T>) -> Self::Output {
    if rhs.n_rows != self.n_cols {
      panic!("dimensions do not match!");
    }
    let mut res: Vec<T> = vec![];
    for i in 0..self.n_rows {
      for j in 0..rhs.n_cols {
        res.push(
          self.data[i * self.n_cols..(i + 1) * self.n_cols]
            .iter()
            .enumerate()
            .map(|(rhs_row_idx, v)| *v * rhs.data[j + rhs_row_idx * rhs.n_cols])
            .sum(),
        );
      }
    }
    return Matrix::create_from_data(res, self.n_rows, rhs.n_cols);
  }
}

impl<T: Sum + Mul<Output = T> + Add<Output = T> + Copy + PartialEq> Mul<Matrix<T>> for Matrix<T> {
  type Output = Matrix<T>;
  fn mul(self, rhs: Matrix<T>) -> Self::Output {
    if rhs.n_rows != self.n_cols {
      panic!("dimensions do not match!");
    }
    let mut res: Vec<T> = vec![];
    for i in 0..self.n_rows {
      for j in 0..rhs.n_cols {
        res.push(
          self.data[i * self.n_cols..(i + 1) * self.n_cols]
            .iter()
            .enumerate()
            .map(|(rhs_row_idx, v)| *v * rhs.data[j + rhs_row_idx * rhs.n_cols])
            .sum(),
        );
      }
    }
    return Matrix::create_from_data(res, self.n_rows, rhs.n_cols);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_mul_1() {
    let m1 = Matrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
    let m2 = Matrix::new(vec![vec![1.0, 1.0], vec![1.0, 1.0]]);
    assert_eq!(
      &m1 * &m2,
      Matrix::create_from_data(vec![1.0, 1.0, 1.0, 1.0], 2, 2)
    );
  }

  #[test]
  fn test_mul_2() {
    let m1 = Matrix::new(vec![vec![1.0, 2.0], vec![2.0, 1.0]]);
    let m2 = Matrix::new(vec![vec![1.0, 1.0], vec![1.0, 1.0]]);
    assert_eq!(
      &m1 * &m2,
      Matrix::create_from_data(vec![3.0, 3.0, 3.0, 3.0], 2, 2)
    );
  }

  #[test]
  fn test_mul_3() {
    let m1 = Matrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
    let m2 = Matrix::new(vec![vec![1.0, 1.0], vec![1.0, 1.0]]);
    let m3 = Matrix::new(vec![vec![1.0, 1.0], vec![1.0, 1.0]]);
    assert_eq!(
      &m1 * (&m2 * &m3),
      Matrix::create_from_data(vec![2.0, 2.0, 2.0, 2.0], 2, 2)
    );
  }

  #[test]
  fn test_mul_4() {
    let m1 = Matrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
    let m2 = Matrix::new(vec![vec![1.0, 1.0], vec![1.0, 1.0]]);
    let m3 = Matrix::new(vec![vec![1.0, 1.0], vec![1.0, 1.0]]);
    assert_eq!(
      (&m1 * &m2) * &m3,
      Matrix::create_from_data(vec![2.0, 2.0, 2.0, 2.0], 2, 2)
    );
  }

  #[test]
  fn test_mul_5() {
    let m1 = Matrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
    let m2 = Matrix::new(vec![vec![1.0, 1.0], vec![1.0, 1.0]]);
    let m3 = Matrix::new(vec![vec![1.0, 1.0], vec![1.0, 1.0]]);
    assert_eq!(
      (&m1 * &m2) * (&m1 * &m3),
      Matrix::create_from_data(vec![2.0, 2.0, 2.0, 2.0], 2, 2)
    );
  }
}
