use num::{
  Integer,
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

/// A 2-dimensional matrix of integers with size `(rows, columns)`.
pub struct Matrix<T: Integer + Clone + Copy> {
  rows: usize,
  cols: usize,
  matrix: Vec<T>
}

impl<T: Integer + NumCast + Clone + Copy> Matrix<T> {
  fn matrix(rows: usize, columns: usize) -> Option<Self> {
    if rows < 1 || columns < 1 {
      return None;
    }

    let new_width: usize = columns;
    let new_height: usize = rows;
    let mut mat: Vec<T> = Vec::new();

    for _ in 0..(new_width * new_height) {
      mat.push(NumCast::from(0).unwrap());
    }

    Some(Matrix { rows: new_height, cols: new_width, matrix: mat })
  }

  /// Creates a new `row x column` integer matrix with all values initalized to zero.
  pub fn new(rows: usize, columns: usize) -> Self {

    Self::matrix(rows, columns).expect("Cannot initialize a matrix with rows or columns less than 1")
  }

  // Creates an iterator for the current matrix.
  pub fn iter(&self) -> MatrixIterator<T> {

    MatrixIterator { inner: self, idx: 0 }
  }

  pub fn size(&self) -> (usize, usize) {

    (self.rows, self.cols)
  }

  pub fn transpose(&self) -> Matrix<T> {
    let mut trans: Vec<T> = Vec::new();
    for i in 0..self.cols {
      for j in 0..self.rows {
        trans.push(self.matrix[i + (j * self.cols)]);
      }
    }

    Matrix {rows: self.cols, cols: self.rows, matrix: trans}
  }

  pub fn t(&self) -> Matrix<T> {

    self.transpose()
  }

  // pub fn inverse(&self) -> Result<Matrix<T>, String> {
  //   if self.rows != self.cols {
  //     let s = format!("Matrix is not square! Size: ({}, {})",
  //                             self.rows, self.cols);
  //     return Err(s);
  //   }

  //   let mut inverse: Vec<T> = Vec::new();

  //   todo!();
  //   Ok(Matrix {rows: self.rows, cols: self.cols, matrix: inverse})
  // }

  pub fn rref(&self) -> Matrix<T> {
    let rref: Vec<T> = self.matrix.clone();
    

    Matrix {rows: self.rows, cols: self.cols, matrix: rref}
  }

  pub fn identity(size: usize) -> Matrix<u8> {
    let mut identity: Matrix<u8> = Matrix::new(size, size);
  
    for i in 0..size {
      identity[(i, i)] = 1;
    }
    
    identity
  }

}

pub struct MatrixIterator<'a, T: Integer + Clone + Copy> {
  inner: &'a Matrix<T>,
  idx: usize
}

impl<'a, T: Integer + Clone + Copy> Iterator for MatrixIterator<'a, T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.idx >= (self.inner.rows * self.inner.cols) {
      return None;
    }

    let val = self.inner.matrix[self.idx];
    self.idx += 1;
    Some(val)
  }
}

impl<T: Integer + Clone + Copy> Add for Matrix<T> {
  type Output = Result<Self, String>;

  fn add(self, other: Self) -> Result<Self, String> {
    if (self.rows != other.rows) || (self.cols != other.cols) {
      let s = format!("Cannot add matrices of size ({}, {}) and size ({}, {}) together",
                              self.rows, self.cols, other.rows, other.cols);
      return Err(s);
    }

    let mut matrix_vec: Vec<T> = Vec::new();
    for i in 0..self.rows {
      for j in 0..self.cols {
        let current_num = self.matrix[j + (i * self.cols)] + other.matrix[j + (i * self.cols)];
        matrix_vec.push(current_num);
      }
    }

    Ok(Matrix {rows: self.rows, cols: self.cols, matrix: matrix_vec})
  }
}

impl<T: Integer + Clone + Copy> Sub for Matrix<T> {
  type Output = Result<Self, String>;

  fn sub(self, other: Self) -> Result<Self, String> {
    if (self.rows != other.rows) || (self.cols != other.cols) {
      let s = format!("Cannot subtract matrices of size ({}, {}) and size ({}, {})",
                              self.rows, self.cols, other.rows, other.cols);
      return Err(s);
    }

    let mut matrix_vec: Vec<T> = Vec::new();
    for i in 0..self.rows {
      for j in 0..self.cols {
        let current_num = self.matrix[j + (i * self.cols)] - other.matrix[j + (i * self.cols)];
        matrix_vec.push(current_num);
      }
    }

    Ok(Matrix {rows: self.rows, cols: self.cols, matrix: matrix_vec})
  }
}

impl<T: Integer + Clone + Copy + NumCast> Mul for Matrix<T> {
  type Output = Result<Self, String>;

  fn mul(self, other: Self) -> Result<Self, String> {
    if self.cols != other.rows {
      let s = format!("Cannot multiply matrices of size ({}, {}) and size ({}, {}) together",
                              self.rows, self.cols, other.rows, other.cols);
      return Err(s);
    }

    let mut matrix_vec: Vec<T> = Vec::new();
    for i in 0..self.rows {
      for j in 0..other.cols {
        let mut sum: T = NumCast::from(0).unwrap();
        for k in 0..self.cols {
          let current_num = self.matrix[k + (i * self.cols)] * other.matrix[j + (k * other.cols)];
          sum = sum + current_num;
        }
        matrix_vec.push(sum);
      }
    }

    Ok(Matrix {rows: self.rows, cols: other.cols, matrix: matrix_vec})
  } 
}

impl<T: Integer + std::fmt::Display + Clone + Copy> fmt::Display for Matrix<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut text: String = String::from("");
    text.push_str(&format!("Matrix at addr {:p}:\n", self));
    
    for i in 0..self.rows {
      text.push_str("[ ");
      for j in 0..self.cols {
        let current_num = self.matrix[j + (i * self.cols)];
        let s = format!("{:>4} ", current_num);
        text.push_str(&s);
      }
      text.push_str("]\n");
    }

    write!(f, "{}", text)
  }
}

impl<T: Integer + Clone + Copy> IntoIterator for Matrix<T> {
  type Item = T;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {

    self.matrix.into_iter()
  }
}

impl<T: Integer + Clone + Copy> Index<(usize, usize)> for Matrix<T> {
  type Output = T;

  fn index(&self, index: (usize, usize)) -> &Self::Output {
    let (i, j) = index;

    &self.matrix[j + (i * self.cols)]
  }
}

impl<T: Integer + Clone + Copy> IndexMut<(usize, usize)> for Matrix<T> {
  fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
    let (i, j) = index;

    &mut self.matrix[j + (i * self.cols)]
  }
}

macro_rules! impl_from {
  ($($x:ty, $y:ty;)*) => {
    $(
      impl From<Matrix<$y>> for Matrix<$x> {
        fn from(m: Matrix<$y>) -> Self {
          let mut data: Vec<$x> = Vec::new();
      
          for i in m.matrix {
            data.push(i.into());
          }
      
          Matrix { rows: m.rows, cols: m.cols, matrix: data }
        }
      }
    )*
  }
}

impl_from! {
  u16, u8;
  u32, u16; u32, u8;
  u64, u32; u64, u16; u64, u8;
  u128, u64; u128, u32; u128, u16; u128, u8;
  i16, i8;
  i32, i16; i32, i8;
  i64, i32; i64, i16; i64, i8;
  i128, i64; i128, i32; i128, i16; i128, i8;
}