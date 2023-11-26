
/// Creates a [FloatMatrix] with the arguments.
/// 
/// `float_mat!` allows you to build and initialize a matrix of size `[rows, columns]`:
/// 
/// ```
/// use gauss as gs;
/// use gs::{FloatMatrix, float_mat};
/// 
/// let mat: FloatMatrix<f32> = float_mat![2, 2];
/// for val in mat {
///     assert_eq!(val, 0.0);
/// }
/// ```
/// 
/// The macro also allows you to build a matrix with a list of elements, demarcating each row with a semi-colon:
/// 
/// ```
/// use gauss as gs;
/// use gs::{FloatMatrix, float_mat};
/// 
/// let mat: FloatMatrix<f32> = float_mat![
///     1.0, 2.0, 3.0;
///     4.0, 5.0, 6.0;
///     7.0, 8.0, 9.0; 
/// ];
/// 
/// println!("{}", mat);
/// 
/// // Matrix at addr 0xccccfed00:
/// // [ 1.0000 2.0000 3.0000 ]
/// // [ 4.0000 5.0000 6.0000 ]
/// // [ 7.0000 8.0000 9.0000 ]
/// 
/// ```
#[macro_export]
macro_rules! float_mat {
  [ $rows:literal, $cols:literal ] => {
    {
      let m: FloatMatrix<f32> = FloatMatrix::new($rows, $cols);

      m.into()
    }
  };

  [$($($x:literal),*;)*] => {
    {
      let mut temp = Vec::new();
      let mut row_size: usize = 0;
      let mut col_size: usize = 0;
      $(
        let mut temp_col_size: usize = 0;
        $(
          temp.push($x);
          temp_col_size += 1;
        )*
        if (col_size != temp_col_size) && (col_size != 0) {
          panic!("Inconsistent matrix columns");
        }
        col_size = temp_col_size;
        row_size += 1;
      )*

      FloatMatrix::from_1d_vec(temp, row_size, col_size)
    }
  };
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
  };
}