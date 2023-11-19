use num::{Float, NumCast};
pub struct Matrix<T: Float> {
  rows: u32,
  cols: u32,
  matrix: Vec<T>
}

impl From<Matrix<f32>> for Matrix<f64> {

  fn from(m: Matrix<f32>) -> Self {
    let mut data: Vec<f64> = Vec::new();

    for i in m.matrix {
      data.push(i.into());
    }

    Matrix { rows: m.rows, cols: m.cols, matrix: data }
  }

}

impl<T: Float> Matrix<T> {

  fn create_matrix(rows: i32, columns: i32) -> Option<Self> {

    if rows < 1 && columns < 1 {
      return None;
    }

    let new_width: u32 = columns.try_into().unwrap();
    let new_height: u32 = rows.try_into().unwrap();
    let mut matrix: Vec<T> = Vec::new();

    for _ in 0..(new_width * new_height) {
      matrix.push(NumCast::from(0.0).unwrap());
    }

    Some(Matrix {rows: new_height, cols: new_width, matrix: matrix})
  }
}

pub fn matrix<T: Float>(width: i32, height: i32) -> Option<Matrix<T>> {

  Matrix::create_matrix(width, height)
}

#[macro_export]
macro_rules! matrix {
  [ $type:ty; $rows:expr, $cols:expr ] => {
    {
      let m: Matrix<$type> = matrix($rows, $cols).unwrap();

      m
    }
  };

  [ $rows:expr, $cols:expr ] => {
    {
      let m: Matrix<f32> = matrix($rows, $cols).unwrap();

      m.into()
    }
  }
}