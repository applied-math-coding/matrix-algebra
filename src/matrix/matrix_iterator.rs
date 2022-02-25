pub struct MatrixIterator<'a, T: Copy> {
  row_idx: usize,
  col_idx: usize,
  idx: usize,
  data: &'a [T],
  n_rows: usize,
  n_cols: usize,
}

impl<'a, T: Copy> MatrixIterator<'a, T> {
  pub fn new(
    row_idx: usize,
    col_idx: usize,
    idx: usize,
    data: &'a [T],
    n_rows: usize,
    n_cols: usize,
  ) -> MatrixIterator<'a, T> {
    MatrixIterator {
      row_idx,
      col_idx,
      idx,
      data,
      n_rows,
      n_cols,
    }
  }
}

impl<'a, T: Copy> std::iter::Iterator for MatrixIterator<'a, T> {
  type Item = (&'a T, usize, usize);
  fn next(&mut self) -> Option<Self::Item> {
    if self.col_idx == self.n_cols {
      self.row_idx += 1;
      self.col_idx = 0;
    }
    return if self.row_idx == self.n_rows {
      None
    } else {
      let col_idx = self.col_idx;
      let idx = self.idx;
      self.col_idx += 1;
      self.idx += 1;
      Some((&self.data[idx], self.row_idx, col_idx))
    };
  }
}
