mod matrix_add;
mod matrix_div;
mod matrix_iter_mut;
mod matrix_iterator;
mod matrix_mul;
mod matrix_scalar_mul;
mod matrix_rem;
mod matrix_sub;
mod matrix_neg;
use matrix_iter_mut::MatrixIteratorMut;
use matrix_iterator::MatrixIterator;

#[derive(PartialEq, Debug, Clone)]
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

  pub fn get_row(&self, i: usize) -> Matrix<T> {
    let mut row = vec![];
    for e in &self.data[i * self.n_cols..(i + 1) * self.n_cols] {
      row.push(*e);
    }
    let n_cols = row.len();
    Matrix::create_from_data(row, 1, n_cols)
  }

  pub fn get_col(&self, i: usize) -> Matrix<T> {
    let col = self
      .iter()
      .filter(|(_, _, col)| *col == i)
      .map(|(e, _, _)| *e)
      .collect::<Vec<T>>();
    let n_rows = col.len();
    Matrix::create_from_data(col, n_rows, 1)
  }

  pub fn iter<'a>(&'a self) -> MatrixIterator<'a, T> {
    MatrixIterator::new(0, 0, 0, &self.data, self.n_rows, self.n_cols)
  }

  pub fn iter_mut<'a>(&'a mut self) -> MatrixIteratorMut<'a, T> {
    MatrixIteratorMut::new(0, 0, &mut self.data, self.n_rows, self.n_cols)
  }

  pub fn trans(&self) -> Matrix<T> {
    let mut data = vec![];
    for j in 0..self.n_cols {
      for i in 0..self.n_rows {
        data.push(self.data[j + i * self.n_cols]);
      }
    }
    Matrix::create_from_data(data, self.n_cols, self.n_rows)
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

  #[test]
  fn test_get_row_1() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(m.get_row(0), Matrix::create_from_data(vec![0, 1], 1, 2));
  }

  #[test]
  fn test_get_row_2() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(m.get_row(1), Matrix::create_from_data(vec![2, 3], 1, 2));
  }

  #[test]
  fn test_get_col_1() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(m.get_col(0), Matrix::create_from_data(vec![0, 2], 2, 1));
  }

  #[test]
  fn test_get_col_2() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(m.get_col(1), Matrix::create_from_data(vec![1, 3], 2, 1));
  }

  #[test]
  fn test_trans() {
    let m = Matrix::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(m.trans(), Matrix::new(vec![vec![0, 2], vec![1, 3]]));
  }
}
