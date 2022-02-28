use super::Matrix;
use core::ops::Div;

impl<T: Div<Output = T> + Copy + PartialEq> Div for &Matrix<T> {
  type Output = Matrix<T>;
  fn div(self, rhs: Self) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for (e, row, col) in self.iter() {
      res.push(*e / rhs.get(row, col));
    }
    return Matrix::create_from_data(res, self.n_rows, self.n_cols);
  }
}

impl<T: Div<Output = T> + Copy + PartialEq> Div<&Matrix<T>> for Matrix<T> {
  type Output = Matrix<T>;
  fn div(self, rhs: &Matrix<T>) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for (e, row, col) in self.iter() {
      res.push(*e / rhs.get(row, col));
    }
    return Matrix::create_from_data(res, self.n_rows, self.n_cols);
  }
}

impl<T: Div<Output = T> + Copy + PartialEq> Div<Matrix<T>> for &Matrix<T> {
  type Output = Matrix<T>;
  fn div(self, rhs: Matrix<T>) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for (e, row, col) in self.iter() {
      res.push(*e / rhs.get(row, col));
    }
    return Matrix::create_from_data(res, self.n_rows, self.n_cols);
  }
}

impl<T: Div<Output = T> + Copy + PartialEq> Div<Matrix<T>> for Matrix<T> {
  type Output = Matrix<T>;
  fn div(self, rhs: Matrix<T>) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for (e, row, col) in self.iter() {
      res.push(*e / rhs.get(row, col));
    }
    return Matrix::create_from_data(res, self.n_rows, self.n_cols);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_div_1() {
    let m1 = Matrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
    let m2 = Matrix::new(vec![vec![1.0, 1.0], vec![1.0, 1.0]]);
    assert_eq!(
      &m1 / &m2,
      Matrix::create_from_data(vec![1.0, 0.0, 0.0, 1.0], 2, 2)
    );
  }

  #[test]
  fn test_div_2() {
    let m1 = Matrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
    let m2 = Matrix::new(vec![vec![1.0, 1.0], vec![1.0, 1.0]]);
    let m3 = Matrix::new(vec![vec![1.0, 1.0], vec![1.0, 1.0]]);
    assert_eq!(
      &m1 / &m2 / &m3,
      Matrix::create_from_data(vec![1.0, 0.0, 0.0, 1.0], 2, 2)
    );
  }

  #[test]
  fn tes_div_3() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m3 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(
      &m1 / (&m2 / &m3),
      Matrix::create_from_data(vec![1, 0, 0, 1], 2, 2)
    );
  }

  #[test]
  fn tes_div_4() {
    let m1 = Matrix::new(vec![vec![1, 0], vec![0, 1]]);
    let m2 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m3 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    let m4 = Matrix::new(vec![vec![1, 1], vec![1, 1]]);
    assert_eq!(
      (&m1 / &m4) / (&m2 / &m3),
      Matrix::create_from_data(vec![1, 0, 0, 1], 2, 2)
    );
  }
}
