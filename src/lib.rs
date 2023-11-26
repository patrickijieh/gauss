pub mod matrix;
pub mod float_matrix;
pub mod macros;
pub mod common;

pub use matrix::{*};
pub use float_matrix::{*};
pub use macros::{*};
pub use common::{*};

/*pub fn matrix<T: Float>(width: i32, height: i32) -> Option<Matrix<T>> {
  let mat = matrix![f32; 1, 2];
  matrix::matrix_creation(width, height)
}*/
