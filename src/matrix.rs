mod matrix_add;
mod matrix_iter_mut;
mod matrix_iterator;
mod matrix_sub;
use matrix_iter_mut::MatrixIteratorMut;
use matrix_iterator::MatrixIterator;

#[derive(PartialEq, Debug)]
pub struct Matrix<T: Copy + PartialEq> {
  data: Vec<T>,
  n_rows: usize,
  n_cols: usize,
}

impl<T: Copy + PartialEq> Matrix<T> {
  pub fn new(mut v: Vec<Vec<T>>) -> Matrix<T> {
    let n_rows = v.len();
    let n_cols = v[0].len();
    let mut data = vec![];
    for row in v.iter_mut() {
      while row.len() > 0 {
        data.push(row.remove(0));
      }
    }
    Matrix {
      data,
      n_rows,
      n_cols,
    }
  }

  pub fn create_from_data(data: Vec<T>, n_rows: usize, n_cols: usize) -> Matrix<T> {
    if data.len() != n_rows * n_cols {
      panic!("not compatible dimension!");
    }
    Matrix {
      data,
      n_rows,
      n_cols,
    }
  }

  pub fn get(&self, i: usize, j: usize) -> T {
    return self.data[i * self.n_cols + j];
  }

  pub fn iter<'a>(&'a self) -> MatrixIterator<'a, T> {
    MatrixIterator::new(0, 0, 0, &self.data, self.n_rows, self.n_cols)
  }

  pub fn iter_mut<'a>(&'a mut self) -> MatrixIteratorMut<'a, T> {
    MatrixIteratorMut::new(0, 0, &mut self.data, self.n_rows, self.n_cols)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_iter() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(
      m.iter().collect::<Vec<(&u32, usize, usize)>>(),
      vec![(&0, 0, 0), (&1, 0, 1), (&2, 1, 0), (&3, 1, 1)]
    );
  }

  #[test]
  fn test_iter_mut() {
    let mut m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(
      m.iter_mut().collect::<Vec<(&mut u32, usize, usize)>>(),
      vec![
        (&mut 0, 0, 0),
        (&mut 1, 0, 1),
        (&mut 2, 1, 0),
        (&mut 3, 1, 1)
      ]
    );
  }
}
