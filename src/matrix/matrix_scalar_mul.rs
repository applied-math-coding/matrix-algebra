use super::Matrix;
use core::ops::Mul;

#[derive(PartialEq, Debug, Clone)]
pub struct Scalar<T>(T);

impl<T: Mul<Output = T> + Copy> Mul<Scalar<T>> for Scalar<T> {
  type Output = Scalar<T>;
  fn mul(self, rhs: Scalar<T>) -> Self::Output {
    return Scalar(self.0 * rhs.0);
  }
}

impl<T: Mul<Output = T> + Copy> Mul<Scalar<T>> for &Scalar<T> {
  type Output = Scalar<T>;
  fn mul(self, rhs: Scalar<T>) -> Self::Output {
    return Scalar(self.0 * rhs.0);
  }
}

impl<T: Mul<Output = T> + Copy> Mul<&Scalar<T>> for &Scalar<T> {
  type Output = Scalar<T>;
  fn mul(self, rhs: &Scalar<T>) -> Self::Output {
    return Scalar(self.0 * rhs.0);
  }
}

impl<T: Mul<Output = T> + Copy> Mul<&Scalar<T>> for Scalar<T> {
  type Output = Scalar<T>;
  fn mul(self, rhs: &Scalar<T>) -> Self::Output {
    return Scalar(self.0 * rhs.0);
  }
}

impl<T: Mul<Output = T> + Copy + PartialEq> Mul<Matrix<T>> for Scalar<T> {
  type Output = Matrix<T>;
  fn mul(self, rhs: Matrix<T>) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for v in rhs.data.iter() {
      res.push(self.0 * *v);
    }
    return Matrix::create_from_data(res, rhs.n_rows, rhs.n_cols);
  }
}

impl<T: Mul<Output = T> + Copy + PartialEq> Mul<&Matrix<T>> for Scalar<T> {
  type Output = Matrix<T>;
  fn mul(self, rhs: &Matrix<T>) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for v in rhs.data.iter() {
      res.push(self.0 * *v);
    }
    return Matrix::create_from_data(res, rhs.n_rows, rhs.n_cols);
  }
}

impl<T: Mul<Output = T> + Copy + PartialEq> Mul<Matrix<T>> for &Scalar<T> {
  type Output = Matrix<T>;
  fn mul(self, rhs: Matrix<T>) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for v in rhs.data.iter() {
      res.push(self.0 * *v);
    }
    return Matrix::create_from_data(res, rhs.n_rows, rhs.n_cols);
  }
}

impl<T: Mul<Output = T> + Copy + PartialEq> Mul<&Matrix<T>> for &Scalar<T> {
  type Output = Matrix<T>;
  fn mul(self, rhs: &Matrix<T>) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for v in rhs.data.iter() {
      res.push(self.0 * *v);
    }
    return Matrix::create_from_data(res, rhs.n_rows, rhs.n_cols);
  }
}

impl<T: Mul<Output = T> + Copy + PartialEq> Mul<T> for Matrix<T> {
  type Output = Matrix<T>;
  fn mul(self, rhs: T) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for v in self.data.iter() {
      res.push(rhs * *v);
    }
    return Matrix::create_from_data(res, self.n_rows, self.n_cols);
  }
}

impl<T: Mul<Output = T> + Copy + PartialEq> Mul<T> for &Matrix<T> {
  type Output = Matrix<T>;
  fn mul(self, rhs: T) -> Self::Output {
    let mut res: Vec<T> = vec![];
    for v in self.data.iter() {
      res.push(rhs * *v);
    }
    return Matrix::create_from_data(res, self.n_rows, self.n_cols);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_scalar_mul_1() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(m * 2, Matrix::create_from_data(vec![0, 2, 4, 6], 2, 2));
  }

  #[test]
  fn test_scalar_mul_2() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(&m * 2, Matrix::create_from_data(vec![0, 2, 4, 6], 2, 2));
  }

  #[test]
  fn test_scalar_mul_3() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(
      Scalar(2) * m,
      Matrix::create_from_data(vec![0, 2, 4, 6], 2, 2)
    );
  }

  #[test]
  fn test_scalar_mul_4() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(
      Scalar(2) * &m,
      Matrix::create_from_data(vec![0, 2, 4, 6], 2, 2)
    );
  }

  #[test]
  fn test_scalar_mul_5() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(
      &Scalar(2) * &m,
      Matrix::create_from_data(vec![0, 2, 4, 6], 2, 2)
    );
  }

  #[test]
  fn test_scalar_mul_6() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(
      (&Scalar(2) * &Scalar(1)) * &m,
      Matrix::create_from_data(vec![0, 2, 4, 6], 2, 2)
    );
  }
}
