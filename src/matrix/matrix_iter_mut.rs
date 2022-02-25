pub struct MatrixIteratorMut<'a, T: Copy> {
  row_idx: usize,
  col_idx: usize,
  data: &'a mut [T],
  n_rows: usize,
  n_cols: usize,
}

impl<'a, T: Copy> MatrixIteratorMut<'a, T> {
  pub fn new(
    row_idx: usize,
    col_idx: usize,
    data: &'a mut [T],
    n_rows: usize,
    n_cols: usize,
  ) -> MatrixIteratorMut<'a, T> {
    MatrixIteratorMut {
      row_idx,
      col_idx,
      data,
      n_rows,
      n_cols,
    }
  }
}

impl<'a, T: Copy> std::iter::Iterator for MatrixIteratorMut<'a, T> {
  type Item = (&'a mut T, usize, usize);
  fn next(&mut self) -> Option<Self::Item> {
    if self.col_idx == self.n_cols {
      self.row_idx += 1;
      self.col_idx = 0;
    }
    return if self.row_idx == self.n_rows {
      None
    } else {
      let col_idx = self.col_idx;
      self.col_idx += 1;
      let data = std::mem::replace(&mut self.data, &mut []);
      if let Some((v, rest)) = data.split_first_mut() {
        self.data = rest;
        Some((v, self.row_idx, col_idx))
      } else {
        None
      }
    };
  }
}
