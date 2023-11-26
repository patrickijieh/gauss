use crate::{Matrix, FloatMatrix};

pub fn int_identity(size: usize) -> Matrix<u8> {
  let mut identity: Matrix<u8> = Matrix::new(size, size);

  for i in 0..size {
    identity[(i, i)] = 1;

  }
  
  identity
}

pub fn float_identity(size: usize) -> FloatMatrix<f32> {
  let mut identity: FloatMatrix<f32> = FloatMatrix::new(size, size);

  for i in 0..size {
    identity[(i, i)] = 1.0;

  }
  
  identity
}