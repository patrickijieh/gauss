use num::{
  Float,
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

/// A 2-dimensional matrix of floating-point numbers with size `(rows, columns)`.
pub struct FloatMatrix<T: Float> {
  rows: usize,
  cols: usize,
  matrix: Vec<T>
}

pub struct FloatMatrixIterator<'a, T: Float> {
  matrix: &'a FloatMatrix<T>,
  idx: usize
}

impl<'a, T: Float> Iterator for FloatMatrixIterator<'a, T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.idx >= (self.matrix.rows * self.matrix.cols) {
      None
    }
    else {
      let val = self.matrix.matrix[self.idx as usize];
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
  type Output = Result<Self, &'static str>;

  fn sub(self, other: Self) -> Result<Self, &'static str> {
    if (self.rows != other.rows) || (self.cols != other.cols) {
      Err("Subtraction not defined for matrices of different dimensions")
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
  type Output = Result<Self, &'static str>;

  fn mul(self, other: Self) -> Result<Self, &'static str> {
    if (self.rows != other.rows) || (self.cols != other.cols) {
      Err("Wrong dimensions for multiplication")
    }
    else {
      todo!();
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

impl<T: Float> FloatMatrix<T> {
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

  // Creates an iterator for the current matrix.
  pub fn iter(&self) -> FloatMatrixIterator<T> {

    FloatMatrixIterator { matrix: self, idx: 0 }
  }

  pub fn transpose(&self) -> FloatMatrix<T> {
    let mut trans: Vec<T> = Vec::new();
    for i in 0..self.rows {
      for j in 0..self.cols {
        trans.push(self.matrix[i + (j * self.rows)]);
      }
    }

    FloatMatrix {rows: self.cols, cols: self.rows, matrix: trans}
  }

  pub fn t(&self) -> FloatMatrix<T> {

    self.transpose()
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



/// A 2-dimensional matrix of integers with size `(rows, columns)`.
pub struct Matrix<T: Integer + Clone + Copy> {
  rows: usize,
  cols: usize,
  matrix: Vec<T>
}

pub struct MatrixIterator<'a, T: Integer + Clone + Copy> {
  matrix: &'a Matrix<T>,
  idx: usize
}

impl<'a, T: Integer + Clone + Copy> Iterator for MatrixIterator<'a, T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.idx >= (self.matrix.rows * self.matrix.cols) {
      None
    }
    else {
      let val = self.matrix.matrix[self.idx];
      self.idx += 1;
      Some(val)
    }
  }
}

impl<T: Integer + Clone + Copy> Add for Matrix<T> {
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

      Ok(Matrix {rows: self.rows, cols: self.cols, matrix: matrix_vec})
    }
  }
}

impl<T: Integer + Clone + Copy> Sub for Matrix<T> {
  type Output = Result<Self, &'static str>;

  fn sub(self, other: Self) -> Result<Self, &'static str> {
    if (self.rows != other.rows) || (self.cols != other.cols) {
      Err("Subtraction not defined for matrices of different dimensions")
    }

    else {
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
}

impl<T: Integer + Clone + Copy> Mul for Matrix<T> {
  type Output = Result<Self, &'static str>;

  fn mul(self, other: Self) -> Result<Self, &'static str> {
    if (self.rows != other.rows) || (self.cols != other.cols) {
      Err("Wrong dimensions for multiplication")
    }
    else {
      todo!();
    }
  } 
}

impl<T: Integer + std::fmt::Display + Clone + Copy> fmt::Display for Matrix<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut text: String = String::from("");
    text.push_str("Matrix:\n");
    
    for i in 0..self.rows {
      text.push_str("[ ");
      for j in 0..self.cols {
        let current_num = self.matrix[j + (i * self.cols)];
        let s = format!("{} ", current_num);
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

impl<T: Integer + num::NumCast + Clone + Copy> Matrix<T> {
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

    MatrixIterator { matrix: self, idx: 0 }
  }

  pub fn transpose(&self) -> Matrix<T> {
    let mut trans: Vec<T> = Vec::new();
    for i in 0..self.rows {
      for j in 0..self.cols {
        trans.push(self.matrix[i + (j * self.rows)]);
      }
    }

    Matrix {rows: self.cols, cols: self.rows, matrix: trans}
  }

  pub fn t(&self) -> Matrix<T> {

    self.transpose()
  }

  //pub fn raw(rows: usize, columns: usize)  -> Self {
  //} 
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