pub mod matrix;
use crate::matrix::{*};
pub fn fun() {
  let mut _macro_matrix: Matrix<f64> = matrix![1, 2];
  let mut _matrix: Matrix<f64> = matrix::matrix(3, 4).unwrap();
}
