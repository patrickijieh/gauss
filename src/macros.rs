
/// Creates a floating-point matrix of size `[rows, columns]`.
#[macro_export]
macro_rules! float_mat {
  [ $rows:literal, $cols:literal ] => {
    {
      let m: FloatMatrix<f32> = FloatMatrix::new($rows, $cols);

      m.into()
    }
  }
}

/// Creates an integer matrix of size `[rows, columns]`.
#[macro_export]
macro_rules! mat {
  [ $rows:literal, $cols:literal ] => {
    {
      todo!();
      let u: Matrix<u8> = Matrix::new($rows, $cols);
      u.into()
    }
  }
}