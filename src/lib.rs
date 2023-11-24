pub mod matrix;
pub mod macros;
pub use matrix::{*};
pub use macros::{*};

/*pub fn matrix<T: Float>(width: i32, height: i32) -> Option<Matrix<T>> {
  let mat = matrix![f32; 1, 2];
  matrix::matrix_creation(width, height)
}*/
