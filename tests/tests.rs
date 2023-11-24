
use gauss as gs;
use gs::{Matrix, FloatMatrix};
use gs::float_mat;
use gs::mat;

#[test]
fn float_test() {

  // Test 1
  let mut mat1: FloatMatrix<f64> = FloatMatrix::new(2, 2);
  let mat2: FloatMatrix<f64> = float_mat![2, 2];
  println!("{}", mat1);
  println!("{}", mat2);
  mat1[(0, 0)] = 1.2;
  mat1[(0, 1)] = 2.3;
  mat1[(1, 0)] = 3.4;
  mat1[(1, 1)] = 4.5;
  println!("{}", mat1);

  // vec!(1, 2, 3);
  // let mut vector: Vec<i32> = Vec::new();
  // vector.push(1);
  // let val = vector.get(0);
  // let res = val.unwrap().to_owned();

  // Test 2
  let mat3 = mat1.transpose();
  println!("{}", mat3);
  let sum = (mat1 + mat3).unwrap();
  println!("{}", sum);

  // Test 3
  let mat4: FloatMatrix<f64> = FloatMatrix::new(1, 1);
  let mat5: FloatMatrix<f64> = FloatMatrix::new(1, 2);
  match mat4 + mat5 {
    Ok(_) => panic!("Test 3 failed"),
    Err(e) => println!("{}\nTest 3 passed", e),
  }

  // Test 4
  let epochs = 1000;
  for i in 1..=epochs {
    let temp_1: FloatMatrix<f32> = FloatMatrix::new(i, i);
    let temp_2: FloatMatrix<f32> = FloatMatrix::new(i, i);
    let temp_3 = temp_1 + temp_2;
    match temp_3 {
      Ok(_) => {}
      Err(_) => panic!("Addition was undefined at epoch {}", i),
    }
    if i % (epochs / 10) == 0 {
      println!("Epoch {} has been completed...", i);
    }
  }

  // Test 5
  print!("Sum matrix: ");
  for val in sum {
    print!("{} ", val);
  }
  println!();

  println!("Float Test finished!");
}
// #[test]
// fn macro_test() {
//   let mat1: FloatMatrix<f32> = float_mat![2, 2];
//   let mat2: Matrix<i32> = mat![2, 2];

//   let mat3: FloatMatrix<f64> = float_mat![
//     1.0, 2.0, 3.0;
//     4.0, 5.0, 6.0;
//     7.0, 8.0, 9.0;
//   ];

//   let mat4: Matrix<i64> = mat![
//     1, 2, 3;
//     4, 5, 6;
//     7, 8, 9;
//   ];
// }

#[test]
fn integer_test() {
  let mut mat1: Matrix<i32> = Matrix::new(2, 2);
  mat1[(0,0)] = 1;
  mat1[(0,1)] = 2;
  mat1[(1, 0)] = 3;
  mat1[(1, 1)] = 4;

  println!("{}", mat1);

  //let mat2: Matrix<i32> = mat![2, 2];
  //println!("{}", mat2);

  //let mat3 = mat2 + mat1;
}