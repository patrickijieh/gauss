
use gauss as gs;
use gs::example;
use gs::{Matrix, FloatMatrix};
use gs::float_mat;
// use gs::mat;

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

  vec!(1, 2, 3);
  let _vector: Vec<i32> = Vec::new();
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

  let mut mat6: FloatMatrix<f64> = FloatMatrix::new(3, 2);
  let mut mat7: FloatMatrix<f64> = FloatMatrix::new(2, 3);

  (mat6[(0, 0)], mat7[(0, 0)]) = (1.2, 1.2);
  (mat6[(0, 1)], mat7[(0, 1)]) = (2.3, 2.3);
  (mat6[(1, 0)], mat7[(1, 0)]) = (3.4, 3.4);
  (mat6[(1, 1)], mat7[(1, 1)]) = (4.5, 4.5);
  mat6[(2, 0)] = 5.6;
  mat6[(2, 1)] = 6.7;
  mat7[(0, 2)] = 5.6;
  mat7[(1, 2)] = 6.7;

  let mat8 = mat6.clone();
  let mat9 = (mat6 * mat7).unwrap();

  println!("{}\n", mat9);
  println!("mat8:{}\nmat8.t():{}", mat8, mat8.t());

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
    print!("{:.4} ", val);
  }
  println!();

  println!("Float Test finished!");
}

#[test]
fn macro_test() {
  let mat1: FloatMatrix<f32> = float_mat![2, 2];
  // let mat2: Matrix<i32> = mat![2, 2];

  let mat3: FloatMatrix<f64> = float_mat![
    1.0, 2.0, 3.0;
    4.0, 5.0, 6.0;
    7.0, 8.0, 9.0;
  ];
  
  println!("{}\nsize: {:?}", mat3, mat3.size());
  println!("{}", mat1);
  println!("done!");

  // let mat4: Matrix<i64> = mat![
  //   1, 2, 3;
  //   4, 5, 6;
  //   7, 8, 9;
  // ];

  example![1, 2, 3];

}

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

#[test]
fn rref_test() {
  let mat1: FloatMatrix<f32> = float_mat![
    6.0, 2.0, 4.0, 5.0;
    1.0, 3.0, 2.0, 5.0;
    4.0, 8.0, 12.0, 5.0;
  ];

  println!("{}", mat1);

  let mat2 = mat1.rref();

  println!("{}", mat2);
}

#[test]
fn common_test() {
  let i: FloatMatrix<f32> = FloatMatrix::<f32>::identity(3);
  let j = Matrix::<u8>::identity(3);
  println!("{i}\n{j}");
}