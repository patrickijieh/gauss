use num::{
  Float,
  NumCast
};

use std::{
  fmt, 
  ops::{
    Add,
    Sub,
    Mul,
    Index,
    IndexMut
  }
};

/// A 2-dimensional matrix of floating-point numbers with size `(rows, columns)`.
pub struct FloatMatrix<T: Float> {
  rows: usize,
  cols: usize,
  matrix: Vec<T>
}

impl<T: Float + std::fmt::Debug> FloatMatrix<T> {
  fn matrix(rows: usize, columns: usize) -> Option<Self> {
    if rows < 1 || columns < 1 {
      return None;
    }

    let new_width: usize = columns;
    let new_height: usize = rows;
    let mut mat: Vec<T> = Vec::new();

    for _ in 0..(new_width * new_height) {
      mat.push(NumCast::from(0.0).unwrap());
    }

    Some(FloatMatrix { rows: new_height, cols: new_width, matrix: mat })
  }

  /// Creates a new `row x column` float matrix with all values initalized to zero.
  pub fn new(rows: usize, columns: usize) -> Self {

    Self::matrix(rows, columns).expect("Cannot initialize a matrix with rows or columns less than 1")
  }

  pub fn from_1d_vec(matrix: Vec<T>, rows: usize, cols: usize) -> Self {

    FloatMatrix {rows, cols, matrix}
  }

  // Creates an iterator for the current matrix.
  pub fn iter(&self) -> FloatMatrixIterator<T> {

    FloatMatrixIterator { inner: self, idx: 0 }
  }

  pub fn size(&self) -> (usize, usize) {

    (self.rows, self.cols)
  }

  pub fn transpose(&self) -> FloatMatrix<T> {
    let mut trans: Vec<T> = Vec::new();
    for i in 0..self.cols {
      for j in 0..self.rows {
        trans.push(self.matrix[i + (j * self.cols)]);
      }
    }

    FloatMatrix {rows: self.cols, cols: self.rows, matrix: trans}
  }

  pub fn t(&self) -> FloatMatrix<T> {

    self.transpose()
  }

  pub fn rref(&self) -> FloatMatrix<T> {
    let mut rref: Vec<T> = self.matrix.clone();
    
    for i in 0..self.rows {
      for j in i..self.rows {
        if (self.matrix[i * self.cols] != NumCast::from(1.0).unwrap()) && (self.matrix[j * self.rows] > NumCast::from(0.0).unwrap()) {
          swap_rows(&mut rref, i, j, self.cols);
        }
      }
      
      if self.matrix[i * self.cols] != NumCast::from(1.0).unwrap() {
        row_with_leading_one(&mut rref, i, i, self.cols);
      }
  
      for j in i+1..self.rows {
        row_sub(&mut rref, self.matrix[j * self.cols], j, i, self.cols);
      }

      println!("{:?}", rref);
    }

    FloatMatrix {rows: self.rows, cols: self.cols, matrix: rref}
  } 
}

pub struct FloatMatrixIterator<'a, T: Float> {
  inner: &'a FloatMatrix<T>,
  idx: usize
}

impl<'a, T: Float> Iterator for FloatMatrixIterator<'a, T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.idx >= (self.inner.rows * self.inner.cols) {
      None
    }
    else {
      let val = self.inner.matrix[self.idx as usize];
      self.idx += 1;
      Some(val)
    }
  }
}

impl<T: Float> Add for FloatMatrix<T> {
  type Output = Result<Self, String>;

  fn add(self, other: Self) -> Result<Self, String> {
    if (self.rows != other.rows) || (self.cols != other.cols) {
      let s = format!("Cannot add matrices of size ({}, {}) and size ({}, {}) together",
                              self.rows, self.cols, other.rows, other.cols);
      Err(s)
    }

    else {
      let mut matrix_vec: Vec<T> = Vec::new();
      for i in 0..self.rows {
        for j in 0..self.cols {
          let current_num = self.matrix[j + (i * self.cols)] + other.matrix[j + (i * self.cols)];
          matrix_vec.push(current_num);
        }
      }

      Ok(FloatMatrix {rows: self.rows, cols: self.cols, matrix: matrix_vec})
    }
  }
}

impl<T: Float> Sub for FloatMatrix<T> {
  type Output = Result<Self, String>;

