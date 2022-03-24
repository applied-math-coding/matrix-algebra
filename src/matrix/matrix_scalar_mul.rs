use super::{scalar::Scalar, Matrix};
use core::ops::Mul;

macro_rules! scalar_mult {
  ($LHS:ty, $RHS:ty, $ScalarType:tt ) => {
    impl<$ScalarType: Mul<Output = $ScalarType> + Copy> Mul<$RHS> for $LHS {
      type Output = Scalar<$ScalarType>;
      fn mul(self, rhs: $RHS) -> Self::Output {
        return Scalar(self.0 * rhs.0);
      }
    }
  };
}
scalar_mult!(Scalar<T>, Scalar<T>, T);
scalar_mult!(&Scalar<T>, Scalar<T>, T);
scalar_mult!(&Scalar<T>, &Scalar<T>, T);
scalar_mult!(Scalar<T>, &Scalar<T>, T);

macro_rules! scalar_matrix_mult {
  ($LHS:ty, $RHS:ty, $ScalarType:tt ) => {
    impl<$ScalarType: Mul<Output = $ScalarType> + Copy + PartialEq> Mul<$RHS> for $LHS {
      type Output = Matrix<$ScalarType>;
      fn mul(self, rhs: $RHS) -> Self::Output {
        let mut res: Vec<$ScalarType> = vec![];
        for v in rhs.data.iter() {
          res.push(self.0 * *v);
        }
        return Matrix::create_from_data(res, rhs.n_rows, rhs.n_cols);
      }
    }
  };
}
scalar_matrix_mult!(Scalar<T>, Matrix<T>, T);
scalar_matrix_mult!(Scalar<T>, &Matrix<T>, T);
scalar_matrix_mult!(&Scalar<T>, Matrix<T>, T);
scalar_matrix_mult!(&Scalar<T>, &Matrix<T>, T);

macro_rules! scalar_type_matrix_mult {
  ($LHS:ty, $ScalarType:tt ) => {
    impl<$ScalarType: Mul<Output = $ScalarType> + Copy + PartialEq> Mul<$ScalarType> for $LHS {
      type Output = Matrix<$ScalarType>;
      fn mul(self, rhs: $ScalarType) -> Self::Output {
        let mut res: Vec<$ScalarType> = vec![];
        for v in self.data.iter() {
          res.push(rhs * *v);
        }
        return Matrix::create_from_data(res, self.n_rows, self.n_cols);
      }
    }
  };
}
scalar_type_matrix_mult!(Matrix<T>, T);
scalar_type_matrix_mult!(&Matrix<T>, T);

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

  #[test]
  fn test_scalar_mul_7() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(
      (Scalar(2) * Scalar(1)) * &m,
      Matrix::create_from_data(vec![0, 2, 4, 6], 2, 2)
    );
  }
}
