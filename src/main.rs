use matrix::Matrix;

fn main() {
    // A
    let a01: Vec<f32> = vec![1., 3., 3., 1.];
    let a02: Vec<f32> = vec![1., 2., 6. , 3.];
    let a03: Vec<f32> = vec![1., 5., 8., 2.];
    let a04: Vec<f32> = vec![1., 2., 2., 0.];
    let a = match Matrix::new(&vec![a01, a02, a03, a04]) {
        Ok(mat) => mat,
        Err(e) => panic!("{:?}", e)
    };
    
    // B
    let b01: Vec<f32> = vec![1., 0.];
    let b02: Vec<f32> = vec![1., 6.];
    let b = match Matrix::new(&vec![b01, b02]) {
        Ok(mat) => mat,
        Err(e) => panic!("{:?}", e)
    };

    // C
    let c01: Vec<f32> = vec![0., 9.];
    let c02: Vec<f32> = vec![3., 7.];
    let c = match Matrix::new(&vec![c01, c02]) {
        Ok(mat) => mat,
        Err(e) => panic!("{:?}", e)
    };

    // Transposition
    match a.transpose() {
        Ok(mat) => println!("Transposed:\n{}", mat),
        Err(_) => {},
    };
    
    
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