  fn sub(self, other: Self) -> Result<Self, String> {
    if (self.rows != other.rows) || (self.cols != other.cols) {
      let s = format!("Cannot subtract matrices of size ({}, {}) and size ({}, {})",
                              self.rows, self.cols, other.rows, other.cols);
      Err(s)
    }

    else {
      let mut matrix_vec: Vec<T> = Vec::new();
      for i in 0..self.rows {
        for j in 0..self.cols {
          let current_num = self.matrix[j + (i * self.cols)] - other.matrix[j + (i * self.cols)];
          matrix_vec.push(current_num);
        }
      }

      Ok(FloatMatrix {rows: self.rows, cols: self.cols, matrix: matrix_vec})
    }
  }
}

impl<T: Float> Mul for FloatMatrix<T> {
  type Output = Result<Self, String>;

  fn mul(self, other: Self) -> Result<Self, String> {
    if self.cols != other.rows {
      let s = format!("Cannot multiply matrices of size ({}, {}) and size ({}, {}) together",
                              self.rows, self.cols, other.rows, other.cols);
      Err(s)
    }
    else {
      let mut matrix_vec: Vec<T> = Vec::new();
      for i in 0..self.rows {
        for j in 0..other.cols {
          let mut sum: T = NumCast::from(0.0).unwrap();
          for k in 0..self.cols {
            let current_num = self.matrix[k + (i * self.cols)] * other.matrix[j + (k * other.cols)];
            sum = sum + current_num;
          }
          matrix_vec.push(sum);
        }
      }

      Ok(FloatMatrix {rows: self.rows, cols: other.cols, matrix: matrix_vec})
    }
  } 
}



impl<T: Float + std::fmt::Display> fmt::Display for FloatMatrix<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut text: String = String::from("");
    text.push_str(&format!("Matrix at addr {:p}:\n", self));

    for i in 0..self.rows {
      text.push_str("[ ");
      for j in 0..self.cols {
        let current_num = self.matrix[j + (i * self.cols)];
        let s = format!("{:>4.4} ", current_num);
        text.push_str(&s);
      }
      text.push_str("]\n");
    }

    write!(f, "{}", text)
  }
}

impl<T: Float> IntoIterator for FloatMatrix<T> {
  type Item = T;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {

    self.matrix.into_iter()
  }
}

impl<T: Float> Index<(usize, usize)> for FloatMatrix<T> {
  type Output = T;

  fn index(&self, index: (usize, usize)) -> &Self::Output {
    let (i, j) = index;

    &self.matrix[j + (i * self.cols)]
  }
}

impl<T: Float> IndexMut<(usize, usize)> for FloatMatrix<T> {
  fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
    let (i, j) = index;

    &mut self.matrix[j + (i * self.cols)]
  }
}

impl<T: Float> Clone for FloatMatrix<T> {
  fn clone(&self) -> Self {

    FloatMatrix {rows: self.rows, cols: self.cols, matrix: self.matrix.clone()}
  }
}

impl From<FloatMatrix<f32>> for FloatMatrix<f64> {
  fn from(m: FloatMatrix<f32>) -> Self {
    let mut data: Vec<f64> = Vec::new();

    for i in m.matrix {
      data.push(i.into());
    }

    FloatMatrix { rows: m.rows, cols: m.cols, matrix: data }
  }
}

fn swap_rows<T: Float>(matrix: &mut Vec<T>, to: usize, from: usize, column_size: usize) {
  let mut temp: T;

  for i in 0..column_size {
    temp = matrix[i + (to * column_size)];
    matrix[i + (to * column_size)] = matrix[i + (from * column_size)];
    matrix[i + (from * column_size)] = temp;
  }
}

fn row_with_leading_one<T: Float>(matrix: &mut Vec<T>, idx: usize, row: usize, column_size: usize) {
  if matrix[idx + (row * column_size)] == NumCast::from(0.0).unwrap() {
    return;
  }

  for i in 0..column_size {
    matrix[i + (row * column_size)] = matrix[i + (row * column_size)] / matrix[idx + (row * column_size)];
  }
}

fn row_sub<T: Float>(matrix: &mut Vec<T>, factor: T, to: usize, from: usize, column_size: usize) {
  for i in 0..column_size {
    matrix[i + (to * column_size)] = matrix[i + (to * column_size)] - (factor * matrix[i + (from * column_size)]);
  }
}