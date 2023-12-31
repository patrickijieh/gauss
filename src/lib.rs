// pub mod matrix;
// pub mod float_matrix;
pub mod macros;

// pub use matrix::{*};
// pub use float_matrix::{*};
pub use macros::{*};

pub use gauss_core::matrix::{*};
pub use gauss_core::float_matrix::{*};

pub use gauss_macros::example;

/*pub fn matrix<T: Float>(width: i32, height: i32) -> Option<Matrix<T>> {
  let mat = matrix![f32; 1, 2];
  matrix::matrix_creation(width, height)
}*/
