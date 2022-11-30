//#![allow(dead_code, unused_imports, unused_mut, unused_variables)]
#![allow(dead_code)]

mod matrix;
use matrix::Matrix;

fn main() {
    // A
    let a01: Vec<f32> = vec![1., 3., 3., 1.];
    let a02: Vec<f32> = vec![1., 2., 6. , 3.];
    let a03: Vec<f32> = vec![1., 5., 8., 2.];
    let a04: Vec<f32> = vec![1., 2., 2., 0.];
    let a = Matrix::new(&vec![a01, a02, a03, a04]);
    
    // B
    let b01: Vec<f32> = vec![1., 0.];
    let b02: Vec<f32> = vec![1., 6.];
    let b = Matrix::new(&vec![b01, b02]);

    // C
    let c01: Vec<f32> = vec![0., 9.];
    let c02: Vec<f32> = vec![3., 7.];
    let c = Matrix::new(&vec![c01, c02]);

    // Transposition
    let mat = a.transpose();
    println!("Transposed:\n{}", mat);
    
    // Matrix addition
    match b.add(&c) {
        Ok(mat) => println!("Addition:\n{}", mat),
        Err(_) => println!("can't add")
    }

    // Matrix multiplication
    match b.mult(&c) {
        Ok(mat) => println!("Multiplication:\n{}", mat),
        Err(_) => println!("can't mult")
    }
}